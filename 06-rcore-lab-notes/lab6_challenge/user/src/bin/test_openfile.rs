#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::string::String;

#[no_mangle]
pub fn main() -> usize {
    let fd = user_lib::sys_open("hello", 0, 0) as usize;
    let mut buffer: [u8; 32] = [0; 32];
    user_lib::sys_read(fd, &mut buffer);
    let mut res = String::new();
    for ch in buffer.iter() {
        res.push(*ch as char);
    }
    println!("{}", res);
    user_lib::sys_close(fd);
    0
}

