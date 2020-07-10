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

## 题外话

在做Lab1的过程中发现了几处文档中代码的缺失，以提交Pull Request。具体内容如下：

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

