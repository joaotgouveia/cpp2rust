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
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        return min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
}
pub unsafe fn operator_eq_4(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl std::cmp::PartialEq for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__int__ {}
pub unsafe fn node_factory_5(c: *mut (), key: *const (), mapped: *const ()) -> () {
    let mut it: () = unimplemented!();
    return unimplemented!();
}
pub unsafe fn node_factory_6(c: *mut (), key: *const (), mapped: *const ()) -> () {
    let mut it: () = unimplemented!();
    return unimplemented!();
}
pub unsafe fn test_7(c: *mut ()) {
    let mut nf: Option<unsafe fn(*mut (), *const (), *const ()) -> ()> = (Some(node_factory_5));
    let mut c2: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) != (10_())) {
        let mut node: () = (unsafe {
            let _arg1: *const () = &i;
            let mut _arg2: () = ((i) + (1_()));
            (nf).unwrap()(&mut c2, _arg1, &mut _arg2)
        });
        unimplemented!();
        let mut it: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    {
        let mut def: () = unimplemented!();
        let mut it: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut dupl: () = (unsafe {
            let mut _arg1: () = 0_();
            let mut _arg2: () = 42_();
            (nf).unwrap()(&mut c2, &mut _arg1, &mut _arg2)
        });
        let mut it: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) != (10_())) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_8(c: *mut ()) {
    let mut nf: Option<unsafe fn(*mut (), *const (), *const ()) -> ()> = (Some(node_factory_6));
    let mut c2: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) != (10_())) {
        let mut node: () = (unsafe {
            let _arg1: *const () = &i;
            let mut _arg2: () = ((i) + (1_()));
            (nf).unwrap()(&mut c2, _arg1, &mut _arg2)
        });
        unimplemented!();
        let mut it: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    {
        let mut def: () = unimplemented!();
        let mut it: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut dupl: () = (unsafe {
            let mut _arg1: () = 0_();
            let mut _arg2: () = 42_();
            (nf).unwrap()(&mut c2, &mut _arg1, &mut _arg2)
        });
        let mut it: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) != (10_())) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_9() -> () {
    let mut m: () = unimplemented!();
    (unsafe { test_7(&mut m) });
    let mut m2: () = unimplemented!();
    (unsafe { test_8(&mut m2) });
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
    (unsafe { test_9() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
