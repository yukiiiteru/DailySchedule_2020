# 实验二：内存分配

## 实验题

1. 回答：我们在动态内存分配中实现了一个堆，它允许我们在内核代码中使用动态分配的内存，例如 `Vec` `Box` 等。那么，如果我们在实现这个堆的过程中使用 `Vec` 而不是 `[u8]`，会出现什么结果？

   理论上：会陷入死循环，堆的初始化需要使用堆，使用堆需要将堆初始化。

   实际上：我还不知道如何实现将其陷入互相调用的死循环。

2. 实验

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

3. 挑战实验（选做）

   等Lab全部看完后再写
