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
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_pair_const_char__short___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_char__short___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_std_pair_const_char__short___std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_std_pair_const_char__short___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_char__short___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { ptr_ : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_char__short___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_char__short___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_char__short__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_char__short___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        const _: () = assert!(true, "(std::is_same<C::key_type, char>::value)");;
        const _: () = assert!(true, "(std::is_same<C::mapped_type, short>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::hasher, std::hash<C::key_type> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::key_equal, std::equal_to<C::key_type> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::allocator_type, std::allocator<C::value_type> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::value_type, std::pair<const C::key_type, C::mapped_type> >::value)"
        );;
        const _: () = assert!(true, "(std::is_same<C::reference, C::value_type&>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_reference, const C::value_type&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<C::pointer, C::value_type*>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_pointer, const C::value_type*>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<C::size_type, std::size_t>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::difference_type, std::ptrdiff_t>::value)"
        );;
    }
    {
        const _: () = assert!(true, "(std::is_same<C::key_type, char>::value)");;
        const _: () = assert!(true, "(std::is_same<C::mapped_type, short>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::hasher, std::hash<C::key_type> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::key_equal, std::equal_to<C::key_type> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::allocator_type, min_allocator<C::value_type> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::value_type, std::pair<const C::key_type, C::mapped_type> >::value)"
        );;
        const _: () = assert!(true, "(std::is_same<C::reference, C::value_type&>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_reference, const C::value_type&>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::pointer, min_pointer<C::value_type>>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_pointer, min_pointer<const C::value_type>>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::size_type, std::make_unsigned<C::difference_type>::type>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::difference_type, std::ptrdiff_t>::value)"
        );;
    }
    return 0_();
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_char__short___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
