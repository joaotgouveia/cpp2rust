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
    let mut v: std_vector_int__std_allocator_int__ =
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None);
    let mut f: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let v: std_vector_int__std_allocator_int__ = v.clone();
        },
        || -> () {
            return (unsafe { v.size() });
        }
    );
    let mut g: FnPtr<fn() -> ()> = f.copy_from();
    let mut h: FnPtr<fn() -> ()> = f.move_from();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let u: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
                let mut ___args: () = 5_();
                make_unique_0(&mut ___args)
            });
        },
        || -> () {
            return if !(unsafe { u.get() }).is_null() {
                (*u)
            } else {
                0_()
            };
        }
    );
    let mut q: FnPtr<fn() -> ()> = p.move_from();
    unimplemented!();
    unimplemented!();
    let mut n: () = 0_();
    let mut inner: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let n: () = n;
        },
        || -> () {
            return n.prefix_inc();
        }
    );
    let mut outer: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let inner: FnPtr<fn() -> ()> = inner.copy_from();
        },
        || -> () {
            return (unsafe { inner.call() });
        }
    );
    unimplemented!();
    let mut outer2: FnPtr<fn() -> ()> = outer.copy_from();
    unimplemented!();
    unimplemented!();
    let mut outer3: FnPtr<fn() -> ()> = outer.move_from();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
