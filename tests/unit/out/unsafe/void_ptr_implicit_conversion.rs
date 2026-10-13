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
pub unsafe fn bump_0(mut arg: *mut ::libc::c_void) -> i32 {
    let mut value: *mut i32 = (arg as *mut i32);
    (*value) += 1;
    return (*value);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut value: i32 = 41;
    let mut opaque: *mut ::libc::c_void = ((&mut value as *mut i32) as *mut ::libc::c_void);
    let mut typed: *mut i32 = (opaque as *mut i32);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { bump_0(opaque) }) == (42)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"bump(opaque) == 42".as_ptr().cast_mut()).cast_const(),
                        (c"void_ptr_implicit_conversion.c".as_ptr().cast_mut()).cast_const(),
                        14_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((*typed) == (42)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"*typed == 42".as_ptr().cast_mut()).cast_const(),
                        (c"void_ptr_implicit_conversion.c".as_ptr().cast_mut()).cast_const(),
                        15_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    (*typed) = 7;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((value) == (7)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"value == 7".as_ptr().cast_mut()).cast_const(),
                        (c"void_ptr_implicit_conversion.c".as_ptr().cast_mut()).cast_const(),
                        17_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
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
