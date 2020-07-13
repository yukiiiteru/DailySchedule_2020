# Daily Schedule for OS Tutorial Summer of Code 2020

## TOC

七月

|       Mon       |       Tue       |       Wed       |       Thu       |       Fri       |       Sat       |       Sun       |
|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|
|                 |                 |        1        |        2        |   3([D0][D0])   |   4([D1][D1])   |   5([D2][D2])   |
|   6([D3][D3])   |   7([D4][D4])   |   8([D5][D5])   |   9([D6][D6])   |  10([D7][D7])   |  11([D8][D8])   |  12([D9][D9])   |
| 13              | 14              | 15              | 16              | 17              | 18              | 19              |
| 20              | 21              | 22              | 23              | 24              | 25              | 26              |
| 27              | 28              | 29              | 30              |                 |                 |                 |

---

到目前为止完成的内容（括号内为完成时间）：

* [(Day 2) Rust by Example](01-rust-by-example)
* [(Day 3) 《Rust编程之道》第10章 模块化编程](02-modular-programing)
* [(Day 3) rustlings exercises](03-rustlilngs-exercises)
* [(Day 4) Learn Rust the Hard Way](04-learn-rust-the-hard-way)
* [(Day 4) Leetcode by Rust](05-leetcode)
* (Day 5) 计算机组成与设计：RISC-V 浙大在线课程 前三章
* (Day 6) 《RISC-V手册》 第一、二、三、四、五、六、七、十章
* (Day 7) RISC-V特权指令规范 第一、二、四章
* [(Day 7) Lab 0](06-rcore-lab-notes/lab0.md)
* [(Day 8) Lab 1](06-rcore-lab-notes/lab1.md)
* [(Day 8) 实验一：中断](06-rcore-lab-notes/lab1_practice.md)

---

## Day 0 2020-07-03

今天上午终于考完试了，开始学Rust！

### Rust环境配置

说起来，还没报名这个活动的时候，我就对rCore有点兴趣了，然后用一天时间大致刷了一下Rust的语法，并做完了rCore的第零章和第一章的内容。我在配环境上是卡了比较久的。

我用的是自己的电脑，没有用虚拟机。由于个人习惯装着一个ArchLinux的衍生：Manjaro。由于用的是Arch系而非教程文档所用的Debian系，配环境还要自己折腾一阵子。

我用`pacman`试过了各种版本的Rust相关环境，比如`rust`、`rust-nightly`、`cargo-nightly`等等，最后选择了从源里直接安装`rustup`，然后使用`rustup`和`cargo`安装Rust环境和需要的工具链，所有问题迎刃而解。

我之所以在这里写出来，也是希望我的经历能给后人带来一点帮助。如果你用的是ArchLinux，那么请按照如下方法配置：

