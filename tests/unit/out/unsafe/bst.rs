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
pub struct node_t {
    pub left: *mut node_t,
    pub right: *mut node_t,
    pub value: (),
}
pub unsafe fn find_0(mut node: *mut node_t, mut value: ()) -> *mut node_t {
    if ((value) < ((*node).value)) && (!(((*node).left).is_null())) {
        return (unsafe { find_0((*node).left, value) });
    } else if ((value) > ((*node).value)) && (!(((*node).right).is_null())) {
        return (unsafe { find_0((*node).right, value) });
    } else if ((value) == ((*node).value)) {
        return node;
    }
    return std::ptr::null_mut();
}
pub unsafe fn insert_1(mut node: *mut node_t, mut new_node: *mut node_t) -> *mut node_t {
    if (node).is_null() {
        return new_node;
    }
    if (((*new_node).value) < ((*node).value)) {
        (*node).left = (unsafe { insert_1((*node).left, new_node) });
    } else if (((*new_node).value) > ((*node).value)) {
        (*node).right = (unsafe { insert_1((*node).right, new_node) });
    }
    return node;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut tree: () = unimplemented!();
    let mut n1: () = unimplemented!();
    let mut n2: () = unimplemented!();
    let mut n3: () = unimplemented!();
    let mut n4: () = unimplemented!();
    let mut ptr1: *mut node_t = (&mut (*unimplemented!()) as *mut node_t);
    ptr1 = (unsafe { insert_1(ptr1, (&mut (*unimplemented!()) as *mut node_t)) });
    ptr1 = (unsafe { insert_1(ptr1, (&mut (*unimplemented!()) as *mut node_t)) });
    ptr1 = (unsafe { insert_1(ptr1, (&mut (*unimplemented!()) as *mut node_t)) });
    ptr1 = (unsafe { insert_1(ptr1, (&mut (*unimplemented!()) as *mut node_t)) });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
