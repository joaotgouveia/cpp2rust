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
pub struct node {
    pub value: (),
    pub next: *mut node,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct pair_t {
    pub a: (),
    pub n: *mut node,
}
pub static mut global_node_0: std::cell::LazyCell<*mut node> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub unsafe fn id_1(mut n: *mut node) -> *mut node {
    return n;
}
pub unsafe fn pick_2(mut a: *mut node, mut b: *mut node) -> *mut node {
    return if !(a).is_null() { a } else { b };
}
pub unsafe fn local_ptr_3(mut n: *mut node) -> *mut node {
    let mut p: *mut node = (*n).next;
    return p;
}
pub unsafe fn call_once_4(mut n: *mut node, mut m: *mut node) -> *mut node {
    return (unsafe { pick_2(n, m) });
}
pub unsafe fn call_twice_5(mut n: *mut node) -> *mut node {
    return (unsafe {
        let _a: *mut node = n;
        let _b: *mut node = n;
        pick_2(_a, _b)
    });
}
pub unsafe fn next_of_6(mut n: *mut node) -> *mut node {
    return (*n).next;
}
pub unsafe fn ret_global_7() -> *mut node {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut global_node_0));
}
pub unsafe fn address_taken_8(mut n: *mut node) -> *mut node {
    let mut pp: *mut *mut node = (&mut n as *mut *mut node);
    return if !(*pp).is_null() {
        n
    } else {
        std::ptr::null_mut()
    };
}
pub unsafe fn ret_struct_9(mut a: (), mut n: *mut node) -> pair_t {
    let mut p: pair_t = <pair_t>::default();
    p.a = a;
    p.n = n;
    return p;
}
pub unsafe fn ret_struct_param_10(mut p: pair_t) -> pair_t {
    return p;
}
pub unsafe fn ret_struct_ref_11(p: *mut pair_t) -> pair_t {
    return (*p);
}
pub unsafe fn ret_vec_12(mut v: ()) -> () {
    return unimplemented!();
}
pub unsafe fn ret_vec_local_13() -> () {
    let mut v: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn ret_loop_14(mut n: *mut node) -> *mut node {
    'loop_: while !((*n).next).is_null() {
        if (((*n).value) == (2_())) {
            return n;
        }
        n = (*n).next;
    }
    return n;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut c: node = node {
        value: 3_(),
        next: std::ptr::null_mut(),
    };
    let mut b: node = node {
        value: 2_(),
        next: (&mut c as *mut node),
    };
    let mut a: node = node {
        value: 1_(),
        next: (&mut b as *mut node),
    };
    (*std::cell::LazyCell::force_mut(&mut *&raw mut global_node_0)) = (&mut a as *mut node);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: pair_t = (unsafe { ret_struct_9(4_(), (&mut a as *mut node)) });
    unimplemented!();
    let mut q: pair_t = (unsafe { ret_struct_param_10(p) });
    unimplemented!();
    let mut r: pair_t = (unsafe { ret_struct_ref_11(&mut q) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_node_0);
}
