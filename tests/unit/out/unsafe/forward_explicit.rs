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
pub const Overload_kIntLvalueOverload: Overload = 3;
pub const Overload_kIntRvalueOverload: Overload = 4;
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
pub unsafe fn chosen_overload_2(_a0: *const ()) -> Overload {
    return Overload_kIntLvalueOverload;
}
pub unsafe fn chosen_overload_3(_a0: *mut ()) -> Overload {
    return Overload_kIntRvalueOverload;
}
pub unsafe fn copy_or_move_into_param_4(mut t: Tracked) -> Tracked {
    return Tracked::move_from({ &mut t });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: Tracked = Tracked::new({ 5_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b: Tracked = Tracked::new({ 6_() });
    let mut moved: Tracked = (unsafe { copy_or_move_into_param_4(Tracked::move_from({ &mut b })) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c: Tracked = Tracked::new({ 7_() });
    let mut copied: Tracked = (unsafe { copy_or_move_into_param_4(Tracked::copy_from({ &c })) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 8_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