```bash
sudo pacman -S rustup
rustup install stable
rustup target add riscv64imac-unknown-none-elf
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

注意我这里安装的是Rust的stable版本，并非文档所说的nightly版本。目前还没有遇到问题，那就先用着stable吧。

**20200706补充**：用stable版本真的遇到问题了，在做基准测试的时候需要用到nightly的一个feature，所以我stable和nightly两个都装了，这样就不会有什么问题了。

另外`qemu`也是之前我做uCore的时候就装好了的，如果还没装，可以使用`pacman`安装`qemu`和`qemu-arch-extra`。

### `vim`环境配置

折腾`vim`的Rust自动补全也折腾了好久，用了`YouCompleteMe`跟`rust-analyzer`的lsp，下面是`vim`的配置：

```vimrc
Plug 'ycm-core/YouCompleteMe', {'do': 'python install.py --clangd-completer'}
Plug 'ycm-core/lsp-examples', {'do': 'python install.py --enable-rust'}
```

### Day0进度

* [Chapter 1 Hello World](01-rust-by-example/chapter01-hello)
* [Chapter 2 原生类型](01-rust-by-example/chapter02-primitives)

### Day1计划

两天刷完`rust-by-example`时间好像有点紧，我争取一下，明天尽量刷完吧！

## Day 1 2020-07-04

今天就算是活动正式开始了！

（所以前一天是day0，今天是day1，好像并没有什么问题）

照着`rust-by-example`写代码的时候感觉`vim`的Rust自动补全不太舒服，又试了下vscode，觉得用`rust-analyze`的环境自动补全都有种怪怪的感觉。

### 在本地编译`rust-by-example-cn`

另外，其实昨天就感受到`rust-by-example-cn`的网站太慢了，于是我找到了这个网站的源码，自己`clone`下来编译了，快了不少。地址：[rust-lang-cn/rust-by-example-cn](https://github.com/rust-lang-cn/rust-by-example-cn)

使用方法：

```bash
git clone https://github.com/rust-lang-cn/rust-by-example-cn
cd rust-by-example-cn
cargo install mdbook
mdbook build
mdbook serve
```

然后打开 [http://localhost:3000/](http://localhost:3000/) 就可以了

### Day1 进度

* [Chapter 3 自定义类型](01-rust-by-example/chapter03-custom_types)
* [Chapter 4 变量绑定](01-rust-by-example/chapter04-variable_bindings)
* [Chapter 5 类型系统](01-rust-by-example/chapter05-types)
* [Chapter 6 类型转换](01-rust-by-example/chapter06-conversion)
* [Chapter 7 表达式](01-rust-by-example/chapter07-experssion)
* [Chapter 8 流程控制](01-rust-by-example/chapter08-flow_control)
* [Chapter 9 函数](01-rust-by-example/chapter09-fn)
* [Chapter 10 模块](01-rust-by-example/chapter10-mod)
* [Chapter 11 crate](01-rust-by-example/chapter11-ceate)
* [Chapter 12 cargo](01-rust-by-example/chapter12-cargo)
* [Chapter 13 属性](01-rust-by-example/chapter13-attribute)
* [Chapter 14 泛型](01-rust-by-example/chapter14-generics)

### Day1 疑惑

所有权、关联类型、虚类型什么的都没太理解，等明天看书的时候仔细理解一下吧。

### Day2 计划

`rust-by-example`跟《Rust编程之道》同步进行！

另外我想跟《Rust编程之道》的作者道个歉。我是支持正版的，您的书我已经买了，但是由于我们这儿是个小城市，快递比较慢，书还需要几天才能送到，所以我先看着本书的电子版，希望您不要介意。

还有，我还在按照老师的安排，前两天刷`rust-by-example`，过几天看书，再过几天刷`rustlings`，为什么那么多大佬都已经把`rustlings`给刷完了……我好慌啊……

## Day 2 2020-07-05

试了一下发现`rustlings`还挺有趣的，而且我觉得可以跟`rust-by-example`同步进行。就先这样开始吧～

另外，在网上查了一系列资料，并亲自实验我才得知，不是`rust-analyzer`不好用，是因为`rust-by-example`都是直接写`.rs`文件的，`rust-analyzer`需要`Cargo.toml`才能正常使用。所以就先这样忍着吧2333

### Day2 扯淡

话说我们这学期的本来应该出去实习的，因为疫情不能出去了。然后根据小道消息要改成线上实习，又要耽误几天时间。为了腾出时间，我要赶进度了。

`rust-by-example`今天就先刷到第19章吧，后面大体看了一遍，就不亲自写代码运行了。

今天开始刷《Rust编程之道》，把3、4、5、9、13章迅速过一遍之后上手第10章！

### Day2 进度

#### Rust by Example

* [Chapter 15 作用域规则](01-rust-by-example/chapter15-scope)
* [Chapter 16 特性 trait](01-rust-by-example/chapter16-trait)
* [Chapter 17 宏](01-rust-by-example/chapter17-macro)
* [Chapter 18 错误处理](01-rust-by-example/chapter18-error)
* [Chapter 19 标准库类型](01-rust-by-example/chapter19-std)

#### Rust编程之道 Chapter 10

* [csv-read (lib)](02-modular-programing/01-csv-read)
* [csv-read (bin)](02-modular-programing/02-csv-read)
* [use_regex (1)](02-modular-programing/03-use_regex)
* [use_regex (2)](02-modular-programing/04-use_regex)
* [use_regex (3)](02-modular-programing/05-use_regex)
* [static_hashmap](02-modular-programing/06-static_hashmap)

### Day2 收获

通过读书以及在群内交流，更深入地理解了借用和生命周期。

还有，宏的用法实在是太妙了！

### Day2 疑惑

通过读书以及在群内交流，对Rust也越来越懵了……

从一开始我就一直在参考各种书各种文档写代码，还没有亲自上手写过自己的代码，所以才会有这种感觉吧。

### Day3 计划

综上所述，我尽量争取明天上午刷完《Rust编程之道》第10章，下午开始编程小练习。

## Day 3 2020-07-06

### 《Rust编程之道》感悟

~~书收到了，是正版，手感很好，下次还会再买老师的书。~~

Rust真的是一门很严谨的编程语言，而且我也渐渐明白为什么要用Rust写操作系统了。

虽说Rust的优点很多，但是一个致命的缺点就是：太难上手了（哭

另外，由于时间有限，目前没有认真阅读本书，再给老师道个歉。但是我相信这本书在今后至少两个月内会时刻放在我的手边，随时翻阅的。

~~还有，感觉拿Rust去打ACM的话肯定特别酸爽，链表、树、图什么的基本上是别想用了2333~~

### `rustling`感悟

`macros3`和`errorsn`有点难。

`iterators4`很有趣：

```rust
    // Complete this function to return factorial of num
    // Do not use:
    // - return
    // For extra fun don't use:
    // - imperative style loops (for, while)
    // - additional variables
    // For the most fun don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
