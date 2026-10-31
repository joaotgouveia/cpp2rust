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
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn test_4() -> bool {
    {
        let t: [i32; 5] = [0, 1, 2, 3, 4];
        let mut c: () = unimplemented!();
        let mut i: () = unimplemented!();
        assert!(i == unimplemented!());
        assert!(((unimplemented!()) == (4_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (0)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (3)));
        i = unimplemented!();
        assert!(i == unimplemented!());
        assert!(((unimplemented!()) == (3_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (3)));
        i = unimplemented!();
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 1_isize));
        assert!(((unimplemented!()) == (2_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (3)));
        i = unimplemented!();
        assert!(i == unimplemented!());
        assert!(((unimplemented!()) == (1_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        i = unimplemented!();
        assert!(i == unimplemented!());
        assert!(i == unimplemented!());
        assert!(((unimplemented!()) == (0_isize)));
    }
    {
        let t: [i32; 5] = [0, 1, 2, 3, 4];
        let mut c: () = unimplemented!();
        let mut i: () = unimplemented!();
        assert!(i == unimplemented!());
        assert!(((unimplemented!()) == (4_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (0)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (3)));
        i = unimplemented!().clone();
        assert!(i == unimplemented!());
        assert!(((unimplemented!()) == (3_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (3)));
        i = unimplemented!().clone();
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 1_isize));
        assert!(((unimplemented!()) == (2_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (3)));
        i = unimplemented!().clone();
        assert!(i == unimplemented!());
        assert!(((unimplemented!()) == (1_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        i = unimplemented!().clone();
        assert!(i == unimplemented!());
        assert!(i == unimplemented!());
        assert!(((unimplemented!()) == (0_isize)));
    }
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
    assert!((unsafe { test_4() }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
