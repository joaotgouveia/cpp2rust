extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cmpxchg_weak_loop_0(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unimplemented!()) == (true)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_1(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unimplemented!()) == (true)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_2(
    atomic: *mut (),
    expected: *mut (),
    mut desired: (),
    mut success: (),
    mut failure: (),
) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unimplemented!()) == (true)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_3(
    atomic: *mut (),
    expected: *mut (),
    mut desired: (),
    mut success: (),
    mut failure: (),
) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unimplemented!()) == (true)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut obj: () = unimplemented!();
        unimplemented!();
        {
            let mut lockfree: () = unimplemented!();
            &(lockfree);
            {
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut x: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut obj: () = unimplemented!();
        unimplemented!();
        {
            let mut lockfree: () = unimplemented!();
            &(lockfree);
            {
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut x: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut obj: () = unimplemented!();
        unimplemented!();
        {
            let mut lockfree: () = unimplemented!();
            &(lockfree);
            {
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut x: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 1] = [1_()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
