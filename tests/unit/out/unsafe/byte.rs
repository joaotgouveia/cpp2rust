extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __byte_operand_0 {}
impl Clone for __byte_operand_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __byte_operand_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut b1: std_byte = 1_std_byte;
    let mut ushift1: u32 = 3_u32;
    let mut shl1: std_byte = (unsafe {
        let ___b: std_byte = b1;
        operator_shl_1(___b, ushift1)
    });
    (if ((shl1) == ((8) as std_byte)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"shl1 == std::byte(0x08)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ushift2: u32 = 2_u32;
    let mut shr1: std_byte = (unsafe {
        let ___b: std_byte = shl1;
        operator_shr_3(___b, ushift2)
    });
    (if ((shr1) == ((2) as std_byte)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"shr1 == std::byte(0x02)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ushift3: u32 = 5_u32;
    (unsafe {
        let ___b: *mut std_byte = &mut b1;
        operator_shl_assign_4(___b, ushift3)
    });
    (if ((b1) == ((32) as std_byte)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"b1 == std::byte(0x20)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ushift4: u32 = 3_u32;
    (unsafe {
        let ___b: *mut std_byte = &mut b1;
        operator_shr_assign_5(___b, ushift4)
    });
    (if ((b1) == ((4) as std_byte)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"b1 == std::byte(0x04)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn operator_shl_1(mut __b: std_byte, mut __shift: u32) -> std_byte {
    unimplemented!()
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn operator_shr_3(mut __b: std_byte, mut __shift: u32) -> std_byte {
    unimplemented!()
}
pub unsafe fn operator_shl_assign_4(__b: *mut std_byte, mut __shift: u32) -> *mut std_byte {
    unimplemented!()
}
pub unsafe fn operator_shr_assign_5(__b: *mut std_byte, mut __shift: u32) -> *mut std_byte {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
