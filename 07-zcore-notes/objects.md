# zCore 笔记：object

## Day 25

从 `zCore/src/main.rs` 跟踪到 `zircon-loader/src/main.rs`，然后到 `zircon-object/src/lib.rs`，进一步跟踪到 `zircon-object/src/object/mod.rs`，找到了 zCore-Tutorial 里介绍的对象，代码几乎完全一致，就从这开始好了

* 第一步，在 `zircon-object/src/object/mod.rs` 中创建了 `KObjectBase` 对象：

  ```rust
  /// The base struct of a kernel object.
  pub struct KObjectBase {
      /// The object's KoID.
      pub id: KoID,
      inner: Mutex<KObjectBaseInner>,
  }

  /// The mutable part of `KObjectBase`.
  #[derive(Default)]
  struct KObjectBaseInner {
      name: String,
      signal: Signal,
      signal_callbacks: Vec<SignalHandler>,
  }
  ```

  之后的所有对象（如进程）都*算是* `KObjectBase` 的子类，不过在 Rust 中没有继承，而是使用了 trait `KernelObject` 和宏 `impl_kobject` 来模拟继承

* 第二步，创建对象管理器：Process 对象

  * 参考 zCore-Tutorial 的目录，首先建立了句柄的概念，代码位于 `zircon-object/src/object/handle.rs`：
  
    ```rust
    /// A Handle is how a specific process refers to a specific kernel object.
    #[derive(Debug, Clone)]
    pub struct Handle {
        /// The object referred to by the handle.
        pub object: Arc<dyn KernelObject>,
        /// The handle's associated rights.
        pub rights: Rights,
    }
    ```
  
    按照字面意思，句柄是进程引用对象的标识符，它指向一个内核对象，还包含一个权限
  
    权限定义在 `zircon-object/src/object/rights.rs` 中，这里不再细讲
  
  * 下一项工作的代码位于 `zircon-object/src/task/process.rs`，在这里建立了进程的概念：
  
    ```rust
    pub struct Process {
        base: KObjectBase,
        _counter: CountHelper,
        job: Arc<Job>,
        policy: JobPolicy,
        vmar: Arc<VmAddressRegion>,
        ext: Box<dyn Any + Send + Sync>,
        exceptionate: Arc<Exceptionate>,
        debug_exceptionate: Arc<Exceptionate>,
        inner: Mutex<ProcessInner>,
    }
  
    #[derive(Default)]
    struct ProcessInner {
        status: Status,
        max_handle_id: u32,
        handles: HashMap<HandleValue, (Handle, Vec<Sender<()>>)>,
        futexes: HashMap<usize, Arc<Futex>>,
        threads: Vec<Arc<Thread>>,
  
        // special info
        debug_addr: usize,
        dyn_break_on_load: usize,
        critical_to_job: Option<(Arc<Job>, bool)>,
    }
    ```

    > Zircon 的进程是传统意义上程序的实例：由一个或多个线程执行的一组指令以及相关的资源集合组成。
    >
    > A zircon process is an instance of a program in the traditional sense: a set of instructions which will be executed by one or more threads, along with a collection of resources.
  
    进程中包含许多信息，现在先不深究了吧
  
  * 进程的第一件工作是存储内核对象句柄：
  
    ```rust
    impl ProcessInner {
        /// Add a handle to the process
        fn add_handle(&mut self, handle: Handle) -> HandleValue {
            // FIXME: handle value from ptr
            let key = (self.max_handle_id << 2) | 0x3u32;
            info!("add handle: {:#x}, {:?}", key, handle.object);
            self.max_handle_id += 1;
            self.handles.insert(key, (handle, Vec::new()));
            key
        }
    }
    ```
  
    进程还包含 虚拟内存地址区域(Virtual Memory Address Regions) 和 线程(Threads)，这个暂时先不讨论
  
  * 进程的下一项工作是根据句柄查找内核对象，这一步没有找到具体的代码，但是结合上述内容很容易理解：
  
    一个句柄对应着一个对象，将句柄存储到进程的 `handlers` 中，使用 `HandleValue` 作为标识符，根据 `HandleValue` 可以找到 `Handle` 进而找到其对应的 `KernelObject`

