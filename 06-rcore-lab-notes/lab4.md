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
