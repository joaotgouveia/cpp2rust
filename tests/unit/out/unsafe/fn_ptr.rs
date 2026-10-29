extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn my_foo_0(mut p: ()) -> () {
    return (*(p as *mut ()));
}
pub unsafe fn foo_1(mut fn_: Option<unsafe fn(()) -> ()>, mut pi: *mut ()) -> () {
    return (unsafe { (fn_).unwrap()((pi as ())) });
}
pub unsafe fn twice_2(mut x: ()) -> () {
    return (x).wrapping_mul(2_());
}
pub unsafe fn twice_in_place_3(x: *mut ()) -> () {
    (*x) = (*x).wrapping_mul(2_());
    return (*x);
}
pub unsafe fn ret_size_4(mut v: ()) -> () {
    return ((v) + (1_()));
}
pub unsafe fn call_fn_5(mut f: Option<unsafe fn(()) -> ()>, mut v: ()) -> () {
    return (unsafe { (f).unwrap()(v) }).wrapping_mul(2_());
}
pub unsafe fn identity_hash_6(mut v: ()) -> () {
    return v;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HashHolder_unsigned_long__ptr__bool__ {
    pub h: Option<unsafe fn(()) -> ()>,
}
impl HashHolder_unsigned_long__ptr__bool__ {
    pub unsafe fn new(h: *const Option<unsafe fn(()) -> ()>) -> Self {
        let mut this = Self { h: (*h) };
        this
    }
}
pub static mut total_7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn add_to_total_8(mut x: ()) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut total_7)) += x;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut fn_: Option<unsafe fn(()) -> ()> = None;
    unimplemented!();
    unimplemented!();
    fn_ = Some(my_foo_0);
    unimplemented!();
    unimplemented!();
    let mut a: () = 10_();
    unimplemented!();
    let mut ul_fn: Option<unsafe fn(()) -> ()> = (Some(twice_2));
    let mut n: () = 21_();
    let mut r: () = (unsafe { (ul_fn).unwrap()(n) });
    unimplemented!();
    let mut ul_ref_fn: Option<unsafe fn(*mut ()) -> ()> = (Some(twice_in_place_3));
    let mut m: () = 21_();
    let mut q: () = (unsafe { (ul_ref_fn).unwrap()(&mut m) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut hh: HashHolder_unsigned_long__ptr__bool__ = {
        let mut __tmp_0: Option<unsafe fn(()) -> ()> = Some(identity_hash_6);
        HashHolder_unsigned_long__ptr__bool__::new({ &mut __tmp_0 })
    };
    unimplemented!();
    let mut handler: () = Some(add_to_total_8);
    (unsafe { (handler).unwrap()(2_()) });
    (unsafe { (handler).unwrap()(3_()) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const total_7);
}