```

大意是：写一个阶乘函数，为了获得更多的乐趣，请不要用循环和额外的变量；为了获得最多的乐趣，请不要使用递归。

一看到这个我就想到了`map-reduce`，但是这个不需要`map`，直接`reduce`即可。然而`iterator`没有`reduce`这个方法，又查了下这功能在Rust里叫`fold`，直接`fold`一气呵成，一行代码解决～

另外感觉做完之后看一看其他人的解法也是很有趣的，有时候会发现自己的方法不如别人的好，有时候会发现别人的方法不如自己的方法好。参考答案也是一个学习的过程，当然前提是在自己独立完成之后，抄作业就没意思了。

还有`threads1`那也卡了好久，由于一点理解错误。我觉得只有一条线程用`Arc`可以不用`Mutex`，结果发现是两条，而且最后还是用上了。

半天下来，做完`rustlings`最大的感受就是，Rust这个语言很复杂，但是编译器却很贴心2333

随手写个代码都能碰到错误，但是新手犯的很多常识性的错误编译器都会提示该怎么改。

要不是有hint和编译器的提示，独立完成`rustlings`还真有点难呢。

### Day3 进度

《Rust编程之道》第十章用了昨天半天和今天一上午刷完了，`rustlings`也用今天一下午和一晚上刷完了，收获很多，对明天上手写代码充满了信心。

#### Rust编程之道

* [static_hashmap (2015)](02-modular-programing/07-static_hashmap-2015)
* [static_hashmap (2018)](02-modular-programing/08-static_hashmap-2018)
* [csv_challenge](02-modular-programing/09-csv_challenge)
* [csv_challenge (with tests)](02-modular-programing/10-csv_challenge)

#### `rustlings`

* [variables](03-rustlilngs-exercises/variables)
* [if](03-rustlilngs-exercises/if)
* [functions](03-rustlilngs-exercises/functions)
* [primitive_types](03-rustlilngs-exercises/primitive_types)
* [structs](03-rustlilngs-exercises/structs)
* [strings](03-rustlilngs-exercises/strings)
* [enums](03-rustlilngs-exercises/enums)
* [tests](03-rustlilngs-exercises/tests)
* [modules](03-rustlilngs-exercises/modules)
* [macros](03-rustlilngs-exercises/macros)
* [move_semantics](03-rustlilngs-exercises/move_semantics)
* [error_handling](03-rustlilngs-exercises/error_handling)
* [option](03-rustlilngs-exercises/option)
* [clippy](03-rustlilngs-exercises/clippy)
* [standard_library_types](03-rustlilngs-exercises/standard_library_types)
* [traits](03-rustlilngs-exercises/traits)
* [generics](03-rustlilngs-exercises/generics)
* [threads](03-rustlilngs-exercises/threads)
* [conversions](03-rustlilngs-exercises/conversions)

### Day4 计划

我们专业明天就会开始线上实习，大概会占用一个星期时间(?)，我要拼命赶进度了。

明天我争取用实习的空闲时间完成至少20道练习题，加油！

今天就先到这吧，还要准备考研。有点累，但是感觉很充实。我喜欢这种感觉～

补充：看到群里老师说也可以写Leetcode的题目，有点心动，我也想试试了（

## Day 4 2020-07-07

笨办法系列看起来都好简单，不过听说Rust写双向链表很麻烦，就不参考[笨办法学C](https://docs.kilvn.com/lcthw-zh/)了，我要参考的是[笨办法学Python](https://www.2cto.com/shouce/Pythonbbf/index.html)。

另外我们专业去年持续三周的实习，今年变成线上两小时了，省了不少时间。但是要写20页实验报告，好麻烦啊。

### Day4 进度

做了一下Rust版的笨办法学Python，感觉前面除了文件部分都好简单，所以就先做到习题17吧。

前面是`print`，后面是`if`和`while`，感觉到习题30多都没什么挑战性，所以我就先做这些，转战Leetcode！做上5道左右的题目就去写实验报告去！

#### 笨办法学Rust（Learn Rust the Hard Way）

参考：[笨办法学Python](https://www.2cto.com/shouce/Pythonbbf/index.html)

* [习题 1: 第一个程序](04-learn-rust-the-hard-way/ex01.rs)
* [习题 2: 注释和井号](04-learn-rust-the-hard-way/ex02.rs)
* [习题 3: 数字和数学计算](04-learn-rust-the-hard-way/ex03.rs)
* [习题 4: 变量(variable)和命名](04-learn-rust-the-hard-way/ex04.rs)
* [习题 5: 更多的变量和打印](04-learn-rust-the-hard-way/ex05.rs)
* [习题 6: 字符串(string)和文本](04-learn-rust-the-hard-way/ex06.rs)
* [习题 7: 更多打印](04-learn-rust-the-hard-way/ex07.rs)
* [习题 8: 打印，打印](04-learn-rust-the-hard-way/ex08.rs)
* [习题 9: 打印，打印，打印](04-learn-rust-the-hard-way/ex09.rs)
* [习题 10: 那是什么？](04-learn-rust-the-hard-way/ex10.rs)
* [习题 11: 提问](04-learn-rust-the-hard-way/ex11.rs)
* [习题 12: 提示别人](04-learn-rust-the-hard-way/ex12.rs)
* [习题 13: 参数、解包、变量](04-learn-rust-the-hard-way/ex13.rs)
* [习题 14: 提示和传递](04-learn-rust-the-hard-way/ex14.rs)
* [习题 15: 读取文件](04-learn-rust-the-hard-way/ex15.rs)
* [习题 16: 读写文件](04-learn-rust-the-hard-way/ex16.rs)
* [习题 17: 更多文件操作](04-learn-rust-the-hard-way/ex17.rs)

#### Leetcode by Rust

* [Problem 1 两数之和](05-leetcode/prob01.rs)
* [Problem 7 整数反转](05-leetcode/prob07.rs)
* [Problem 9 回文数](05-leetcode/prob09.rs)
* [Problem 14 最长公共前缀](05-leetcode/prob14.rs)
* [Problem 20 有效的括号](05-leetcode/prob20.rs)

上面只是能够解决问题的Rust代码

提交到Leetcode上的Rust代码以及Python代码见：

* [Leetcode-by-Rust](05-leetcode/README.md)

### Day4 感悟

用Rust做算法题果然麻烦，虽然做的那几道题也没涉及到多少算法就是了……

不得不说，Rust的`Option`和`Result`确实既方便又安全，但是Rust的字符串我是真的搞不明白，其它题目我全部没查资料独立完成，字符串的题目查了不少资料才做出来。

不过还是理论结合实践效果更好啊。

今天就先到这吧，这几天还要写20页的实习报告qwq

### Day5 计划

明天开始学RISC-V！

先刷一下浙大的MOOC再刷RISC-V手册吧，看完手册再看特权级相关的内容。

加油！明天也是元气满满的一天！

## Day 5 2020-07-08

浙大的MOOC已经结束了，我在B站找到了搬运。章节不多，但是有前三章就够了。

B站搬运地址：[计算机组成与设计：RISC-V【浙江大学】](https://www.bilibili.com/video/BV1tz411z7GN)

我是学过一点MIPS汇编的，也有亲自上手在FPGA板子上写MIPS指令集的CPU，虽然还没完成。

看了一上午RISC-V，感觉跟MIPS也差不多嘛，都是精简指令集。不过特权级相关的内容我还是不太了解，重点看特权级好了。

我把学RISC-V归类给重要但不紧急的任务，实习报告归类为紧急但不重要的任务。所以学了一上午RISC-V了，下午再写会实习报告先，时间真的挺紧的。

晚上写了一会实习报告，又看了一下手册前两章，还是觉得跟MIPS好像。

今天就当是摸鱼了吧，明天估计也差不多（哭

### Day6 计划

争取把实习报告写完（？

写完之后专心刷RISC-V！

说起来，明天也该看RISC-V汇编了，也想写写试试，但是好像写完之后没法运行试试效果，就很难受了。

先这样吧，晚安！明天也是元气满满的一天！

## Day 6 2020-07-09

### Day6 感悟

#### Day 6 上午

刚发现手册第二章最后还有个附录，比较了过去指令集的缺点以及RISC-V的改进，非常细致了，我感受到RISC-V跟MIPS的不同了

但是我觉得不支持乘除法，或者说可选的乘除法支持不算是一个改进吧……不知道为什么这么说……

虽说是让指令集更简洁了，但是真的要计算起乘除法不就会很麻烦了吗

或许我需要再往后读一读，好好理解一下了

#### Day 6 下午

啊，我终于在今天下午把实习报告写完了，两天肝出来16页，加上封面18页，我觉得这些就够了吧，实在不想再写了，也写不出来了。正式开始刷RISC-V！

现在稍稍有点理解RISC-V的哲学了：“保持简洁，保持功能单一”

一开始RISC-V的目标也有说，“它要能适应包括从最袖珍的嵌入式控制器，到最快的高性能计算机等各种规模的处理器。”

在4.2节结束语也有提到，“为了为嵌入式应用提供最小的RISC-V处理器，乘法和除法被归入RISC-V的第一个可选标准扩展的一部分RV32M。许多RISC-V处理器将包括RV32M。”

总而言之，扩展性是为了可以将处理器做到很小、功能很少，也可以做到很大、功能很多

妙哉

### Day6 笔记

* [RISC-V Book](http://www.riscvbook.com/)
* [RISC-V Book Chinese](http://www.riscvbook.com/chinese/)
* [RISC-V Green Card](http://www.riscvbook.com/greencard-20181213.pdf)

我觉得前面看到第四章整数乘除法就够了，然而我还是看到向量了

后面第十章特权架构必刷，rCore会需要这些知识

#### RISC-V的扩展

* RV32I：基础整数指令集
* RV32M：整数乘除法
* RV32F：单精度浮点数运算
* RV32D：双精度浮点运算
* RV32A：原子指令
* RV32C：压缩指令
* RV32V：向量支持

#### 函数调用规范

函数调用过程通常分为6个阶段[Patterson and Hennessy 2017]。

1. 将参数存储到函数能够访问到的位置；
2. 跳转到函数开始位置（使用RV32I的jal指令）；
3. 获取函数需要的局部存储资源，按需保存寄存器；
4. 执行函数中的指令；
5. 将返回值存储到调用者能够访问到的位置，恢复寄存器，释放局部存储资源；
6. 返回调用函数的位置（使用ret指令）。

#### 一个小插曲

在《RISC-V手册》中文版发现了几个小错误：

* 61页第三行的指令`sgnj.d`应为`fsgnj.d`
* 65页导言第一行 “我们假定你已经了解了ISA对如何支持多进程”，多了个“对”字
* 101页 图10.4注释 “RV32的XLEN时32，RV64是40。”，“时”应为“是”
* 105页 10.5 第二段第三行 “旨在支持现代类Unix操作系统，如Linux，FreeBSD和Windows。” 然而Windows并不是类Unix操作系统
* 105页 10.5 第二段第五行 “本届介绍S模式的中断和异常”，“届”应为“节”

不知道在哪提交勘误

#### RV32/RV64特权架构

在这里记录一下重点吧

特权指令：

* `sret`：supervisor-mode trap return
* `mret`：machine-mode trap return
* `wfi`：wait for interrupt
* `sfence.vma`：supervisor-mode fence virtual memory address

简单的RISC-V微控制器仅支持M模式。

机器模式最重要的特性是拦截和处理异常（不寻常的运行时事件）的能力。RISC-V将异常分为两类。一类是同步异常，另一类是中断。

在M模式运行期间可能发生的同步例外有五种：

* 访问错误异常：当物理内存的地址不支持访问类型时发生（例如尝试写入ROM）。
* 断点异常：在执行`ebreak`指令，或者地址或数据与调试触发器匹配时发生。
* 环境调用异常：在执行`ecall`指令时发生。
* 非法指令异常：在译码阶段发现无效操作码时发生。
* 非对齐地址异常：在有效地址不能被访问大小整除时发生，例如地址为0x12的`amoadd.w`。

有三种标准的中断源：软件、时钟和外部来源。

软件中断通过向内存映射寄存器中存数来触发，并通常用于由一个hart中断另一个hart（在其他架构中称为处理器间中断机制）。

当实时计数器`mtime`大于hart的时间比较器（一个名为`mtimecmp`的内存映射寄存器）时，会触发时钟中断。

八个控制状态寄存器（CSR）是机器模式下异常处理的必要部分：

* `mtvec`（MachineTrap Vector）它保存发生异常时处理器需要跳转到的地址。
* `mepc`（MachineExceptionPC）它指向发生异常的指令。
* `mcause`（MachineExceptionCause）它指示发生异常的种类。
* `mie`（MachineInterruptEnable）它指出处理器目前能处理和必须忽略的中断。
* `mip`（MachineInterruptPending）它列出目前正准备处理的中断。
* `mtval`（MachineTrapValue）它保存了陷入（trap）的附加信息：地址例外中出错的地址、发生非法指令例外的指令本身，对于其他异常，它的值为0。
* `mscratch`（MachineScratch）它暂时存放一个字大小的数据。
* `mstatus`（MachineStatus）它保存全局中断使能，以及许多其他的状态。

处理器在M模式下运行时，只有在全局中断使能位`mstatus.MIE`置1时才会产生中断.此外，每个中断在控制状态寄存器`mie`中都有自己的使能位。

当一个hart发生异常时，硬件自动经历如下的状态转换：

* 异常指令的PC被保存在`mepc`中，PC被设置为`mtvec`。（对于同步异常，`mepc`指向导致异常的指令；对于中断，它指向中断处理后应该恢复执行的位置。）
* 根据异常来源设置`mcause`（如图10.3所示），并将`mtval`设置为出错的地址或者其它适用于特定异常的信息字。
* 把控制状态寄存器`mstatus`中的`MIE`位置零以禁用中断，并把先前的MIE值保留到MPIE中。
* 发生异常之前的权限模式保留在`mstatus`的`MPP`域中，再把权限模式更改为M。（如果处理器仅实现M模式，则有效地跳过这个步骤）。

默认情况下，发生所有异常（不论在什么权限模式下）的时候，控制权都会被移交到M模式的异常处理程序。但是Unix系统中的大多数例外都应该进行S模式下的系统调用。M模式的异常处理程序可以将异常重新导向S模式，但这些额外的操作会减慢大多数异常的处理速度。因此，RISC-V提供了一种异常委托机制。通过该机制可以选择性地将中断和同步异常交给S模式处理，而完全绕过M模式。

页表相关内容跟x86架构差不多，就不再做笔记了

### Day7 计划

今天大致刷了一下RISC-V的手册，明天刷一下特权级指令规范吧。如果进度快的话甚至可以开始刷rCore了。加油！

## Day 7 2020-07-10

开始刷特权级指令规范！

感觉全英文读起来有点慢，还是太菜了啊

另外我觉得吧，这种类似手册的东西，直接刷挺枯燥的，比较适合一遍做一遍查阅参考

所以我就大体过一遍 CSR 和 Supervisor-Level 相关内容吧，过完就开始做rCore的实验！

### Day7 笔记

[RISC-V特权级指令规范](https://content.riscv.org/wp-content/uploads/2019/08/riscv-privileged-20190608-1.pdf)

> The SYSTEM major opcode is used to encode all privileged instructions in the RISC-V ISA. These can be divided into two main classes:  those that atomically read-modify-write control and status registers (CSRs), and all other privileged instructions.
>
> 特权指令可分为两大类：原子地 读取、修改、写入 控制状态寄存器（CSR）的指令，以及其它特权指令

CSR Field Specifications（CSR域规范）

* Reserved Writes Preserve Values, Reads Ignore Values (WPRI)
  * 写入预留值，读取忽略值（即预留字段
* Write/Read Only Legal Values (WLRL)
  * 仅读/写合法值
* Write Any Values, Reads Legal Values (WARL)
  * 写任意值，读合法值

Supervisor CSRs:

* Supervisor Status Register (`sstatus`)
* Supervisor Trap Vector Base Address Register (`stvec`)
* Supervisor Interrupt Registers (`sip` and `sie`)
* Supervisor Timers and Performance Counters
  * Supervisor software uses the same hardware performance monitoring facility as user-mode software,including the `time`, `cycle`, and `instret` CSRs. The implementation should provide a mechanism to modify the counter values.
* Counter-Enable Register (`scounteren`)
* Supervisor Scratch Register (`sscratch`)
* Supervisor Exception Program Counter (`sepc`)
* Supervisor Cause Register (`scause`)
* Supervisor Trap Value (`stval`) Register
* Supervisor Address Translation and Protection (`satp`) Register

### Day7 进度

#### RISC-V学习

昨天刷了一部分RISC-V的手册，今天刷了一部分RISC-V的特权级指令规范

鉴于我学过MIPS汇编，而RV32I的指令跟MIPS挺像的，所以跟特权级无关的内容看得比较快

而特权级的内容我觉得更适合用来当手册，随时查阅的那种，所以我大致看了一遍，就开始刷rCore了

#### rCore实验

* [Lab 0 实验记录](06-rcore-lab-notes/lab0.md)
* [Lab 0 代码](06-rcore-lab-notes/lab0)
* [Lab 1 实验记录](06-rcore-lab-notes/lab1.md)
* [Lab 1 代码](06-rcore-lab-notes/lab1)

感觉rCore挺有意思的，有点想看一下王润基学长的毕设论文了

##### 一点小插曲

发现了一处文档的错误，刚打算提交pr，去文档仓库发现早就已经被修改了，只是文档的网页出了点问题，还没更新

应该进行的修改为：将`os/src/main.rs`文件中第12行的`#![feature(asm)]`修改为`#![feature(llvm_asm)]`，这可能是`rust-nightly`最近的更改

