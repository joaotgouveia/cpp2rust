extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn my_alternative_fread_0(mut p: *mut (), mut n: (), mut m: (), mut f: ()) -> () {
    return 22_();
}
pub unsafe fn my_alternative_fwrite_1(mut p: *const (), mut n: (), mut m: (), mut f: ()) -> () {
    return 33_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut fn1: Option<unsafe fn((), (), (), ()) -> ()> = Some(libcc2rs::fread_unsafe);
    unimplemented!();
    unimplemented!();
    let mut fn2: Option<unsafe fn(*mut (), (), (), ()) -> ()> = std::mem::transmute::<
        Option<unsafe fn((), (), (), ()) -> ()>,
        Option<unsafe fn(*mut (), (), (), ()) -> ()>,
    >(Some(libcc2rs::fread_unsafe));
    unimplemented!();
    let mut f3: Option<unsafe fn((), (), (), ()) -> ()> = std::mem::transmute::<
        Option<unsafe fn(*mut (), (), (), ()) -> ()>,
        Option<unsafe fn((), (), (), ()) -> ()>,
    >(Some(my_alternative_fread_0));
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut stream: () = unimplemented!();
        unimplemented!();
        let mut buf: [(); 16] = [unimplemented!(); 16];
        unimplemented!();
        let mut n: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (10_())) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (16_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut stream: () = unimplemented!();
        unimplemented!();
        let mut buf: [(); 16] = [unimplemented!(); 16];
        unimplemented!();
        let mut n: () = (unsafe { (fn1).unwrap()((buf.as_mut_ptr() as ()), 1_(), 10_(), stream) });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (10_())) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (16_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    let mut gn1: Option<unsafe fn((), (), (), ()) -> ()> = Some(libcc2rs::fwrite_unsafe);
    unimplemented!();
    unimplemented!();
    let mut gn2: Option<unsafe fn(*const (), (), (), ()) -> ()> =
        std::mem::transmute::<
            Option<unsafe fn((), (), (), ()) -> ()>,
            Option<unsafe fn(*const (), (), (), ()) -> ()>,
        >(Some(libcc2rs::fwrite_unsafe));
    unimplemented!();
    let mut g3: Option<unsafe fn((), (), (), ()) -> ()> = std::mem::transmute::<
        Option<unsafe fn(*const (), (), (), ()) -> ()>,
        Option<unsafe fn((), (), (), ()) -> ()>,
    >(Some(my_alternative_fwrite_1));
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut stream: () = unimplemented!();
        unimplemented!();
        let mut buf: [(); 10] = [unimplemented!(); 10];
        unimplemented!();
        let mut n: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut stream: () = unimplemented!();
        unimplemented!();
        let mut buf: [(); 10] = [unimplemented!(); 10];
        unimplemented!();
        let mut n: () = (unsafe { (gn1).unwrap()((buf.as_mut_ptr() as ()), 1_(), 10_(), stream) });
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
