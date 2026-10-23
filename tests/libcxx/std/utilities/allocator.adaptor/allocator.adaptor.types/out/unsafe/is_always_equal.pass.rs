extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A1_int_ {
    id_: (),
}
impl A1_int_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A2_int_ {
    id_: (),
}
impl A2_int_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A3_int_ {
    id_: (),
}
impl A3_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_12))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_13))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_14)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_11)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_12)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_13)) = 0_();
    }
}
pub static mut outstanding_bytes_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    const _: () = assert!(
        true,
        "(std::is_same< std::allocator_traits<A1<int>>::is_always_equal, std::false_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< std::allocator_traits<min_allocator<int>>::is_always_equal, std::true_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< std::scoped_allocator_adaptor<A1<int>>::is_always_equal,\n                               std::allocator_traits<A1<int>>::is_always_equal >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< std::scoped_allocator_adaptor<min_allocator<int>>::is_always_equal,\n                               std::allocator_traits<min_allocator<int>>::is_always_equal >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::scoped_allocator_adaptor<A1<int>, A2<int>>::is_always_equal::value ==\n                 (std::allocator_traits<A1<int>>::is_always_equal::value &&\n                  std::allocator_traits<A2<int>>::is_always_equal::value))"
    );;
    const _: () = assert!(
        true,
        "(std::scoped_allocator_adaptor<A1<int>, min_allocator<int>>::is_always_equal::value ==\n                 (std::allocator_traits<A1<int>>::is_always_equal::value &&\n                  std::allocator_traits<min_allocator<int>>::is_always_equal::value))"
    );;
    const _: () = assert!(
        true,
        "(std::scoped_allocator_adaptor<A1<int>, A2<int>, A3<int>>::is_always_equal::value ==\n                 (std::allocator_traits<A1<int>>::is_always_equal::value &&\n                  std::allocator_traits<A2<int>>::is_always_equal::value &&\n                  std::allocator_traits<A3<int>>::is_always_equal::value))"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_11);
    std::cell::LazyCell::force(&*&raw const alloc_count_12);
    std::cell::LazyCell::force(&*&raw const dealloc_count_13);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_14);
}
