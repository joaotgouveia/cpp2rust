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
pub struct Inner {
    pub x: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Table {}
impl Table {
    pub unsafe fn operator_index(mut i: ()) -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut table_0))[(i) as usize];
    }
}
pub static mut table_0: std::cell::LazyCell<[(); 3]> =
    std::cell::LazyCell::new(|| unsafe { [7_(), 8_(), 9_()] });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub data: [(); 3],
    pub inner: Inner,
}
impl S {
    pub unsafe fn operator_index_1(&mut self, mut i: ()) -> *mut () {
        return &mut self.data[(i) as usize];
    }
    pub unsafe fn operator_index_2(&self, mut i: ()) -> *const () {
        return &self.data[(i) as usize];
    }
    pub unsafe fn operator_deref(&mut self) -> *mut Inner {
        return &mut self.inner;
    }
    pub unsafe fn operator_arrow(&mut self) -> *mut Inner {
        return (&mut self.inner as *mut Inner);
    }
    pub unsafe fn operator_addr(&mut self) -> *mut () {
        return (&mut self.data[(unimplemented!()) as usize] as *mut ());
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S {
        data: [1_(), 2_(), 3_()],
        inner: Inner { x: 9_() },
    };
    unimplemented!();
    (*(unsafe { S::operator_index_1(&mut s, 1_()) })) = 20_();
    unimplemented!();
    let cs: *const S = &s;
    unimplemented!();
    unimplemented!();
    (*(unsafe { S::operator_deref(&mut s) })).x = 10_();
    unimplemented!();
    (*(unsafe { S::operator_arrow(&mut s) })).x = 11_();
    unimplemented!();
    let mut p: *mut () = (unsafe { S::operator_addr(&mut s) });
    unimplemented!();
    (*p) = 5_();
    unimplemented!();
    let mut t: Table = <Table>::default();
    unimplemented!();
    (*(unsafe { Table::operator_index(1_()) })) = 80_();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const table_0);
}
