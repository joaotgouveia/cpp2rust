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
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Counted {
    pub x: (),
    pub y: (),
}
impl Counted {
    pub unsafe fn new(mut x: (), mut y: ()) -> Self {
        let mut this = Self { x: x, y: y };
        this
    }
    pub unsafe fn copy_from(other: *const Counted) -> Self {
        let mut this = Self {
            x: (((*other).x) * (2_())),
            y: (((*other).y) * (2_())),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Counted) -> *mut Counted {
        self.x = (*_a0).x;
        self.y = (*_a0).y;
        return &mut (*(self as *mut Counted));
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::copy_from(self as *const Counted) }
    }
}
pub unsafe fn make_counted_1() -> Counted {
    return Counted::new({ 5_() }, { 6_() });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: Counted = Counted::new({ 1_() }, { 2_() });
    let mut __decomp_2: Counted = Counted::copy_from({ &s });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_3: *mut Counted = &mut s;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_4: *const Counted = &s;
    unimplemented!();
    unimplemented!();
    let mut __decomp_5: Counted = (unsafe { make_counted_1() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
}
