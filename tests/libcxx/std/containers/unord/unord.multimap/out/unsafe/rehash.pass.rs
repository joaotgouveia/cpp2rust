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
pub struct min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub unsafe fn rehash_postcondition_4(c: *const (), mut n: ()) {
    unimplemented!();
}
pub unsafe fn rehash_postcondition_5(c: *const (), mut n: ()) {
    unimplemented!();
}
pub unsafe fn test_6(c: *const ()) {
    unimplemented!();
    let mut eq: () = unimplemented!();
    unimplemented!();
    let mut i: () = eq.first.clone();
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) < (2_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            n.prefix_inc();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) < (2_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            n.prefix_inc();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_7(c: *const ()) {
    unimplemented!();
    let mut eq: () = unimplemented!();
    unimplemented!();
    let mut i: () = eq.first.clone();
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) < (2_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            n.prefix_inc();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) < (2_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            n.prefix_inc();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
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
        let mut a: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut c: () = unimplemented!();
        (unsafe { test_6(&c) });
        unimplemented!();
        unimplemented!();
        (unsafe { rehash_postcondition_4(&c, 3_()) });
        const _: () = assert!(true, "");;
        (unsafe { test_6(&c) });
        unimplemented!();
        unimplemented!();
        (unsafe { rehash_postcondition_4(&c, 3_()) });
        const _: () = assert!(true, "");;
        (unsafe { test_6(&c) });
        unimplemented!();
        (unsafe { rehash_postcondition_4(&c, 31_()) });
        const _: () = assert!(true, "");;
        (unsafe { test_6(&c) });
    }
    {
        let mut a: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut c: () = unimplemented!();
        (unsafe { test_7(&c) });
        unimplemented!();
        unimplemented!();
        (unsafe { rehash_postcondition_5(&c, 3_()) });
        const _: () = assert!(true, "");;
        (unsafe { test_7(&c) });
        unimplemented!();
        unimplemented!();
        (unsafe { rehash_postcondition_5(&c, 3_()) });
        const _: () = assert!(true, "");;
        (unsafe { test_7(&c) });
        unimplemented!();
        (unsafe { rehash_postcondition_5(&c, 31_()) });
        const _: () = assert!(true, "");;
        (unsafe { test_7(&c) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
