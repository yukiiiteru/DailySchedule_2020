# Daily Schedule for OS Tutorial Summer of Code 2020

## TOC

七月

|       Mon       |       Tue       |       Wed       |       Thu       |       Fri       |       Sat       |       Sun       |
|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|
|                 |                 |        1        |        2        |   3([D0][D0])   |   4([D1][D1])   |   5([D2][D2])   |
|   6([D3][D3])   |   7([D4][D4])   |   8([D5][D5])   |   9([D6][D6])   |  10([D7][D7])   |  11([D8][D8])   |  12([D9][D9])   |
| 13([D10][D10])  | 14([D11][D11])  | 15([D12][D12])  | 16([D13][D13])  | 17([D14][D14])  | 18([D15][D15])  | 19([D16][D16])  |
| 20([D17][D17])  | 21([D18][D18])  | 22([D19][D19])  | 23([D20][D20])  | 24([D21][D21])  | 25([D22][D22])  | 26([D23][D23])  |
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
* [(Day 9) Lab 2](06-rcore-lab-notes/lab2.md)
* [(Day 11) Lab 3](06-rcore-lab-notes/lab3.md)
* [(Day 12) Lab 4](06-rcore-lab-notes/lab4.md)
* [(Day 13) Lab 5](06-rcore-lab-notes/lab5.md)
* [(Day 13) Lab 6](06-rcore-lab-notes/lab6.md)
* [(Day 16) 实验二：内存分配](06-rcore-lab-notes/lab2_practice.md)
* [(Day 17) 实验四（上）：线程](06-rcore-lab-notes/lab4_practice_1.md)
* [(Day 18) 实验四（下）：线程调度](06-rcore-lab-notes/lab4_practice_2.md)
* [(Day 19) 实验六：系统调用](06-rcore-lab-notes/lab6_practice.md)
* [(Day 23) 实验三：虚实地址转换](06-rcore-lab-notes/lab3_practice.md)

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

## Day 11 2020-07-14

今天对页表的理解更深了！

实验三目前还没更新出来，等过几天出来了再做

另外，Lab4的坑好多，我不知道踩哪去了，我整的代码老是报错，线程启动不起来：

> src/process/processor.rs:62: 'called \`Option::unwrap()\` on a \`None\` value'

最后我直接把 `rCore-Tutorial` 的代码复制过来，把 `drivers`、`fs`、`kernel` 这几个目录删掉，然后把耦合的代码也全部删掉，再修修补补，`make run`，线程就启动起来了

正在排查到底是哪出的问题……一个个文件 `diff` 好麻烦

终于查出来了，是 `entry.asm` 的问题，发了一条 issue

### Day11 进度

笨办法学Rust：

* [习题 33: While 循环](04-learn-rust-the-hard-way/ex33.rs)
* [习题 34: 访问列表的元素](04-learn-rust-the-hard-way/ex34.rs)
* [习题 35: 分支和函数](04-learn-rust-the-hard-way/ex35.rs)
* 习题 36: 设计和调试
* 习题 37: 复习各种符号

rCore实验：

* [Lab 3 实验记录](06-rcore-lab-notes/lab3.md)
* [Lab 4 实验记录](06-rcore-lab-notes/lab4.md)

### Day12 计划

今天查 bug 查了好久，基本也是复制粘贴，毕竟不复制粘贴就跑不动（

明天再认真读代码，然后写线程启动过程和调度过程，时间足够再做Lab5！

## Day 12 2020-07-15

昨晚被陈晟祺大佬怼了，因为我不知道 `rocket` 和 `boom` 是啥，他就说我“这个水平的话，不建议你瞎折腾FPGA”

立个flag，等我第二阶段完成后尽快完成我的 MIPS 架构的 CPU，然后把 rCore 或者 zCore 跑在上面，初试结束有机会的话也整个编译器试试

### Day12 进度

笨办法学Rust：

