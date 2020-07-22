//! 文件相关的内核功能

use super::*;
use bitflags::*;
use crate::ROOT_INODE;
use core::slice::from_raw_parts_mut;

/// 打开文件
pub(super) fn sys_openat(
    dir_fd: usize,
    path: *const u8,
    flags: usize,
    _mode: usize,
) -> SyscallResult {
    assert_eq!(dir_fd, AT_FDCWD);
    let path = check_and_clone_cstr(path).unwrap_or(String::from("SysError::EFAULT"));
    let path = path.trim();
    let flags = OpenFlags::from_bits_truncate(flags);
    assert!(!flags.contains(OpenFlags::CREATE));
    println!("finding file {}", path);
    let inode = ROOT_INODE.find(&path).unwrap();
    let proc = PROCESSOR.lock().current_thread().process.clone();
    let fd = proc.inner().descriptors.len();
    proc.inner().descriptors.push(inode);
    SyscallResult::Proceed(fd as isize)
}

/// 关闭文件
pub(super) fn sys_close(fd: usize) -> SyscallResult {
    let proc = PROCESSOR.lock().current_thread().process.clone(); 
    proc.inner().descriptors.remove(fd);
    SyscallResult::Proceed(0)
}

/// 从指定的文件中读取字符
///
/// 如果缓冲区暂无数据，返回 0；出现错误返回 -1
pub(super) fn sys_read(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从进程中获取 inode
    let process = PROCESSOR.lock().current_thread().process.clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        // 尝试读取
        if let Ok(ret) = inode.read_at(0, buffer) {
            let ret = ret as isize;
            if ret > 0 {
                return SyscallResult::Proceed(ret);
            }
            if ret == 0 {
                return SyscallResult::Park(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

/// 将字符写入指定的文件
pub(super) fn sys_write(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从进程中获取 inode
    let process = PROCESSOR.lock().current_thread().process.clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        // 尝试写入
        if let Ok(ret) = inode.write_at(0, buffer) {
            let ret = ret as isize;
            if ret >= 0 {
                return SyscallResult::Proceed(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

bitflags! {
    struct OpenFlags: usize {
        /// read only
        const RDONLY = 0;
        /// write only
        const WRONLY = 1;
        /// read write
        const RDWR = 2;
        /// create file if it does not exist
        const CREATE = 1 << 6;
        /// error if CREATE and the file exists
        const EXCLUSIVE = 1 << 7;
        /// truncate file upon open
        const TRUNCATE = 1 << 9;
        /// append on each write
        const APPEND = 1 << 10;
        /// close on exec
        const CLOEXEC = 1 << 19;
    }
}

/*
impl OpenFlags {
    fn readable(&self) -> bool {
        let b = self.bits() & 0b11;
        b == OpenFlags::RDONLY.bits() || b == OpenFlags::RDWR.bits()
    }
    fn writable(&self) -> bool {
        let b = self.bits() & 0b11;
        b == OpenFlags::WRONLY.bits() || b == OpenFlags::RDWR.bits()
    }
    fn to_options(&self) -> OpenOptions {
        OpenOptions {
            read: self.readable(),
            write: self.writable(),
            append: self.contains(OpenFlags::APPEND),
            nonblock: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OpenOptions {
    pub read: bool,
    pub write: bool,
    /// Before each write, the file offset is positioned at the end of the file.
    pub append: bool,
    pub nonblock: bool,
}
*/

const AT_FDCWD: usize = -100isize as usize;

