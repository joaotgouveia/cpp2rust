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
pub unsafe fn operator_lt_4(lhs: *const value, rhs: *const value) -> () {
    return (((*lhs).a) < ((*rhs).a));
}
pub unsafe fn operator_eq_5(lhs: *const value, rhs: *const value) -> () {
    return (((*lhs).a) == ((*rhs).a)) && (((*lhs).b) == ((*rhs).b));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct value {
    pub a: (),
    pub b: (),
}
impl std::cmp::Ord for value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_4(self as *const value, other as *const value) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(other as *const value, self as *const value) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for value {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self as *const value, other as *const value) }
    }
}
impl std::cmp::Eq for value {}
pub unsafe fn test_6() -> () {
    {
        let t1: [(); 6] = [3_(), 5_(), 6_(), 7_(), 12_(), 13_()];
        let t2: [(); 10] = [
            0_(),
            1_(),
            2_(),
            4_(),
            8_(),
            9_(),
            10_(),
            11_(),
            14_(),
            15_(),
        ];
        let t3: [(); 16] = [
            0_(),
            1_(),
            2_(),
            3_(),
            4_(),
            5_(),
            6_(),
            7_(),
            8_(),
            9_(),
            10_(),
            11_(),
            12_(),
            13_(),
            14_(),
            15_(),
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut c3: () = unimplemented!();
        unimplemented!();
    }
    {
        let to: [(); 3] = [0_(), 1_(), 2_()];
        let mut c2: () = unimplemented!();
        let mut io: [(); 3] = [
            unimplemented!(),
            (*unimplemented!()).clone(),
            (*unimplemented!()).clone(),
        ];
        let mut ro: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        let mut po: [*mut (); 3] = [
            (&mut (*unimplemented!()) as *mut ()),
            (&mut (*unimplemented!()) as *mut ()),
            (&mut (*unimplemented!()) as *mut ()),
        ];
        let mut c1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let t1: [value; 3] = [
            value { a: 0_(), b: 0_() },
            value { a: 2_(), b: 0_() },
            value { a: 3_(), b: 0_() },
        ];
        let t2: [value; 4] = [
            value { a: 0_(), b: 1_() },
            value { a: 1_(), b: 1_() },
            value { a: 2_(), b: 1_() },
            value { a: 4_(), b: 1_() },
        ];
        let t3: [value; 7] = [
            value { a: 0_(), b: 0_() },
            value { a: 0_(), b: 1_() },
            value { a: 1_(), b: 1_() },
            value { a: 2_(), b: 0_() },
            value { a: 2_(), b: 1_() },
            value { a: 3_(), b: 0_() },
            value { a: 4_(), b: 1_() },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut c3: () = unimplemented!();
        unimplemented!();
    }
    {
        let t1: [(); 6] = [3_(), 5_(), 6_(), 7_(), 12_(), 13_()];
        let t2: [(); 10] = [
            0_(),
            1_(),
            2_(),
            4_(),
            8_(),
            9_(),
            10_(),
            11_(),
            14_(),
            15_(),
        ];
        let t3: [(); 16] = [
            0_(),
            1_(),
            2_(),
            3_(),
            4_(),
            5_(),
            6_(),
            7_(),
            8_(),
            9_(),
            10_(),
            11_(),
            12_(),
            13_(),
            14_(),
            15_(),
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut c3: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let mut c: () = unimplemented!();
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
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
