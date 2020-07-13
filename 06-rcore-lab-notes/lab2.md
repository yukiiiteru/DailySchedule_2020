# Lab 2 实验记录

## 动态内存分配

（前面几个Lab的代码还可以直接用的，最多加几句`use`语句就好，这里的缺少好多

### 思考题1

动态分配的内存地址在哪个范围里？

> 回答：~~动态分配的内存被分配到了堆中。~~（**错误**）
>
> 标准答案：在 `.bss` 段中，因为我们用来存放动态分配的这段是一个静态的没有初始化的数组，算是内核代码的一部分。

`.bss`段的作用：

> .bss 段：存放被初始化为 0 的可读写数据，与 .data 段的不同之处在于我们知道它要被初始化为 0，因此在可执行文件中只需记录这个段的大小以及所在位置即可，而不用记录里面的数据，也不会实际占用二进制文件的空间

## 物理内存管理

### 思考题2

运行下面的代码：

```rust
/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // 初始化各种模块
    interrupt::init();
    memory::init();

    // 物理页分配
    match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
    };

    panic!()
```

思考，和上面的代码有何不同，我们的设计是否存在一些语法上的设计缺陷？

回答：

> 会导致死锁
>
> 原因：`match`语句上了锁，在代码块结束的时候`frame_tracker`的生命周期结束试图释放，等待锁结束，而`match`语句也在等待内部代码执行完毕，即发生了死锁
>
> 用`gdb`调试可以发现，代码停在了`match`语句的右花括号位置，QEMU在不停输出时钟中断的信号，程序依然原地不动

参考答案：

> 这里的 `frame_tracker` 变量会在 `match` 语法里面析构。但是析构的时候，外层的 `lock()` 函数还没有释放锁，这样写会导致死锁。

意思相同，但是答案写得更简练。

## 分析

1. 如果不初始化内存会发生什么？

   将 `memort::init()` 注释掉，然后使用 `gdb` 从 `let v = Box::new(5);` 开始跟踪：

   1. 执行到 `rustlib/src/rust/liballoc/boxed.rs`，`Box::new` 函数，直接返回 `box x`

   2. 进入到 `alloc::alloc::exchange_malloc` （路径略），开始申请内存

   3. 执行 `exchange_malloc` 函数：

      ```rust
      #[cfg(not(test))
      #[lang = "exchange_malloc"]
      #[inline]
      unsafe fn exchange_malloc(size: usize, align: usize) -> *mut u8 {
          let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
          match Global.alloc(layout, AllocInit::Uninitialized) {
              Ok(memory) => memory.ptr.as_ptr()
              Err(_) => handle_alloc_error(layout)
          }
      }
      ```

      首先获取 `layout`

   4. 进入 `core::alloc::layout::Layout::from_size_align_unc` 执行 `Layout { size_: size, align_: NonZeroUsize::new_unchecked(align) }` 以创建 `layout`

   5. 进入 `core::num::NonZeroUsize::new_unchecked` 创建一个非零值传给 `layout`

   6. 创建 `layout` 成功，返回到 `alloc::alloc::exchange_malloc`，执行 `match Global.alloc(layout, AllocInit::Uninitialized)`，开始申请内存

   7. 进入 `core::alloc::AllocRef`，这是一个为 `alloc::alloc::Global` 实现的 `impl`，并开始执行 `alloc`：

      ```rust
      fn alloc(&mut self, layout: Layout, init: AllocInit) -> Result<MemoryBlock, AllocError> {
          unsafe {
              let size = layout.size();
              if size == 0 {
                  Ok(MemoryBlock { ptr: layout.dangling(), size: 0 })
              } else {
                  let raw_ptr = match init {
                      AllocInit::Uninitialized => alloc(layout),
                      AllocInit::Zeroed => alloc_zeroed(layout),
                  };
                  let ptr = NonNull::new(raw_ptr).ok_or(AllocErr)?;
                  Ok(MemoryBlock { ptr, size })
              }
          }
      }
      ```

      此时 `size = 4`，进入 `else` 分支

      传入的 `init` 为 `core::alloc::AllocInit::Uninitialized`，进入 `match`语句第一条分支

   8. 进入 `alloc::alloc::alloc`，执行 `alloc` 函数：

      ```rust
      #[stable(feature = "global_alloc", since = "1.28.0")]
      #[inline]
      pub unsafe fn alloc(layout: Layout) -> *mut u8 {
          unsafe { __rust_alloc(layout.size(), layout.align()) }
      }
      ```

      正常执行

      **注**：这里一直定位不到`__rust_alloc`的具体实现，查询得知该函数使用 `extern` 语句引入，但没找到引入来源

      ~~个人理解是进入到了`buddy_system`的`src/frame.rs`，调用`FrameAllocator::alloc`来申请堆内存~~

      之前的理解应该是错误的，我在`src/memory/heap.rs`中，`static HEAP`前发现了 `#[global_allocator]` 标记，它可以为全局需要用到堆的地方分配空间

   9. 返回到 `core::alloc::AllocRef` 的 `alloc` 函数，为 `ptr` 赋值

   10. 进入 `core::ptr::non_null::NonNull`，执行 `new` 函数：

       ```rust
       #[stable(feature = "nonnull", since = "1.25.0")]
       #[inline]
       pub fn new(ptr: *mut T) -> Option<Self> {
           if !ptr.is_null() {
               // SAFETY: The pointer is already checked and is not null
               Some(unsafe { Self::new_unchecked(ptr) })
           } else {
               None
           }
       }
       ```

       此时传入的 `ptr = (*mut u8) 0x0`，是一个空值，返回 `Null`

   11. `let ptr = NonNull::new(raw_ptr).ok_or(AllocErr)?;` 语句抛出异常，申请内存失败

   12. 返回到 `alloc::alloc::exchange_malloc`，进入 `match` 语句的 `Err` 分支

   13. 进入 `src/memory/heap.rs` 文件内的 `alloc_error_handler`，发生恐慌，输出错误信息并关机

2. 正确初始化内存后会发生什么？

   跟上述过程类似，但在执行 `Global.alloc` 时可以得到非零的 `raw_ptr`，从而正确执行

3. 内存初始化的流程？

   1. 从 `rust_main` 执行 `memory::init()`

   2. 进入 `os::memory::init`执行 `heap::init()`：

      ```rust
      /// 初始化操作系统运行时堆空间
      pub fn init() {
          // 告诉分配器使用这一段预留的空间作为堆
          unsafe {
              HEAP.lock()
                  .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
          }
      }
      ```

   3. 进入 `buddy_system_allocator::Heap::add_to_heap` 的 `add_to_heap` 函数：

      ```rust
      /// Add a range of memory [start, end) to the heap
      pub unsafe fn add_to_heap(&mut self, mut start: usize, mut end: usize) {
          // avoid unaligned access on some platforms
          start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
          end = end & (!size_of::<usize>() + 1);
          assert!(start <= end);
  
          let mut total = 0;
          let mut current_start = start;
  
          while current_start + size_of::<usize>() <= end {
              let lowbit = current_start & (!current_start + 1);
              let size = min(lowbit, prev_power_of_two(end - current_start));
              total += size;
  
              self.free_list[size.trailing_zeros() as usize].push(current_start as *mut usize);
              current_start += size;
          }
  
          self.total += total;
      }
      ```

      这段代码的核心即，为 `free_list` 链表添加 `current_start`，执行完后内存初始化就完成了
