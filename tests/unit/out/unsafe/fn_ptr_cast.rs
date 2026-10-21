extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut x: ()) -> () {
    return ((x) * (2_()));
}
pub unsafe fn test_roundtrip_1() {
    let mut fn_: Option<unsafe fn(()) -> ()> = Some(double_it_0);
    unimplemented!();
    let mut gfn: Option<unsafe fn()> =
        std::mem::transmute::<Option<unsafe fn(()) -> ()>, Option<unsafe fn()>>(fn_);
    unimplemented!();
    let mut fn2: Option<unsafe fn(()) -> ()> =
        std::mem::transmute::<Option<unsafe fn()>, Option<unsafe fn(()) -> ()>>(gfn);
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_double_cast_2() {
    let mut fn_: Option<unsafe fn(()) -> ()> = Some(double_it_0);
    let mut fn2: Option<unsafe fn(()) -> ()> =
        std::mem::transmute::<Option<unsafe fn()>, Option<unsafe fn(()) -> ()>>(
            std::mem::transmute::<Option<unsafe fn(()) -> ()>, Option<unsafe fn()>>(fn_),
        );
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Command {
    pub data: (),
}
pub unsafe fn test_void_ptr_to_fn_3() {
    let mut cmd: Command = <Command>::default();
    cmd.data = std::mem::transmute::<Option<unsafe fn(()) -> ()>, ()>(Some(double_it_0));
    let mut fn_: Option<unsafe fn(()) -> ()> =
        std::mem::transmute::<(), Option<unsafe fn(()) -> ()>>(cmd.data);
    unimplemented!();
}
pub unsafe fn add_offset_4(mut base: *mut (), mut offset: ()) -> () {
    return ((*base) + (offset));
}
pub unsafe fn test_call_through_cast_5() {
    let mut gfn: Option<unsafe fn((), ()) -> ()> = std::mem::transmute::<
        Option<unsafe fn(*mut (), ()) -> ()>,
        Option<unsafe fn((), ()) -> ()>,
    >(Some(add_offset_4));
    let mut val: () = 100_();
    let mut result: () = (unsafe { (gfn).unwrap()(((&mut val as *mut ()) as ()), 42_()) });
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_roundtrip_1() });
    (unsafe { test_double_cast_2() });
    (unsafe { test_void_ptr_to_fn_3() });
    (unsafe { test_call_through_cast_5() });
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
