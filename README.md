# Daily Schedule for OS Tutorial Summer of Code 2020

## TOC

七月

|    Mon    |    Tue    |    Wed    |    Thu    |    Fri    |    Sat    |    Sun    |
|-----------|-----------|-----------|-----------|-----------|-----------|-----------|
|           |           |     1     |     2     |  3([D0])  |  4([D1])  |  5([D2])  |
|  6([D3])  |  7([D4])  |  8([D5])  |  9([D6])  | 10([D7])  | 11([D8])  | 12([D9])  |
| 13([D10]) | 14([D11]) | 15([D12]) | 16([D13]) | 17([D14]) | 18([D15]) | 19([D16]) |
| 20([D17]) | 21([D18]) | 22([D19]) | 23([D20]) | 24([D21]) | 25([D22]) | 26([D23]) |
| 27([D24]) | 28([D25]) | 29([D26]) | 30([D27]) | 31([D28]) |           |           |

八月

|    Mon    |    Tue    |    Wed    |    Thu    |    Fri    |    Sat    |    Sun    |
|-----------|-----------|-----------|-----------|-----------|-----------|-----------|
|           |           |           |           |           |  1([D29]) |  2([D30]) |
|  3([D31]) |  4([D32]) |  5([D33]) |  6([D34]) |  7([D35]) |  8([D36]) |  9([D37]) |
| 10([D38]) | 11([D39]) | 12([D40]) | 13([D41]) | 14([D42]) | 15([D43]) | 16([D44]) |
| 17([D45]) | 18([D46]) | 19([D47]) | 20([D48]) | 21([D49]) | 22([D50]) | 23([D51]) |
| 24([D52]) | 25([D53]) | 26([D54]) | 27([D55]) | 28([D56]) | 29([D57]) | 30        |
| 31        |           |           |           |           |           |           |

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
* [(Day 27) zircon-objects](07-zcore-notes/objects.md)
* [(Day 27) zircon-syscall](07-zcore-notes/syscall.md)
* (Day 33) 在 zCore 中运行外部编译的 HelloWorld
* (Day 34) LibOS 中实现 stdin
* (Day 35) LibOS 中 shell 移植成功
* (Day 40) QEMU 中实现 stdin
* (Day 41) LibOS 中 GCC 移植成功
* (Day 43) QEMU 中 shell 移植成功

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

## Day 24 2020-07-27

在 `rcore-os` 内搜索，突然发现了 `zCore-Tutorial`，赶紧来做一下试试先