最终的`src/main.rs`有点好笑，第11行写着：

```rust
//!   任何没有注释的地方都会产生警告：这个属性用来压榨写实验指导的学长，同学可以删掉了
```

##### 本地编译rCore-Tutorial文档

鉴于[rCore-Tutorial V3](https://rcore-os.github.io/rCore-Tutorial-deploy/)网站的文档更新问题，我打算直接`clone`该文档的仓库，然后在本地编译：

```bash
git clone https://github.com/rcore-os/rCore-Tutorial.git
cd rCore-Tutorial
npm install -g gitbook-cli
gitbook install
gitbook serve
```

编译后发现lab1和lab2还多了几个小练习，有点心动

### Day8 计划

明天做一下Lab1和Lab2以及小练习吧，今天还没运动没记单词，就先到这

另外感觉自己做的Rust小练习有点少，要不每天再加几个？

**Day 7 深夜补充**：刚才突然觉得无聊，于是一不小心就把Lab1做完了，还发现了几处文档中代码的缺失，提交了一发pr，应该能过（

晚安

## Day 8 2020-07-11

昨天睡太晚，今天状态不佳，少做一点，早休息，休息好了明天继续

### Day8 进度

笨办法学Rust：

* [习题 18: 命名、变量、代码、函数](04-learn-rust-the-hard-way/ex18.rs)
* [习题 19: 函数和变量](04-learn-rust-the-hard-way/ex19.rs)
* [习题 20: 函数和文件](04-learn-rust-the-hard-way/ex20.rs)
* [习题 21: 函数可以返回东西](04-learn-rust-the-hard-way/ex21.rs)
* 习题 22: 到现在你学到了哪些东西？

rCore：

* 完善了[Lab 1 实验记录](06-rcore-lab-notes/lab1.md)，在里面仔细分析了操作系统产生时钟中断的全过程
* 完成了[实验一：中断](06-rcore-lab-notes/lab1_practice.md)

### Day9 计划

明天争取做完Lab2、Lab3和实验二！加油！

## Day 9 2020-07-12

### Day9 进度

笨办法学Rust：

* 习题 23: 读代码
* [习题 24: 更多练习](04-learn-rust-the-hard-way/ex24.rs)
* [习题 25: 更多更多的练习](04-learn-rust-the-hard-way/ex25.rs)
* 习题 26: 恭喜你，现在可以考试了！
* 习题 27: 记住逻辑关系

rCore实验：

* [Lab 2 实验记录](06-rcore-lab-notes/lab2.md)
* [实验二：内存分配](06-rcore-lab-notes/lab2_practice.md)

### Day10 计划

今天跟导员联系折腾复课证明的事情，再加上开会占用了一点时间，没做太多东西

所以明天的计划就是先把复课证明申请下来！再把Lab2没做完的部分做完！

然后老师发了最终的通过要求，该认真做一下规划了，我想实现一下调度算法或者页面置换算法，毕竟我没有啥硬件，QEMU整文件系统也不好搞估计

就先这样，晚安！

## Day 10 2020-07-13

感受到了被踢皮球的感觉

实习方说需要提供复课申请，返校申请也可以

但是导员说按道理，假期不用复课也不用返校，而开学后复课返校不需要申请，所以开这种证明不合理，也开不下来

实习方说具体事项咨询学校，学校说问清楚实习方到底要开啥证明

都这样了，就不浪费时间了，复课申请我放弃了，在家也一样可以做

### Day10 进度

笨办法学Rust：

* 习题 28: 布尔表达式练习
* [习题 29: 如果(if)](04-learn-rust-the-hard-way/ex29.rs)
* [习题 30: Else 和 If](04-learn-rust-the-hard-way/ex30.rs)
* [习题 31: 作出决定](04-learn-rust-the-hard-way/ex31.rs)
* 习题 32: 循环和列表 （语法限制，无法完成）

rCore实验：

* [Lab 3 实验记录](06-rcore-lab-notes/lab3.md)

### Day11 计划

今天只是对代码的复制粘贴，还没有认真读过代码

明天打算认真读一遍代码，并分析内核重映射的过程，时间足够的话再做Lab4！

现在有点找到感觉了，接下来打算每1~2天完成并理解一个Lab

加油！

---

[D0]: #day-0-2020-07-03
[D1]: #day-1-2020-07-04
[D2]: #day-2-2020-07-05
[D3]: #day-3-2020-07-06
[D4]: #day-4-2020-07-07
[D5]: #day-5-2020-07-08
[D6]: #day-6-2020-07-09
[D7]: #day-7-2020-07-10
[D8]: #day-8-2020-07-11
[D9]: #day-9-2020-07-12
[D10]: #day-10-2020-07-13