* 习题 38: 阅读代码
* [习题 39: 列表的操作](04-learn-rust-the-hard-way/ex39.rs)
* [习题 40: 字典, 可爱的字典](04-learn-rust-the-hard-way/ex40.rs)

笨办法系列就到这吧，后面的受语言限制太多了，比如 Rust 没有内置的 `rand` 库等

rCore实验：

* [Lab 4 实验记录](06-rcore-lab-notes/lab4.md)

### Day13 计划

晚上8点有个会，时间挺长的，今天就先到这

明天打算完成Lab5，还有这部分学过的内容都有点忘记了，再刷一下操作系统课，就先不赶Lab6了

## Day 13 2020-07-16

今天大致看了一下 Linux 的系统调用，感觉好多

然后就是做 Lab 5 了，这个 Lab 需要一个 `test.img` 文件，这个文件需要使用 `rcore-fs-fuse` 创建，折腾这个文件折腾了半天

一开始创建出来后，QEMU 报错：镜像文件不是 `qcow2` 的格式，研究了一会发现是我 `Makefile` 文件没改

创建出来之后，代码中第一处 `ls` 可以正常执行，但是下一步的创建文件夹则失败，研究了一会我认为是因为 `test.img` 文件太小了，改大一点就好了，然后研究了一下 `dd` 命令，问题解决

放解决方案：

```bash
git clone https://github.com/rcore-os/rcore-fs.git
cd rcore-fs/rcore-fs-fuse
cargo build
mkdir test
touch test/temp
touch test/rust
cargo run tmp.img test zip
dd if=/dev/zero of=test.img count=2048
dd if=tmp.img of=test.img conv=notrunc
rm tmp.img
```

直接使用 `rcore-fs-fuse` 命令创建出的文件只有 529 KB，这个容量不允许再往里创建文件了，而先使用 `dd` 命令创建一个空的 1 M 大小的镜像文件，再将原来得到的 `sfs` 文件系统的镜像写入，这样就得到了一个 `sfs` 文件系统而且尺寸为 1 M 的镜像了

### Day13 进度

rCore实验：

* [Lab 5 实验记录](06-rcore-lab-notes/lab5.md)
* [Lab 6 实验记录](06-rcore-lab-notes/lab6.md)

### Day14 计划

六个 Lab 就这样完成了，感觉并没有很好地理解，因为很多重要部分都被封装在库里了

明天的计划：

1. 完善 Lab5 和 Lab6 的实验记录
2. 做 实验二 的题目，实现基于线段树的物理页面分配算法

等之后的实验更新期间，我先刷着 zCore 吧，顺便也再看看 Linux 的系统调用，我就决定做这一个了

## Day 14 2020-07-17

又仔细看了一下 Lab5 和 Lab6，设备树封装好了，文件系统封装好了，线程分析过了，没什么可以完善的，就先算了

本来想用 Rust 手写线段树来着，太难了，解决了大部分语法问题后，收获了满屏幕的 `cannot borrow as mutable`

然后我就去洛谷抄答案了（

### Day14 进度

折腾线段树折腾了一整天，挺累的，但是对内存分配有了更深的理解，也学会了如何使用 Rust 写复杂的数据结构

rCore实验：

* ~~基于线段树实现的 buddy system~~（20200718翻车了）

## Day 15 2020-07-18

昨天忘记定计划，今天定个吧，做实验二的挑战实验1

**20200718下午补充**：挑战实验本来打算继续用我昨天写得线段树实现的 `buddy system`来着，结果调试了半天发现根本不行，经过单步跟踪我发现，线段树是要用 `Rc` 作为指针的，但是 `Rc` 的底层调用了 `Box`，就发生了实验二第一题所描述的那种场景，堆的实现需要申请堆空间，结果 `panic` 了

但是我也在想，为什么昨天模仿 rCore 仓库里用链表实现的 `buddy system` 没有出现问题？会不会是因为用了 `Mutex`，为什么没有发生死锁？

