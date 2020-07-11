# Lab 1 实验记录

## RISC-V 与中断相关的寄存器和指令

默认所有中断实际上是交给机器态处理的，但是为了实现更多功能，机器态会将某些中断交由内核态处理。

发生中断时，硬件自动填写的寄存器：

* `sepc`：记录触发中断指令的地址
* `scause`：记录中断是否为硬件中断，以及具体的中断原因
* `stval`：`scause`不足以存下中断所有的必须信息。例如缺页异常，就会将`stval`设置成需要访问但是不在内存中的地址，以便于操作系统将这个地址所在的页面加载进来

指导硬件处理中断的寄存器：

* `stvec`：设置内核态中断处理流程的入口地址
* `sstatus`：具有许多状态位，如控制全局中断使能等
* `sie`：控制具体中断使能
* `sip`：记录每种中断是否触发

操作CSR的指令：

* `csrrw dst, csr, src`：同时读写的原子操作，将指定CSR的值写入`dst`，同时将`src`的值写入CSR
* `csrr dst, csr`：仅读取一个CSR寄存器
* `csrw csr, src`：仅写入一个CSR寄存器
* `csrc(i) csr, rs1`：将CSR寄存器中指定的bit清零
* `csrs(i) csr, rs1`：将CSR寄存器中指定的bit置1

## 分析中断的处理流程

以时钟中断为例：

