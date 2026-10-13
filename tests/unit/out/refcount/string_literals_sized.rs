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
    (if ((empty_buf[(0) as usize] as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"empty_buf[0] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((empty_buf[(255) as usize] as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"empty_buf[255] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut prefix_buf: [i8; 32] =
        b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".map(i8::from_byte);
    (if ((prefix_buf[(0) as usize] as i32) == (('%' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"prefix_buf[0] == \'%\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((prefix_buf[(1) as usize] as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"prefix_buf[1] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((prefix_buf[(31) as usize] as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"prefix_buf[31] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut short_buf: [i8; 16] = b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0".map(i8::from_byte);
    (if ((short_buf[(0) as usize] as i32) == (('h' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"short_buf[0] == \'h\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((short_buf[(1) as usize] as i32) == (('i' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"short_buf[1] == \'i\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((short_buf[(2) as usize] as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"short_buf[2] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((short_buf[(15) as usize] as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"short_buf[15] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut exact_buf: [i8; 6] = b"hello\0".map(i8::from_byte);
    (if ((exact_buf[(0) as usize] as i32) == (('h' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"exact_buf[0] == \'h\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((exact_buf[(4) as usize] as i32) == (('o' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"exact_buf[4] == \'o\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((exact_buf[(5) as usize] as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"exact_buf[5] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (6usize == 6_usize) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"sizeof(\"hello\") == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((6usize as usize).wrapping_sub(1_usize) == 5_usize) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"sizeof(\"hello\") - 1 == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (1usize == 1_usize) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"sizeof(\"\") == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((16usize as usize).wrapping_sub(1_usize) == 15_usize) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"sizeof(\"fifteen-bytes!!\") - 1 == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_0(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