* 第三步：创建对象传送器：Channel 对象

  * Channel 对象位于 `zircon-object/src/ipc/channel.rs`
  
    这里的 IPC 指的是进程间通信(Inter-Process Communication)，而 Channel 是一种可以使进程间进行双向通信的对象：

    ```rust
    /// Bidirectional interprocess communication
    pub struct Channel {
        base: KObjectBase,
        _counter: CountHelper,
        peer: Weak<Channel>,
        recv_queue: Mutex<VecDeque<T>>,
        call_reply: Mutex<HashMap<TxID, Sender<ZxResult<T>>>>,
        next_txid: AtomicU32,
    }
    ```

    > Channel 维护了一个有序的消息队列，以便在任一方向上传递由一些数据和句柄组成的消息。
    >
    > Channels maintain an ordered queue of messages to be delivered in either direction. A message consists of some amount of data and some number of handles.

    用于进程间通信(IPC)的对象还有 Socket, FIFO，在这里先略过

    Channel 首先需要创建一对内核对象，这样才可以在这一对内核对象之间传输信息：

    ```rust
    impl Channel {
        /// Create a channel and return a pair of its endpoints
        #[allow(unsafe_code)]
        pub fn create() -> (Arc<Self>, Arc<Self>) {
            let mut channel0 = Arc::new(Channel {
                base: KObjectBase::with_signal(Signal::WRITABLE),
                _counter: CountHelper::new(),
                peer: Weak::default(),
                recv_queue: Default::default(),
                call_reply: Default::default(),
                next_txid: AtomicU32::new(0x8000_0000),
            });
            let channel1 = Arc::new(Channel {
                base: KObjectBase::with_signal(Signal::WRITABLE),
                _counter: CountHelper::new(),
                peer: Arc::downgrade(&channel0),
                recv_queue: Default::default(),
                call_reply: Default::default(),
                next_txid: AtomicU32::new(0x8000_0000),
            });
            // no other reference of `channel0`
            unsafe {
                Arc::get_mut_unchecked(&mut channel0).peer = Arc::downgrade(&channel1);
            }
            (channel0, channel1)
        }
    }
    ```

    这一步看代码很容易理解，不再赘述
  
  * 下一项任务则是实现数据传输了

    数据传输相关的方法好多，有 `check_and_read`，`read`，`write`，`call`，`push_general`

    * `check_and_read`：读数据，顺便加上一个 `checker` 函数
    * `read`：读数据，`checker` 函数已经写好了，不用再写
    * `write`：发送一个数据包
    * `call`：发送一条消息并等待回应
    * `push_general`：将消息发送到队列，然后 called from peer. （这句话没看懂，翻译了一下，结果是：“从对等方调用”，还是不太懂）

    **20200729补充**：我觉得 `push_general` 应该是“被 `peer` 调用”，`peer` 不用翻译

## Day 26

zCore-Tutorial 文档第二章 “任务管理” 的第一节是 “Zircon 任务管理体系”，然而这个 zCore-Tutorial 文档还没有写，而 Fuchsia 文档里也没有介绍，只有具体的对 `Process`, `Thread`, `Job`, `Task` 的介绍

先啃 `Job` 和 `Task` 吧

