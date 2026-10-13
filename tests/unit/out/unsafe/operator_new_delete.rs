extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_exception {}
impl Clone for std_exception {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_exception {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_from(_a0: *const std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_from(_a0: *mut std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_assign(&mut self, _a0: *mut std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_bad_alloc {}
impl Clone for std_bad_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bad_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_from(_a0: *const std_bad_alloc) -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_bad_alloc) -> *mut std_bad_alloc {
        unimplemented!()
    }
}
pub type std_align_val_t = usize;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_nothrow_t {}
impl Clone for std_nothrow_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_nothrow_t {
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
    let mut a: *mut i32 = ((unsafe { operator_new_0(::std::mem::size_of::<i32>()) }) as *mut i32);
    (*a) = 42;
    (if ((*a) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"*a == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { operator_delete_2((a as *mut ::libc::c_void)) });
    let mut arr: *mut i32 = ((unsafe {
        operator_new_array_3((::std::mem::size_of::<i32>() as usize).wrapping_mul(2_usize))
    }) as *mut i32);
    (*arr.offset((0) as isize)) = 0;
    (*arr.offset((1) as isize)) = 1;
    (if (((*arr.offset((0) as isize)) + (*arr.offset((1) as isize))) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"arr[0] + arr[1] == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { operator_delete_array_4((arr as *mut ::libc::c_void)) });
    return 0;
}
pub unsafe fn operator_new_0(mut _a0: usize) -> *mut ::libc::c_void {
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
pub unsafe fn operator_delete_2(mut _a0: *mut ::libc::c_void) {
    unimplemented!()
}
pub unsafe fn operator_new_array_3(mut _a0: usize) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn operator_delete_array_4(mut _a0: *mut ::libc::c_void) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
