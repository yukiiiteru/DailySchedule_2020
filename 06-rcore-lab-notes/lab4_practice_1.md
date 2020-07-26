# 实验四（上）：线程

## 实验题

1. 原理：线程切换之中，页表是何时切换的？页表的切换会不会影响程序 / 操作系统的运行？为什么？

   回答：时钟中断触发时调用 `PROCESSOR.get().prepare_next_thread()`，在该函数内执行 `next_thread.prepare()`，然后执行 `self.process.write().memory_set.activate()` 激活页表，标志着切换成功

   页表切换不会影响程序运行，因为保存的上下文可以恢复，寄存器及堆栈不会发生变化，可以正常执行

   页表切换不会影响操作系统的运行，因为页表切换是发生在操作系统内的，而每个进程的页表都映射了操作系统内的部分空间，如中断处理部分

   参考答案：

   > 页表是在 `Process::prepare_next_thread()` 中调用 `Thread::prepare()`，其中换入了新线程的页表。
   >
   > 它不会影响执行，因为在中断期间是操作系统正在执行，而操作系统所用到的内核线性映射是存在于每个页表中的。

2. 设计：如果不使用 `sscratch` 提供内核栈，而是像原来一样，遇到中断就直接将上下文压栈，请举出（思路即可，无需代码）：

   * 一种情况不会出现问题
   * 一种情况导致异常无法处理（指无法进入 `handle_interrupt`）
   * 一种情况导致产生嵌套异常（指第二个异常能够进行到调用 `handle_interrupt`，不考虑后续执行情况）
   * 一种情况导致一个用户进程（先不考虑是怎么来的）可以将自己变为内核进程，或以内核态执行自己的代码

   回答：

   * 程序不操作 `sp` 寄存器
   * 程序把 `sp` 寄存器丢失
   * 不知道
   * 不知道

   参考答案：

   > * 只运行一个非常善意的线程，比如 `loop {}`
   > * 线程把自己的 `sp` 搞丢了，比如 `mv sp, x0`。此时无法保存寄存器，也没有能够支持操作系统正常运行的栈
   > * 运行两个线程。在两个线程切换的时候，会需要切换页表。但是此时操作系统运行在前一个线程的栈上，一旦切换，再访问栈就会导致缺页，因为每个线程的栈只在自己的页表中
   > * 用户进程巧妙地设计 `sp`，使得它恰好落在内核的某些变量附近，于是在保存寄存器时就修改了变量的值。这相当于任意修改操作系统的控制信息

   第三个没想到，这个是不应该的，第四个还是不太理解

3. 实验：当键盘按下 Ctrl + C 时，操作系统应该能够捕捉到中断。实现操作系统捕获该信号并结束当前运行的线程（你可能需要阅读一点在实验指导中没有提到的代码）

   从 Lab6 将开启外部中断的代码迁移过来，然后在 `supervisor_external` 函数内处理 Ctrl C 按键即可

   刚开始不知道 Ctrl C 的 key code，于是在中断的地方 `println` 了一下，得知其 key code 为 `3`，于是在此添加判断，然后 `kill_current_thread` 即可

4. 实验：实现线程的 `fork()`。目前的内核线程不能进行系统调用，所以我们先简化地实现为“按 F 进入 fork”。fork 后应当为目前的线程复制一份几乎一样的拷贝，新线程与旧线程同属一个进程，公用页表和大部分内存空间，而新线程的栈是一份拷贝。

   这个有点难，按 f 进入 `fork` 做完了，但是正确 `fork` 还是有点难度的。我 `fork` 出的线程总是能执行一会，但是执行一会之后就会因为 `LoadPageFault` 或者 `StorePageFault` 而被杀死

   研究了半天，发现我犯了个低级错误，`sp` 寄存器的值没有加上偏移，浪费了我半天时间（哭

   总算是完成了，地址：[实验四（上）代码](06-rcore-lab-notes/lab4_practice_1)

