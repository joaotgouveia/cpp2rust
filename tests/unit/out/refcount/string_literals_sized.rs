extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut empty_buf: [i8; 256] = [0i8; 256];
    ({ cpp2rust_assert_fail_0(((empty_buf[(0) as usize] as i32) == (('\0' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0(((empty_buf[(255) as usize] as i32) == (('\0' as i8) as i32))) });
    let mut prefix_buf: [i8; 32] =
        b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".map(i8::from_byte);
    ({ cpp2rust_assert_fail_0(((prefix_buf[(0) as usize] as i32) == (('%' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0(((prefix_buf[(1) as usize] as i32) == (('\0' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0(((prefix_buf[(31) as usize] as i32) == (('\0' as i8) as i32))) });
    let mut short_buf: [i8; 16] = b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0".map(i8::from_byte);
    ({ cpp2rust_assert_fail_0(((short_buf[(0) as usize] as i32) == (('h' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0(((short_buf[(1) as usize] as i32) == (('i' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0(((short_buf[(2) as usize] as i32) == (('\0' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0(((short_buf[(15) as usize] as i32) == (('\0' as i8) as i32))) });
    let mut exact_buf: [i8; 6] = b"hello\0".map(i8::from_byte);
    ({ cpp2rust_assert_fail_0(((exact_buf[(0) as usize] as i32) == (('h' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0(((exact_buf[(4) as usize] as i32) == (('o' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0(((exact_buf[(5) as usize] as i32) == (('\0' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0((::std::mem::size_of::<[i8; 6]>() == 6_u64)) });
    ({
        cpp2rust_assert_fail_0(
            ((::std::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1_u64) == 5_u64),
        )
    });
    ({ cpp2rust_assert_fail_0((::std::mem::size_of::<[i8; 1]>() == 1_u64)) });
    ({
        cpp2rust_assert_fail_0(
            ((::std::mem::size_of::<[i8; 16]>() as u64).wrapping_sub(1_u64) == 15_u64),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
