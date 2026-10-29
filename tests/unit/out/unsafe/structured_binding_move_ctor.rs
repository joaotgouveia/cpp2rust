extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moves_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
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
        (*other).x = 0_();
        (*other).y = 0_();
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
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub xs: std_vector_int__std_allocator_int__,
    pub ys: std_vector_int__std_allocator_int__,
}
impl Holder {
    pub unsafe fn extract(&mut self) -> Holder {
        return Holder {
            xs: self.xs,
            ys: self.ys,
        };
    }
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            xs: (*_a0).xs,
            ys: (*_a0).ys,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        self.xs = ((*_a0).xs).clone();
        self.ys = ((*_a0).ys).clone();
        return &mut (*(self as *mut Holder));
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
    let mut __decomp_2: Movable = Movable::move_from({ &mut m });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut h: Holder = Holder {
        xs: std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None),
        ys: std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None),
    };
    let mut __decomp_3: Holder = (unsafe { Holder::extract(&mut h) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
}
