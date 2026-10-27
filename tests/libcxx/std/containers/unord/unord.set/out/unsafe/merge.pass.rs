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
pub struct Counter_base {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Counter_int_ {
    data_: (),
}
impl Counter_int_ {
    pub unsafe fn new_1(data: *const ()) -> Self {
        let mut this = Self { data_: (*data) };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(rhs: *const Counter_int_) -> Self {
        let mut this = Self {
            data_: (*rhs).data_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_0)).prefix_dec();
    }
    pub unsafe fn get(&self) -> *const () {
        return &self.data_;
    }
    pub unsafe fn operator_eq(&self, x: *const Counter_int_) -> () {
        return ((self.data_) == ((*x).data_));
    }
}
impl std::cmp::PartialEq for Counter_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Counter_int_::operator_eq(self, other as *const Counter_int_) }
    }
}
impl std::cmp::Eq for Counter_int_ {}
impl Clone for Counter_int_ {
    fn clone(&self) -> Self {
        unsafe { Counter_int_::copy_from(self as *const Counter_int_) }
    }
}
pub static mut gConstructed_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn set_equal_1(set: *const (), mut other: ()) -> () {
    return unimplemented!();
}
pub unsafe fn set_equal_2(set: *const (), mut other: ()) -> () {
    return unimplemented!();
}
pub unsafe fn set_equal_3(set: *const (), mut other: ()) -> () {
    return unimplemented!();
}
pub unsafe fn set_equal_4(set: *const (), mut other: ()) -> () {
    return unimplemented!();
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
    {
        let mut src: () = unimplemented!();
        let mut dst: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        {
            let mut first: () = unimplemented!();
            let mut second: () = unimplemented!();
            let mut third: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        {
            let mut first: () = unimplemented!();
            let mut second: () = unimplemented!();
            let mut third: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
    }
    {
        let mut first: () = unimplemented!();
        {
            let mut second: () = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut second: () = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct equal_5 {}
impl equal_5 {
    pub unsafe fn operator_call(&self, lhs: *const Counter_int_, rhs: *const Counter_int_) -> () {
        return (unsafe {
            let _x: *const Counter_int_ = rhs;
            Counter_int_::operator_eq(&(*lhs), _x)
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct hasher_6 {}
impl hasher_6 {
    pub unsafe fn operator_call(&self, p: *const Counter_int_) -> () {
        return (unsafe {
            std_hash_Counter_int__::operator_call(&<std_hash_Counter_int__>::default(), p)
        });
    }
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std_hash_Counter_int__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const gConstructed_0);
}
