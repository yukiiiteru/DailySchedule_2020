# 第二阶段实验日志

## 进度汇总

* 实现 stdin
* LibOS 中 移植 shell 成功
* QEMU 中 移植 shell 成功
* LibOS 中 移植 GCC 成功
  * 由于 HostFS 限制，无法执行 `CHMOD`，所以编译出的程序无运行权限，需要手动添加
* QEMU 中 移植 GCC 进行中
  * QEMU 中 运行会因为写入文件问题而 panic
* LibOS 中 移植 Rust 工具链进行中
  * 目前可以在 LibOS 中输出 help 信息
  * 编译时 `sys_pipe` 和 `sys_poll` 会有一点冲突
* 移植 Nginx 未开始
  * zCore 中还没有实现网络相关系统调用

## Day 14 2020-08-16

突然发现 QEMU 里的 shell 修好之后 LibOS 里的 shell 又坏掉了，不知道为什么

### Day 14 进度

* 实现了三个 signal 相关的系统调用

## Day 13 2020-08-15

今天开会，经王润基学长提示，解决了 QEMU 中 GCC 报 Page Fault 的问题，现在 Page Fault 的问题解决了，但是之后也会因为 `sys_wait4` 而阻塞

此外，王润基学长发现 busybox 编译时有一个 Force NOMMU 的编译参数，加上该参数后编译，shell 就可以在 LibOS 中运行了

### Day 13 进度

* shell 在 LibOS 中移植成功（感谢 rjgg
* shell 在 QEMU 中也可以几乎完美运行了

## Day 12 2020-08-14

攒了一堆不知道该怎么解决的任务，如下

* shell 无法直接用 PATH 里的命令（**20200815补充**：云微同学修好啦）
* shell 用一会就因为 `sys_wait4` 而阻塞（**20200815补充**：我修好啦）
* GNU Make 使用 `sys_pipe` 之后用 `sys_read` 而阻塞
* GCC 在 QEMU 中使用 `sys_vfork` 而报 Page Fault（**20200815补充**：经 rjgg 提示，这个问题解决了，但是又出现了新的问题...）
* rustc 在 LibOS 中使用 `sys_pipe` 后会死循环调用 `sys_poll`

今天没有进度

## Day 11 2020-08-13

之前的日志过于日常向了，重新开一个比较正式的日志

从 [GregorR / musl-cross](https://github.com/GregorR/musl-cross) 找到了编译 `musl-gcc` 的方法，在本机编译失败，去 alpine 的 docker 环境编译，并加上 `-pie -fpie` 参数，编译成功，`musl-gcc` 可以在 zCore 中运行了

### Day 11 进度

* GCC 在 LibOS 中移植成功，可以正常编译程序，
  * 不过缺少系统调用 `CHMOD`，不会为编译出的程序增加运行权限
  * 编写 `CHMOD` 完成，然后发现 HostFS 并不支持...

## Day 10 2020-08-12

排查了几天，最后经王润基学长指点，突然发现不是 bug，浪费了三天时间

### Day 10 进度

* 完成了带有 `EventBus` 的 stdin

## Day 9 2020-08-11

debug...

反汇编排查 `cli` 指令，未果

今天没有进度

## Day 8 2020-08-10

debug...

排查 `executor` 库完成，未果

今天没有进度

## Day 7 2020-08-09

发现用户程序收不到中断，debug...

排查 `spawn` 函数完成，未果

今天没有进度

## Day 6 2020-08-08

回家，休息

### Day 6 进度

* 把 zCore 的 Linux 模式在 QEMU 里跑起来

## Day 5 2020-08-07

开会汇报进度的时候只实现了 stdin，在开完会之后才能把 shell ~~勉强~~跑起来，有点遗憾

### Day 5 进度

* 实现 `sys_poll`
* 可以勉强使用 shell，只能执行内置命令，如 `cd`, `pwd` 等

## Day 4 2020-08-06

尝试编译 no-pie 的 GCC 失败，打算先移植 shell，第一步是实现 stdin

### Day 4 进度

* 完成简单的 stdin

## Day 3 2020-08-05

运行静态链接的程序一直报错，咨询王润基学长才知道 zCore 由于 LibOS 的原因，只能运行动态链接的程序，然后经过搜索，找到了交叉编译的方法，如下

尝试编译许久 `musl-gcc`，未果，偶然发现一神奇的方式：用 docker 整一个 alpine 系统的容器，然后在容器里面安装 `musl-gcc`，再复制出来，就解决了不少问题

但是 GCC 由于 PIE-disabled 的问题，还是不能在 zCore 中运行

### Day 3 进度

* 解决交叉编译问题，使用如下指令可将本地程序编译为 zCore 可以运行的程序：

  ```bash
  gcc tmp.c -Wl,--dynamic-linker=/lib/ld-musl-x86_64.so.1
  ```

* 发现将程序迁移到 zCore 的方法：在 alpine 的虚拟机或 docker 容器内安装/编译，然后复制出来
* 迁移进度
  * gcc、nginx、rustup 均解决动态链接的问题
  * gcc 由于编译的问题，没有 PIE-enabled，启动不起来，正在找解决方案
  * nginx 能启动起来，缺少系统调用 SCHED_GETAFFINITY
  * rustup 能启动起来，缺少系统调用 GETRANDOM，补上后会报 assert failed，还没有排查

注：启动起来，指可以找到 ELF 入口并运行，不代表可以正常使用

## Day 2 2020-08-04

此外，我发现把 gcc 和 rustc 直接复制到 `rootfs/bin` 里面，并把库直接复制到 `rootfs/lib` 里面仍然无法运行，会报段错误

我认为需要重新编译，让 gcc 和 rustc 动态链接到 `/lib/ld-musl-x86_64.so.1` 上或者其它库上才可以，或者直接使用静态链接的程序

收到国科大学长们的建议：

   1. 基础设施很重要（如 nemu）
   2. 系统能力很重要
   3. 学会敏捷开发，学会版本管理

以及王润基学长的经验

* 代码质量控制：`cargo fmt` + `cargo clippy`
* 文档和单元测试：`cargo doc` + `cargo test` + `grcov`
* crate 的拆分和发布流程：`cargo publish`
* 持续集成和自动测试：GitHub Actions
* 社区合作开发：GitHub issue + PR

### Day 2 进度

* 整理了 rCore 已实现但 zCore 未实现的系统调用列表
  * [syscall.md](07-zcore-notes/syscall.md)

## Day 1 2020-08-03

向老师建议我们参考：[2020操作系统课程设计：为 rCore 实现更多 Linux 系统调用](http://os.cs.tsinghua.edu.cn/oscourse/OS2020spring/projects/g02)

由于 zCore 是 x86 架构，感觉可以直接把我系统里面的程序复制进去运行

### Day 1 进度

* 成功运行 zCore 代码
* 正式选题：rCore 到 zCore 的功能迁移
* 制定计划：
  * 完善对 gcc 的支持，测试 C/C++ 标准库实现
  * 支持 make，使 rCore 具备从源码安装软件的能力
    * 还可考虑支持 CMake
  * 支持 Git
  * 支持 Rust 工具链，主要包括：
    * rustc
    * cargo
    * 测试 rust 标准库实现
  * 支持更多的编辑器
    * nano
    * vim
