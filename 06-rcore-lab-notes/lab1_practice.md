# 实验一：中断

## 实验之前

```bash
git clone https://github.com/rcore-os/rCore-Tutorial.git
cd rCore-Tutorial
git checkout lab-1
```

## 实验题

1. 简述：在 rust_main 函数中，执行 ebreak 命令后至函数结束前，sp 寄存器的值是怎样变化的？

   分析过程：

   0. 修改`Makefile`，为`gdb`增加`-tui`参数，方便调试

   1. 执行`make debug`，并在`rust_main`、`__interrupt`处，及第44行打断点，然后`continue`：

      ```bash
      $ make debug
      (gdb) b rust_main
      (gdb) b __interrupt
      (gdb) b 44
      (gdb) c
      ```

   2. 使用`next`指令单步执行到`llvm_asm!("ebreak");`，执行`info registers sp`查看此时的`sp`值，

      ```gdb
      (gdb) info registers sp
      sp             0x802165f0       0x802165f0
      ```

   3. 输入`continue`使其继续执行，然后代码在`__interrupt`处暂停，再次查看此时的`sp`值：

      ```gdb
      (gdb) info registers sp
      sp             0x802164e0       0x802164e0
      ```

      这里`sp`的值改变了，原因为`__interrupt`内第一条指令即为：`addi sp, sp, -34*8`

      计算一下它们的差值：`0x802165f0 - 0x802164e0 = 272 = 34 * 8`，没有问题

   4. 继续在`handle_interrupt`函数处打断点，然后`continue`，此时`sp`的值为：

      ```gdb
      (gdb) info registers sp
      sp             0x80216490       0x80216490
      ```

      计算得，栈顶减小了`80 = 10 * 8`字节

   5. 函数将执行`match`语句，并进入到`breakpoint`函数中，所以再打断点、`continue`，此时`sp`的值为：

      ```gdb
      (gdb) info registers sp
      sp             0x80216400       0x80216400
      ```

      计算得，栈顶减小了`144 = 18 * 8`字节

   6. `breakpoint`函数结束后，栈顶变回`0x80216490`

   7. `handler_interrupt`函数结束后，栈顶变回`0x802164e0`

   8. `__restore`执行结束后，`panic!`宏执行前，栈顶变回`0x802165f0`

   回答：执行`ebreak`前，栈顶位置在`0x802165f0`，进入`__interrupt`后栈顶下降到`0x802164e0`，进入`handler_interrupt`函数后栈顶下降到`0x80216490`，进入`breakpoint`函数后栈顶下降到`0x80216400`；

   从函数中退出后，栈顶再沿下降顺序逐次上升，最终执行完`ebreak`后又回到`0x802165f0`

2. 回答：如果去掉 `rust_main` 后的 `panic` 会发生什么，为什么？

   首先，进入`src/main.rs`，将第44行`panic!`语句注释起来，然后开启`gdb`，在44行打断点，然后`continue`，`step`，发现此时函数进入了`riscv::register::scause::Scause::is_interrupt`函数内

   继续使用`next`单步跟踪，进入到了`riscv::register::scause::Scause::cause`函数……

   最终的结果为，触发`LoadFault`异常而退出：

   > src/interrupt/handler.rs:59: 'Unresolved interrupt: Exception(LoadFault)

   解答：去掉`panic`后，代码会随即进入`entry.asm`后面的库函数，最终会因为`LoadFault`异常而终止运行。

3. 实验

   1. 在`src/interrupt/handler.rs`的`handler_interrupt`函数的`match`语句中添加如下代码：

      ```rust
      // 访存异常
      Trap::Exception(Exception::LoadFault) => loadfault(context, stval),
      ```

   2. 查询《RISC-V手册》得知：异常发生时，stval会被设置成出错的地址或者其它特定异常的信息字，这里应设为出错的地址，所以在`src/interrupt/handler.rs`中定义如下函数：

      ```rust
      /// 处理访存异常情况
      ///
      /// 如果要访问的内存为0x0，则输出SUCCESS!
      /// 处理完成后要将 `sepc` 增加 2 字节，否则 `pc` 值不变的话会发生死循环，不停输出SUCCESS!
      fn loadfault(context: &mut Context, stval: usize) {
          if stval == 0 {
              println!("SUCCESS!");
          }
          context.sepc += 2;
      }
      ```

   3. 去掉`src/main.rs`中，`rust_main`函数最后的`panic!`，因为它会导致关机，然后在`src/entry.asm`中，`jal rust_main`之后添加`jr x0`，运行即可