仓库地址：[zCore-Tutorial](https://github.com/rcore-os/zCore-Tutorial)

文档地址：[简明 zCore 教程](https://rcore-os.github.io/zCore-Tutorial/)

顺手帮改了点内容提交了 Pull Request，然后发现这文档就只有这一节，剩下的还是要靠我自己摸索了，不过还是有目录可以参考的，至少比没有要强

刚才我又突然想看一下其他同学有没有人实现 `sys_fork`，去参考一下来着，结果发现只有一位同学实现了不太完整的 `fork`，其他同学做的都是 `sys_clone`

我又在 zCore-Tutorial 里发现了 Zircon 和 Fuchsia 的中文文档，也可以先看那个

* [zhangpf / fuchsia-docs-zh-CN](https://github.com/zhangpf/fuchsia-docs-zh-CN)

也有点忘记微内核的概念了，查查维基百科先（

[微内核 - 维基百科](https://zh.wikipedia.org/wiki/%E5%BE%AE%E5%85%A7%E6%A0%B8)

* 微内核的定义：
  * 在计算机科学中，微内核（英语：Microkernel，μ-kernel），是一种内核的设计架构，由尽可能精简的程序所组成，以实现一个操作系统所需要的最基本功能，包括了底层的定址空间管理、线程管理、与进程间通信。
* 微内核的概念：
  * 微核心的设计理念，是将系统服务的实现，与系统的基本操作规则区分开来。它实现的方式，是将核心功能模块化，划分成几个独立的行程，各自运行，这些行程被称为服务（service）。所有的服务行程，都运行在不同的地址空间。只有需要绝对特权的行程，才能在具特权的运行模式下运行，其余的行程则在用户空间运行。

现在有点理解微内核，或者说 zCore 的设计理念了

Zircon 是基于对象的内核，所以第一步是在内核中实现[内核对象](https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/objects.md)

实现对象后再实现[进程](https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/objects/process.md)，然后实现进程间通讯，如
[Channel](https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/objects/channel.md)
[Socket](https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/objects/socket.md)
[FIFO](https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/objects/fifo.md)

之后再逐步实现[线程](https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/objects/thread.md)
[作业](https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/objects/job.md)
[任务](https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/objects/task.md)

接着是内存管理，要实现[虚拟内存对象(VMO)](https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/objects/vm_object.md)
[虚拟内存地址区域(VMAR)](https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/objects/vm_address_region.md)

（这里有点疑惑，为什么不先实现内存管理再实现进程呢）

有了以上的基础，就可以上用户程序并实现外核了

（看到进程的系统调用没有 `fork` 之后松了一口气）

不过我还是不明白真正的用户程序应该怎么向外核申请服务

逛 GitHub 的过程中还偶然发现了潘庆霖学长的 [zircon-notes](https://github.com/PanQL/zircon-notes)，一会读一下

有点看不懂...

今天就先到这，等明天出结果！

### Day25 计划

读 zCore 代码吧，从 `main.rs` 开始跟踪，不过运行不起来是个大问题 emmm

## Day 25 2020-07-28

已移至[objects](07-zcore-notes/objects.md)

### Day26 计划

rCore-Tutorial 的副标题就写到这，剩下的只能啃大标题和 Fuchsia 的文档了

今天就先到这，明天开始任务管理！

估计明天进度会更慢...拿代码和文档的大标题什么的自己摸索...

另外

接到老师的电话了！我通过了！

## Day 26 2020-07-29

已移至[objects](07-zcore-notes/objects.md)

### Day26 碎碎念

我家在山东某小县城，附近没有机场，只能去济南遥墙机场坐飞机，而开车去机场要两三个小时

从济南出发，三个小时左右就能到深圳，但是我没有自己坐过飞机，要去机场摸索很久，还要提前上飞机，大概要提前两个小时到机场

从机场去鹏城实验室，坐地铁的话大概要一个小时多一点

那么我从家到实验室的话，总时间至少要九个小时了，总觉得8月2日出发有点来不及，还是8月1日出发好了，还能提前在深圳转一转，熟悉一下环境

说起来，我好像还有几个朋友在深圳

一个是我之前 ACM 的队友，一起在省赛拿过参与奖，他现在在腾讯工作，而我找不到工作选择考研，我好惨

还有一个是初中时跟我住同一个小区，一起坐公交上学认识的朋友，看朋友圈好像现在也是在深圳工作的样子

不过都好久没有联系了，贸然去联系可能会有点尴尬，就先算了（？

### Day27 计划

继续啃 zCore 代码！

## Day 27 2020-07-30

感觉 zircon 的分析太占行数了，移到新目录里好了

关于 zircon object 的分析已移至[objects](07-zcore-notes/objects.md)

关于 zircon syscall 的分析已移至[syscall](07-zcore-notes/syscall.md)

另外，我在 zCore 的 wiki 里发现了一些有价值的内容：

* [Contribution](https://github.com/rcore-os/zCore/wiki/Contribution)
* [Development Log](https://github.com/rcore-os/zCore/wiki/Development-Log)
* [Documents of zCore](https://github.com/rcore-os/zCore/wiki/documents-of-zcore)

现在就想参与 Contribution，但是奈何 zCore 运行不起来...

## Day 28 2020-07-31

再折腾会 uCore 先，之前答应向勇老师写 uCore 分析文档，咕了好久才写到 Lab 2，趁现在有空再写一篇（

更新文档的时候发现报了莫名其妙的错误，网上查了下才知道我的 ArchLinux 把 NodeJS 给更到最新了，而最新版不向下兼容一些包

我突然想到有个好东西叫 Github Actions，试了一下，但是发现，在老师配置的 GitHub Pages 上整 GitHub Actions 的自动部署好麻烦，于是查降级 NodeJS 的方法，无果，最后找到了如下解决方案：

```bash
sudo pacman -S nodejs-lts-dubnium
```

这样是安装 NodeJS 的 LTS 也就是长期支持版，也相当于降级了

另外，偶然发现了 [zCore 的文档](https://rcore-os.github.io/zCore/zircon_object/)，虽然好像是 `cargo` 自动生成的，不过读起来也会方便一些吧

今天就先到这，该把电脑收拾起来了，明天出发去深圳！

## Day 29 2020-08-01

今天终于到深圳了，还好是今天到的，从家到机场还有在机场摸索的时间跟计划一样，但是飞机延误了半个小时，又因为下雨在深圳上空盘旋了一会，学坐地铁又用了一段时间，等到住下再吃完饭就挺晚了，而且看天气预报还有暴雨，估计明天出发的话就来不及了

今晚吃了煲仔饭，感觉不错～明天想感受一下广东的早茶，不知道在哪可以吃到

嗯就这样吧，今天折腾了一天，今晚休息！要早睡早起！

提前晚安！

## Day 30 2020-08-02

忘记更新惹

## Day 31 2020-08-03

昨天忘记更新，今天补上吧

昨天主要去做了核酸检测，检测完去酒店住下，休息了会，下午在酒店大厅跟几个刚到的朋友聊了会，陪他们去做核酸检测，下午出去约饭，晚上聊天休息，一天就这么结束了

最近的早茶也离这好远，就没有去体验，有点遗憾

唉，是摸鱼的一天呢

~~我是分隔线~~

今天一早起床来到鹏城吃早饭，吃完饭现在在 1604 等开会，顺手更新然后 push 一下，下一次更新就是晚上咯

另外，经过郑权同学的指导，我 clone 下来的 zCore 终于能正确执行了，但是 Busybox 和 Zircon shell 还是会报错，这个不知道该怎么解决了

在 zCore 的文档里是这么写的：

> Clone repo and pull prebuilt fuchsia images:
>
> ```sh
> git clone https://github.com/rcore-os/zCore --recursive
> cd zCore
> git lfs pull
> ```

但事实上需要执行的是

```sh
git clone https://github.com/rcore-os/zCore --recursive
cd zCore
git lfs pull
cd zCore
git lfs pull
```

第一步 `cd` 是进入 clone 下来的 `zCore` 目录，第二步 `cd` 是进入 `zCore/zCore` 目录，然后这里还要再 `git lfs pull` 一下才可以

**20200803下午补充**：我把 zCore 目录给删掉然后重新 clone 一遍，突然就能正常跑了，Busybox 可以正常运行，Zircon shell 会报段错误，zCore 可以正常运行

又重新 clone 了一下，照原来的文档就可以运行...我怀疑是 `git lfs pull` 的问题...

此外，向老师建议我们参考：[2020操作系统课程设计：为 rCore 实现更多 Linux 系统调用](http://os.cs.tsinghua.edu.cn/oscourse/OS2020spring/projects/g02)

我在里面找到了 [方案设计报告幻灯片.pdf](http://os.cs.tsinghua.edu.cn/oscourse/OS2020spring/projects/g02?action=AttachFile&do=view&target=%E6%96%B9%E6%A1%88%E8%AE%BE%E8%AE%A1%E6%8A%A5%E5%91%8A%E5%B9%BB%E7%81%AF%E7%89%87.pdf)

> 工作计划按如下顺序开展：
>
> * 完善对 gcc 的支持，测试 C/C++ 标准库实现
> * 支持make，使 rCore 具备从源码安装软件的能力
>   * 还可考虑支持 CMake
> * 支持 Git
> * 支持 Rust 工具链，主要包括：
>   * rustc
>   * cargo
>   * 测试 rust 标准库实现
> * 支持更多的编辑器
>   * nano
>   * vim

首先需要把 gcc 编译出来，放到 `rootfs/bin` 里面，然后补全系统调用

执行了一下 `file`，发现 `busybox` 是 x86 架构，可以直接用 gcc 编译 gcc 放里面去

然后我尝试 clone 一下 gcc 的仓库，发现里面现在有 242 万多个文件，clone 就要好久了，向老师说光 gcc 的和 gcc 需要的库就能覆盖 Linux 的所有系统调用，我觉得一点也不夸张...

不过 rustc 和 cargo 不知道该怎么移植，直接把二进制复制粘贴过去吗？依赖的库也是这样吗？

## Day 32 2020-08-04

昨天聚餐，晚上没带电脑回去，所以没法 push

今天 push 一发先（

对了，忘了说，我的选题是：rCore 到 zCore 的功能迁移

我整理了一下 zCore 缺失的，rCore 已经实现的系统调用：[syscall.md](07-zcore-notes/syscall.md)

此外，我发现把 gcc 和 rustc 直接复制到 `rootfs/bin` 里面，并把库直接复制到 `rootfs/lib` 里面仍然无法运行，会报段错误

我认为需要重新编译，让 gcc 和 rustc 动态链接到 `/lib/ld-musl-x86_64.so.1` 上或者其它库上才可以（？

去参考一下 rCore 先（

我在 [rcore-user](https://github.com/rcore-os/rCore-user) 发现了 `musl-gcc` 的下载地址，不过也要下载好久

找到 `musl-x86_64-gcc`，`file` 一下，发现是静态链接的，直接解决掉一个大问题x

此外我发现 rCore 中的 nginx 也是静态链接的，不过没有找到 rustc 或者 rustup

~~我是分隔线~~

今天下午开会，听得我想学 Chisel 了（

记录一下以后可能需要的东西

> 处理器在线差分测试仿真验证-原理
>
> * 基于指令级在线比对验证框架
>   * 模拟器作为动态链接库连接到 Verilator 生成的仿真程序中
>   * 处理器在指令完成时发出比对请求
>   * 在比对结果不一致时报错并试试当前处理器状态

使用的模拟器：nemu

> 流片前开发
>
> * 系统软件
>   * Nanos-lite
>   * FreeRTOS
>   * RT-thread
>   * xv6
>   * Linux kernel
>   * Debian

dalao 提醒：

1. xv6 是跑在 qemu 上的，不刷 TLB，如果跑在硬件上需要加上 `sfence.vma`
2. 跨页的 4 字节指令非常恶心，指令未对齐，前两个字节跟后两个字节不在同一个页面，容易出各种莫名其妙的 bug
3. 给低年级同学的建议：
   1. 基础设施很重要（如 nemu）
   2. 系统能力很重要
   3. 学会敏捷开发，学会版本管理

~~我是分隔线~~

我觉得 zCore 的系统调用问题不大，不过 zCore 我不知道该怎么 debug，不能 `println`，用 `log` 输出不出来，不会用 GDB 单步跟踪，加上 `RUST_BACKTRACE=1` 之后关键位置不输出行号，就很难受

经过 Google，我学会了用 GDB 单步跟踪 cargo 编译出的程序，然后终于定位到问题所在了：

因为 gcc 没有 `.rela.dyn` 段，而目前 zCore 只能运行有 `.rela.dyn` 段的程序

我又查了一下，似乎只有动态链接的程序才有 `.rela.dyn` 段，而静态链接的程序没有

那么移植的 gcc 的问题就转化成了：让 zCore 支持静态链接的程序

我把出问题的那行改掉了（其实注释掉那行也可以）：

```rust
// before
elf.relocate(base).map_err(|_| ZxError::INVALID_ARGS)?;

// after
if let Ok(()) = elf.relocate(base) {}
```

这样可以让需要使用 `.rela.dyn` 段重定位的程序重定位，不需要的也不会报错

那么 `cargo run`，好了，段错误，问题又回到原来的位置了...

但是并不影响 busybox 的运行，这是好的

~~我是分隔线~~

今晚王润基学长的 zCore 讲解

> Rust 工程项目实践经验
>
> * 代码质量控制：`cargo fmt` + `cargo clippy`
> * 文档和单元测试：`cargo doc` + `cargo test` + `grcov`
> * crate 的拆分和发布流程：`cargo publish`
> * 持续集成和自动测试：GitHub Actions
> * 社区合作开发：GitHub issue + PR

~~我是分隔线~~

经过单步调试，我发现现在代码又卡在 `thread.start`，但是这次比较奇怪，这次是在返回 `Ok(())` 的地方段错误的，就很迷

经洛佳 dalao 提示，可能需要跟踪一下栈，也可以用 stepi 跟踪一下

今天就到这，push 一发走人

晚安

## Day 33 2020-08-05

昨天整理的系统调用不太全，有些已经写上但是注释掉的我都给算上已经实现了，现在已经修复

debug 过程太长，不放到这里了，我又创建了一个新的文件来记录今天的 debug 过程：[debug_20200805.md](07-zcore-notes/debug_20200805.md)

debug 结束，原来不是 bug，是个 feature

现在可以在 zCore 中运行我自己编译的 HelloWorld 了

那么下一步是编译 `musl-gcc`，代码地址在：[musl](http://git.musl-libc.org/cgit/musl)

今天下午开会，感觉 TBSI 也不错

尝试编译许久 `musl-gcc`，未果，偶然发现一神奇的方式：用 docker 整一个 alpine 系统的容器，然后在容器里面安装 `musl-gcc`，再复制出来，就解决了不少问题

记录一下依赖的包：

> / # apk add gcc
>
> (1/11) Installing libgcc (9.3.0-r2)
>
> (2/11) Installing libstdc++ (9.3.0-r2)
>
> (3/11) Installing binutils (2.34-r1)
>
> (4/11) Installing gmp (6.2.0-r0)
>
> (5/11) Installing isl (0.18-r0)
>
> (6/11) Installing libgomp (9.3.0-r2)
>
> (7/11) Installing libatomic (9.3.0-r2)
>
> (8/11) Installing libgphobos (9.3.0-r2)
>
> (9/11) Installing mpfr4 (4.0.2-r4)
>
> (10/11) Installing mpc1 (1.1.0-r1)
>
> (11/11) Installing gcc (9.3.0-r2)

然后，又出现了一点小问题：

> $ cargo run -p linux-loader /usr/bin/gcc
>
> Finished dev [unoptimized + debuginfo] target(s) in 0.06s
>
> Running \`target/debug/linux-loader /usr/bin/gcc\`
>
> /lib/ld-musl-x86_64.so.1: /usr/bin/gcc: Not a valid dynamic program

`ld-musl-x86_64.so` 报错：`/usr/bin/gcc` 不是一个合法的动态程序

查了好久，找到了原因：[What is the correct way to static link libc?](https://stackoverflow.com/questions/59462844/what-is-the-correct-way-to-static-link-libc)

原因是，复制进去的 GCC 不是 `PIE-enabled`，对其执行一下 `file` 命令跟 Busybox 对比，果然是这样

> $ file rootfs/bin/gcc  
>
> rootfs/bin/gcc: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-x86_64.so.1, stripped
>
> $ file rootfs/bin/busybox
>
> rootfs/bin/busybox: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-x86_64.so.1, stripped

GCC 没有 pie，这个目前还不知道该怎么解决...

那么进行到现在，zCore 只能运行动态链接到 `ld-musl-x86_64.so.1` 的，而且 `PIE-enabled` 的程序，动态链接这个是硬性要求，但是 PIE 这个我觉得可以解决

我又把 `rustc` 及其依赖的动态库复制到 `rootfs/` 里面了，但是缺少 `GETRANDOM` 系统调用

> $ cargo run -p linux-loader /bin/rustc
>
> Finished dev [unoptimized + debuginfo] target(s) in 0.06s
>
> Running \`target/debug/linux-loader /bin/rustc\`
>
> [ERROR][6] unknown syscall: GETRANDOM. exit...

补上 `GETRANDOM` 系统调用后，可以勉强运行了：

> $ cargo run -p linux-loader /bin/rustc
>
> Finished dev [unoptimized + debuginfo] target(s) in 0.06s
>
> Running `target/debug/linux-loader /bin/rustc`
>
> error: no default toolchain configured

跑了一下 `rustup`，报 `assert failed`：

> cargo run -p linux-loader /bin/rustup
>
> Finished dev [unoptimized + debuginfo] target(s) in 0.06s
>
> Running \`target/debug/linux-loader /bin/rustup\`
>
> thread 'async-std/runtime' panicked at 'assertion failed: target + PAGE_SIZE < PMEM_SIZE', kernel-hal-unix/src/lib.rs:231:5
>
> note: run with \`RUST_BACKTRACE=1\` environment variable to display a backtrac

又复制了一下 `nginx`，折腾折腾配置，运行，缺少系统调用，还行

> $ cargo run -p linux-loader /bin/nginx
>
> Finished dev [unoptimized + debuginfo] target(s) in 0.06s
>
> Running \`target/debug/linux-loader /bin/nginx\`
>
> [ERROR][6] unknown syscall: SCHED_GETAFFINITY. exit...

今天的总结：

> gcc、nginx、rustup 均解决动态链接的问题
>
> gcc 由于编译的问题，没有 PIE-enabled，启动不起来，正在找解决方案
>
> nginx 能启动起来，缺少系统调用 SCHED_GETAFFINITY
>
> rustup 能启动起来，缺少系统调用 GETRANDOM，补上后会报 assert failed，还没有排查

今天先到这，晚上回去查 PIE 相关资料

晚安～

## Day 34 2020-08-06

在 alpine 的仓库里找到了 GCC 的 commit 记录，GCC 是在 6.1.0 版本才加入默认 enable-pie 的，我 clone 了 [alpine 包管理器的仓库](https://github.com/alpinelinux/aports)，切到 [enable-pie 的 commit](https://github.com/alpinelinux/aports/commit/5b7befa1b989315a57f4fb49b8381ce06ded96c9) 的 [前一个 commit](https://github.com/alpinelinux/aports/commit/25c19fed5767953094db3d80079717b8c83baa05)，clone，然后编译

编译挺复杂的，我列一下命令吧，不过可能不全

> \# adduser test
>
> \# cp -r gcc /home/test/
>
> \# addgroup test abuild
>
> \# su - test
>
> $ cd ~/gcc
>
> $ abuild-keygen -a
>
> $ abuild -r

编译失败，旧版本还是需要 no-pie，而我试图查一下为什么 GCC 一定需要 PIE-disabled，也没有查到

近期的目标是移植 shell，这需要实现 `sys_poll` 以及 stdin，stdin 需要实现 `Condvar`，`Condvar` 需要获得当前线程并挂起

今天打算先实现不需要 `Condvar` 的 stdin，但是在这卡了好久，没什么进展

老办法，单步跟踪

犯了个低级错误，`serial_read` 接收的参数是 `&mut [u8]`，我用 `Vec::with_capacity(255)` 创建了 `Vec<u8>` 传过去一直读不到，最后读文档才知道 `with_capacity` 创建出来的 `Vec` 的 `len()` 其实是 0

又回酒店折腾了一个多小时，终于把 stdin 给写出来了，但是没有用 `Condvar`，也没有用 `async`，不知道提交上去给不给 merge（

今天就先到这，晚安！

## Day 35 2020-08-07

昨天在酒店网太慢，fork 之后 clone 不下来，现在来到实验室 clone 挺快的，一切正常，

已提交 [Pull Request #131](https://github.com/rcore-os/zCore/pull/131)，

第一次提交代码没有通过 `cargo fmt --all -- --check`，第二次提交代码没有通过 `cargo clippy`，以后需要注意这个

把 `sys_poll` 糊弄过去，然后实现 `sys_nanosleep`，shell 终于可以输出 `/ #` 了，不过还不能用...

> cargo run --release -p linux-loader /bin/sh
>
> / # ^[[51;5R

最后还会输出一些奇怪的字符，输出完成之后就退出了

下一步实现 Condvar 和 `sys_poll`！

经过老师和助教提醒，有了 async/await 机制，Condvar 可能不太需要了，那么 `sys_poll` 不知道该怎么折腾了...

另外，rjgg 真的好严格（小声），不过严格点好，可以保证代码质量

经过一天的折腾，shell 可以勉强运行起来了，现在不知道为什么不输出 `/ #` 了，但是可以使用 `cd`, `pwd` 等 shell 内置的命令了，然而运行其他程序则会报段错误

接下来要折腾 `sys_fork` 了，rjgg 说在 LibOS 上很难解决，建议去 QEMU 里面折腾，那么需要先写一份让 `linux-loader` 在 QEMU 里面跑的 `Makefile`，这里需要参考 `zCore/zCore/Makefile` 了

我们在深圳的活动就这么告一段落了，认识了好多 dalao，也学到了好多东西

完结撒花！

今天也就先到这，明天回家先休息，后天继续！

## Day 36 2020-08-08

昨天忘记 push，今天再补上x

回到家好累哦，从早上 7 点出发，下午 5 点半才到家，这段时间背上一直是被汗浸透的，回家第一件事就是洗澡，洗完澡又是一条好汉（

（另外，还是家里有机械键盘有大显示器舒服啊，天天在会议室敲代码感觉颈椎又有点难受了

经过 rjgg 指点，成功将 zCore 运行在 QEMU 里面了，busybox 可以正常运行，ls 也可以正常运行，但是 sh 并没有反应，我写了一个需要 stdin 的程序丢进去运行，结果也没有反应，我认为是 stdin 还没有实现完全

原来我写好的 stdin 只能在 LibOS 里用，不能在 QEMU 里用

好麻烦啊

明天再写，先睡了，晚安

## Day 37 2020-08-09

为什么 QEMU 里不能用 stdin，完全没有头绪啊...

跟 zircon 那边对比也没有发现有什么明显的区别，下一步是跟 rCore 对比了

对比也没有什么结果，我开始尝试打 log，一番折腾发现，一旦调用 stdin 就收不到中断了

why???

查了半天发现，是中断没启用，具体信息已提交 [Issue #137](https://github.com/rcore-os/zCore/issues/137)

然后...这个 bug，理论上我知道该怎么解决，但是实际上并没有头绪

我认为启动线程不应该是真的启动，应该是将其所属的 `Task` 加入 `executor` 库内的 `GLOBAL_EXECUTOR` 的队列中，然后等待 `idle` 线程调度

然而我不会改...

尝试把 `zircon-loader` 里的 `spawn` 复制到 `linux-loader` 里，但是并没有用

我能力有限，再去问 rjgg 吧

## Day 38 2020-08-10

经 rjgg 提示，其实线程已经加入就绪队列了，但是 `run` 是运行就绪队列里的线程，直到就绪队列为空才返回，然后开中断

我把 `executor` 库往前回滚了一个版本，用旧版本的话 linux 模式可以正常接收中断了，但是 zircon 模式不能工作了

两种模式现在一样，都处于能接收中断，但是不能用 stdin 读输入的状态，就很烦

跟踪了一下，发现是 linux 接收不到 `sys_read` 的系统调用了，而且不止 `sys_read`，任何系统调用都收不到了

又进一步跟踪，发现问题不是系统调用收不到，是程序根本就没有运行 qwq

找到原因了...是 `zCore` 和 `kernel-hal-bare` 里 `executor` 的库版本没有统一，线程添加到的队列跟将要运行的队列不是同一个，导致程序不运行，统一之后还是之前的错误，收不到中断

就很烦 qwq

今天也是没有进度的一天

从回家以来到现在一直没有进度，这可咋整啊

看群里好像 rjgg 也不太明白怎么回事...

先睡吧，晚安

## Day 39 2020-08-11

rjgg 说 `linux-loader/src/lib.rs: spawn` 里面少了 `yield`，但是我看了下也没少啊...

**20200811下午补充**：问了一下，不是少了，是没执行...

QEMU 不能用，先勉强在 LibOS 里移植吧

目前在 LibOS 的情况是：

* 运行 `busybox sh` 或 `busybox ash` 不报错，可以使用内置命令如 `pwd`, `cd`
  * 但是不能运行外部命令，需要 `sys_fork`
  * rjgg 说有一个叫 `fish` 的 shell 不需要 `sys_fork`，尝试移植了一下，发现需要 SOCKET
* 运行 `make` 不报错，但是写 `Makefile` 并运行需要 `sys_fork`
* 运行 `rustc` 不报错，但是没有输出，也不知道该怎么检查是否编译成功
* 运行 `nginx` 报错，需要 SOCKET
* 移植 `gcc` 失败，没有解决 PIE-enable 的问题

进度又卡在这了，我还是继续折腾 QEMU 好了

接下来是反汇编排查，每一条 `cli` 都是嫌疑犯

排查失败...

下一步是排查 `x86_64::instructions::interrupts::disable()`，也失败

每条命令都看起来没啥问题...这可咋整啊...

再去问问 rjgg 吧

rjgg 说切换到用户态的时候会恢复 RFLAGS 寄存器，而其中的 IF 位为 1，表明中断开启，所以按道理应该是没有问题的

我也用 GDB 跟踪了一下，用户态 EFLAGS 的 IF 位也确实是打开的，为啥就收不到呢

魔改 `executor` 的话，加上 `interrupts::enable_interrupts_and_hlt` 之后只能收到一条中断，而且这条中断只能给 yield 不能让用户程序收到

而把 `enable_interrupts_and_hlt` 改成 `enable` 的话，过不了几秒就会发生死锁

rjgg 也没有思路，只能自己 debug 了

今天先到这，晚安

## Day 40 2020-08-12

问题解决了，是我自己闹的乌龙，我错怪 rjgg 了

主要原因还是我被函数名字误导了（哭

我以为 `trap_handler` 是所有中断的入口，没想到只是内核中断的入口，用户中断在另一个地方

中断问题解决了，但是在 QEMU 里还是不能用 stdin

经 rjgg 指导，再去写 async 版 stdin！

参考 [rCore 中对 TTY 的实现](https://github.com/rcore-os/rCore/blob/d3ab8c58adf9df841fdd22f968b2a88dbb0ba370/kernel/src/fs/devfs/tty.rs)

完成啦！

已提交 [Pull Request #143](https://github.com/rcore-os/zCore/pull/143)

shell 终于可以在 QEMU 里运行了！而且可以 `ls`！

折腾了一天，休息！

晚安！

## Day 41 2020-08-13

解决了 stdin 的回显问题，而且经过测试，现在 GNU Make 可以正常运行了！

但是没有 GCC 和 Rust 工具链，只有 Make 并没有什么用处（

Rust 工具链目前还有 `sys_poll` 和 `sys_pipe` 在一起发生的奇怪的问题

GCC 的 PIE-enabled 问题也还没解决

好难啊

**20200813下午补充**：GCC 迁移成功！

感谢 [GregorR / musl-cross](https://github.com/GregorR/musl-cross)

编译也很简单，到 alpine 的 docker 环境下，改一下 `config.sh`，给编译参数加上 `-pie -fpie`，然后运行 `build.sh` 就可以了！

经测试，在 LibOS 中可以正常编译，缺少系统调用 CHMOD，不过这个不是必须的

（看了下洛佳 dalao 的日志，感觉自愧不如，于是我又写了一份更正是一点的日志，然后 push 一发先

现在还有一点问题是，QEMU 中的 shell 经常会由于系统调用 `WAIT4` 被阻塞

不知道从哪下手，对比了一下 rCore 和 zCore 的 `WAIT4`，要不先写一下进程的 `EventBus` 试试

（感觉不太需要呢

另外，在 zCore 的 shell 里直接运行 PATH 里的命令会报错，这个还需要跟踪一下 rCore 的 log 和 zCore 的 log 观察，先在这记下来，明天整

今天先到这，晚安！

## Day 42 2020-08-14

向老师说明天上午要开会汇报进度，我本来打算在 Windows 下装个虚拟机演示我的成果来着，结果由于众所周知的网络原因，Rust 环境一直配不上，换成 WSL 结果还连不上网了...

还是不演示了吧，直接汇报完事儿

今天想折腾一下 GNU Make 来着，但是 make 需要调用 `sys_clone`，然后从 `sys_clone` 到 `fork`，在 LibOS 中调用会报段错误，在 QEMU 中调用会报 Page Fault，不知道该怎么解决

昨天想跟踪 rCore 的 shell 跟 zCore 的 shell 区别，结果发现并没有什么区别，不知道该怎么解决

**20200815补充**：看了先 log，应该是动态链接的问题...

现在不知道该怎么解决的任务有：

* shell 无法直接用 PATH 里的命令（**20200815补充**：云微同学修好啦）
* shell 用一会就因为 `sys_wait4` 而阻塞
* GNU Make 使用 `sys_pipe` 之后用 `sys_read` 而阻塞
* GCC 在 QEMU 中使用 `sys_vfork` 而报 Page Fault
* rustc 在 LibOS 中使用 `sys_pipe` 后会死循环调用 `sys_poll`

真要说的话，移植是不难的，但是解决移植过程中的问题就难多了...

然后我又大体看了下 rCore 的代码，想要实现网络还要移植不少驱动，感觉有点麻烦

跟云微同学聊了一下，TA 最近在忙课程和考试的事情，`sys_pipe` 就由我来折腾一下好了

写好 `EventBus` 和 `async_poll` 之后发现好像并没有什么变化...

今天就先这样，晚安

## Day 43 2020-08-15

昨晚睡前又用 `strace` 跟踪了一下实际操作系统中的 `rustc`，发现其并不需要调用 `sys_pipe` 和 `sys_poll`，于是今天开始四处怀疑

首先怀疑的是信号机制，因为 rCore 中有实现，而 zCore 中没有实现信号以及相关的系统调用，但是迁移了一下发现了不少问题，比如 zCore 中 `LinuxProcess` 结构体的成员全部都不是 `pub` 的，这里需要做比较大的改动，但是擅自改成 `pub` 就不符合 zCore 的哲学

感觉需要再加一个新的 `Inner`？

下午开会，经王润基学长提示，`sys_vfork` 是为 LibOS 设计的，因为 LibOS 里面地址空间全部是共享的，回到 QEMU 里有页表隔离，就 Page Fault 了

把 `linux-syscall/src/lib.rs` 里的 `Sys::VFORK` 重定向到 `sys_fork`，报 OOM，把 HEAP 从 16M 改成 64M 就可以了

然后，GCC 变成了跟 shell 一样的结果：阻塞在 `sys_wait4` 上

这么说，只要解决 `sys_wait4` 的问题，就算同时移植 shell 和 GCC 成功了？

但是不知道怎么解决...

观察了一下 log，我认为正常的流程应该有以下两种情况：

1. 子进程先调用 `sys_exit_group` 退出进程，父进程后调用 `sys_wait4`，不等待
2. 父进程先调用 `sys_wait4`，阻塞，子进程后调用 `sys_exit_group` 退出进程，然后父进程不再阻塞

现在的 zCore 中，第一种情况是没有问题的，问题出在第二种情况上，如果 `sys_wait4` 阻塞，那么进程将不会调度，然后就会一直阻塞

至于解决方法，我认为可以把 `sys_wait4` 改得更 `async` 一些

**20200815傍晚补充**：王润基学长在群里发了一个编译时开启 Force NOMMU 选项的 busybox，我下载试了下发现 shell 可以在 LibOS 里用了！

打了下 log 发现 shell 调用的 fork 变成 vfork 了

然后，`sys_wait4` 的问题折腾了一晚上，终于解决了！

解决方法很简单，在 `wait_signal` 前面，把需要 wait 的 signal 给 clear 掉，否则如果想要等待的 signal 本身就已经存在的，而且想要等待的事件没有完成的话，等待 signal 的这一函数就会陷入死循环

此外，云微同学也把 shell 不能直接运行 PATH 里的命令这一问题给修复啦！

现在 shell 就算是完美移植了吧（除了不显示 `/ #`

但是 GCC 还是有点问题，在 QEMU 中会先在 `linux-object/src/fs/device.rs` 中因为减法溢出而 panic，解决溢出的 panic 问题之后会报：

> panicked at 'cannot write block 81208 offset 0 to device' rcore-fs-sfs/src/lib.rs:44:18

这个就不知道是什么原因了，按道理 `device.rs` 里的减法不该下溢的，可能是内存不够用？

今天就先到这吧，晚安

## Day 44 2020-08-16

今天突然发现，QEMU 里的 shell 被修好之后，LibOS 里的 shell 又被我搞坏了...?

checkout 回我修改 `sys_wait4` 之前的那个分支就好好的，切到 master 就只能执行一条命令了，之后就会一直 `await`

why???

先不管了吧（

今天折腾了一下信号相关的系统调用，因为之前怀疑 rustc 用 `sys_pipe` 和 `sys_poll` 是因为信号相关系统调用没有实现，现在实现之后发现并没有什么用（

但还是~~水~~提交了一发 PR（x

明天继续研究 rustc，晚安

**20200816深夜补充**：我的 PR 被 rjgg 重构了，重构过的代码被放到了另一个分支，但是我还是不太熟悉 Git 和 GitHub 的使用，不知道如何照 rjgg 说的做，折腾了一晚上，最后用复制粘贴解决了...改天有机会问一下这种情况该怎么整

我来复述一下当时的情况吧：

1. 我 fork 了 zCore，然后添加的 signal 相关代码并提交 PR
2. 学长重构了我的代码，并把重构后的代码放到了 signal 分支，然后让我 reset your branch to 'signal' and force push

但是当时我 fork 的仓库里并没有新增的分支，从原仓库向我的仓库 PR 也不行，我就试图曲线救国，新建一个 signal 分支，把原仓库的 signal 分支 pull 过来，解决文件冲突，然后 checkout 到 master，再 reset 到 origin/signal，然后 force push

但是我卡在了 checkout 上，报了一条我一已经忘记内容的错误，reset 也不行，一着急把我本地的 fork 给 rm 掉了，然后试图重新 clone 重新解决，但是最终还是一样的结果

最后，我打算用笨办法，不折腾分支了，在本地 clone 一份原仓库一份我的 fork，把我 fork 的仓库里需要改的文件删掉，然后再从原仓库的 signal 分支复制过来，force push，完成

最后还是笨办法管用（

唉，我还是 GitHub 用得不熟练啊，还要多学习

先睡了，不打扰 rjgg 了，这次是真的晚安

## Day 45 2020-08-17

继续研究 rustc，并补充其需要的系统调用

补上了 `sys_sched_getaffinity` rustc 依旧运行不起来

我在怀疑是由于 zCore 是单核，我的 docker 环境是四核才导致 rustc 需要使用 pipe 和 poll，然后切到 Windows，在虚拟机下装了 AlpineLinux，CPU 设置为单核，`strace rustc`，并没有调用 pipe 和 poll，场面一度陷入僵局

下午打算把 rustc 放到 QEMU 里跑一下试试，结果 QEMU 启动不起来，原因是 Rust 工具链太大了，一共需要 400 多 MB，而 zCore 内存开太小的话运行 rustc 会报 OOM，内存开太大的话 zCore 本身也会比较大，跟磁盘文件 `x86_64.img` 放到一起的话会超过 FAT16 文件系统的上限，导致 QEMU 无法启动，就很尴尬

Rust 工具链里面有一个库文件特别大，有 168.8 MB，这一个文件占了 Rust 工具链的接近一半空间，然而删掉这个文件的话 rustc 又报错

我以为这是 FAT16 的问题，参考网上找到的方案，把 `Makefile` 第 41 行从：

```makefile
-drive format=raw,file=fat:rw:$(ESP) \
```

改为：

```makefile
-drive file.driver=vvfat,file.dir=$(ESP),file.fat-type=32 \
```

将 ESP 目录作为 FAT32 分区，结果还是不行，仍然会报错，甚至还多了一个 warning：

> warning: FAT32 has not been tested. You are welcome to do so!
>
> Directory does not fit in FAT32 (capacity 516.06 MB)

事实证明，这是 QEMU 的问题

在网上找到了一种方案：[\[Qemu-devel\] Re: Size of virtual FAT disk limit?](https://lists.gnu.org/archive/html/qemu-devel/2007-07/msg00154.html)

这种方案的思路是把文件创建成 iso 文件，感觉有一定参考价值，然后顺着这种思路找到了 [How can an image file be created for a directory?](https://unix.stackexchange.com/questions/503211/how-can-an-image-file-be-created-for-a-directory)

参考这种方法把 ESP 目录制作成一个 1G 大小的 img 文件，Makefile 放到我的仓库里了，地址：[Makefile](07-zcore-notes/Makefile)

第一次制作 `disk.img` 需要 root 权限。其实理论上是可以不需要的，我懒得改了（

然后 QEMU 启动，运行 rustc，报错：OOM

我一怒之下把内存从 64M 改成了 512M，光编译就编译了好久，然后运行 rustc，报错：OOM

我觉得可能需要 `sys_brk` 系统调用了...但是我不会写...

今天先到这，晚安

## Day 46 2020-08-18

本来想跟踪一下 rCore 运行 rustc，看一下系统调用的运行情况，但是不知道怎么运行，自己也没摸索出来

要不整个 nightly 版本的试试？比较旧版本的也行

下载失败，还浪费了好多时间（

此外，我发现清华的 rustup 源不能下指定日期的 nightly，科大的源下载慢，交大的源特快

研究了一会 `sys_brk` 的实现，感觉需要读不少 zircon 的代码...

我用 `strace` 跟踪系统调用的时候突然发现有个 `-f` 参数，可以跟踪 fork 出来进程的系统调用，改成 `-ff` 后还会在系统调用前面标记进程的 PID，然后我发现 rustc 真的会调用 `sys_pipe` 和 `sys_poll`，我还发现调用的是 `sys_pipe2`，给 pipe 加了一个 flag：`O_CLOEXEC`

一个多小时后：加上了，并没有什么用处

我在想要不要给这些没有用到的 syscall 提交 PR...因为之前的 `sys_chmod` 和 `sys_sched_getaffinity` 都一不小心删掉了...

不管了，先 push 到自己仓库再说吧（

今天也是没有进展的一天，晚安

## Day 47 2020-08-19

在各种 bug 中研究了大半天，没有进展，感觉进行不下去了...

再总结一下现在解决不了的问题，然后去问一下 rjgg 吧

1. QEMU 中运行 GCC 会在 `linux-object/src/fs/device.rs:25` 处报减法下溢
2. LibOS 中运行 rustc 会无限循环调用 `sys_poll`
3. QEMU 中运行 rustc 会报 OOM，堆内存改成 512M 也不行

rjgg 的回答：

> 可能磁盘镜像不够大？
>
> 把 x86_64.img 调大点试试？
>
> 哦我知道了，`mmap` 的时候把文件所有内容都读到 `vec` 里了……rCore 里面是直接文件映射的，zCore 当时移植的时候简化了这个

读了一下 `sys_mmap` 的代码，的确如此：

在 rCore 中，先获得 `file_like`，再创建映射区域，使文件映射：

```rust
        if flags.contains(MmapFlags::ANONYMOUS) {
          ...
        } else {
            let file_like = proc.get_file_like(fd)?;
            let area = MMapArea {
                start_vaddr: addr,
                end_vaddr: addr + len,
                prot: prot.bits(),
                flags: flags.bits(),
                offset,
            };
            file_like.mmap(area)?;
            Ok(addr)
        }
```

在 zCore 中，直接把 `file` 读到 `buf` 中，然后直接写入 `vmo`，

```rust
        if flags.contains(MmapFlags::ANONYMOUS) {
            ...
        } else {
            let file = self.linux_process().get_file(fd)?;
            let mut buf = vec![0; len];
            let len = file.read_at(offset, &mut buf).await?;
            let vmo = VmObject::new_paged(pages(len));
            vmo.write(0, &buf[..len])?;
            let addr = vmar.map(vmar_offset, vmo.clone(), 0, vmo.len(), prot.to_flags())?;
            Ok(addr)
        }
```

...我觉得这么实现，除了有点简单粗暴，好像也没什么问题

然后我把 `x86_64.img` 调大了一点，`gcc` 正常运行...

直接编译出的程序不能运行，会报不是可用的动态链接程序，这时候只需要把：

```bash
x86_64-linux-musl-gcc xxx.c
```

改成：

```bash
x86_64-linux-musl-gcc -pie -fpie xxx.c
```

就好啦！

好想移植 GNU Make，这样就可以在 zCore 里编译 uCore 了

今天先到这，晚安～

## Day 48 2020-08-20

总结一下 GNU Make 遇到的问题：

* LibOS 中，会因为 `sys_clone` 不支持的参数而 panic，把 `unimplemented` 改成 `sys_vfork` 之后（好像）会因为 `sys_vfork` 而报段错误，不知道怎么解决
* QEMU 中，会因为 `sys_clone` 不支持的参数而 panic，把 `unimplemented` 改成 `sys_fork` 之后就会报不支持用户态中断，不知道怎么解决

继续研究 rustc 好了，话说我一直搞不懂为什么 rustc 会死循环调用 `sys_poll`，查了好多资料，查到的结果是，如果 `sys_poll` 返回 0，也就是 timeout 才会循环，如果返回非 0 值的话就会 `break`，按道理是不会死循环的啊

而且根据 `strace` 跟踪的结果，Alpine Linux 中的 rustc 也是的确 `sys_poll` 返回 1 之后就从 pipe 里 read 了，但是不知道为什么 zCore 里会死循环

试图搜 rustc 的代码，也没得到什么结果，我甚至都有点怀疑是 `sys_poll` 的实现出的问题了

把 `sys_poll` 的返回值改掉也不行，对比了一下 rCore 和 zCore 的 `sys_poll` 也没什么区别，把 `sys_poll` 里面的 `async_poll` 改成 `poll` 也不行，看来不是实现的问题了

但是 rustc 也没什么问题啊，在 Alpine Linux 里面运行就正常

`sys_pipe` 没问题，这个有测试过，`sys_poll` 应该没问题，rustc 也没问题，还能是哪出的问题呢...

我甚至都想 `objdump` rustc 了...

**几个小时后**：我注意到 `strace` 的结果里有这样一段：

> [pid    45] poll([{fd=3, events=POLLIN}], 1, -1) = 1 ([{fd=3, revents=POLLIN}])

注意最后的括号里面，`sys_poll` 有返回 `revents`，我又去查 [poll 的文档](https://www.man7.org/linux/man-pages/man2/poll.2.html)，发现有这样一句话：

> The field `revents` is an output parameter, filled by the kernel with
> the events that actually occurred.

我改了一下 `sys_poll`，在最后把 `ufds` 写回去，问题解决了，现在运行 rustc 编译程序，会报如下错误：

> error[E0463]: can't find crate for \`std\`
>
> error: aborting due to previous error
>
> For more information about this error, try \`rustc --explain E0463\`.

这是好的，是程序迁移的问题，不是系统的问题了，但是这个该怎么解决呢...

感觉这个有参考价值：[error[E0463]: can't find crate for \`std\` #60335](https://github.com/rust-lang/rust/issues/60335)

新的安装方法放到 [我们组的仓库](https://github.com/yunwei37/zcore_migration_notes/tree/master/migration) 了

按照该 issue 所说的方法安装后不报找不到 `std` 了，但是会报不明原因的段错误，而且报错的位置还每次都不一样，就很迷

今天到这，明天继续 debug，晚安

## Day 49 2020-08-21

昨天会报找不到文件 `/proc/cpuinfo`，我把自己电脑上的复制过去一份，之后就不报段错误了，而是

> thread 'async-std/runtime' panicked at 'not implemented', rcore-fs/src/std.rs:25:18
>
> note: run with \`RUST_BACKTRACE=1\` environment variable to display a backtrace

好吧，原来是因为 `cpuinfo` 的权限没有修改，导致 HostFS 没有读权限，改好之后还是原来的段错误...

看了下时间，明天就该开会了，然后我切到 Windows 试图再配一个展示用的环境，尝试了各种方式，最后决定在虚拟机里装裸的 ArchLinux，然后在 Windows 里用 ssh 连接到 Linux，果然舒服（

**晚上补充**：今晚又试了下，在虚拟机里面不能跑 QEMU，因为虚拟化指令不能嵌套，只能跑跑 LibOS 了

然后晚上又试图在 rCore 里运行 rustc，观察结果，发现会发生跟之前 zCore 一样的问题：无限循环 `sys_poll`，这次我知道怎么改了，但是改完之后还是会报错：

> [ERROR][0,-] Mutex: deadlock detected! locked by cpu 0 thread 0 @ 0xffffff0002d0f010

但是看起来进展比 zCore 顺利，已经进行到用 `cc` 链接 (?) 了

但是只有第一次是这样子的，之后几次运行都是报

> panicked at 'assertion failed: !self.free_map.read()[id]', /rcore-fs-sfs/src/lib.rs:890:9
>
> === BEGIN rCore stack trace ===
>
> #00 ...
>
> === END rCore stack trace ===

甚至 `ls` 都报错，可能是因为我改了 `sys_poll` 吧，但是不改的话 rustc 就运行不起来，就很难受

到底是哪出的段错误呢

## Day 50 2020-08-22

下午开会，提前准备一下

之前的进展：

* LibOS 中 shell 移植成功
* QEMU 中 shell 移植成功
* LibOS 中 GCC 移植成功

本周的进展：

* QEMU 中 GCC 移植成功

正在进行的工作：

* 在 LibOS 中移植 GNU Make
* 在 LibOS 中移植 rustc

目前的进度：

* rustc
  * 与编译无关的功能可以正常使用
  * 编译时会报段错误，但是可以产生编译中间的 `*.o` 文件

遇到的困难：

* GNU Make 在 LibOS 中会报段错误，可能是因为 `sys_clone`
* GNU Make 在 QEMU 中会 panic，原因为不支持用户态中断
* rustc 在 LibOS 中编译会报段错误，且出现位置不固定，原因不明
* rustc 在 QEMU 中会报 OOM (out of memory)，原因不确定，但不是因为内存不足

开会可以演示 LibOS 中的 shell、GCC、rustc，但是 QEMU 由于虚拟机的限制无法演示

下午补充：向老师说下周会有汇报，所以为了节省时间，这周就先不展示了

下周的话，我用我的旧 MacBook 配一下环境，然后在那上面演示 QEMU 试试

然后是陈老师和王润基学长的一些提示

* LibOS 中移植 rustc 会报段错误，位置不确定
  * 陈老师提示，可能是动态链接的问题，因为动态链接库加载的位置不固定
  * 此外，陈老师认为 QEMU 是更重要的
* QEMU 中移植 rustc 会报 OOM
  * 陈老师提示，可以检查一下内存中可能设为固定值的部分 (比如 `.bss` 段？)
  * 王润基学长提示，可能是 `sys_mmap` 的问题，因为 rCore 中加载文件会按需加载，zCore 中是会一次性全部加载的，而 rustc 有一些特别大的动态链接库，可能是 mmap 的时候加载导致了内存溢出

今天事情有点多，没有折腾，明天继续

晚安

## Day 51 2020-08-23

`sys_mmap` 好难写

卡在 `commit_pages_with` 上面了，不是报段错误就是报 `Not a valid dynamic program`

实在不行把内存改成 1G...这么大总行吧...

明天再写一天 `sys_mmap`，没有进展就放弃吧，留给我的时间不多了

## Day 52 2020-08-24

写 `sys_mmap` 有思路，但是 debug 完全没有思路，而且时间真的不多了

我想暂时先放弃

**下午补充**：QEMU 里把内存改成 2G 都报 OOM，改成 3G 就编译失败了

而 LibOS 中根据老师提示，可能是动态链接库的问题，而动态链接库（似乎）是用 `sys_mmap` 加载到内存的，看来 `sys_mmap` 是绕不过去了...

用我写的 `sys_mmap` 运行 `busybox ls` 总是在第二个 `mmap` 之后报段错误

我知道是 `commit_page_with` 的问题，但是我不知道怎么改...

但是我寻思，LibOS 里不应该出错吧，我电脑内存够大，而且 QEMU 里内存我都改成 2G 了，为什么也会 OOM，rustc 需要的动态链接库也就 400 多 M 吧

完全没有思路啊，剩下的时间里我做点杂活吧，比如移植 git 什么的

加上 `sys_symlink` 就可以用 `git init` 了，其他功能估计需要网络

然后试了下 `cargo`，发现调用 `sys_mmap` 之后就报段错误了，看来 `sys_mmap` 还有好多坑

我试了下，把 `VmObjectPaged` 的 `commit_page` 有关的内容原封不动复制到我写的 `MmapFile` 里面，依旧段错误，还报在相同的位置，看来不是 `commit_page` 出的问题，而是其他地方的问题了

说起来，我虽然勉强实现了 `commit_page`，但是我连这个函数的功能都没有理解，或许应该读一下 zircon 有关 VMO 的文档了

这条路是移植 Rust 工具链必须要走的，已经没有退路了

明天继续折腾 `sys_mmap`，先从 zircon 的文档下手

今天先到这，晚安

## Day 53 2020-08-25

果然读文档有用啊，而且要读英文原版文档

> Pages are committed (allocated) for VMOs on demand through `zx_vmo_read()`, `zx_vmo_write()`, or by writing to a mapping of the VMO created using `zx_vmar_map()`. Pages can be committed and decommitted from a VMO manually by calling `zx_vmo_op_range()` with the **ZX_VMO_OP_COMMIT** and **ZX_VMO_OP_DECOMMIT** operations, but this should be considered a low level operation. `zx_vmo_op_range()` can also be used for cache and locking operations against pages a VMO holds.

VMO 的 `commit_page` 其实就是 *allocate memory*，这么说就好理解多了

但是，作为一个 `MmapFile`，为什么需要 allocate memory 呢？

如果非要 commit 的话，应该像 `VmObjectPhysical` 一样直接返回地址，还是像 `VmObjectPaged` 一样申请页面呢？

我觉得应该像 `VmObjectPhysical` 一样，直接返回地址就行，但是为什么都不调用 `read`，就直接段错误了？是因为直接读内存了吗？

打了下 log 发现还真是，不调用 `read` 直接读内存

那么我该怎么知道程序想读的是被映射的文件哪部分呢...

去群里问了下，rjgg 说 map 之后就不知道读哪了，一种可能的解决方法是：先不映射，让程序读内存的时候缺页，在 handle pagefault 的地方就可以知道程序要读的地方，这时候再映射就可以了

我又试了下，LibOS 里触发不了缺页异常，因为内存够大；QEMU 里只要用了我写的 `sys_mmap`，不管映射不映射，总是会触发 page fault，而且不能 handle，page fault 之后就会 panic

没有思路，就去写了 [rCore 到 zCore 功能迁移组汇报](https://github.com/yunwei37/zcore_migration_notes/blob/master/migration/usage.md)，因为开会的时候陈老师说要写一份已经完成成果的使用方法，我就先写着这个

现在的版本只是我凭记忆写的，可能不太完善，明天有空再换台电脑照说明自己试试，有问题的地方再改改

明天还打算给 `zCore/Makefile` 完善一下生成 `x86_64.img` 的部分

今天先这样，晚安

## Day 54 2020-08-26

今天事情好多，家里老人过生日，出门了半天，还办理返校手续，折腾了好久都还没办完，也该收拾回学校的东西了

整了半个下午的 `Makefile`，已提交 Pull Request，也算有点成果吧

## Day 55 2020-08-27

今天也好忙，一直在折腾返校手续和收拾东西，手续算是基本办完了

配好了 macOS 的演示环境，不过我刚提交的 PR [#165](https://github.com/rcore-os/zCore/pull/165) 还没有被合并，`make image` 相关功能还用不了

（写到这里突然想起，好像可以从我 fork 的仓库 clone 然后测试...是我蠢了

然后写了好久的组内汇报

今天好忙

## Day 56 2020-08-28

今天也好忙，空余时间拿来整理，然后学了下 Beamer，大致做了我们组的最终课上报告

我的 PR 终于合并了，在 macOS 上测试很成功，但是涉及 `cp` 的指令需要做一些改动，Linux 的 `cp` 跟 macOS 的 `cp` 参数不太一样，此外 PR 提交上去的部分也要在文档里更新，今天太晚了，明天再做吧

忙到脑壳疼，物理的那种脑壳疼，要早点睡了

## Day 57 2020-08-29

做完最终汇报了，由于开学在即，准备比较仓促，我也比较紧张，讲得不太好

写汇报文件我自认为还写得不错，但是讲出来就完全不行了...

希望各位评价的老师同学手下留情（x

然后，我明天就开学了，原定于 8 月 31 日结束的活动，我就先提前结束吧

开完会收拾东西，明天返校，后天就开始准备上课和考研了

清华大学等我！

**开完会补充**：

果然被指出来了，不过说的的确有道理

老师说我可能做得不错，但是讲得时候讲得不尽人意

做报告的话，应该先简单介绍一下背景，阐述要清晰，然后说清楚有什么问题，为什么要解决这个问题，怎么解决这个问题，这样才是一个报告应该有的样子

理清脉络、瞄准主题、层次递进

总结一下我犯的几个错误：

1. 我们组两个人共用一个幻灯片，导致前面讲完了我就有点尴尬了
2. 忘记参会的还有外面的老师，导致没有介绍背景
3. 准备不够充分，最近也在忙开学、返校、选课的事情，有点仓促了
4. 参考资料选择错误，不应该参考 [2020操作系统课程设计：为 rCore 实现更多 Linux 系统调用](http://os.cs.tsinghua.edu.cn/oscourse/OS2020spring/projects/g02)，因为面向的听众不同
5. 这个不算错误了，是我的缺点，表达能力太差，尤其是口头表达能力

如果有下次，我一定认真准备qwq

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
[D24]: #day-24-2020-07-27
[D25]: #day-25-2020-07-28
[D26]: #day-26-2020-07-29
[D27]: #day-27-2020-07-30
[D28]: #day-28-2020-07-31
[D29]: #day-29-2020-08-01
[D30]: #day-30-2020-08-02
[D31]: #day-31-2020-08-03
[D32]: #day-32-2020-08-04
[D33]: #day-33-2020-08-05
[D34]: #day-34-2020-08-06
[D35]: #day-35-2020-08-07
[D36]: #day-36-2020-08-08
[D37]: #day-37-2020-08-09
[D38]: #day-38-2020-08-10
[D39]: #day-39-2020-08-11
[D40]: #day-40-2020-08-12
[D41]: #day-41-2020-08-13
[D42]: #day-42-2020-08-14
[D43]: #day-43-2020-08-15
[D44]: #day-44-2020-08-16
[D45]: #day-45-2020-08-17
[D46]: #day-46-2020-08-18
[D47]: #day-47-2020-08-19
[D48]: #day-48-2020-08-20
[D49]: #day-49-2020-08-21
[D50]: #day-50-2020-08-22
[D51]: #day-51-2020-08-23
[D52]: #day-52-2020-08-24
[D53]: #day-53-2020-08-25
[D54]: #day-54-2020-08-26
[D55]: #day-55-2020-08-27
[D56]: #day-56-2020-08-28
[D57]: #day-57-2020-08-29
