#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::string::String;

#[no_mangle]
pub fn main() -> usize {
    let mut fd: [u32; 2] = [0; 2];
    user_lib::sys_pipe2(&mut fd as *mut [u32] as *mut u32, 0);
    let tid = user_lib::sys_clone();
    let read = fd[0] as usize;
    let write = fd[1] as usize;
    if tid == 0 {
        let mut buffer: [u8; 32] = [0; 32];
        println!("This is child thread, read_fd is {}", read);
        user_lib::sys_read(read, &mut buffer);
        let mut res = String::new();
        for ch in buffer.iter() {
            res.push(*ch as char);
        }
        println!("I recived message: {}", res);
        user_lib::sys_close(read);
    } else {
        println!("This is parent thread, write fd is {}", write);
        let buf: &[u8] = b"Hello from pipe write";
        user_lib::sys_write(write, buf);
        println!("I've sent message!");
        user_lib::sys_close(write);
    }
    0
}

