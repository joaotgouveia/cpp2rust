extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Flag = ();
pub const Flag_FLAG_OFF: Flag = false;
pub const Flag_FLAG_ON: Flag = true;
pub type Small = ();
pub const Small_SMALL_ZERO: Small = 0;
pub const Small_SMALL_MAX: Small = 255;
pub type Signed = ();
pub const Signed_SIGNED_MIN: Signed = -128;
pub const Signed_SIGNED_ONE: Signed = 1;
pub type Wide = ();
pub const Wide_WIDE_NEG: Wide = -1;
pub const Wide_WIDE_BIG: Wide = 1099511627776;
pub type Sized = ();
pub const Sized_SIZED_A: Sized = 1;
pub const Sized_SIZED_B: Sized = 65535;
pub type Scoped = ();
pub const Scoped_NEG: Scoped = -2;
pub const Scoped_POS: Scoped = 2;
pub type Empty = ();
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HoldsEmpty {
    pub e: Empty,
}
impl Default for HoldsEmpty {
    fn default() -> Self {
        unsafe {
            HoldsEmpty {
                e: unimplemented!(),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut flag: Flag = Flag_FLAG_ON;
    unimplemented!();
    unimplemented!();
    flag = Flag_FLAG_OFF;
    unimplemented!();
    let mut b: () = flag;
    unimplemented!();
    let mut s: Small = Small_SMALL_MAX;
    unimplemented!();
    unimplemented!();
    s = Small_SMALL_ZERO;
    unimplemented!();
    let mut sg: Signed = Signed_SIGNED_MIN;
    unimplemented!();
    unimplemented!();
    let mut w: Wide = Wide_WIDE_BIG;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut z: Sized = Sized_SIZED_B;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sc: Scoped = Scoped_NEG;
    unimplemented!();
    unimplemented!();
    let mut he: HoldsEmpty = <HoldsEmpty>::default();
    unimplemented!();
    let mut ev: Empty = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