试了一下，也死锁了……但是我记得昨天 `make` 之前 `make clean` 了啊……

奇了个怪了

今天没有成果，很失败

## Day 16 2020-07-19

昨天在 `unsafe rust` 上浪费了太多时间，总是有莫名其妙的 bug，所以我放弃了这种写法，转到传统的数组形式的线段树

理论上，线段树是非常适合用来写 `buddy system` 的，这两者的思想非常契合

但实际上，线段树需要的空间太大了，还是传统的链表实现比较合理

我把我写的两个版本线段树内存分配算法都放进去吧，前天完成的用指针实现的线段树和今天完成的用数组实现的线段树

这两个线段树的实现都可以通过 `cargo test`，但是用于真正的操作系统中的话，前者由于使用了 `Rc`，其底层调用了 `Box`，会发生死循环，即堆的实现过程中需要申请堆空间的循环；而后者开的空间过小则满足不了需求，开得空间过大就通过不了编译

挑战实验暂时先放弃了

我看到涂轶翔助教已经把后面的实验提交了 Pull Request，我先参考这里做着实验三先

鹏城实验室的李老师邀请我下午会议上做一下分享，先 `push` 一发，晚上开完会继续更新

### Day16 进度

* [实验三：虚实地址转换](06-rcore-lab-notes/lab3_practice.md)

### Day17 计划

明天做实验四（上）！尽量完成！

## Day 17 2020-07-20

今天在实验四（上）这里犯了一处低级错误，直接导致浪费了半天的时间

### Day17 进度

rCore实验：

* [实验四（上）：线程](06-rcore-lab-notes/lab4_practice_1.md)

### Day18 计划

明天争取完成实验四（下），然后做一部分实验六！

## Day 18 2020-07-21

写了一整天的 Stride Scheduling 调度算法，写是写出来了，但是有个大问题，如果所有线程的优先级一样的话不能退化成 FIFO 的形式，而是首尾交换，这就很难改，优先队列就有这么一缺点

就先这样吧，能满足基本需求，即不同线程有不同优先级的话，优先级高的用时间少，优先级低的用时间多

### Day18 进度

rCore实验：

* [实验四（下）：线程调度](06-rcore-lab-notes/lab4_practice_2.md)
* [实验六：系统调用](06-rcore-lab-notes/lab6_practice.md)

### Day19 计划

今天完成了实验六的大部分实验题，只剩一个 `sys_open` 的实验和 `sys_pipe` 的挑战实验了，我打算明天先做 `sys_open` 的实验，争取做完

实验六还没做完，所以就先不传代码了

另外明天有几个老同学回来，约我出去吃饭聊天，可能不会剩下太多时间

做完 `sys_open` 的话先不做挑战实验了，先自己准备一下实验三的实验框架，然后把实验三做完了吧，基础实验全部完成了再开始挑战实验

今天就先到这，晚安～

## Day 19 2020-07-22

rCore-Tutorial 仓库的代码发生了比较大的改动，我在考虑要不要跟进

跟进吧，改动不大，一会就改完了，而且文件相关有一些改动对后面影响挺大的，打开的文件现在是属于进程而不是线程了

写了几个小时，一直在查 rCore 的实现，终于完成了！虽然功能不全（

只实现了 `sys_openat` 的小部分功能，只能打开文件不能创建文件，还不能识别 flags 等等

但是至少可以读文件了

（另外，感觉下个月为 zCore 实现 Linux 的系统调用是个大工程呢

然后我突然发现 rCore-Tutorial 仓库里有关 swap 的代码都没了，就不准备实验三的框架了，等 tyx 大佬给框架

### Day19 进度

rCore实验：

* [实验六：系统调用](06-rcore-lab-notes/lab6_practice.md)

### Day20 计划

今天先到这，明天开始挑战实验！

## Day 20 2020-07-23

经过一番调查，感觉写 buddy system 好难

首先，用 `bitmap` 不合适，第一 `bitmap` 会有限制，第二 `bitmap` 用空间比较大

