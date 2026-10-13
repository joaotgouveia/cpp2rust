extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
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
    let mut buf: [libc::c_char; 6] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (4 as libc::c_char),
        (5 as libc::c_char),
        (6 as libc::c_char),
    ];
    (unsafe {
        let ___dest: *mut ::libc::c_void =
            (buf.as_mut_ptr().offset((2) as isize) as *mut ::libc::c_void);
        let ___src: *const ::libc::c_void = (buf.as_mut_ptr() as *const ::libc::c_void);
        memmove_0(___dest, ___src, 4_usize)
    });
    (if ((buf[(0) as usize] as i32) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"buf[0] == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((buf[(1) as usize] as i32) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"buf[1] == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((buf[(2) as usize] as i32) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"buf[2] == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((buf[(3) as usize] as i32) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"buf[3] == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((buf[(4) as usize] as i32) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"buf[4] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((buf[(5) as usize] as i32) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"buf[5] == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn memmove_0(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
