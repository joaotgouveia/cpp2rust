extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
pub static mut S_c_0: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe { 5_usize });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
pub static mut Table_char__table_size_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 256_usize });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Table_1 {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pa: *const usize =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut S_c_0)) as *const usize);
    (if (((*pa).wrapping_add(1_usize)) == (6_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"*pa + 1 == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut G: *const usize =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut Table_char__table_size_2))
            as *const usize);
    (if ((*G) >= (256_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"*G >= 256".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_3(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const S_c_0);
    std::cell::LazyCell::force(&*&raw const Table_char__table_size_2);
}
