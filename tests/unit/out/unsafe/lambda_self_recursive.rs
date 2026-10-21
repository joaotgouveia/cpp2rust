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
    let mut fact: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |mut self_: FnPtr<Generic>, mut n: ()| -> () {
            if ((n) <= (1_())) {
                return 1_();
            }
            return ((n)
                * (unsafe {
                    let _self_: FnPtr<Generic> = self_.copy_from();
                    self_
                        .spec::<fn(FnPtr<Generic>, ()) -> ()>(0)
                        .call(_self_, ((n) - (1_())))
                }));
        });
    unimplemented!();
    let mut calls: () = 0_();
    let mut fib: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let calls: *mut () = &mut calls;
        },
        |self_: *mut FnPtr<Generic>, mut n: ()| -> () {
            (*calls).postfix_inc();
            if ((n) <= (2_())) {
                return 1_();
            }
            return ((unsafe {
                let _self_: *mut FnPtr<Generic> = self_;
                let _n: () = ((n) - (1_()));
                (*self_)
                    .spec::<fn(*mut FnPtr<Generic>, ()) -> ()>(0)
                    .call(_self_, _n)
            }) + (unsafe {
                let _self_: *mut FnPtr<Generic> = self_;
                let _n: () = ((n) - (2_()));
                (*self_)
                    .spec::<fn(*mut FnPtr<Generic>, ()) -> ()>(0)
                    .call(_self_, _n)
            }));
        }
    );
    unimplemented!();
    unimplemented!();
    let mut depth: () = 0_();
    let mut count_down: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let depth: *mut () = &mut depth;
        },
        |self_: *const FnPtr<Generic>, mut n: ()| {
            if ((n) == (0_())) {
                return;
            }
            (*depth).postfix_inc();
            (unsafe {
                let _self_: *const FnPtr<Generic> = self_;
                let _n: () = ((n) - (1_()));
                (*self_)
                    .spec::<fn(*const FnPtr<Generic>, ())>(0)
                    .call(_self_, _n)
            });
        }
    );
    (unsafe {
        let _self_: *const FnPtr<Generic> = &count_down;
        count_down
            .spec::<fn(*const FnPtr<Generic>, ())>(0)
            .call(_self_, 4_())
    });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
