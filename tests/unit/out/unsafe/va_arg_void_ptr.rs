extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct registry {
    pub slot: (),
    pub level: (),
}
pub type field = ();
pub const field_FIELD_SLOT: field = 0;
pub const field_FIELD_LEVEL: field = 1;
pub unsafe fn registry_update_0(
    mut r: *mut registry,
    mut field: field,
    __args: Box<[VaArg]>,
) -> () {
    let mut result: () = unimplemented!();
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    'switch: {
        match { (field as ()) } {
            __v if __v == (field_FIELD_SLOT as i32) => {
                (*r).slot = ap.arg::<()>();
                break 'switch;
            }
            __v if __v == (field_FIELD_LEVEL as i32) => {
                (*r).level = ap.arg::<()>();
                break 'switch;
            }
            _ => {
                result = 1_();
                break 'switch;
            }
        }
    };
    return result;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut r: registry = registry {
        slot: unimplemented!(),
        level: unimplemented!(),
    };
    let mut payload: () = 7_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
