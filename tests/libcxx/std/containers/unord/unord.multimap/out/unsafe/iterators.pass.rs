extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut (i32, Vec<libc::c_char>),
}
impl min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut a: [(i32, Vec<libc::c_char>); 6] = [
            (
                1,
                std::slice::from_raw_parts(
                    c"one".as_ptr(),
                    libcc2rs::c_str_len(c"one".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"two".as_ptr(),
                    libcc2rs::c_str_len(c"two".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                3,
                std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                4,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                1,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let mut c: () = unimplemented!();
        assert!(((unimplemented!()) >= (7_())));
        assert!(((unimplemented!()) == (6_())));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
        i = unimplemented!();
        unimplemented!();
        assert!(
            (*i.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"ONE".as_ptr(),
                    libcc2rs::c_str_len(c"ONE".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 6] = [
            (
                1,
                std::slice::from_raw_parts(
                    c"one".as_ptr(),
                    libcc2rs::c_str_len(c"one".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"two".as_ptr(),
                    libcc2rs::c_str_len(c"two".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                3,
                std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                4,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                1,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let c: () = unimplemented!();
        assert!(((unimplemented!()) >= (7_())));
        assert!(((unimplemented!()) == (6_())));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 6] = [
            (
                1,
                std::slice::from_raw_parts(
                    c"one".as_ptr(),
                    libcc2rs::c_str_len(c"one".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"two".as_ptr(),
                    libcc2rs::c_str_len(c"two".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                3,
                std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                4,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                1,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let mut c: () = unimplemented!();
        assert!(((unimplemented!()) >= (7_())));
        assert!(((unimplemented!()) == (6_())));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
        i = unimplemented!();
        unimplemented!();
        assert!(
            (*i.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"ONE".as_ptr(),
                    libcc2rs::c_str_len(c"ONE".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 6] = [
            (
                1,
                std::slice::from_raw_parts(
                    c"one".as_ptr(),
                    libcc2rs::c_str_len(c"one".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"two".as_ptr(),
                    libcc2rs::c_str_len(c"two".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                3,
                std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                4,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                1,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let c: () = unimplemented!();
        assert!(((unimplemented!()) >= (7_())));
        assert!(((unimplemented!()) == (6_())));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
    }
    {
        let mut ii1: () = unimplemented!();
        let mut ii2: () = unimplemented!();
        let mut ii4: () = ii1.clone();
        let mut cii: () = unimplemented!();
        assert!(ii1 == ii2);
        assert!(ii1 == ii4);
        assert!(!(ii1 != ii2));
        assert!((ii1 == cii));
        assert!((cii == ii1));
        assert!(!(ii1 != cii));
        assert!(!(cii != ii1));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