* Zircon 任务管理体系

  * `Job`：控制一组进程

    > 概要：作业是一组进程，以及其它可能的（子）作业的集合。作业用于追踪用于执行内核操作的权限（即使用各种选项进行各种系统调用），并追踪和限制基本资源（例如，内存和CPU）的消耗。 每个进程都属于一个作业中。作业也可以相互嵌套，除根作业之外的每个作业也属于某个单个（父）作业。
    >
    > 描述：作业是由以下内容组成的对象
    >
    > * 指向父作业的引用
    > * 一组子作业（每个子作业都以本作业作为其父作业）
    > * 一组进程
    > * 一组策略（未实现）
    >
    > 作业控制“应用程序”由作为单个项被控制的多个进程所组成。

    以下为 `Job` 的定义：

    ```rust
    #[allow(dead_code)]
    pub struct Job {
        base: KObjectBase,
        _counter: CountHelper,
        parent: Option<Arc<Job>>,
        parent_policy: JobPolicy,
        exceptionate: Arc<Exceptionate>,
        debug_exceptionate: Arc<Exceptionate>,
        inner: Mutex<JobInner>,
    }

    #[derive(Default)]
    struct JobInner {
        policy: JobPolicy,
        children: Vec<Arc<Job>>,
        processes: Vec<Arc<Process>>,
        // if the job is killed, no more child creation should works
        killed: bool,
        timer_policy: TimerSlack,
    }
    ```

    可以看到，在 `JobInner` 里面包含了一个 `processes: Vec<Arc<Process>>` 的结构，但是我并没有找到有关运行进程或线程的功能，看来如概要所说，只能进行追踪和限制资源消耗了，而限制功能应该是由 `policy` 来管理的
  
  * `Task`：内核对象中的“可运行”子类（包括线程，进程和作业）

    > 概要：线程，进程和作业对象都是任务类型。他们都具有被挂起，恢复和终止的能力。

    以下为 `Task` 的定义：

    ```rust
    /// Task (Thread, Process, or Job)
    pub trait Task: Sync + Send {
        /// Kill the task.
        fn kill(&self);

        /// Suspend the task. Currently only thread or process handles may be suspended.
        fn suspend(&self);

        /// Resume the task
        fn resume(&self);
    }
    ```

    看起来 `Task` 只是一个抽象的 `trait`，并不是一个具体的 `struct`，而具有该 `trait` 的对象则具有被挂起、恢复、终止的功能

* 硬件抽象层与异步运行时

  这个指的是 `zCore/kernel-hal-bare` 和 `zCore/kernel-hal-unix` 吧，没有文档可读，暂时先跳过

* 线程管理：Thread 对象

  > 线程，即可运行/计算实体
  >
  >描述：线程对象是表示分时CPU执行上下文的概念。 线程对象与特定的进程对象相关联，进程为其他对象所需的I/O和计算提供内存和句柄。
  >
  > 线程的生命周期始于 `Thread::start()` 或 `Process::start()`，而终止的方式有：
  >
  > * 调用 `Thread::exit()`
  > * 父进程终止
  > * 调用 `Task::kill()`
  > * 产生异常
  > * (Fuchsia) 通过调用 `zx_vmar_unmap_handle_close_thread_exit()`
  > * (Fuchsia) 通过调用 `zx_futex_wake_handle_close_thread_exit()`

  线程的定义为：

  ```rust
  pub struct Thread {
      base: KObjectBase,
      _counter: CountHelper,
      proc: Arc<Process>,
      ext: Box<dyn Any + Send + Sync>,
      inner: Mutex<ThreadInner>,
      exceptionate: Arc<Exceptionate>,
  }

  #[derive(Default)]
  struct ThreadInner {
      /// Thread context
      ///
      /// It will be taken away when running this thread.
      context: Option<Box<UserContext>>,

      /// The number of existing `SuspendToken`.
      suspend_count: usize,
      /// The waker of task when suspending.
      waker: Option<Waker>,
      /// A token used to kill blocking thread
      killer: Option<Sender<()>>,
      /// Thread state
      ///
      /// NOTE: This variable will never be `Suspended`. On suspended, the
      /// `suspend_count` is non-zero, and this represents the state before suspended.
      state: ThreadState,
      /// The currently processing exception
      exception: Option<Arc<Exception>>,
      /// The time this thread has run on cpu
      time: u128,
      flags: ThreadFlag,
  }
  ```

  下面还定义了 `impl Task for Thread {}`，表明 `Thread` 是一种 `Task`

* 进程管理：Process 与 Job 对象

  `Process` 与 `Job` 在上面都讲过了，但是不知道怎么结合起来进行进程管理...

  毕竟 `Job` 没有启动进程的功能，只有添加删除进程的功能，如 `add_process`，`remove_process`，可能这就是所谓的进程管理...?

下面是 zCore-Tutorial 第三章的内容

* Zircon 内存管理模型

  这一节大概就是介绍 VMO 对象 和 VMAR 对象吧，不知道该怎么大体介绍，放到后面几节详细讲讲

