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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Item {
    pub flags: u8,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut item: Item = Item { flags: 0_u8 };
    let mut ptr: *mut Item = (&mut item as *mut Item);
    (*(ptr)).flags = (((*(ptr)).flags as i32) | ((1) << (0))) as u8;
    (*(ptr)).flags = (((*(ptr)).flags as i32) | ((1) << (1))) as u8;
    (if (((*ptr).flags as i32) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"ptr->flags == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(ptr)).flags = (((*(ptr)).flags as i32) & ((!((1) << (0)) as u8) as i32)) as u8;
    (if (((*ptr).flags as i32) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"ptr->flags == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut bits: [u8; 4] = [0_u8, 0_u8, 0_u8, 0_u8];
    (bits)[((5) / (8)) as usize] =
        (((bits)[((5) / (8)) as usize] as i32) | ((((1) << ((5) & (7))) as u8) as i32)) as u8;
    (bits)[((13) / (8)) as usize] =
        (((bits)[((13) / (8)) as usize] as i32) | ((((1) << ((13) & (7))) as u8) as i32)) as u8;
    (if ((bits[(0) as usize] as i32) == (32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bits[0] == 32".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((bits[(1) as usize] as i32) == (32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bits[1] == 32".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((bits[(2) as usize] as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bits[2] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    if (((*ptr).flags as i32) != (0)) {
        (*(ptr)).flags = (((*(ptr)).flags as i32) & ((!((1) << (1)) as u8) as i32)) as u8;
    }
    (if (((*ptr).flags as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"ptr->flags == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
