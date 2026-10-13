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
    let mut arr: [libc::c_char; 3] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
    ];
    {
        let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
        p = (p).wrapping_add((1_u8 as i32) as usize);
        (if (((*p) as i32) == (2)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"*p == 2".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    {
        let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
        p = (p).wrapping_add((1_i8 as i32) as usize);
        (if (((*p) as i32) == (2)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"*p == 2".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    {
        let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
        p = (p).wrapping_add((1_u16 as i32) as usize);
        (if (((*p) as i32) == (2)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"*p == 2".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    {
        let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
        p = (p).wrapping_add((1_i16 as i32) as usize);
        (if (((*p) as i32) == (2)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"*p == 2".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    {
        let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
        p = (p).wrapping_add((1_u32 as u32) as usize);
        (if (((*p) as i32) == (2)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"*p == 2".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    {
        let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
        p = (p).wrapping_add(((1 as i32) as i32) as usize);
        (if (((*p) as i32) == (2)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"*p == 2".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    {
        let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
        p = (p).wrapping_add((1_u64 as u64) as usize);
        (if (((*p) as i32) == (2)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"*p == 2".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    {
        let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
        p = (p).wrapping_add((1_i64 as i64) as usize);
        (if (((*p) as i32) == (2)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"*p == 2".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
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
