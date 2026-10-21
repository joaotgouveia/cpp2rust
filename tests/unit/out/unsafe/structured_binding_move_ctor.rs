extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut moves_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Movable {
    pub x: (),
    pub y: (),
}
impl Movable {
    pub unsafe fn new(mut x: (), mut y: ()) -> Self {
        let mut this = Self { x: x, y: y };
        this
    }
    pub unsafe fn copy_from(other: *const Movable) -> Self {
        let mut this = Self {
            x: (*other).x,
            y: (*other).y,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut Movable) -> Self {
        let mut this = Self {
            x: (*other).x,
            y: (*other).y,
        };
        (*other).x = unimplemented!();
        (*other).y = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
        this
    }
}
impl Clone for Movable {
    fn clone(&self) -> Self {
        unsafe { Movable::copy_from(self as *const Movable) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub xs: (),
    pub ys: (),
}
impl Holder {
    pub unsafe fn extract(&mut self) -> Holder {
        return Holder {
            xs: unimplemented!(),
            ys: unimplemented!(),
        };
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut m: Movable = Movable::new({ 3_() }, { 4_() });
    let mut __decomp_2: Movable = Movable::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut h: Holder = Holder {
        xs: unimplemented!(),
        ys: unimplemented!(),
    };
    let mut __decomp_3: Holder = (unsafe { Holder::extract(&mut (*unimplemented!())) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
}
