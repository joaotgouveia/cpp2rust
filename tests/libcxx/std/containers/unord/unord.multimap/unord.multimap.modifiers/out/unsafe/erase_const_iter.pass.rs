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
impl TemplateConstructor {
    pub unsafe fn new(_a0: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TemplateConstructor {}
impl std::cmp::PartialEq for TemplateConstructor {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const TemplateConstructor,
                other as *const TemplateConstructor,
            )
        }
    }
}
impl std::cmp::Eq for TemplateConstructor {}
pub unsafe fn operator_eq_5(
    _a0: *const TemplateConstructor,
    _a1: *const TemplateConstructor,
) -> () {
    return false;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Hash {}
impl Hash {
    pub unsafe fn operator_call(&self, _a0: *const TemplateConstructor) -> () {
        return 0_();
    }
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
        let mut i: () = unimplemented!();
        let mut i_next: () = i.clone();
        unimplemented!();
        let mut es: () = unimplemented!();
        let mut j: () = unimplemented!();
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
        let mut k: () = eq.first.clone();
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
        let mut i_next: () = i.clone();
        unimplemented!();
        let mut es: () = unimplemented!();
        let mut j: () = unimplemented!();
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
        let mut k: () = eq.first.clone();
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
        let mut m: () = unimplemented!();
        let mut a: TemplateConstructor = {
            let mut __tmp_0: () = 0_();
            TemplateConstructor::new({ &mut __tmp_0 })
        };
        let mut it: () = unimplemented!();
        if !(unimplemented!()) {
            unimplemented!();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
