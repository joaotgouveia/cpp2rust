extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut x: Vec<UnsafeRandomAccessIterator<i32>> = (0..(2_usize) as usize)
        .map(|_| UnsafeRandomAccessIterator::default())
        .collect::<Vec<_>>();
    let mut i2: UnsafeRandomAccessIterator<i32> = UnsafeRandomAccessIterator::default();
    let mut i3: UnsafeRandomAccessIterator<i32> = UnsafeRandomAccessIterator::default();
    let mut ci1: UnsafeRandomAccessIterator<i32> =
        <UnsafeRandomAccessIterator<i32>>::from((i2).clone());
    i2 = i3.clone();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
