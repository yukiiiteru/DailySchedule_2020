# 实验二：内存分配

## 实验题

1. 回答：.bss 字段是什么含义？为什么我们要将动态分配的内存（堆）空间放在 .bss 字段？

   回答：按照常规，.bss 段是存储程序中静态变量的，而且可以保证其初值为 0；但是在 rCore 操作系统中，.bss 段被作为堆使用。具体原因我也不知道

   参考文献：[关于 Lab2 动态内存分配 一节中思考题的探讨](https://github.com/rcore-os/rCore-Tutorial/issues/69)

   参考答案：

   > 对于一个 ELF 程序文件而言，.bss 字段一般包含全局变量的名称和长度，在执行时由操作系统分配空间并初始化为零。
   >
   > *不过，在我们执行 `rust-objcopy` 时，不同的字段会相应地被处理而形成一段连续的二进制数据，这段二进制数据会直接写入到 QEMU 所模拟的机器的 `0x80200000` 位置。这是因为我们写的操作系统是直接运行在机器上的，而不是一个被操作系统加载的程序。*
   >
   > 我们一般遇到应用程序的动态内存分配（堆）是由其操作系统提供的。例如在 C 语言中的 `malloc()`，glibc 运行库会维护一个堆空间，而这个空间是通过 `brk()` 等系统调用向内核索要的。由于我们编写操作系统，自然就无法像这样获取空间。但是此时我们具有随意使用内存空间的权力，因此我们可以在内存中随意划一段空间，然后用相应的算法来实现一个堆。
   >
   > 至于为何堆在 .bss 字段，实际上这也不是必须的——我们完全可以随意指定一段可以访问的内存空间。不过，在代码中用全局变量来表示堆并将其放在 .bss 字段，是一个很简单的实现：这样堆空间就包含在内核的二进制数据之中了，而自 `KERNEL_END_ADDRESS` 以后的空间就都可以给进程使用。

2. 回答：我们在动态内存分配中实现了一个堆，它允许我们在内核代码中使用动态分配的内存，例如 `Vec` `Box` 等。那么，如果我们在实现这个堆的过程中使用 `Vec` 而不是 `[u8]`，会出现什么结果？

   理论上：会陷入死循环，堆的初始化需要使用堆，使用堆需要将堆初始化。

   实际上：我还不知道如何实现将其陷入互相调用的死循环。

   **20200719补充**：我已经在20200717已经成功实现了（哭

3. 实验

   1. 回答：`algorithm/src/allocator` 下有一个 `Allocator trait`，我们之前用它实现了物理页面分配。这个算法的时间和空间复杂度是什么？

      `Allocator`是一个`trait`，本身没有复杂度，要根据具体实现判断

      * `BitmapVectorAllocator`：时间O(n)，空间O(n)

      * `StackedAllocator`：时间O(1)，空间O(n)

      参考答案：

      > 时间复杂度是 O(1)，空间复杂度是 O(n)

   2. 实现基于线段树的物理页面分配算法

      参考文献：

      * [使用线段树实现简单的内存管理](https://blog.csdn.net/feng964497595/article/details/100080920)
      * [mufeng964497595 / seg_tree_memory_management](https://github.com/mufeng964497595/seg_tree_memory_management)
      * [P3372 【模板】线段树 1](https://www.luogu.com.cn/record/24986607)
      * [rcore-os / buddy_system_allocator](https://github.com/rcore-os/buddy_system_allocator)

      此外，我也参考了 [@yunwei37](https://github.com/yunwei37) 仓库中的链表实现，在 `Rc` 中添加 `RefCell` 也是受该仓库的启发，感谢云微同学

      代码地址：~~基于线段树实现的 buddy system~~

      **20200718补充**：突然翻车

      **20200719补充**：昨天的其实也不太算翻车，理论上可行，实践上不可行，代码重新放出来吧

      代码地址：

      * [基于线段树实现的 `buddy system` - 数组版本](06-rcore-lab-notes/lab2_practice/segment_tree_allocator_array)
      * [基于线段树实现的 `buddy system` - 指针版本](06-rcore-lab-notes/lab2_practice/segment_tree_allocator_pointer)

      这两种版本都可以通过 `cargo test`，但接入真正的操作系统中后会出现各种 bug，数组版本空间不足，指针版本会死循环

4. 挑战实验（选做）

   等Lab全部看完后再写