* 物理内存：VMO 对象

  > Virtual Memory Object（虚拟内存对象）
  >
  > 名称：vm_object —— 虚拟内存的容器抽象
  >
  > 概要：虚拟内存对象（VMO）表示可以映射到多个地址空间的虚拟内存连续区域。
  >
  > 描述：内核和用户空间使用VMO来表示分页和物理内存。它们是在进程之间以及内核和用户空间之间共享内存的标准方法。

  代码位于 `zircon-object/vm/vmo/mod.rs`，其定义为：

  ```rust
  pub struct VmObject {
      base: KObjectBase,
      parent: Mutex<Weak<VmObject>>, // Parent could be changed
      children: Mutex<Vec<Weak<VmObject>>>,
      _counter: CountHelper,
      resizable: bool,
      inner: Arc<dyn VMObjectTrait>,
  }
  ```

  还定义了一个 `VMObjectTrait` 的特性，太长了就先不放代码了

  然后...道理我都懂，但是管理*物理内存*的对象为什么要叫作*虚拟内存对象*？是把物理内存抽象出来的吗？

  在 `zircon-object/vm/vmo/physical.rs` 中定义了 `VMObjectPhysical`：

  ```rust
  /// VMO representing a physical range of memory.
  pub struct VMObjectPhysical {
      paddr: PhysAddr,
      pages: usize,
      /// Lock this when access physical memory.
      data_lock: Mutex<()>,
      inner: Mutex<VMObjectPhysicalInner>,
  }
  ```

  该结构体实现了 `VMObjectTrait` 特性，可以放入 `VmObject` 中的 `inner`，是跟这个有关吗？

  还有，在 `zircon-object/vm/vmo/paged.rs` 中定义了 `VMObjectPaged`：

  ```rust
  /// The main VM object type, holding a list of pages.
  pub struct VMObjectPaged {
      /// The lock that protected the `inner`
      /// This lock is shared between objects in the same clone tree to avoid deadlock
      lock: Arc<Mutex<()>>,
      inner: RefCell<VMObjectPagedInner>,
  }
  ```

* 虚拟内存：VMAR 对象

  明天再写  

## Day 27

* 虚拟内存：VMAR 对象

  > Virtual Memory Address Region（虚拟内存地址区域）
  >
  > 名称：vm_address_region —— 虚拟内存地址空间的连续区域
  >
  > 概要：虚拟内存地址区域（VMAR）表示虚拟地址空间的连续部分。
  >
  > 描述：内核和用户空间使用VMAR来表示地址空间的分配。

  文件位于 `zircon-object/src/vm/vmar.rs`，其定义为：

  ```rust
  /// Virtual Memory Address Regions
  pub struct VmAddressRegion {
      flags: VmarFlags,
      base: KObjectBase,
      _counter: CountHelper,
      addr: VirtAddr,
      size: usize,
      parent: Option<Arc<VmAddressRegion>>,
      page_table: Arc<Mutex<dyn PageTableTrait>>,
      /// If inner is None, this region is destroyed, all operations are invalid.
      inner: Mutex<Option<VmarInner>>,
  }

  /// The mutable part of `VmAddressRegion`.
  #[derive(Default)]
  struct VmarInner {
      children: Vec<Arc<VmAddressRegion>>,
      mappings: Vec<Arc<VmMapping>>,
  }
  ```

  下面还定义有 `VmMapping`，表示虚拟内存映射：

  ```rust
  /// Virtual Memory Mapping
  pub struct VmMapping {
      flags: MMUFlags,
      vmo: Arc<VmObject>,
      page_table: Arc<Mutex<dyn PageTableTrait>>,
      inner: Mutex<VmMappingInner>,
  }

  #[derive(Debug, Clone)]
  struct VmMappingInner {
      addr: VirtAddr,
      size: usize,
      vmo_offset: usize,
  }
  ```

  这两者都引用了 `kernel_hal`(内核-硬件抽象层) 中的 `PageTableTrait`，即页表

  代码读到这里，内存相关的结构就有点清晰了，VMO 负责管理物理内存和物理页，VMAR 负责管理虚拟内存和页表映射
  
  （虽然还是不太明白管理内存的为什么叫作 VMO...
