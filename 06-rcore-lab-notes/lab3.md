# Lab 3 实验记录

## 从虚拟地址到物理地址

RV64中，使用Sv39模式作为页表的实现。

在Sv39模式中，物理地址有56位，虚拟地址有64位，其中低39位有效，第63-39位的值必须等于第38位的值。

## 分析：页机制的启动过程

1. 修改 `linker.ld`，将基地址从物理地址 `0x80200000` 修改为虚拟地址 `0xffffffff80200000`，并将各段对其到 `4K`

2. 在 `entry.asm` 中，创建大页页表：

   ```asm
       # 初始内核映射所用的页表
       .section .data
       .align 12
   boot_page_table:
       .quad 0
       .quad 0
       # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
       .quad (0x80000 << 10) | 0xcf
       .zero 507 * 8
       # 第 510 项：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
       .quad (0x80000 << 10) | 0xcf
       .quad 0
   ```

3. 将页表地址减去 `0xffffffff00000000` 来计算页表的物理页号

4. 将页号与 `8 << 60` 做或运算，并将所得结果写入 `satp` 寄存器并刷新 TLB，开启 Sv39 模式的页机制

   **注**：`satp` 寄存器全称：Supervisor Address Translationand Protection，监管者地址转换和保护

   在 RV64 中，`satp` 寄存器的值高四位为 MODE，若值为 `0` 表示未开启页机制，`8` 表示开启 Sv39 模式的页机制，`9` 表示开启 Sv48 模式的页机制

   参考文献：[RISC-V 手册 中文版](http://www.riscvbook.com/chinese/RISC-V-Reader-Chinese-v2p1.pdf)

5. 将 `sp` 寄存器的值设为栈顶的虚拟地址

6. 将 `rust_main` 函数的地址存入寄存器并跳转，这里从物理地址跳转到虚拟地址，表示页机制启动完成

   **注**：跳转前 PC 寄存器的值为 `0x80200030`，将要跳转到的地址为 `0xffffffff80204880`。由于 RISC-V 架构中，立即数跳转指令都是使用相对地址，而立即数无法表示这么大的值，所以此处先将 `rust_main` 函数的地址存入寄存器，再使用 `jr` 指令进行跳转

   此外，启动页机制后，PC 寄存器的值仍为物理地址，而此时应该使用虚拟地址，所以在页表中将虚拟地址 `0x0000000080000000` 映射到了物理地址 `0x80000000`，PC 寄存器保持原值不变即可正常执行指令

## 分析：为什么第 510 项对应虚拟地址 `0xffff_ffff_8000_0000`

此时已开启了 Sv39 模式的页表机制，每个三级页表对应一个 1GB 的大页，而 1GB 为 2^30 Byte，即其低30位均为 `0`，所以此时的虚拟地址为 `510 << 30`，而由于 Sv39 模式的虚拟地址仅低 39 位有效，高 25 位均为第 38 位的值，

我写了一个 Python 代码来由页表项数计算其对应的虚拟地址：

```python
def calc(index) -> str:
    idx = index << 30
    idx_bin = '{:039b}'.format(idx)
    idx_sv39 = idx_bin[0] * 25 + idx_bin
    idx_num = int(idx_sv39, base=2)
    return '{:016x}'.format(idx_num)
```

用 Python 而非 Rust 的原因是，Python对字符串处理及进制转换比较方便

可以用此代码计算得到：

第 2 项对应的物理地址为：`0x0000_0000_8000_0000`

第 510 项对应的物理地址为：`0xffff_ffff_8000_0000`

此外，第 255 项对应的物理地址为： `0x0000003fc0000000`，第 256 项对应的物理地址为：`0xffffffc000000000`，这中间隔了很多物理地址，而那些地址都是无效的，因为其高 25 位与第 38 位的值不同

## 分析：页表精细化的全过程

虽然文档中已经分析得很详细了，但那是实现顺序而非运行顺序，这里我再单步跟踪一下以加深理解：

1. 在 `rust_main` 中，执行 `let remap = memory::mapping::MemorySet::new_kernel().unwrap();`

2. 进入 `memory::mapping::memory_set::MemorySet` 中的 `new_kernel` 函数，首先在此建立各个段与虚拟地址之间的映射：

   ```rust
   let segments = vec![
       // .text 段，r-x
       Segment {
           map_type: MapType::Linear,
           range: Range::from((text_start as usize)..(rodata_start as usize)),
           flags: Flags::READABLE | Flags::EXECUTABLE,
       },
       // .rodata 段，r--
       Segment {
           map_type: MapType::Linear,
           range: Range::from((rodata_start as usize)..(data_start as usize)),
           flags: Flags::READABLE,
       },
       // .data 段，rw-
       Segment {
           map_type: MapType::Linear,
           range: Range::from((data_start as usize)..(bss_start as usize)),
           flags: Flags::READABLE | Flags::WRITABLE,
       },
       // .bss 段，rw-
       Segment {
           map_type: MapType::Linear,
           range: Range::from(VirtualAddress::from(bss_start as usize)..*KERNEL_END_ADDRESS),
           flags: Flags::READABLE | Flags::WRITABLE,
       },
       // 剩余内存空间，rw-
       Segment {
           map_type: MapType::Linear,
           range: Range::from(*KERNEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS)),
           flags: Flags::READABLE | Flags::WRITABLE,
       },
   ];
   ```

3. 建立一个新的 `Mapping`，用来维护页表和映射关系：`let mut mapping = Mapping::new()?;`

   其中，`Mapping` 的定义为：

   ```rust
   #[derive(Default)]
   /// 某个线程的内存映射关系
   pub struct Mapping {
       /// 保存所有使用到的页表
       page_tables: Vec<PageTableTracker>,
       /// 根页表的物理页号
       root_ppn: PhysicalPageNumber,
   }
   ```

4. 建立新的 `allocated_pairs`，准备保存所有新分配的物理页面

   ```rust
   // 准备保存所有新分配的物理页面
   let mut allocated_pairs = Vec::new();
   ```

5. 将每个字段在页表中进行映射，同时将新分配的映射关系保存到 `allocated_pairs` 中

   ```rust
   // 每个字段在页表中进行映射
   for segment in segments.iter() {
       // 同时将新分配的映射关系保存到 allocated_pairs 中
       allocated_pairs.extend(mapping.map(segment, None)?);
   }
   ```

   其中， `Mapping.map` 的实现过长，也过于复杂，这里省略

   重点是 `Mapping.map_one` 的实现：

   ```rust
   /// 为给定的虚拟 / 物理页号建立映射关系
   fn map_one(
       &mut self,
       vpn: VirtualPageNumber,
       ppn: PhysicalPageNumber,
       flags: Flags,
   ) -> MemoryResult<()> {
       // 定位到页表项
       let entry = self.find_entry(vpn)?;
       assert!(entry.is_empty(), "virtual address is already mapped");
       // 页表项为空，则写入内容
       *entry = PageTableEntry::new(ppn, flags);
       Ok(())
   }
   ```

   此处的 `find_entry` 用来查找给定虚拟页号的三级页表项，如果页表不存在，则会相应创建页表，页表的精细化主要发生在这一步的 `for` 循环中

   ```rust
   /// 找到给定虚拟页号的三级页表项
   ///
   /// 如果找不到对应的页表项，则会相应创建页表
   pub fn find_entry(&mut self, vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry> {
       // 从根页表开始向下查询
       // 这里不用 self.page_tables[0] 避免后面产生 borrow-check 冲突（我太菜了）
       let root_table: &mut PageTable = PhysicalAddress::from(self.root_ppn).deref_kernel();
       let mut entry = &mut root_table.entries[vpn.levels()[0]];
       for vpn_slice in &vpn.levels()[1..] {
           if entry.is_empty() {
               // 如果页表不存在，则需要分配一个新的页表
               let new_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
               let new_ppn = new_table.page_number();
               // 将新页表的页号写入当前的页表项
               *entry = PageTableEntry::new(new_ppn, Flags::VALID);
               // 保存页表
               self.page_tables.push(new_table);
           }
           // 进入下一级页表（使用偏移量来访问物理地址）
           entry = &mut entry.get_next_table().entries[*vpn_slice];
       }
       // 此时 entry 位于第三级页表
       Ok(entry)
   }
   ```

   这一步将每个虚拟地址跟物理地址通过 `PageTableEntry` 对应起来

6. 将虚拟地址跟物理地址的映射建立起来后，下一步就是更新页表并刷新 TLB 了：

   通过 `src/main.rs` 中的 `remap.activate();` 语句，调用 `activate` 函数：

   ```rust
   impl Mapping {
       /// 将当前的映射加载到 `satp` 寄存器并记录
       pub fn activate(&self) {
           // satp 低 27 位为页号，高 4 位为模式，8 表示 Sv39
           let new_satp = self.root_ppn.0 | (8 << 60);
           unsafe {
               // 将 new_satp 的值写到 satp 寄存器
               llvm_asm!("csrw satp, $0" :: "r"(new_satp) :: "volatile");
               // 刷新 TLB
               llvm_asm!("sfence.vma" :::: "volatile");
           }
       }
   }
   ```

   至此，页表精细化完成
