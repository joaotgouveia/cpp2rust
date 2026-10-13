extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[repr(C, align(16))]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub c: libc::c_char,
    pub x: i64,
}
pub unsafe fn pack_size_0() -> u64 {
    return ((0 as usize).wrapping_add((0 as usize)) as u64);
}
pub unsafe fn pack_size_1(mut args_0: i32, mut args_1: f64) -> u64 {
    return ((2 as usize).wrapping_add((2 as usize)) as u64);
}
pub unsafe fn may_throw_2() {}
pub unsafe fn no_throw_3() {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [i64; 4] = [0_i64, 0_i64, 0_i64, 0_i64];
    let mut s: S = <S>::default();
    (if ((::std::mem::size_of::<i32>()) == (4_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"sizeof(int32_t) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((::std::mem::size_of::<[i64; 4]>()) == (32_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"sizeof(arr) == 32".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((::std::mem::size_of::<S>()) == (16_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"sizeof(s) == 16".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((::std::mem::align_of::<i32>()) == (4_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"alignof(int32_t) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((::std::mem::align_of::<S>()) == (16_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"alignof(S) == 16".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((::std::mem::align_of::<[i64; 4]>()) == (8_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"__alignof(arr) == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((::std::mem::align_of::<S>()) == (16_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"__alignof(s) == 16".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { pack_size_0() }) == (0_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"pack_size() == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { pack_size_1(1, 2.0E+0) }) == (4_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"pack_size(1, 2.0) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if true {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"noexcept(no_throw())".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(false) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"!noexcept(may_throw())".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut may: bool = false;
    (if !(may) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"!may".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