1. `src/interrupt/handler.rs`中，`init()`函数声明汇编代码中存在`__interrupt`函数，并调用`stvec::write(__interrupt as usize, stvec::TrapMode::Direct);`来将其地址写入`stvec`寄存器，并设置为`Direct`模式。

   其中，`write`函数的定义如下：

   ```rust
   /// Writes the CSR
   #[inline]
   pub unsafe fn write(addr: usize, mode: TrapMode) {
       _write(addr + mode as usize);
   }
   ```

   其调用了`_write`函数，而其定义及声明如下：

   ```rust
   write_csr!(0x105, __write_stvec);
   ```

   ```rust
   macro_rules! write_csr {
       ($csr_number:expr, $asm_fn: ident) => {
           /// Writes the CSR
           #[inline]
           #[allow(unused_variables)]
           unsafe fn _write(bits: usize) {
               match () {
                   #[cfg(all(riscv, feature = "inline-asm"))]
                   () => llvm_asm!("csrrw x0, $1, $0" :: "r"(bits), "i"($csr_number) :: "volatile"),

                   #[cfg(all(riscv, not(feature = "inline-asm")))]
                   () => {
                       extern "C" {
                           fn $asm_fn(bits: usize);
                       }

                       $asm_fn(bits);
                   }

                   #[cfg(not(riscv))]
                   () => unimplemented!(),
               }
           }
       };
   }
   ```

   该`write_csr!`宏定义了`_write`函数，它可以将参数（即中断处理函数入口地址+定位入口地址的模式）写入`0x105`号CSR寄存器（即`stvec`寄存器）。有了中断处理函数的地址，就可以处理中断了。

   （注：CSR寄存器编号见[RISC-V特权级指令规范](https://content.riscv.org/wp-content/uploads/2019/08/riscv-privileged-20190608-1.pdf)第8-11页）

2. `src/interrupt/timer.rs`中，`init()`函数调用`sie::set_timer()`

   `sie::set_timer`是`riscv/src/register/sie.rs`中使用宏定义的函数：

   ```rust
   set_clear_csr!(
       /// Supervisor Timer Interrupt Enable
       , set_stimer, clear_stimer, 1 << 5);
   ```

   经过查询`set_clear_csr!`宏定位到`riscv/src/register/macros.rs`中的`set_csr!`宏，其在`unsafe`块中调用了`_set`函数：

   ```rust
   macro_rules! set_csr {
       ($(#[$attr:meta])*, $set_field:ident, $e:expr) => {
           $(#[$attr])*
           #[inline]
           pub unsafe fn $set_field() {
               _set($e);
           }
       }
   }
   ```

   `_set`函数的实现为：

   ```rust
   macro_rules! set {
       ($csr_number:expr, $asm_fn: ident) => {
           /// Set the CSR
           #[inline]
           #[allow(unused_variables)]
           unsafe fn _set(bits: usize) {
               match () {
                   #[cfg(all(riscv, feature = "inline-asm"))]
                   () => llvm_asm!("csrrs x0, $1, $0" :: "r"(bits), "i"($csr_number) :: "volatile"),

                   #[cfg(all(riscv, not(feature = "inline-asm")))]
                   () => {
                       extern "C" {
                           fn $asm_fn(bits: usize);
                       }

                       $asm_fn(bits);
                   }

                   #[cfg(not(riscv))]
                   () => unimplemented!(),
               }
           }
       };
   }
   ```

   此`_set`函数由`set!`宏定义，其在`riscv/src/register/sie.rs`中被调用：

   ```rust
   set!(0x104, __set_sie);
   ```

   其最终调用了`csrrs`指令，即`CSR Read & Set Bit`。但由于其将`rd`参数设为了`x0`寄存器，故该指令相当于上述伪指令`csrs`。

   总结：`riscv/src/register/sie.rs`中使用宏定义了`_set`及`set_stimer`函数，`set_stimer`调用`_set`来将`0x104`号CSR寄存器（SIE）的第5位（从第0位开始）设为`1`，即开启了时钟中断。

3. `src/interrupt/timer.rs`中，`init()`函数调用`sstatus::set_sie();`

   其执行过程与`sie::set_stimer()`类似，在`riscv/src/register/sstatus.rs`中定义`_set`函数使其设置`0x100`号寄存器（SSTATUS），然后使用`set_sie`函数设置该寄存器的第1位SIE位，开启S态中断使能。

4. `src/interrupt/timer.rs`中，`init()`函数调用`set_next_timeout();`

   `set_next_timeout`函数定义为：

   ```rust
   /// 设置下一次时钟中断
   ///
   /// 获取当前时间，加上中断间隔，通过 SBI 调用预约下一次中断
   fn set_next_timeout() {
       set_timer(time::read() + INTERVAL);
   }
   ```

   `timer::read()`的执行过程也与以上函数类似，定义`_read`函数以读取`0xC01`号寄存器（`time`）内的时间，然后将时间值加上中断间隔，再通过`set_timer`函数调用OpenSBI的`0`号系统调用`SBI_SET_TIMER`来预约下一次中断。

5. 当处理器的实时计数器`mtime`大于`hart`的时间比较器（一个名为`mtimecmp`的内存映射寄存器）时，会触发时钟中断。

6. 中断触发时，处理器查询`stvec`寄存器内中断处理函数入口地址及地址模式，得到函数`__interrupt`的地址，并将PC寄存器的值设置为该函数入口地址。

7. 执行`src/interrupt/interrupt.asm`中的`__interrupt`函数（函数内容过长，就不列出来了），具体流程如下：

   1. 在栈上开辟`34×8`字节的空间，即，将`sp`寄存器内的值减去`34×8`；
   2. 将`x0`-`x31`号寄存器的值压栈，其中`x2`，即`sp`寄存器的值设为其原来的值，即现在`sp`寄存器的值再加上`34×8`；
   3. 将CSR寄存器`sstatus`及`sepc`的值分别存入`s1`、`s2`寄存器，再将其压栈；
   4. 将`sp`寄存器内的值（即组成结构体`Context`的地址）存入`a0`寄存器；
   5. 将CSR寄存器`scause`及`stval`的值分别存入`a1`、`a2`寄存器；
   6. 将`a0`、`a1`、`a2`寄存器的值作为参数，调用`handle_interrupt`函数。

8. 在`handle_interrupt`中处理（S态）时钟中断，即调用`src/interrupt/handler.rs`中的`supervisor_timer`函数，并将寄存器上下文`Context`传入。

9. `supervisor_timer`函数调用`src/interrupt/timer.rs`中的`timer::tick()`函数，设置下一次时钟中断，并输出“xxx tick”。

10. 返回到`src/interrupt/interrupt.asm`文件中，继续向下执行，到达`__restore`，恢复`sstatus`、`sepc`、`x1`、`x3`-`x31`、`x2`寄存器，然后执行`sret`指令，继续执行`sepc`指向位置的指令，中断结束。

## 题外话

在做Lab1的过程中发现了几处文档中代码的缺失，已提交Pull Request。具体内容如下：

* “程序运行状态”一节中，添加了`use core::fmt;`以及
  
  ```rust
  /// 格式化输出
  ///
  /// # Example
  ///
  /// ```rust
  /// println!("{:x?}", Context);   // {:x?} 表示用十六进制打印其中的数值
  /// ```
  impl fmt::Debug for Context {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          f.debug_struct("Context")
              .field("registers", &self.x)
              .field("sstatus", &self.sstatus)
              .field("sepc", &self.sepc)
              .finish()
    }
  }
  ```

  否则在“时钟中断”一节，`os/src/interrupt/handler.rs`文件的`panic!`处会报出`context`无法被输出

* “状态的保存与恢复”一节中，`os/src/interrupt.asm`文件应位于`os/src/interrupt/interrupt.asm`处，否则“进入中断处理流程”一节的`global_asm!(include_str!("./interrupt.asm"));`将无法正常引入汇编文件

* “进入中断处理流程”一节中，添加了`use riscv::register::scause::Scause;`，否则`handle_interrupt`函数的定义会报找不到`Scause`类型

* “时钟中断”一节中，添加了：

  ```rust
  use riscv::register::{
      scause::{Exception, Interrupt, Scause, Trap},
      sie, stvec,
  };
  ```

  否则编译时会报多个类型或者crate找不到（可能措辞不太正确）

  以及在最后添加了一句“去掉main函数的返回值”，否则main函数无法正常编译
