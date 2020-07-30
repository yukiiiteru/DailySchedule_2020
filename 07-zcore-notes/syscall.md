# zCore 笔记：syscall

## Day 28

根据 zCore Summer of Code 的日程，下一步应该了解的是 Zircon 的系统调用了

下面列一下所有的系统调用

* Handle（句柄）
  * `handle_close` - 关闭一个句柄
  * `handle_close_many` - 关闭多个句柄
  * `handle_duplicate` - 创建句柄副本（可选项为缩减的权限值）
  * `handle_replace` - 创建新句柄（可选项为缩减的权限值），并销毁旧句柄
* Object（对象）
  * `object_get_child` - 通过 koid 查找某个对象的子对象
  * `object_get_info` - 获取对象的信息
  * `object_get_property` - 读取对象的属性值
  * `object_set_profile` - 将 profile 应用到线程
  * `object_set_property` - 修改对象属性值
  * `object_signal` - 设置或清除某个对象上的用户信号
  * `object_signal_peer` - 设置或清除相反对等方向上的用户信号
  * `object_wait_many` - 等待多个对象上的信号产生
  * `object_wait_one` - 等待单个对象上的信号产生
  * `object_wait_async` - 在信号改变时进行异步通知
* Thread（线程）
  * `thread_create` - 在进程中创建新线程
  * `thread_exit` - 退出当前线程
  * `thread_read_state` - 从线程中读取寄存器状态
  * `thread_start` - 运行新线程
  * `thread_write_state` - 修改线程的寄存器状态
* Process（进程）
  * `process_create` - 在作业中创建新进程
  * `process_read_memory` - 从进程的地址空间中读取数据
  * `process_start` - 启动新进程
  * `process_write_memory` - 向进程的地址空间中写入数据
  * `process_exit` - 退出当前进程
* Job（作业）
  * `job_create` - 在作业中创建新的子作业
  * `job_set_critical` - 将进程设为对作业 critical (重要？)
  * `job_set_policy` - 改变作业和它的子作业的策略
* Task（任务），包括线程, 进程和作业
  * `task_create_exception_channel` - 在任务上创建异常通道
  * `task_kill` - 终止任务的运行
  * `task_suspend` - 将一个任务中断挂起
* Profile
  * `profile_create` - 创建一个新的 profile 对象
* Exception（异常）
  * `exception_get_thread` - 为异常线程创建一个句柄
  * `exception_get_process` - 为异常进程创建一个句柄
* Channel（通道）
  * `channel_call` - 同步发送消息并接受响应
  * `channel_call_etc` - 同步发送消息并接受响应并返回句柄信息
  * `channel_create` - 创建新 channel
  * `channel_read` - 从 channel 中读取消息
  * `channel_read_etc` - 从 channel 中读取消息并返回句柄信息
  * `channel_write` - 向channel中读取消息
  * `channel_write_etc` - 向 channel 中读取消息并返回句柄信息
* Socket（套接字）
  * `socket_create` - 创建 socket
  * `socket_read` - 从 socket 中读数据
  * `socket_shutdown` - 阻止读写
  * `socket_write` - 向 socket 写入数据
* Stream（流）
  * `zx_stream_create()` - 从VMO创建流
  * `zx_stream_readv()` - 从流中以当前寻道偏移量读取数据
  * `zx_stream_readv_at()` - 从流中读取给定偏移量的数据
  * `zx_stream_writev()` - 以当前寻道偏移量将数据写入流
  * `zx_stream_writev_at()` - 以给定的偏移量将数据写入流
  * `zx_stream_seek()` - 修改流的当前寻道偏移量
* Fifo（先进先出队列）
  * `fifo_create` - 创建队列
  * `fifo_read` - 从队列中读取数据
  * `fifo_write` - 写入数据到队列中
* Event 和 Event Pair
  * `event_create` - 创建Event对象
  * `eventpair_create` - 创建一对Event Pair
  * `system_get_event` - 检索系统事件的句柄
* Port（端口）
  * `port_create` - 创建端口
  * `port_queue` - 向端口发送数据包
  * `port_wait` - 等待端口接收数据包
  * `port_cancel` - 取消获取来自 `async_wait` 的通知
* Futex
  * `futex_wait` - 等待futex变成可用
  * `futex_wake` - 唤醒futex的等待者
  * `futex_requeue` - 唤醒futex的一些等待者，并将其他等待者重新加入队列中
* 虚拟内存对象（VMO）
  * `vmo_create` - 创建 VMO
  * `vmo_read` - 读取 VMO
  * `vmo_write` - 写入 VMO
  * `vmo_create_child` - 创建子 VMO
  * `vmo_clone` - 关闭 VMO
  * `vmo_get_size` - 获取 VMO 的大小
  * `vmo_set_size` - 调整 VMO 的大小
  * `vmo_op_range` - 在 VMO 的一段区域内执行操作
  * `vmo_replace_as_executable` - 为 VMO 添加执行权限
  * `vmo_create_physical` - 创建一个 VM 对象，该对象引用物理内存的特定连续范围
  * `vmo_set_cache_policy` - 为 VMO 保留的页面设置缓存策略
