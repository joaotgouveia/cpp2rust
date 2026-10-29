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
pub struct min_allocator_int_ {}
pub static mut randomness_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn test_5(mut N: ()) {
    let mut v: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut j: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        {
            i.prefix_inc();
            unimplemented!()
        };
    }
}
pub unsafe fn test_6(mut N: ()) {
    let mut v: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut j: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        {
            i.prefix_inc();
            unimplemented!()
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Payload {
    pub val: (),
    pub side: (),
}
impl Payload {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v, side: 0_() };
        this
    }
    pub unsafe fn new_2(mut v: (), mut s: ()) -> Self {
        let mut this = Self { val: v, side: s };
        this
    }
    pub unsafe fn operator_lt(&self, rhs: *const Payload) -> () {
        return ((self.val) < ((*rhs).val));
    }
}
impl std::cmp::Ord for Payload {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Payload::operator_lt(self, other as *const Payload) {
                std::cmp::Ordering::Less
            } else if Payload::operator_lt(other, self as *const Payload) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Payload {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Payload {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Payload::operator_lt(self, other as *const Payload))
                && !(Payload::operator_lt(other, self as *const Payload))
        }
    }
}
impl std::cmp::Eq for Payload {}
pub unsafe fn test_stable_7(mut N: ()) {
    let mut v: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*unimplemented!()).side = i;
        i.prefix_inc();
    }
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut j: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        {
            i.prefix_inc();
            unimplemented!()
        };
    }
    let mut it: () = unimplemented!();
    'loop_: while !(unimplemented!()) {
        let mut next: () = unimplemented!();
        if (!(unimplemented!())) && (((*unimplemented!()).val) == ((*unimplemented!()).val)) {
            unimplemented!();
        }
        unimplemented!();
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
    let mut i: () = 0_();
    'loop_: while ((i) < (40_())) {
        (unsafe { test_5(i) });
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (40_())) {
        (unsafe { test_6(i) });
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (40_())) {
        (unsafe { test_stable_7(i) });
        i.prefix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const randomness_4);
}