5. （实验4修改版）实验：实现进程的 fork()。目前的内核线程不能进行系统调用，所以我们先简化地实现为“按 F 进行 fork”。fork 后应当为目前的进程复制一份几乎一样的拷贝。

   这个失败了，但是还是放一下实现过程吧，给其他人作参考

   1. 修改 `process.rs` 中的 `ProcessInner`，为其添加线程：

      ```rust
      pub struct ProcessInner {
          /// 进程中的线程公用页表 / 内存映射
          pub memory_set: MemorySet,
          /// 打开的文件描述符
          pub descriptors: Vec<Arc<dyn INode>>,
          /// 进程中的所有线程
          pub threads: Vec<Arc<Thread>>,
      }
      ```

   2. 修改 `processor.rs` 中的 `add_thread` 和 `kill_current_thread`，在添加线程和杀死线程的时候对 `threads` 进行操作，代码略

   3. 写 `fork`，失败代码如下：

      ```rust
      /// fork 进程
      pub fn fork(&self, context: &Context) -> MemoryResult<Arc<Self>> {
          let process = Arc::new(Self {
              id: unsafe {
                  PROCESS_COUNTER += 1;
                  PROCESS_COUNTER
              },
              is_user: self.is_user,
              inner: Mutex::new(ProcessInner {
                  memory_set: MemorySet::new_kernel()?,
                  descriptors: Vec::new(),
                  threads: Vec::new(),
              }),
          });
          for inode in self.inner().descriptors.iter() {
              process.inner().descriptors.push(inode.clone());
          }
          let cur_thread = PROCESSOR.lock().current_thread();
          let inner = self.inner();
          let memset = &inner.memory_set;
          for thread in inner.threads.iter() {
              let stack = process.alloc_page_from(
                  thread.stack.start.0,
                  STACK_SIZE,
                  Flags::READABLE | Flags::WRITABLE
              )?;
              // TODO: How to copy thread's stack?
              process.inner().memory_set.activate();
              let pa_to = Mapping::lookup(stack.start).unwrap();
              memset.activate();
              for p in 0..STACK_SIZE {
                  *PhysicalAddress(pa_to.0 + p).deref_kernel::<u8>()
                      = *VirtualAddress(thread.stack.start.0 + p).deref::<u8>();
              }
              let new_context =
                  if let Some(cur_context) = thread.inner().context { cur_context.clone() } else { context.clone() };
              let sleeping = thread.inner().sleeping;
              let dead = thread.inner().dead;
              let new_thread = Arc::new(Thread {
                  id: unsafe {
                      THREAD_COUNTER += 1;
                      THREAD_COUNTER
                  },
                  stack,
                  process: process.clone(),
                  inner: Mutex::new(ThreadInner {
                      context: Some(new_context),
                      sleeping,
                      dead,
                  }),
              });
              PROCESSOR.lock().add_thread(new_thread);
          }
          Ok(process)
      }
      ```

      其它过程不多解释了，重点是复制线程栈的部分，为了复制过去我用了最麻烦的方法：先激活新进程的页表，获取新线程栈的物理地址，再切会就进程的页表，然后把旧线程的栈用虚拟地址复制给新线程栈的物理地址

      不知道为什么会失败，部分报错内容为：

      ```rust
      Thread {
          thread_id: 0x14,
          stack: Range {
              start: VirtualAddress(
                  0x1200000,
              ),
              end: VirtualAddress(
                  0x1280000,
              ),
          },
          context: None,
      } terminated: unimplemented interrupt type
      cause: Exception(LoadPageFault), stval: 0
      Thread {
          thread_id: 0x10,
          stack: Range {
              start: VirtualAddress(
                  0x1000000,
              ),
              end: VirtualAddress(
                  0x1080000,
              ),
          },
          context: None,
      } terminated: unimplemented interrupt type
      cause: Exception(StorePageFault), stval: 107fe58
      ```

      `stval` 为 0 的我怀疑是内存没复制好的问题，但是下面这个不知道有什么问题
