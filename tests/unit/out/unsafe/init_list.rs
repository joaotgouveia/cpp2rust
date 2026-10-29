extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f_0(mut list: std_initializer_list_int_) {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub a: (),
    pub b: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Holder {
    pub p: Pair,
}
impl Holder {
    pub unsafe fn operator_assign_1(&mut self, o: *mut Pair) -> *mut Holder {
        self.p = (*o);
        return &mut (*(self as *mut Holder));
    }
}
impl Default for Holder {
    fn default() -> Self {
        unsafe {
            Holder {
                p: Pair { a: 0_(), b: 0_() },
            }
        }
    }
}
pub unsafe fn sum_1(p: *const Pair) -> () {
    return (((*p).a) + ((*p).b));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut i1: () = 3_();
    let mut i2: () = unimplemented!();
    let mut carr1: [(); 2] = [1_(), 2_()];
    let mut carr2: [(); 3] = [1_(), unimplemented!(), unimplemented!()];
    let mut arr: std_array_int__3_ = unimplemented!();
    let mut vec_: std_vector_int__std_allocator_int__ =
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None);
    (unsafe { f_0(unimplemented!()) });
    let mut p: Pair = Pair { a: 1_(), b: 2_() };
    let mut h: Holder = <Holder>::default();
    (unsafe { Holder::operator_assign_1(&mut h, &mut p) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
