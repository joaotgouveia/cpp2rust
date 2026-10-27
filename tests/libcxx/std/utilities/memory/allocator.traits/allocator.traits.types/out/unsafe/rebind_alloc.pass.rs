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
pub struct ReboundA_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_char__rebind_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReboundB_double__char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_int__char__rebind_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_int__char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D_int__char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D_double__char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct E_char__rebind_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct E_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct E_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct F_char__rebind_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct F_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct F_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct G_char__rebind_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct G_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct G_double_ {}
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
        "(std::is_same<std::allocator_traits<A<char> >::rebind_alloc<double>, ReboundA<double> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::allocator_traits<B<int, char> >::rebind_alloc<double>, ReboundB<double, char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::allocator_traits<C<char> >::rebind_alloc<double>, C<double> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::allocator_traits<D<int, char> >::rebind_alloc<double>, D<double, char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::allocator_traits<E<char> >::rebind_alloc<double>, E<double> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::allocator_traits<F<char> >::rebind_alloc<double>, F<double> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::allocator_traits<G<char> >::rebind_alloc<double>, G<double> >::value)"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
