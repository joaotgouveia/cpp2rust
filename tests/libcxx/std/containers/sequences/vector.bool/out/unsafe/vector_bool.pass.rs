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
pub struct min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {
    pub unsafe fn new_1(mut _a0: min_allocator_bool_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_unsigned_long_ {}
impl min_allocator_unsigned_long_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_unsigned_long_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            let ___p: *mut u64 = p.ptr_;
            let ___n: usize = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
impl min_allocator_unsigned_long_ {}
pub unsafe fn test_4() {
    const _: () = assert!(true, "");;
    let mut ba: [bool; 5] = [true, false, true, true, false];
    let mut vb: Vec<bool> = {
        let mut __v = Vec::new();
        let mut __it = (unsafe { begin_5(&mut ba) });
        while __it != (unsafe { end_6(&mut ba) }) {
            __v.push(<bool>::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    let mut h: std_hash_std_vector_bool__std_allocator_bool___ =
        unsafe { std::mem::zeroed::<std_hash_std_vector_bool__std_allocator_bool___>() };
    {
        let hash_value: usize = (unsafe { h(&vb) });
        assert!(((unsafe { h(&vb,) }) == (hash_value)));
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_7() {
    const _: () = assert!(true, "");;
    let mut ba: [bool; 5] = [true, false, true, true, false];
    let mut vb: Vec<bool> = {
        let mut __v = Vec::new();
        let mut __it = (unsafe { begin_5(&mut ba) });
        while __it != (unsafe { end_6(&mut ba) }) {
            __v.push(<bool>::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    let mut h: std_hash_std_vector_bool__min_allocator_bool___ =
        unsafe { std::mem::zeroed::<std_hash_std_vector_bool__min_allocator_bool___>() };
    {
        let hash_value: usize = (unsafe { h(&vb) });
        assert!(((unsafe { h(&vb,) }) == (hash_value)));
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn tests_8() -> bool {
    (unsafe { test_4() });
    (unsafe { test_7() });
    return true;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { tests_8() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
