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
pub struct Val {
    pub x: (),
}
pub unsafe fn sum_0(mut a: Val, mut b: Val) -> () {
    return ((a.x) + (b.x));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut total: () = 0_();
    let mut tally: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let total: *mut () = &mut total;
        },
        || {
            (*total) = (*total).wrapping_add(
                ((::std::mem::size_of::<()>() as ())
                    .wrapping_add((::std::mem::size_of::<()>() as ())) as ()),
            );
        },
        || {
            (*total) = (*total).wrapping_add(
                ((::std::mem::size_of::<()>() as ())
                    .wrapping_add((::std::mem::size_of::<()>() as ())) as ()),
            );
        }
    );
    (unsafe { tally.spec::<fn()>(0).call() });
    (unsafe { tally.spec::<fn()>(1).call() });
    unimplemented!();
    let mut v: Val = Val { x: 5_() };
    let mut acc: () = 0_();
    let mut pick: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: *mut Val = &mut v;
            let acc: *mut () = &mut acc;
        },
        || {
            (*acc) += (unsafe {
                let _a: Val = (*v);
                let _b: Val = (*v);
                sum_0(_a, _b)
            });
        },
        || {
            (*acc) += (unsafe {
                let _a: Val = (*v);
                let _b: Val = (*v);
                sum_0(_a, _b)
            });
        },
        || {
            (*acc) += (unsafe {
                let _a: Val = (*v);
                let _b: Val = (*v);
                sum_0(_a, _b)
            });
        }
    );
    (unsafe { pick.spec::<fn()>(0).call() });
    (unsafe { pick.spec::<fn()>(1).call() });
    (unsafe { pick.spec::<fn()>(2).call() });
    unimplemented!();
    let mut cast_to: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut x: ()| -> () {
            return ((x as ()) / (2_()));
        },
        |mut x: ()| -> () {
            return ((x) / (2_()));
        }
    );
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
