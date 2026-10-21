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
pub unsafe fn insert_1(mut node: *mut node_t, mut value: ()) -> *mut node_t {
    if (node).is_null() {
        return (Box::leak(Box::new(node_t {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            value: value,
        })) as *mut node_t);
    }
    if ((value) < ((*node).value)) {
        (*node).left = (unsafe { insert_1((*node).left, value) });
    } else if ((value) > ((*node).value)) {
        (*node).right = (unsafe { insert_1((*node).right, value) });
    }
    return node;
}
pub unsafe fn del_2(mut node: *mut node_t) {
    if !(((*node).left).is_null()) {
        (unsafe { del_2((*node).left) });
    }
    if !(((*node).right).is_null()) {
        (unsafe { del_2((*node).right) });
    }
    {
        let __p = node;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut root: *mut node_t = (Box::leak(Box::new(node_t {
        left: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
        value: unimplemented!(),
    })) as *mut node_t);
    root = (unsafe { insert_1(root, 1_()) });
    root = (unsafe { insert_1(root, 2_()) });
    root = (unsafe { insert_1(root, 3_()) });
    root = (unsafe { insert_1(root, 4_()) });
    let mut out: () = (((((((*(unsafe { find_0(root, unimplemented!()) })).value)
        == (unimplemented!()))
        && (((*(unsafe { find_0(root, 1_()) })).value) == (1_())))
        && (((*(unsafe { find_0(root, 2_()) })).value) == (2_())))
        && (((*(unsafe { find_0(root, 3_()) })).value) == (3_())))
        && (((*(unsafe { find_0(root, 4_()) })).value) == (4_())))
        && ((unsafe { find_0(root, 5_()) }).is_null());
    (unsafe { del_2(root) });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
