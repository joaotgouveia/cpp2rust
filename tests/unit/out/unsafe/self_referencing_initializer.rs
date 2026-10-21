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
pub struct list_head {
    pub next: *mut list_head,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct node {
    pub value: (),
    pub value_ptr: *mut (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct node_with_defaults {
    pub next: *mut node_with_defaults,
    pub value: (),
}
impl Default for node_with_defaults {
    fn default() -> Self {
        unsafe {
            node_with_defaults {
                next: std::ptr::null_mut(),
                value: 3_(),
            }
        }
    }
}
pub unsafe fn init_0(mut l: *mut list_head) -> *mut list_head {
    (*l).next = std::ptr::null_mut();
    return l;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut list: list_head = <list_head>::default();
    list = list_head {
        next: (&mut list as *mut list_head),
    };
    unimplemented!();
    let mut n: node = <node>::default();
    n = node {
        value: 42_(),
        value_ptr: (&mut n.value as *mut ()),
    };
    unimplemented!();
    (*n.value_ptr) = 7_();
    unimplemented!();
    let mut arr: [list_head; 2] = [<list_head>::default(); 2];
    arr = [
        list_head {
            next: (&mut arr[(1_()) as usize] as *mut list_head),
        },
        list_head {
            next: (&mut arr[(unimplemented!()) as usize] as *mut list_head),
        },
    ];
    unimplemented!();
    unimplemented!();
    let mut d: node_with_defaults = <node_with_defaults>::default();
    d = node_with_defaults {
        next: (&mut d as *mut node_with_defaults),
        value: 3_(),
    };
    unimplemented!();
    unimplemented!();
    let mut called: list_head = <list_head>::default();
    called = list_head {
        next: (unsafe { init_0((&mut called as *mut list_head)) }),
    };
    unimplemented!();
    let mut size: () = ::std::mem::size_of::<()>();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
