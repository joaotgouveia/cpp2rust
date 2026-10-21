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
    let mut twice: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut x: ()| -> () {
            return ((x) + (x));
        },
        |mut x: ()| -> () {
            return ((x) + (x));
        }
    );
    unimplemented!();
    unimplemented!();
    let mut base: () = 10_();
    let mut add_base: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let base: () = base;
        },
        |mut x: ()| -> () {
            return ((x) + (base));
        },
        |mut x: ()| -> () {
            return ((x) + (base));
        }
    );
    unimplemented!();
    unimplemented!();
    let mut total: () = unimplemented!();
    let mut accumulate: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let total: *mut () = &mut total;
        },
        |mut x: (), mut y: ()| {
            (*total) += ((x) * (y));
        },
        |mut x: (), mut y: ()| {
            (*total) = (*total).wrapping_add((x).wrapping_mul(y));
        }
    );
    (unsafe { accumulate.spec::<fn((), ())>(0).call(2_(), 3_()) });
    (unsafe { accumulate.spec::<fn((), ())>(1).call(4_(), 5_()) });
    unimplemented!();
    let mut sub: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut x: (), mut y: ()| -> () {
            return ((x) - (y));
        },
        |mut x: (), mut y: ()| -> () {
            return ((x) - (y));
        }
    );
    unimplemented!();
    unimplemented!();
    let mut mixed: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let base: () = base;
        },
        |mut x: (), mut y: ()| -> () {
            return (((x) * (y)) + (base));
        },
        |mut x: (), mut y: ()| -> () {
            return (((x) * (y)) + (base));
        }
    );
    unimplemented!();
    unimplemented!();
    let mut unused: FnPtr<Generic> = lambda_unsafe!(Generic, {});
    &(unused);
    let mut outer: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let base: () = base;
        },
        |mut y: ()| -> () {
            let mut inner: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut z: ()| -> () {
                return ((z) + (1_()));
            });
            return ((unsafe { inner.spec::<fn(()) -> ()>(0).call(y) }) + (base));
        }
    );
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
