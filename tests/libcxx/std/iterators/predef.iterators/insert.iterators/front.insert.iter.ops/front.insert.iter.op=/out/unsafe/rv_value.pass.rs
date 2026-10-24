extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConstexprFixedCapacityDeque_int__10_ {
    data_: [(); 10],
    size_: (),
}
impl ConstexprFixedCapacityDeque_int__10_ {
    pub unsafe fn begin_2(&mut self) -> *mut () {
        return self.data_.as_mut_ptr();
    }
    pub unsafe fn front(&self) -> *const () {
        unimplemented!();
        return &self.data_[(0_()) as usize];
    }
    pub unsafe fn insert(&mut self, mut pos: *const (), mut t: ()) -> *mut () {
        let mut i: () =
            ((pos as usize - self.data_.as_mut_ptr() as usize) / ::std::mem::size_of::<()>()) as ();
        if ((i) != (self.size_)) {
            unimplemented!();
        }
        self.data_[(i) as usize] = (*unimplemented!());
        self.size_ += 1_();
        return self.data_.as_mut_ptr().offset((i) as isize);
    }
    pub unsafe fn push_front(&mut self, mut t: ()) {
        (unsafe {
            let _pos: *const () =
                (unsafe { ConstexprFixedCapacityDeque_int__10_::begin_2(self) }).cast_const();
            ConstexprFixedCapacityDeque_int__10_::insert(self, _pos, (*unimplemented!()))
        });
    }
}
impl Default for ConstexprFixedCapacityDeque_int__10_ {
    fn default() -> Self {
        unsafe {
            ConstexprFixedCapacityDeque_int__10_ {
                data_: [unimplemented!(); 10],
                size_: 0_(),
            }
        }
    }
}
pub unsafe fn test_0(mut c: ()) -> () {
    let mut i: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_1(mut c: ConstexprFixedCapacityDeque_int__10_) -> () {
    let mut i: () = unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test_0(unimplemented!()) });
    (unsafe { test_1(<ConstexprFixedCapacityDeque_int__10_>::default()) });
    const _: () = assert!(true, "test(ConstexprFixedCapacityDeque<int, 10>())");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
