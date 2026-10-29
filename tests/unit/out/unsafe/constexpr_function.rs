extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn runtime_only_0(mut x: ()) -> () {
    return ((x) * (2_()));
}
pub unsafe fn first_1(mut p: *const ()) -> () {
    return (*p);
}
pub unsafe fn scaled_2(mut x: ()) -> () {
    if ((x) < (0_())) {
        return (unsafe { runtime_only_0(-x) });
    }
    return x;
}
pub unsafe fn half_3(mut x: ()) -> () {
    return ((x) / (2_f64));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Flag {
    pub v: (),
}
impl Flag {
    pub unsafe fn to_(&self) -> () {
        return ((self.v) != (0_()));
    }
}
pub unsafe fn use_4(mut f: Flag) -> () {
    unimplemented!();
    return f.v;
}
pub unsafe fn checked_5(mut x: ()) -> () {
    unimplemented!();
    return ((x) + (1_()));
}
pub unsafe fn in_constant_context_6() -> () {
    {}
    return ((unsafe { runtime_only_0(1_()) }) == (0_()));
}
const _: () = assert!(true, "in_constant_context()");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct P {
    pub v: (),
}
impl P {
    pub unsafe fn get(&self) -> () {
        return self.v;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut runtime_context: () = (unsafe { in_constant_context_6() });
    unimplemented!();
    let mut arr: [(); 2] = [7_(), 8_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c: () = (unsafe { checked_5(4_()) });
    unimplemented!();
    unimplemented!();
    let u: () = (unsafe { use_4(Flag { v: 3_() }) });
    unimplemented!();
    let mut ptr: *mut () = arr.as_mut_ptr();
    unimplemented!();
    let mut p: P = P { v: 9_() };
    unimplemented!();
    let k: () = (unsafe { scaled_2(4_()) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
