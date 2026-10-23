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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn g_5(mut x: (), mut y: ()) -> () {
    return ((x) == (y));
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
pub unsafe fn test_6() -> () {
    {
        let mut a1: [(); 9] = [2_(), 1_(), 1_(), 4_(), 4_(), 4_(), 4_(), 3_(), 3_()];
        let mut a2: [(); 4] = [2_(), 1_(), 4_(), 3_()];
        let mut c: () = std_list_int__std_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset(
                ((::std::mem::size_of::<[(); 9]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ()))) as isize,
            )
        });
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a1: [(); 9] = [1_(), 1_(), 1_(), 2_(), 3_(), 5_(), 5_(), 2_(), 11_()];
        let mut a2: [(); 6] = [1_(), 2_(), 3_(), 5_(), 2_(), 11_()];
        let mut c: () =
            std_list_PredLWG526__std_allocator_PredLWG526__::new_1({ a1.as_mut_ptr() }, {
                a1.as_mut_ptr().offset((9_()) as isize)
            });
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut a1: [(); 9] = [2_(), 1_(), 1_(), 4_(), 4_(), 4_(), 4_(), 3_(), 3_()];
        let mut a2: [(); 4] = [2_(), 1_(), 4_(), 3_()];
        let mut c: () = std_list_int__min_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset(
                ((::std::mem::size_of::<[(); 9]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ()))) as isize,
            )
        });
        unimplemented!();
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
