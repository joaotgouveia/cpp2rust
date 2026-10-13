extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut out: i32 = 0;
    let mut arr: [i32; 5] = [1, 2, 3, 4, 0];
    let mut ptr: *const i32 = (&mut arr[(0) as usize] as *mut i32).cast_const();
    'loop_: while ((*ptr) != (0)) {
        out += (*ptr);
        ptr.prefix_inc();
    }
    let mut ptr: *mut i32 = (&mut arr[(1) as usize] as *mut i32);
    'loop_: while ((*ptr) != (4)) {
        out += (*ptr);
        ptr.postfix_inc();
    }
    let mut ptr: *mut i32 = (&mut arr[(4) as usize] as *mut i32);
    'loop_: while ((*ptr) != (1)) {
        out += (*ptr);
        ptr.postfix_dec();
    }
    let mut ptr: *const i32 = (&mut arr[(3) as usize] as *mut i32).cast_const();
    'loop_: while ((*ptr) != (2)) {
        out += (*ptr);
        ptr.prefix_dec();
    }
    let mut ptr: *mut i32 = (&mut arr[(0) as usize] as *mut i32);
    'loop_: while ((*ptr) != (0)) {
        out += (*ptr);
        ptr = ptr.offset((1) as isize);
    }
    let mut ptr: *mut i32 = (&mut arr[(0) as usize] as *mut i32);
    let mut i: i32 = 0;
    'loop_: while ((i) < (5)) {
        out += (*ptr.offset((i) as isize));
        i.prefix_inc();
    }
    (if ((out) == (51)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"out == 51".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut step: i64 = 2_i64;
    let mut offset: *mut i32 = arr.as_mut_ptr().offset((step) as isize);
    (if ((*offset) == (arr[(2) as usize])) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*offset == arr[2]".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    offset = offset.offset(-((step) as isize));
    (if ((offset) == (&mut arr[(0) as usize] as *mut i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"offset == &arr[0]".as_ptr(),
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
