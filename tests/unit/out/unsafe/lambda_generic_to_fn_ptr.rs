extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut negate: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut x: ()| -> () {
            return -x;
        },
        |mut x: ()| -> () {
            return -x;
        }
    );
    let mut fi: Option<unsafe fn(()) -> ()> = Some(|mut x: ()| -> () {
        return -x;
    });
    let mut fd: Option<unsafe fn(()) -> ()> = Some(|mut x: ()| -> () {
        return -x;
    });
    unimplemented!();
    unimplemented!();
    let mut square: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut x: ()| -> () {
            return ((x) * (x));
        },
        |mut x: ()| -> () {
            return ((x) * (x));
        }
    );
    let mut si: Option<unsafe fn(()) -> ()> = Some(|mut x: ()| -> () {
        return ((x) * (x));
    });
    let mut sd: Option<unsafe fn(()) -> ()> = Some(|mut x: ()| -> () {
        return ((x) * (x));
    });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
