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
pub struct Item {
    pub key: (),
    pub value: (),
}
pub unsafe fn Compare_0(a: *const Item, b: *const Item) -> () {
    return (((*a).key) < ((*b).key));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: std_vector_Item__std_allocator_Item__ =
        <std_vector_Item__std_allocator_Item__>::default();
    (unsafe {
        let mut ___x: Item = Item {
            key: 3_(),
            value: 30_(),
        };
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: Item = Item {
            key: 1_(),
            value: 10_(),
        };
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: Item = Item {
            key: 2_(),
            value: 20_(),
        };
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let ___first: __gnu_cxx___normal_iterator_Item_ptr__std_vector_Item__std_allocator_Item___ =
            (unsafe { v.begin_26() });
        let ___last: __gnu_cxx___normal_iterator_Item_ptr__std_vector_Item__std_allocator_Item___ =
            (unsafe { v.end_28() });
        stable_sort_1(___first, ___last, Some(Compare_0))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
