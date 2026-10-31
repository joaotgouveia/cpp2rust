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
pub unsafe fn operator_gt_4(lhs: *const value, rhs: *const value) -> bool {
    return (((*lhs).a) > ((*rhs).a));
}
pub unsafe fn operator_eq_5(lhs: *const value, rhs: *const value) -> bool {
    return (((*lhs).a) == ((*rhs).a)) && (((*lhs).b) == ((*rhs).b));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct value {
    pub a: i32,
    pub b: i32,
}
impl std::cmp::PartialEq for value {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self as *const value, other as *const value) }
    }
}
impl std::cmp::Eq for value {}
pub unsafe fn test_6() -> bool {
    {
        let t1: [i32; 6] = [13, 12, 7, 6, 5, 3];
        let t2: [i32; 10] = [15, 14, 11, 10, 9, 8, 4, 2, 1, 0];
        let t3: [i32; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        let mut c3: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let to: [i32; 3] = [2, 1, 0];
        let mut c2: () = unimplemented!();
        let mut io: [(); 3] = [
            unimplemented!(),
            (*unimplemented!().iter_inc()).clone(),
            (*(*unimplemented!().iter_inc()).iter_inc()).clone(),
        ];
        let mut ro: [*mut i32; 3] = [
            io[(0) as usize].iter_deref(),
            io[(1) as usize].iter_deref(),
            io[(2) as usize].iter_deref(),
        ];
        let mut po: [*mut i32; 3] = [
            (&mut (*io[(0) as usize].iter_deref()) as *mut i32),
            (&mut (*io[(1) as usize].iter_deref()) as *mut i32),
            (&mut (*io[(2) as usize].iter_deref()) as *mut i32),
        ];
        let mut c1: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (3_usize)) {
            assert!(((to[(i)]) == (*io[(i)].iter_deref())));
            assert!(((to[(i)]) == (*ro[(i)])));
            assert!(((to[(i)]) == (*po[(i)])));
            i.prefix_inc();
        }
    }
    {
        let t1: [value; 3] = [
            value { a: 3, b: 0 },
            value { a: 2, b: 0 },
            value { a: 0, b: 0 },
        ];
        let t2: [value; 4] = [
            value { a: 4, b: 1 },
            value { a: 2, b: 1 },
            value { a: 1, b: 1 },
            value { a: 0, b: 1 },
        ];
        let t3: [value; 7] = [
            value { a: 4, b: 1 },
            value { a: 3, b: 0 },
            value { a: 2, b: 0 },
            value { a: 2, b: 1 },
            value { a: 1, b: 1 },
            value { a: 0, b: 0 },
            value { a: 0, b: 1 },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        let mut c3: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let t1: [i32; 6] = [13, 12, 7, 6, 5, 3];
        let t2: [i32; 10] = [15, 14, 11, 10, 9, 8, 4, 2, 1, 0];
        let t3: [i32; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        let mut c3: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut a: [i32; 5] = [5, 4, 3, 2, 1];
        let mut c: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
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
    assert!((unsafe { test_6() }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
