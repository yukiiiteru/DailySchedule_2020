#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> usize {
    let tid = user_lib::sys_gettid();
    println!("My thread id is {}", tid);
    0
}

