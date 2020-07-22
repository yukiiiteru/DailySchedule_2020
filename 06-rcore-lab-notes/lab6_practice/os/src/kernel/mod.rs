//! 为进程提供系统调用等内核功能

mod condvar;
mod fs;
mod process;
mod thread;
mod syscall;

use crate::interrupt::*;
use crate::process::*;
use alloc::{string::String, sync::Arc, vec::Vec};
pub(self) use fs::*;
pub(self) use process::*;
pub(self) use thread::*;
use spin::Mutex;
pub(self) use syscall::*;

pub use condvar::Condvar;
pub use syscall::syscall_handler;

pub fn check_and_clone_cstr(user: *const u8) -> Result<String, String> {
    if user.is_null() {
        Ok(String::new())
    } else {
        let mut buffer = Vec::new();
        for i in 0.. {
            let addr = unsafe { user.add(i) };
            let data = copy_from_user(addr).ok_or(String::from("SysError::EFAULT"))?;
            if data == 0 {
                break;
            }
            buffer.push(data);
        }
        String::from_utf8(buffer).map_err(|_| String::from("SysError::EFAULT"))
    }
}

pub fn copy_from_user<T>(addr: *const T) -> Option<T> {
    // #[naked]
    // #[inline(never)]
    // #[link_section = ".text.copy_user"]
    unsafe extern "C" fn read_user<T>(dst: *mut T, src: *const T) -> usize {
        dst.copy_from_nonoverlapping(src, 1);
        0
    }
    let mut dst: T = unsafe { core::mem::zeroed() };
    match unsafe { read_user(&mut dst, addr) } {
        0 => Some(dst),
        _ => None,
    }
}

