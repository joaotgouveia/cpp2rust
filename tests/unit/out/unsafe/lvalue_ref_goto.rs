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
    let mut __tmp_0: i32 = 5;
    let mut r: *const i32 = <*const i32>::default();
    goto_block!({
        '__entry: {
            r = &mut __tmp_0;
            goto!('body);
        }
        'body: {
            (if ((*r) == (5)) {
                &(0);
            } else {
                (unsafe {
                    __assert_fail_0(
                        c"r == 5".as_ptr(),
                        concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                        line!() as u32,
                        c"int main()".as_ptr(),
                    )
                });
            });
            return 0;
        }
    });
    panic!("ub: non-void function does not return a value")
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
