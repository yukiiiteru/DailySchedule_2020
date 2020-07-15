# Lab 4 实验记录

## 线程的创建

**思考：为什么线程即便与操作系统无关，也需要在内存中映射操作系统的内存空间呢？**

回答：处理中断？

参考答案：

> 当发生中断时，需要跳转到 `stvec` 所指向的中断处理过程。如果操作系统的内存不在页表之中，将无法处理中断。
>
> 当然，也不是所有操作系统的代码都需要被映射，但是为了实现简便，我们会为每个进程的页表映射全部操作系统的内存。而由于这些页表都标记为**内核权限**（即 `U` 位为 0），也不必担心用户线程可以随意访问。

**思考：`__restore` 现在会将 `a0` 寄存器视为一个 `*mut Context` 来读取，因此我们在执行第一个线程时只需调用 `__restore(context)`。那么，如果是程序发生了中断，执行到 `__restore` 的时候，`a0` 的值又是谁赋予的呢？**

回答：`handle_interrupt` 的返回值类型也是 `*mut Context`，而该返回值的地址是存在 `a0` 寄存器中的

参考答案：

> 当发生中断时，在 `__restore` 时，`a0` 寄存器的值是 `handle_interrupt` 函数的返回值。也就是说，如果我们令 `handle_interrupt` 函数返回另一个线程的 `*mut Context`，就可以在时钟中断后跳转到这个线程来执行。

## 线程的切换

**思考：在 `run` 函数中，我们在一开始就激活了页表，会不会导致后续流程无法正常执行？**

回答：~~不会，因为在页表中有虚拟地址 `0x80000000` 到物理地址 `0x80000000` 的映射~~（答非所问）

20200715回答：现在理解了，线程切换是在操作系统内发生的，而每一个进程的 `MemorySet` 都会映射这部分操作系统的空间，所以切换页表并不影响内核中代码的运行

参考答案：

> 不会，因为每一个进程的 `MemorySet` 都会映射操作系统的空间，否则在遇到中断的时候，将无法执行异常处理。

没太看懂

## 内核栈

**思考：sscratch 的值最初是在什么地方写入的？**

