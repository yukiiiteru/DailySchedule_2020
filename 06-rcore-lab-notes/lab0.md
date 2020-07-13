# Lab 0 实验记录

上个月的时候做过一遍Lab0了，但是没有写实验记录，于是决定删掉再重新做一遍。

这里就不记录详细的实验过程了，跟实验指导书差不多的内容再重复写一遍也没什么意思，所以只记录我学到的东西好了。

## 移除标准库依赖

感觉Rust这门语言是真的严谨，标准库里带着对`panic`的处理以及堆栈展开的处理，很贴心了，所以禁用掉标准库之后要自己实现一遍。

堆栈展开就比较麻烦，而且rCore是作为一个操作系统，如果发生了什么异常而不进行堆栈展开直接结束的话也不会发生内存泄露，因为rCore本身就是在`qemu`模拟器中运行的，所以堆栈展开直接禁用掉就好了。

## 移除运行时环境依赖

做个笔记：

`#[no_mangle]`可以使编译的时候编译器不会为了实现重载而把函数名字改得乱七八糟

定义函数时在`fn`前添加`extern "C"`可以将其声明为C函数

感觉这两点在以后可能会用得到，比如Rust跟汇编互相调用

## 生成内核镜像

`objdump -x`可以查看程序的元信息，`objdump -d`可以对程序进行反汇编

`objcopy`在uCore中也有用到过，转换成`binary`格式一方面可以直接运行，另一方面可以有效缩小程序尺寸，这样才能使uCore的bootloader缩小到512字节以内的。好吧有点扯远了

## 调整内存布局

> 一般来说，一个程序按照功能不同会分为下面这些段：
>
> * .text 段：代码段，存放汇编代码
> * .rodata 段：只读数据段，顾名思义里面存放只读数据，通常是程序中的常量
> * .data 段：存放被初始化的可读写数据，通常保存程序中的全局变量
> * .bss 段：存放被初始化为 0 的可读写数据，与 .data 段的不同之处在于我们知道它要被初始化为 0，因此在可执行文件中只需记录这个段的大小以及所在位置即可，而不用记录里面的数据，也不会实际占用二进制文件的空间
> * Stack：栈，用来存储程序运行过程中的局部变量，以及负责函数调用时的各种机制。它从高地址向低地址增长
> * Heap：堆，用来支持程序运行过程中内存的动态分配，比如说你要读进来一个字符串，在你写程序的时候你也不知道它的长度究竟为多少，于是你只能在运行过程中，知道了字符串的长度之后，再在堆中给这个字符串分配内存

有关Linker Script可以参考[Scripts (LD)](https://sourceware.org/binutils/docs/ld/Scripts.html)

OpenSBI 将自身放在 0x80000000，完成初始化后会跳转到 0x80200000

## 重写程序入口点 `_start`

CPU加电后会先进行自检，通过自检后会跳到bootloader的入口（`0x80000000`）

RISC-V不需要再编写bootloader，RISC-V有现成的bootloader实现：OpenSBI固件

`entry.asm`的内容很重要，要记下来：

```riscvasm
# 操作系统启动时所需的指令以及字段
#
# 我们在 linker.ld 中将程序入口设置为了 _start，因此在这里我们将填充这个标签
# 它将会执行一些必要操作，然后跳转至我们用 rust 编写的入口函数
#
# 关于 RISC-V 下的汇编语言，可以参考 https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md

    .section .text.entry
    .globl _start
# 目前 _start 的功能：将预留的栈空间写入 $sp，然后跳转至 rust_main
_start:
    la sp, boot_stack_top
    call rust_main

    # 回忆：bss 段是 ELF 文件中只记录长度，而全部初始化为 0 的一段内存空间
    # 这里声明字段 .bss.stack 作为操作系统启动时的栈
    .section .bss.stack
    .global boot_stack
boot_stack:
    # 16K 启动栈大小
    .space 4096 * 16
    .global boot_stack_top
boot_stack_top:
    # 栈结尾
```

然后在Rust中使用`global_asm!(include_str!("entry.asm"));`来引入汇编代码

## 使用 QEMU 运行内核

新版 QEMU 中内置了 OpenSBI 固件，它主要负责在操作系统运行前的硬件初始化和加载操作系统的功能。

按`Ctrl A`然后按`x`退出QEMU。

这里`Makefile`直接复制会出现问题，把各命令前面的空格替换成TAB就好了。

## 接口封装和代码整理

OpenSBI的系统调用：

| Function Name                | Function ID | Extension ID | Replacement Extension |
| ---------------------------- | ----------- | ------------ | --------------------- |
| `sbi_set_timer`              |           0 |         0x00 |                   N/A |
| `sbi_console_putchar`        |           0 |         0x01 |                   N/A |
| `sbi_console_getchar`        |           0 |         0x02 |                   N/A |
| `sbi_clear_ipi`              |           0 |         0x03 |                   N/A |
| `sbi_send_ipi`               |           0 |         0x04 |                   N/A |
| `sbi_remote_fence_i`         |           0 |         0x05 |                   N/A |
| `sbi_remote_sfence_vma`      |           0 |         0x06 |                   N/A |
| `sbi_remote_sfence_vma_asid` |           0 |         0x07 |                   N/A |
| `sbi_shutdown`               |           0 |         0x08 |                   N/A |
| *RESERVED*                   |             |    0x09-0x0F |                       |

[OpenSBI文档](https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc)

> `a7` (or `t0` on RV32E-based systems) encodes the SBI extension ID, which matches how the system call ID is encoded in the standard UNIX system call ABI.
>
> SBI functions must return a pair of values in a0 and a1, with a0 returning an error code. This is analogous to returning the C structure
>
> ```c
>     struct sbiret {
>         long error;
>         long value;
>     };
> ```

这里我注意到OpenSBI的系统调用返回值有两个，但rCore只接收了第一个返回值，即错误代码。

错误代码的含义就先不在这里列出了。

此外，Rust的format字符串功能是内置的，并不在标准库里面，这就很舒服了。
