extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Overload = ();
pub const Overload_kLvalueOverload: Overload = 1;
pub const Overload_kRvalueOverload: Overload = 2;
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Tracked {
    pub v: (),
    pub copies: (),
    pub moves: (),
}
impl Tracked {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            v: v,
            copies: 0_(),
            moves: 0_(),
        };
        this
    }
    pub unsafe fn copy_from(o: *const Tracked) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (((*o).copies) + (1_())),
            moves: (*o).moves,
        };
        this
    }
    pub unsafe fn move_from(o: *mut Tracked) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (*o).copies,
            moves: (((*o).moves) + (1_())),
        };
        (*o).v = 0_();
        this
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        unsafe { Tracked::copy_from(self as *const Tracked) }
    }
}
pub unsafe fn chosen_overload_0(_a0: *const Tracked) -> Overload {
    return Overload_kLvalueOverload;
}
pub unsafe fn chosen_overload_1(_a0: *mut Tracked) -> Overload {
    return Overload_kRvalueOverload;
}
impl Holder {
    pub unsafe fn new_1(x: *mut Tracked) -> Self {
        let mut this = Self {
            t: Tracked::copy_from({ x }),
        };
        this
    }
}
impl Holder {
    pub unsafe fn new_2(x: *mut Tracked) -> Self {
        let mut this = Self {
            t: Tracked::move_from({ x }),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Holder {
    pub t: Tracked,
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            t: Tracked::move_from({ &mut (*_a0).t }),
        };
        this
    }
}
pub unsafe fn forward_once_2(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_0(x) });
}
pub unsafe fn forward_once_3(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_1(x) });
}
pub unsafe fn forward_twice_4(x: *mut Tracked) -> Overload {
    return (unsafe { forward_once_2(x) });
}
pub unsafe fn forward_twice_5(x: *mut Tracked) -> Overload {
    return (unsafe { forward_once_3(x) });
}
pub unsafe fn forward_into_ctor_6(x: *mut Tracked) -> Holder {
    return Holder::new_1({ x });
}
pub unsafe fn forward_into_ctor_7(x: *mut Tracked) -> Holder {
    return Holder::new_2({ x });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut lvalue: Tracked = Tracked::new({ 7_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut relayed: Tracked = Tracked::new({ 9_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut kept: Tracked = Tracked::new({ 11_() });
    let mut from_lvalue: Holder = (unsafe { forward_into_ctor_6(&mut kept) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut from_rvalue: Holder = (unsafe {
        let mut _x: Tracked = Tracked::new({ 12_() });
        forward_into_ctor_7(&mut _x)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