回答：在 OpenSBI 固件中，[OpenSBI/lib/sbi/sbi_hart.c](https://github.com/riscv/opensbi/blob/2314101989684585f942b50a827aac4886825ba1/lib/sbi/sbi_hart.c)，`sbi_hart_switch_mode` 函数内

```c
    if (next_mode == PRV_S) {
        csr_write(CSR_STVEC, next_addr);
        csr_write(CSR_SSCRATCH, 0);
        csr_write(CSR_SIE, 0);
        csr_write(CSR_SATP, 0);
    } else if (next_mode == PRV_U) {
        csr_write(CSR_UTVEC, next_addr);
        csr_write(CSR_USCRATCH, 0);
        csr_write(CSR_UIE, 0);
    }
```

根据函数名即变量名可以得知，在 `hart` 要切换 MODE 时，如果要切换到 S 态，那么将 `sscratch` 寄存器内写入 0。

而题目问的是最初，那么最初写入是在 bootloader (OpenSBI) 加载完毕，将控制权交给操作系统时这一特权切换过程中，在 OpenSBI 内写入

**思考：a0 是在哪里被赋值的？（有两种情况）**.

1. 执行第一个线程时直接调用 `__restore(context)`，`a0` 作为传入参数被赋值

2. 程序发生中断，执行完 `handle_interrupt` 后，`a0` 作为返回值被赋值

**思考：在栈的切换过程中，会不会导致一些栈空间没有被释放，或者被错误释放的情况？**

回答：切换过程中可能会有栈未被释放，但切换回该上下文后可以继续释放

## 分析：进程启动的全过程

1. 新建一个带有内核映射的进程，从 `main.rs` 中的 `rust_main` 进入 `src/process/process.rs` 中的 `Process::new_kernel` 函数

   ```rust
   let process = Process::new_kernel().unwrap();
   ```

   ```rust
   pub fn new_kernel() -> MemoryResult<Arc<RwLock<Self>>> {
       Ok(Arc::new(RwLock::new(Self {
           is_user: false,
           memory_set: MemorySet::new_kernel()?,
       })))
   }
   ```

2. 进入 `src/memory/mapping/memory_set.rs` 中的 `MemorySet::new_kernel` 函数，为进程创建 `MemorySet`

   `MemorySet` 包含了一个进程中关于内存空间管理的所有信息

   该部分在 Lab3 中详细介绍过，这里不再赘述

3. 进程创建完毕后，则在 `rust_main` 中为该进程创建线程：

   ```rust
   for message in 0..8 {
       let thread = Thread::new(
           process.clone(),            // 使用同一个进程
           sample_process as usize,    // 入口函数
           Some(&[message]),           // 参数
       ).unwrap();
       PROCESSOR.get().add_thread(thread);
   }
   ```

   其中，线程的定义为：

   ```rust
   /// 线程的信息
   pub struct Thread {
       /// 线程 ID
       pub id: ThreadID,
       /// 线程的栈
       pub stack: Range<VirtualAddress>,
       /// 所属的进程
       pub process: Arc<RwLock<Process>>,
       /// 用 `Mutex` 包装一些可变的变量
       pub inner: Mutex<ThreadInner>,
   }

   /// 线程中需要可变的部分
   pub struct ThreadInner {
       /// 线程执行上下文
       ///
       /// 当且仅当线程被暂停执行时，`context` 为 `Some`
       pub context: Option<Context>,
       /// 是否进入休眠
       pub sleeping: bool,
   }
   ```

   而线程创建的过程为：

   ```rust
   /// 创建一个线程
   pub fn new(
       process: Arc<RwLock<Process>>,
       entry_point: usize,
       arguments: Option<&[usize]>,
   ) -> MemoryResult<Arc<Thread>> {
       // 让所属进程分配并映射一段空间，作为线程的栈
       let stack = process
           .write()
           .alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;

       // 构建线程的 Context
       let context = Context::new(
           stack.end.into(),
           entry_point,
           arguments,
           process.read().is_user,
       );

       // 打包成线程
       let thread = Arc::new(Thread {
           id: unsafe {
               THREAD_COUNTER += 1;
               THREAD_COUNTER
           },
           stack,
           process,
           inner: Mutex::new(ThreadInner {
               context: Some(context),
               sleeping: false,
           }),
       });

       Ok(thread)
   }
   ```

   首先让所属进程分配并映射一段空间，作为线程的栈，然后构建线程的上下文：

   ```rust
   /// 为线程构建初始 `Context`
   pub fn new(
       stack_top: usize,
       entry_point: usize,
       arguments: Option<&[usize]>,
       is_user: bool,
   ) -> Self {
       let mut context = Self::default();

       // 设置栈顶指针
       context.set_sp(stack_top).set_ra(-1isize as usize);
       // 设置初始参数
       if let Some(args) = arguments {
           context.set_arguments(args);
       }
       // 设置入口地址
       context.sepc = entry_point;

       // 设置 sstatus
       context.sstatus = sstatus::read();
       if is_user {
           context.sstatus.set_spp(User);
       } else {
           context.sstatus.set_spp(Supervisor);
       }
       // 这样设置 SPIE 位，使得替换 sstatus 后关闭中断，
       // 而在 sret 到用户线程时开启中断。详见 SPIE 和 SIE 的定义
       context.sstatus.set_spie(true);

       context
   }
   ```

   构建完成后，将所有的参数打包为线程，打包完成后即可返回该线程

4. 线程创建完成后，为全局的 `Processor` 添加该线程

   ```rust
   PROCESSOR.get().add_thread(thread);
   ```

   其中，`Processor` 负责系统内线程的调度和管理：

   ```rust
   /// 线程调度和管理
   ///
   /// 休眠线程会从调度器中移除，单独保存。在它们被唤醒之前，不会被调度器安排。
   #[derive(Default)]
   pub struct Processor {
       /// 当前正在执行的线程
       current_thread: Option<Arc<Thread>>,
       /// 线程调度器，记录活跃线程
       scheduler: SchedulerImpl<Arc<Thread>>,
       /// 保存休眠线程
       sleeping_threads: HashSet<Arc<Thread>>,
   }
   ```

   而 `add_thread` 的实现为：

   ```rust
   /// 添加一个待执行的线程
   pub fn add_thread(&mut self, thread: Arc<Thread>) {
       if self.current_thread.is_none() {
           self.current_thread = Some(thread.clone());
       }
       self.scheduler.add_thread(thread, 0);
   }
   ```

   它将当前线程替换为 `Processor` 的 `current_thread`，然后将其添加到调度器

5. 在将线程启动前，先把多余的 `process` 引用丢弃掉：

   ```rust
   // 把多余的 process 引用丢弃掉
   drop(process);
   ```

   因为启动线程的过程是不会返回的，也就是说，在 `rust_main` 函数中 `process` 不会自动被释放

6. 启动线程：

   ```rust
   PROCESSOR.get().run();
   ```

   其中，`Processor::run` 的实现为：

   ```rust
   /// 第一次开始运行
   ///
   /// 从 `current_thread` 中取出 [`Context`]，然后直接调用 `interrupt.asm` 中的 `__restore`
   /// 来从 `Context` 中继续执行该线程。
   ///
   /// 注意调用 `run()` 的线程会就此步入虚无，不再被使用
   pub fn run(&mut self) -> ! {
       // interrupt.asm 中的标签
       extern "C" {
           fn __restore(context: usize);
       }
       // 从 current_thread 中取出 Context
       if self.current_thread.is_none() {
           panic!("no thread to run, shutting down");
       }
       let context = self.current_thread().prepare();
       // 从此将没有回头
       unsafe {
           __restore(context as usize);
       }
       unreachable!()
   }
   ```

   它将 `current_thread` 这一线程的上下文取出，然后恢复到寄存器

   其中 `sepc` 寄存器的值为中断返回后的地址，而该地址在上下文中被替换为线程入口地址

   当 `__restore` 执行 `sret` 指令，函数跳转到 `sample_process`，线程启动

   ```rust
   // 从此将没有回头
   ```

## 分析：线程是如何终止的

1. 在创建线程时，设置线程的上下文，这里将线程入口函数的返回值地址（`ra` 寄存器的值） 设为了一个不存在的地址：

   ```rust
   // 设置栈顶指针
   context.set_sp(stack_top).set_ra(-1isize as usize);
   ```

2. 线程指向的函数运行结束，执行 `jr ra` 跳转到这一不存在的地址，并引发异常

3. 异常处理：

   ```rust
   /// 出现未能解决的异常，终止当前线程
   fn fault(_context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
       println!(
           "{:x?} terminated with {:x?}",
           PROCESSOR.get().current_thread(),
           scause.cause()
       );
       println!("stval: {:x}", stval);
       PROCESSOR.get().kill_current_thread();
       // 跳转到 PROCESSOR 调度的下一个线程
       PROCESSOR.get().prepare_next_thread()
   }
   ```

   该函数杀死当前线程，即，将线程从调度器中移除，然后让 `Processor` 调度下一个线程

   至此，线程生命周期结束
