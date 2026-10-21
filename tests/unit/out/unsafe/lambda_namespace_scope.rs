extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut counter_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut inc_1: std::cell::LazyCell<FnPtr<fn(()) -> ()>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
            unsafe {
                return ((x) + (1_()));
            }
        })
    });
pub static mut bump_2: std::cell::LazyCell<FnPtr<fn() -> ()>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)).postfix_inc();
                return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0));
            }
        })
    });
pub unsafe fn apply_3(mut f: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
    return (unsafe { f.call(x) });
}
pub unsafe fn by_ref_4(v: *const ()) -> () {
    return ((*v) * (2_()));
}
pub static mut twice_next_5: std::cell::LazyCell<FnPtr<fn(()) -> ()>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
            unsafe {
                return (unsafe {
                    let mut _v: () = ((x) + (1_()));
                    by_ref_4(&mut _v)
                });
            }
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
    (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut bump_2)).call() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut copy: FnPtr<fn(()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut inc_1)).copy_from();
    unimplemented!();
    let mut fp: Option<unsafe fn(()) -> ()> = Some(|mut x: ()| -> () {
        unsafe {
            return ((x) + (1_()));
        }
    });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const counter_0);
    std::cell::LazyCell::force(&*&raw const inc_1);
    std::cell::LazyCell::force(&*&raw const bump_2);
    std::cell::LazyCell::force(&*&raw const twice_next_5);
}
