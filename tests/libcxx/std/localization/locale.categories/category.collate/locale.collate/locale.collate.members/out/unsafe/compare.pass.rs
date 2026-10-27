extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
    let mut l: () = unimplemented!();
    {
        let ia: [(); 5] = std::mem::transmute(*b"1234\0");
        let sa: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let ib: [(); 4] = std::mem::transmute(*b"123\0");
        let f: *const () = (unsafe { use_facet_0(&l) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ia: [(); 5] = [49 as (), 50 as (), 51 as (), 52 as (), 0 as ()];
        let sa: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let ib: [(); 4] = [49 as (), 50 as (), 51 as (), 0 as ()];
        let f: *const () = (unsafe { use_facet_1(&l) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
