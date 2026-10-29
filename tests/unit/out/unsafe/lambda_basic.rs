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
    let mut zero: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
        unsafe {
            return 42_();
        }
    });
    unimplemented!();
    let mut one: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) + (1_()));
        }
    });
    unimplemented!();
    let mut three: FnPtr<fn((), (), ()) -> ()> =
        FnPtr::<fn((), (), ()) -> ()>::new(|mut x: (), mut y: (), mut z: ()| -> () {
            unsafe {
                return ((((x) * (100_())) + ((y) * (10_()))) + (z));
            }
        });
    unimplemented!();
    let k: () = 3_();
    let m: () = 4_();
    let mut constants: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) + (3_())) + (4_()));
        }
    });
    unimplemented!();
    let n: () = ((k) + (m));
    let mut derived: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) + ((3_()) + (4_())));
        }
    });
    unimplemented!();
    let mut implicit: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) + (3_()));
        }
    });
    unimplemented!();
    let mut bump: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            x += 1_();
            return x;
        }
    });
    unimplemented!();
    let mut through_ptr: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            let mut p: *mut () = (&mut x as *mut ());
            (*p) = ((*p) * (2_()));
            return x;
        }
    });
    unimplemented!();
    let mut seed: () = 5_();
    let mut boxed: () = (unsafe { through_ptr.call(seed) });
    let mut boxed_ptr: *mut () = (&mut boxed as *mut ());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
