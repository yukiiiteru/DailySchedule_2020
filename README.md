# Daily Schedule for OS Tutorial Summer of Code 2020

## TOC

*七月*

|       Mon       |       Tue       |       Wed       |       Thu       |       Fri       |       Sat       |       Sun       |
|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|
|                 |                 |        1        |        2        |   3([D0][D0])   |   4([D1][D1])   |   5             |
|   6             |   7             |   8             |   9             |  10             |  11             |  12             |
| 13              | 14              | 15              | 16              | 17              | 18              | 19              |
| 20              | 21              | 22              | 23              | 24              | 25              | 26              |
| 27              | 28              | 29              | 30              |                 |                 |                 |

---

## Day 0 2020-07-03

### 前言

今天上午终于考完试了，开始学Rust！

### Rust环境配置

说起来，还没报名这个活动的时候，我就对rCore有点兴趣了，然后用一天时间大致刷了以下Rust的语法，并做完了rCore的第零章和第一章的内容。我在配环境上是卡了比较久的。

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

另外`qemu`也是之前我做uCore的时候就装好了的，如果还没装，可以使用`pacman`安装`qemu`和`qemu-arch-extra`。

### `vim`环境配置

折腾`vim`的Rust自动补全也折腾了好久，用了`YouCompleteMe`跟`rust-analyzer`的lsp，下面是`vim`的配置：

```vimrc
Plug 'ycm-core/YouCompleteMe', {'do': 'python install.py --clangd-completer'}
Plug 'ycm-core/lsp-examples', {'do': 'python install.py --enable-rust'}
```

### Day0进度

* [Chapter 1 Hello World](rust-by-example/chapter01-hello)
* [Chapter 2 原生类型](rust-by-example/chapter02-primitives)

### Day1计划

两天刷完`rust-by-example`时间好像有点紧，我争取一下，明天尽量刷完吧！

## Day 1 2020-07-04

### 扯淡

今天就算是活动正式开始了！

（所以前一天是day0，今天是day1，好像并没有什么问题）

照着`rust-by-example`写代码的时候感觉`vim`的Rust自动补全不太舒服，又试了下vscode，感觉用`rust-analyze`的环境自动补全都有种怪怪的感觉。

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

* [Chapter 3 自定义类型](rust-by-example/chapter03-custom_types)
* [Chapter 4 变量绑定](rust-by-example/chapter04-variable_bindings)
* [Chapter 5 类型系统](rust-by-example/chapter05-types)
* [Chapter 6 类型转换](rust-by-example/chapter06-conversion)
* [Chapter 7 表达式](rust-by-example/chapter07-experssion)
* [Chapter 8 流程控制](rust-by-example/chapter08-flow_control)
* [Chapter 9 函数](rust-by-example/chapter09-fn)
* [Chapter 10 模块](rust-by-example/chapter10-mod)
* [Chapter 11 crate](rust-by-example/chapter11-ceate)
* [Chapter 12 cargo](rust-by-example/chapter12-cargo)
* [Chapter 13 属性](rust-by-example/chapter13-attribute)
* [Chapter 14 泛型](rust-by-example/chapter14-generics)

### Day1 疑问

所有权、关联类型、虚类型什么的都没太理解，等明天看书的时候仔细理解一下吧。

### Day2 计划

`rust-by-example`跟《Rust编程之道》同步进行！

另外我想跟《Rust编程之道》的作者道个歉。我是支持正版的，您的书我已经买了，但是由于我们这儿是个小城市，快递比较慢，书还需要几天才能送到，所以我先看着本书的电子版，希望您不要介意。

还有，我还在按照老师的安排，前两天刷`rust-by-example`，过几天看书，再过几天刷`rustlings`，为什么那么多大佬都已经把`rustlings`给刷完了……我好慌啊……

---

[D0]: #day-0-2020-07-03
[D1]: #day-1-2020-07-04
