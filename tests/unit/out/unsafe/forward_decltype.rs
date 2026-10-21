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
            copies: unimplemented!(),
            moves: unimplemented!(),
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
        (*o).v = unimplemented!();
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
pub unsafe fn forward_by_decltype_2(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_0(x) });
}
pub unsafe fn forward_by_decltype_3(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_1(x) });
}
pub unsafe fn forward_abbreviated_4(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_0(x) });
}
pub unsafe fn forward_abbreviated_5(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_1(x) });
}
pub unsafe fn forward_abbreviated_pack_6(args_0: *mut Tracked, args_1: *mut Tracked) -> () {
    return (((unsafe { chosen_overload_0(args_0) }) as ())
        + ((unsafe { chosen_overload_1(args_1) }) as ()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: Tracked = Tracked::new({ 3_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b: Tracked = Tracked::new({ 5_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c: Tracked = Tracked::new({ 7_() });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
