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
    let mut __do_while = true;
    'loop_: while __do_while || (unimplemented!() != 0) {
        __do_while = false;
        let mut p: *mut () = (unimplemented!() as *mut ());
        (*p) = 42_();
        unimplemented!();
        unimplemented!();
        let mut arr: *mut () = (unimplemented!() as *mut ());
        let mut i: () = unimplemented!();
        'loop_: while ((((i) < (4_())) as ()) != 0) {
            (*arr.offset((i) as isize)) = ((i) * (10_()));
            i.postfix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut grow: *mut () = (unimplemented!() as *mut ());
        (*grow.offset((unimplemented!()) as isize)) = 1_();
        (*grow.offset((1_()) as isize)) = 2_();
        grow = (unimplemented!() as *mut ());
        (*grow.offset((2_()) as isize)) = 3_();
        (*grow.offset((3_()) as isize)) = 4_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut zeros: *mut () = (unimplemented!() as *mut ());
        let mut i: () = unimplemented!();
        'loop_: while ((((i) < (4_())) as ()) != 0) {
            unimplemented!();
            i.postfix_inc();
        }
        unimplemented!();
    }
    let mut pmalloc: Option<unsafe fn(()) -> ()> = Some(libcc2rs::malloc_unsafe);
    let mut pfree: Option<unsafe fn(())> = Some(libcc2rs::free_unsafe);
    let mut prealloc: Option<unsafe fn((), ()) -> ()> = Some(libcc2rs::realloc_unsafe);
    let mut pcalloc: Option<unsafe fn((), ()) -> ()> = Some(libcc2rs::calloc_unsafe);
    let mut __do_while = true;
    'loop_: while __do_while || (unimplemented!() != 0) {
        __do_while = false;
        let mut p: *mut () =
            ((unsafe { (pmalloc).unwrap()(::std::mem::size_of::<()>()) }) as *mut ());
        (*p) = 42_();
        unimplemented!();
        (unsafe { (pfree).unwrap()((p as ())) });
        let mut arr: *mut () = ((unsafe {
            (pmalloc).unwrap()((4_()).wrapping_mul((::std::mem::size_of::<()>() as ())))
        }) as *mut ());
        let mut i: () = unimplemented!();
        'loop_: while ((((i) < (4_())) as ()) != 0) {
            (*arr.offset((i) as isize)) = ((i) * (10_()));
            i.postfix_inc();
        }
        unimplemented!();
        unimplemented!();
        (unsafe { (pfree).unwrap()((arr as ())) });
        let mut grow: *mut () = ((unsafe {
            (pmalloc).unwrap()((2_()).wrapping_mul((::std::mem::size_of::<()>() as ())))
        }) as *mut ());
        (*grow.offset((unimplemented!()) as isize)) = 1_();
        (*grow.offset((1_()) as isize)) = 2_();
        grow = ((unsafe {
            (prealloc).unwrap()(
                (grow as ()),
                (4_()).wrapping_mul((::std::mem::size_of::<()>() as ())),
            )
        }) as *mut ());
        (*grow.offset((2_()) as isize)) = 3_();
        (*grow.offset((3_()) as isize)) = 4_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { (pfree).unwrap()((grow as ())) });
        let mut zeros: *mut () =
            ((unsafe { (pcalloc).unwrap()(4_(), ::std::mem::size_of::<()>()) }) as *mut ());
        let mut i: () = unimplemented!();
        'loop_: while ((((i) < (4_())) as ()) != 0) {
            unimplemented!();
            i.postfix_inc();
        }
        (unsafe { (pfree).unwrap()((zeros as ())) });
    }
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
