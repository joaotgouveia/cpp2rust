extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn bar_0() -> i32 {
    return 1;
}
pub unsafe fn foo_1(x: *mut i32) -> *mut i32 {
    return x;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 5;
    let mut y: i32 = (*(unsafe { foo_1(&mut x) }));
    let z: *mut i32 = (unsafe { foo_1(&mut x) });
    (if (((((*(unsafe { foo_1(&mut x) })) + (*(unsafe { foo_1(&mut y) })))
        + (*(unsafe { foo_1(z) })))
        + (unsafe { bar_0() }))
        == (16))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"foo(x) + foo(y) + foo(z) + bar() == 16".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
