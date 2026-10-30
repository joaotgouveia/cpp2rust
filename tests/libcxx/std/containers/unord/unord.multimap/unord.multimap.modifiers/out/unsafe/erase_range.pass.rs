extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn CheckConsecutiveKeys_0(mut pos: (), mut end: (), key: (), values: *mut ()) {
    'loop_: while !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_4)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)) = 0_();
    }
}
pub static mut outstanding_bytes_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_4: std::cell::LazyCell<()> =
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
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut k: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut eq: () = unimplemented!();
        unimplemented!();
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = unimplemented!();
            let _end: () = unimplemented!();
            CheckConsecutiveKeys_0(_pos, _end, 1_(), &mut s)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = unimplemented!();
            let _end: () = unimplemented!();
            CheckConsecutiveKeys_0(_pos, _end, 2_(), &mut s)
        });
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
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = unimplemented!();
            let _end: () = unimplemented!();
            CheckConsecutiveKeys_0(_pos, _end, 1_(), &mut s)
        });
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
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut map: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut map: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut map: () = unimplemented!();
        let mut collision_val: () = (2_()).wrapping_add(unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        let mut erase: () = unimplemented!();
        unimplemented!();
        'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
            let v: *const () = &(*unimplemented!());
            unimplemented!();
        }
    }
    {
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut pair: () = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut pair: () = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut k: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut eq: () = unimplemented!();
        unimplemented!();
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = unimplemented!();
            let _end: () = unimplemented!();
            CheckConsecutiveKeys_0(_pos, _end, 1_(), &mut s)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = unimplemented!();
            let _end: () = unimplemented!();
            CheckConsecutiveKeys_0(_pos, _end, 2_(), &mut s)
        });
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
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = unimplemented!();
            let _end: () = unimplemented!();
            CheckConsecutiveKeys_0(_pos, _end, 1_(), &mut s)
        });
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
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyHash_5 {}
impl MyHash_5 {
    pub unsafe fn operator_call(&self, mut v: ()) -> () {
        return v;
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
