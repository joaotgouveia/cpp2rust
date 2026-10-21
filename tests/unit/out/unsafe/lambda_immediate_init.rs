extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn pick_0(mut x: Option<()>) -> () {
    let mut x: () = x.unwrap_or_else(|| unsafe {
        (unsafe {
            FnPtr::<fn() -> ()>::new(|| -> () {
                unsafe {
                    return 237_();
                }
            })
            .call()
        })
    });
    return x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S {
    pub j: (),
    pub k: (),
}
impl Default for S {
    fn default() -> Self {
        unsafe {
            S {
                j: 10_(),
                k: ((unsafe {
                    FnPtr::<fn() -> ()>::new(|| -> () {
                        unsafe {
                            return 3_();
                        }
                    })
                    .call()
                }) + (1_())),
            }
        }
    }
}
pub static mut g_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (unsafe {
        FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                let mut s: () = unimplemented!();
                let mut i: () = 1_();
                'loop_: while ((i) <= (4_())) {
                    s += i;
                    i.postfix_inc();
                }
                return s;
            }
        })
        .call()
    })
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    let mut s: S = <S>::default();
    unimplemented!();
    unimplemented!();
    let mut a: () = 2_();
    let c: () = (unsafe {
        lambda_unsafe!(
            {
                let a: *mut () = &mut a;
            },
            || -> () {
                (*a).postfix_inc();
                return ((*a) * (10_()));
            }
        )
        .call()
    });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const g_1);
}
