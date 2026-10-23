extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
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
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_5(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_6(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_void__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_5(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl std::cmp::PartialEq for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn do_unique_7(l: *mut (), mut pred: Option<unsafe fn((), ()) -> ()>, mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn do_unique_8(l: *mut (), mut pred: (), mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn do_unique_9(l: *mut (), mut pred: Option<unsafe fn((), ()) -> ()>, mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct PredLWG526 {
    pub i_: (),
}
impl PredLWG526 {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.i_ = -32767_();
    }
    pub unsafe fn operator_call(&self, lhs: *const PredLWG526, rhs: *const PredLWG526) -> () {
        return (((*lhs).i_) == ((*rhs).i_));
    }
    pub unsafe fn operator_eq(&self, mut i: ()) -> () {
        return ((i) == (self.i_));
    }
}
pub unsafe fn g_10(mut x: (), mut y: ()) -> () {
    return ((x) == (y));
}
pub unsafe fn test_11() -> () {
    {
        let t1: [(); 7] = [0_(), 5_(), 5_(), 0_(), 0_(), 0_(), 5_()];
        let t2: [(); 4] = [0_(), 5_(), 0_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe { do_unique_7(&mut c1, Some(g_10), 3_()) });
        unimplemented!();
    }
    {
        let t1: [(); 4] = [0_(), 0_(), 0_(), 0_()];
        let t2: [(); 1] = [0_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe { do_unique_7(&mut c1, Some(g_10), 3_()) });
        unimplemented!();
    }
    {
        let t1: [(); 3] = [5_(), 5_(), 5_()];
        let t2: [(); 1] = [5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe { do_unique_7(&mut c1, Some(g_10), 2_()) });
        unimplemented!();
    }
    {
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe { do_unique_7(&mut c1, Some(g_10), 0_()) });
        unimplemented!();
    }
    {
        let t1: [(); 4] = [5_(), 5_(), 5_(), 0_()];
        let t2: [(); 2] = [5_(), 0_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe { do_unique_7(&mut c1, Some(g_10), 2_()) });
        unimplemented!();
    }
    {
        let mut a1: [(); 8] = [1_(), 1_(), 1_(), 2_(), 3_(), 5_(), 2_(), 11_()];
        let mut a2: [(); 6] = [1_(), 2_(), 3_(), 5_(), 2_(), 11_()];
        let mut c1: () = unimplemented!();
        (unsafe {
            let _l: *mut () = &mut c1;
            let _pred: () = unimplemented!();
            do_unique_8(_l, _pred, 2_())
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (6_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let t1: [(); 7] = [0_(), 5_(), 5_(), 0_(), 0_(), 0_(), 5_()];
        let t2: [(); 4] = [0_(), 5_(), 0_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe { do_unique_9(&mut c1, Some(g_10), 3_()) });
        unimplemented!();
    }
    {
        let t1: [(); 4] = [0_(), 0_(), 0_(), 0_()];
        let t2: [(); 1] = [0_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe { do_unique_9(&mut c1, Some(g_10), 3_()) });
        unimplemented!();
    }
    {
        let t1: [(); 3] = [5_(), 5_(), 5_()];
        let t2: [(); 1] = [5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe { do_unique_9(&mut c1, Some(g_10), 2_()) });
        unimplemented!();
    }
    {
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe { do_unique_9(&mut c1, Some(g_10), 0_()) });
        unimplemented!();
    }
    {
        let t1: [(); 4] = [5_(), 5_(), 5_(), 0_()];
        let t2: [(); 2] = [5_(), 0_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe { do_unique_9(&mut c1, Some(g_10), 2_()) });
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
