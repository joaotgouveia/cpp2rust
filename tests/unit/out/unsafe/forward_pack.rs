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
pub unsafe fn forward_pack_4() -> () {
    let mut digits: () = 0_();
    unimplemented!();
    return digits;
}
pub unsafe fn forward_pack_5(args: *mut Tracked) -> () {
    let mut digits: () = 0_();
    digits = (((digits) * (10_())) + ((unsafe { chosen_overload_0(args) }) as ()));
    return digits;
}
pub unsafe fn forward_pack_6(args: *mut Tracked) -> () {
    let mut digits: () = 0_();
    digits = (((digits) * (10_())) + ((unsafe { chosen_overload_1(args) }) as ()));
    return digits;
}
pub unsafe fn forward_pack_7(
    args_0: *mut Tracked,
    args_1: *mut Tracked,
    args_2: *mut (),
    args_3: *mut (),
) -> () {
    let mut digits: () = 0_();
    {
        digits = (((digits) * (10_())) + ((unsafe { chosen_overload_0(args_0) }) as ()));
        {
            digits = (((digits) * (10_())) + ((unsafe { chosen_overload_1(args_1) }) as ()));
            {
                digits = (((digits) * (10_())) + ((unsafe { chosen_overload_2(args_2) }) as ()));
                digits = (((digits) * (10_())) + ((unsafe { chosen_overload_3(args_3) }) as ()))
            }
        }
    };
    return digits;
}
impl Pair {
    pub unsafe fn new(x: *mut Tracked, y: *mut Tracked) -> Self {
        let mut this = Self {
            a: Tracked::copy_from({ x }),
            b: Tracked::move_from({ y }),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub a: Tracked,
    pub b: Tracked,
}
pub unsafe fn forward_pack_into_ctor_8(args_0: *mut Tracked, args_1: *mut Tracked) -> Pair {
    return Pair::new({ args_0 }, { args_1 });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    let mut a: Tracked = Tracked::new({ 1_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 3_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut lhs: Tracked = Tracked::new({ 6_() });
    let mut p: Pair = (unsafe {
        let mut _args_1: Tracked = Tracked::new({ 7_() });
        forward_pack_into_ctor_8(&mut lhs, &mut _args_1)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