其次，用二叉树，比如线段树，用数组形式会有限制，用指针形式不管是 `Rc` 还是 `Arc` 什么的内部总会用到 `Box`，然而 Unsafe Rust 我又不会用（其实也不是不会用，只是碰到了莫名其妙的 bug，不知道该如何 debug）

所以，能用的只剩下链表了

但是，用 `LinkedList` 不合适，因为我看了下 `LinkedList` 的内部实现使用了 `Box`，又只剩 Unsafe Rust 了...

尝试了各种 `Option`、`NonNull`、`*mut T` 的各种组合后，我选择放弃...

折腾了一天，感觉 Unsafe Rust 太玄学了，看起来正常的代码总是出现莫名其妙的值

晚上开始 `sys_pipe` 试试

**20200723晚上补充**：完成了 `sys_pipe`，感觉也不难嘛，就是在 `Condvar` 上卡了一会

我觉得作为一个管道的接收端应该要让线程休眠，所以就加上了 `Condvar`，但是一直收不到消息，之后不知道哪冒出来的想法，把 `Condvar` 给注释掉了，然后就成功了

虽然成功了，但是不懂为什么去掉 `Condvar` 会成功

还有我想吐槽一下，为了让系统调用能用上 `usize`，各种类型转换来转换去的好烦哦

### Day20 进度

* [实验六：系统调用 - 挑战实验](06-rcore-lab-notes/lab6_challenge)

### Day21 计划

明天开始看 zCore 吧

我还在纠结做 K210 的移植还是做 Linux 的系统调用，两个我都不熟悉，但是我对这两个比较感兴趣，剩下的写文档太简单，RVM 完全不懂，ARM 指令集也没接触过

等研究完 zCore 再做决定也不迟吧

另外有时间的话再把之前写得实验记录完善一下也行

## Day 21 2020-07-24

今天老师发了调查问卷，需要写篇博客，我先写这个吧，半个上午时间足够

是我判断错了，我写了一个上午...感觉码字比写代码难多了qwq

我果然还是更适合敲代码

今天事情也有点多，就当休息一天吧（

### Day22 计划

明天真的要开始 zCore 了

## Day 22 2020-07-25

开始研究 zCore！第一步是 `clone` 然后运行试试

`git lfs pull` 真的好慢好慢好慢

`make rootfs` 也好慢好慢好慢

一切准备就绪后，我运行了 `cargo run --release -p linux-loader /bin/busybox`，报错：

> error: no release found for 'nightly-2020-06-04'

我觉得应该是开发的大佬们用的是 macOS，而我用的是 Linux 的问题，我把 `rust-toolchain` 里的 nightly 版本改成 `nightly-2020-06-27`，结果又报错：

> error: \`extern\` fn uses type \`core::option::Option\<dummy::PhysFrame\>\`, which is not FFI-safe
>
> error: \`extern\` fn uses type \`core::option::Option<usize>\`, which is not FFI-safe

我参照提示，把所有的 `#![deny(warnings)]` 给注释掉，这些 `error` 就变成 `warning` 了，然后继续编译运行，又报错

> thread 'main' panicked at 'called \`Result::unwrap()\` on an \`Err\` value: EINVAL', linux-loader/src/lib.rs:37:68

我突然意识到，开发 zCore 的大佬们好像用的都是 macOS 而不是 Linux，所以我掏出了我家八年前买的 MacBook Air，开始装系统、配环境，没有出现 Rust 版本的错误、没有出现 `not FFI-safe` 的错误，但是出现了跟 Linux 最后相同的错误：

> thread 'main' panicked at 'called \`Result::unwrap()\` on an \`Err\` value: EINVAL', linux-loader/src/lib.rs:37:68

查了下这个文件，在这样一行报的错：

```rust
let (entry, sp) = loader.load(&proc.vmar(), &data, args, envs).unwrap();
```

应该是加载文件出的问题，不知道怎么解决

