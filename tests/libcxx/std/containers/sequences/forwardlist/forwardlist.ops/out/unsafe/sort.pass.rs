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
pub static mut randomness_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn test_5(mut N: i32) {
    let mut v: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        {
            let a0_clone = i.clone();
            v.push(a0_clone)
        };
        i.prefix_inc();
    }
    unimplemented!();
    let mut c: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (N as isize)));
    let mut j: () = <()>::from(unimplemented!());
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((*j.iter_deref()) == (i)));
        {
            i.prefix_inc();
            j.iter_inc()
        };
    }
}
pub unsafe fn test_6(mut N: i32) {
    let mut v: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        {
            let a0_clone = i.clone();
            v.push(a0_clone)
        };
        i.prefix_inc();
    }
    unimplemented!();
    let mut c: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (N as isize)));
    let mut j: () = <()>::from(unimplemented!());
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((*j.iter_deref()) == (i)));
        {
            i.prefix_inc();
            j.iter_inc()
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new_1, value)]
pub struct Payload {
    pub val: i32,
    pub side: i32,
}
impl Payload {
    pub unsafe fn new_1(mut v: i32) -> Self {
        let mut this = Self { val: v, side: 0 };
        this
    }
    pub unsafe fn new_2(mut v: i32, mut s: i32) -> Self {
        let mut this = Self { val: v, side: s };
        this
    }
    pub unsafe fn operator_lt(&self, rhs: *const Payload) -> bool {
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
pub unsafe fn test_stable_7(mut N: i32) {
    let mut v: Vec<Payload> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        {
            let __a1 = Payload::new_1({ ((i) / (2)) });
            v.push(__a1)
        };
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*(&mut v).as_mut_ptr().add((i as usize) as usize)).side = i;
        i.prefix_inc();
    }
    let mut c: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (N as isize)));
    let mut j: () = <()>::from(unimplemented!());
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!((((*j.iter_deref()).val) == ((i) / (2))));
        {
            i.prefix_inc();
            j.iter_inc()
        };
    }
    let mut it: () = <()>::from(unimplemented!());
    'loop_: while it != <()>::from(unimplemented!()) {
        let mut next: () = libcc2rs::next_unsafe(it.clone(), 1_isize);
        if (next != <()>::from(unimplemented!()))
            && (((*it.iter_deref()).val) == ((*next.iter_deref()).val))
        {
            assert!((((*it.iter_deref()).side) < ((*next.iter_deref()).side)));
        }
        it.iter_inc();
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    let mut i: i32 = 0;
    'loop_: while ((i) < (40)) {
        (unsafe { test_5(i) });
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (40)) {
        (unsafe { test_6(i) });
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (40)) {
        (unsafe { test_stable_7(i) });
        i.prefix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const randomness_4);
}
