extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn call_with_ulong_0(mut g: Option<unsafe fn(()) -> ()>) -> () {
    return (unsafe { (g).unwrap()(3_()) }).wrapping_add(1_());
}
pub unsafe fn same_type_1(mut a: ()) -> () {
    return a;
}
pub unsafe fn via_size_t_param_2(mut b: ()) -> () {
    return b;
}
pub unsafe fn via_size_t_return_3(mut b: ()) -> () {
    return b;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct pair {
    pub a: (),
    pub b: (),
}
pub unsafe fn pair_scaled_4(mut p: pair, mut n: ()) -> () {
    return ((p.a).wrapping_mul(n)).wrapping_add(p.b);
}
pub unsafe fn make_pair_5(mut n: ()) -> pair {
    let mut p: pair = pair {
        a: (n as ()),
        b: ((n as ()) * (2_())),
    };
    return p;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut original: Option<unsafe fn(()) -> ()> = Some(via_size_t_param_2);
    let mut adapted: Option<unsafe fn(()) -> ()> =
        std::mem::transmute::<Option<unsafe fn(()) -> ()>, Option<unsafe fn(()) -> ()>>(original);
    let mut back: Option<unsafe fn(()) -> ()> =
        std::mem::transmute::<Option<unsafe fn(()) -> ()>, Option<unsafe fn(()) -> ()>>(adapted);
    unimplemented!();
    unimplemented!();
    let mut scaled: Option<unsafe fn(pair, ()) -> ()> = std::mem::transmute::<
        Option<unsafe fn(pair, ()) -> ()>,
        Option<unsafe fn(pair, ()) -> ()>,
    >(Some(pair_scaled_4));
    let mut p: pair = pair { a: 3_(), b: 4_() };
    unimplemented!();
    let mut make: Option<unsafe fn(()) -> pair> = std::mem::transmute::<
        Option<unsafe fn(()) -> pair>,
        Option<unsafe fn(()) -> pair>,
    >(Some(make_pair_5));
    let mut q: pair = (unsafe { (make).unwrap()(5_()) });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