绝望

我又换回 Linux，尝试运行下一个 `cargo run --release -p zircon-loader prebuilt/zircon/x64`，结果又报错：

> thread 'main' panicked at 'called \`Result::unwrap()\` on an \`Err\` value: "Did not find ELF magic number"', zircon-loader/src/lib.rs:58:58

那再试试第三个吧，运行 `cd zCore && make run mode=release`，相同的 `not FFI-safe`，这个我已经会解决了，解决掉之后，又报没有 `nightly-2020-06-04` 版本，这次是改 `rboot/rust-toolchain`，改掉之后成功运行，进入了熟悉的 QEMU 界面，然后又...

> panicked at 'called \`Result::unwrap()\` on an \`Err\` value: "Did not find ELF magic number"', zircon-loader/src/lib.rs:58:58

绝望

又换回 macOS，一样的错误

查了下这个文件，是这样一行代码报的错：

```rust
let elf = ElfFile::new(images.userboot.as_ref()).unwrap();
```

是磁盘镜像出的问题，读出来的 ELF 文件检验 `magic number` 失败

绝望

我是完全照着 `README.md` 里的步骤做的，为什么要这样对我

```bash
rm -rf zCore
```

再见，我决定这个月先不碰 zCore 了，老老实实折腾 rCore

**20200725晚上补充**：我突然发现 Rust 找不到 `nightly-06-04` 这个版本是源的问题，把 rustup 的 TUNA 源去掉就可以了，所以...重新 `clone`...

装完了，一样的错误，再见

### Day23 计划

实验四又改了，把对线程的 `clone` 改成对进程的 `fork`，虽然助教说了已经做过的不用重新做了，但是我还是想试一下

## Day 23 2020-07-26

查了下 RISC-V 的系统调用，发现里面没有 `sys_fork`，那么我该怎么整系统调用号呢...

拿 `sys_clone` +1 好了

写了半天，感觉这个有点麻烦诶，调度器里的线程没有 `pub` 不能直接访问，只能通过 `PROCESSOR` 访问当前线程，而且线程可以找到所属进程，但是进程找不到其所有线程，大概需要改很多东西吧，就很麻烦

另外突然发现 Lab 3 的实验框架准备好了，先做 Lab 3 吧（

Lab 3 不难，先 push 一发然后去开会，开完会继续折腾进程和线程了，实现 `fork` 的话要改很多东西咯

大部分问题都解决了，但是碰到一个小问题：如何复制栈？

这时候被复制的进程和复制到的进程虚拟地址是相同的，但是现在启用的是被复制进程的页表，也就是说不知道该如何访问复制到的进程的栈

这时候我想起 Lab 5 有一个用虚拟地址查询物理地址的函数，想参考一下，后来看了下那并不是用某个 `Mapping` 查询的，而是用全局的页表查询的

我又参考了一下 rCore 的 `sys_fork` 实现，发现并没有参考价值

我把虚拟地址换成物理地址都不行...

具体实现放在 [实验四（上）：线程](06-rcore-lab-notes/lab4_practice_1.md) 里面了，虽然是失败的，但是给其他人留作参考吧

### Day23 进度

* [实验三：虚实地址转换](06-rcore-lab-notes/lab3_practice.md)

### Day24 计划

最晚后天就能公布名单了，我觉得有希望，所以这几天就赶快刷 zCore 了

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
[D11]: #day-11-2020-07-14
[D12]: #day-12-2020-07-15
[D13]: #day-13-2020-07-16
[D14]: #day-14-2020-07-17
[D15]: #day-15-2020-07-18
[D16]: #day-16-2020-07-19
[D17]: #day-17-2020-07-20
[D18]: #day-18-2020-07-21
[D19]: #day-19-2020-07-22
[D20]: #day-20-2020-07-23
[D21]: #day-21-2020-07-24
[D22]: #day-22-2020-07-25
[D23]: #day-23-2020-07-26
