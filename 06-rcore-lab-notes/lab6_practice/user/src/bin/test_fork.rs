#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> usize {
    let tid = user_lib::sys_clone();
    println!("I'm cloning my self, my id is {}", tid);
    0
}