* 虚拟动态内存对象（VMAR）
  * `vmar_allocate` -创建子 VMAR
  * `vmar_map` - 将 VMO 对象映射到某个进程中
  * `vmar_unmap` - 将某个内存区域从进程中取消映射
  * `vmar_protect` - 调整内存访问的权限
  * `vmar_op_range` - 对映射到某个 VMAR 范围内的 VMO 执行操作
  * `vmar_destroy` - 销毁 VMAR 和它所有的子代 VMAR
* Userspace Pagers
  * `pager_create` - 创建一个新的 pager 对象
  * `pager_create_vmo` - 创建一个拥有 VMO 的 pager
  * `pager_detach_vmo` - 从 VMO 分离 pager
  * `pager_supply_pages` - 将 page 供应到 pager 拥有的 VMO 中
  * `pager_op_range` - 在拥有 pager 的 VMO 的范围上执行操作
* Cryptographically Secure RNG（加密安全的随机数发生器）
  * `cprng_draw`
  * `cprng_add_entropy`
* Time（时间）
  * `nanosleep` - 休眠一段时间（以 ns 为单位）
  * `clock_get` - 读取系统时钟
  * `clock_get_monotonic` - 读取 monotonic 系统时钟
  * `ticks_get` - 读取高精度计时器 tick 数
  * `ticks_per_second` - 读取一秒时间内高精度计时器 tick 数
  * `deadline_after` - 将相对于现在的时间转换为绝对 deadline
  * `clock_adjust`
* Timer（计时器）
  * `timer_create` - 创建计时器对象
  * `timer_set` - 启动计时器
  * `timer_cancel` - 取消计时器
* Hypervisor guests（虚拟机监视器管理的客户机）
  * `guest_create` - 创建客户虚拟机
  * `guest_set_trap` - 在客户虚拟机机中设置陷入中断
* Virtual CPUs（虚拟 CPU）
  * `vcpu_create` - 创建虚拟cpu
  * `vcpu_resume` - 恢复虚拟cpu的运行
  * `vcpu_interrupt` - 在虚拟cpu上触发中断
  * `vcpu_read_state` - 读取虚拟cpu的状态
  * `vcpu_write_state` - 向虚拟cpu写入状态信息
  * `interrupt_bind_vcpu` - 将中断对象绑定到虚拟 CPU
* Global system information（全局系统信息）
  * `system_get_dcache_line_size`
  * `system_get_features` - 获取硬件相关功能
  * `system_get_num_cpus` - 获取CPU核数
  * `system_get_physmem` - 获取物理内存大小
  * `system_get_version` - 获取版本信息字符串
* Debug Logging（日志）
  * `debuglog_create` - 创建内核托管的日志读写器
  * `debuglog_write` - 向日志项中写入日志
  * `debuglog_read` - 从日志项中读取日志
  * `debug_read` - TODO(fxbug.dev/32938)
  * `debug_write` - TODO(fxbug.dev/32938)
  * `debug_send_command` - TODO(fxbug.dev/32938)
* Multi-function（操作合并函数）
  * `vmar_unmap_handle_close_thread_exit` - 三合一操作（取消 VMAR 映射，关闭句柄和退出线程）
  * `futex_wake_handle_close_thread_exit` - 三合一操作（唤醒 futex，关闭句柄和退出线程）
* System（系统）
  * `system_mexec` - 使用新的内核和引导映像软重启系统
  * `system_mexec_payload_get` - 返回包含启动该系统所需的ZBI条目的ZBI
  * `system_powerctl`
* DDK
  * `bti_create` - create a new bus transaction initiator
  * `bti_pin` - pin pages and grant devices access to them
  * `bti_release_quarantine` - releases all quarantined PMTs
  * `cache_flush` - Flush CPU data and/or instruction caches
  * `interrupt_ack` - Acknowledge an interrupt object
  * `interrupt_bind` - Bind an interrupt object to a port
  * `interrupt_create` - Create a physical or virtual interrupt object
  * `interrupt_destroy` - Destroy an interrupt object
  * `interrupt_trigger` - Trigger a virtual interrupt object
  * `interrupt_wait` - Wait on an interrupt object
  * `iommu_create` - create a new IOMMU object in the kernel
  * `pmt_unpin` - unpin pages and revoke device access to them
  * `resource_create` - create a resource object
  * `smc_call` - Make an SMC call from user space
* Display drivers（显示驱动）
  * `framebuffer_get_info`
  * `framebugger_set_range`
* Tracing
  * `ktrace_control`
  * `ktrace_read`
  * `ktrace_write`
  * `mtrace_control`
* Others/Work in progress

  略

这些系统调用看名字和介绍就能大概知道功能，不过感觉最新的英文文档和两年前的中文文档差距还是有点大的，看了看 `zircon-syscall` 里的代码，感觉应该是比较新的

然后...就卡在这了，接下来没有文档可以读，只能硬啃代码了qwq
