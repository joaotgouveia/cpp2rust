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
    pub value: (),
}
impl Item {
    pub unsafe fn foo(&mut self, mut other: *mut Item) {
        (*other).value = 10_();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut arr: *mut Item = Box::leak(
        (0..2_())
            .map(|_| <Item>::default())
            .collect::<Box<[Item]>>(),
    )
    .as_mut_ptr();
    (*arr.offset((unimplemented!()) as isize)).value = 1_();
    (*arr.offset((1_()) as isize)).value = 2_();
    (unsafe {
        let _other: *mut Item = (&mut (*arr.offset((1_()) as isize)) as *mut Item);
        Item::foo(&mut (*arr.offset((unimplemented!()) as isize)), _other)
    });
    let mut result: () = (((*arr.offset((unimplemented!()) as isize)).value)
        + ((*arr.offset((1_()) as isize)).value));
    {
        let __p = arr;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<Item>(),
            )))
        }
    };
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
