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
pub unsafe fn only_deletions_4(whole: *const (), part: *const ()) -> bool {
    let mut w: () = unimplemented!();
    let mut p: () = unimplemented!();
    'loop_: while (w != unimplemented!()) && (p != unimplemented!()) {
        if ((*w.iter_deref()) == (*p.iter_deref())) {
            p.iter_post_inc();
        }
        w.iter_post_inc();
    }
    return p == unimplemented!();
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
    {
        let mut a: [i32; 6] = [1, 2, 3, 4, 1, 2];
        let mut c: () = unimplemented!();
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (6_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (4_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (4_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (0_())));
    }
    {
        let mut a: [i32; 6] = [1, 2, 3, 4, 1, 2];
        let mut c: () = unimplemented!();
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (6_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (4_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (4_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(((unimplemented!()) == (0_())));
    }
    {
        let mut m: () = unimplemented!();
        let mut m2: () = unimplemented!();
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = unimplemented!();
        let mut ctr: i32 = 0;
        'loop_: while i != unimplemented!() {
            if (((ctr.postfix_inc()) % (2)) == (0)) {
                unimplemented!();
            } else {
                i.iter_inc();
            }
        }
        assert!((unsafe { only_deletions_4(&m, &m2,) }));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
