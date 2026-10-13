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
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut val: u64 = 578437695752307201_u64;
    let mut dwords: *mut u32 = ((&mut val as *mut u64) as *mut u32);
    (if ((*dwords.offset((0) as isize)) == (67305985_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"dwords[0] == 0x04030201".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*dwords.offset((1) as isize)) == (134678021_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"dwords[1] == 0x08070605".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut words: *mut u16 = (dwords as *mut u16);
    (if (((*words.offset((0) as isize)) as i32) == (513)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[0] == 0x0201".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*words.offset((1) as isize)) as i32) == (1027)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[1] == 0x0403".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*words.offset((2) as isize)) as i32) == (1541)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[2] == 0x0605".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*words.offset((3) as isize)) as i32) == (2055)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[3] == 0x0807".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*words.offset((1) as isize)) = 48042_u16;
    (if ((*dwords.offset((0) as isize)) == (3148481025_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"dwords[0] == 0xBBAA0201".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((val) == (578437698833482241_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"val == 0x08070605BBAA0201ULL".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*dwords.offset((1) as isize)) = 4293844428_u32;
    (if ((val) == (18441921395520307713_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"val == 0xFFEEDDCCBBAA0201ULL".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*words.offset((2) as isize)) as i32) == (56780)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[2] == 0xDDCC".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*words.offset((3) as isize)) as i32) == (65518)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[3] == 0xFFEE".as_ptr(),
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
