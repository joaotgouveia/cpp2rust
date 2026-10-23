extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn truncate_fp_0(mut val: ()) -> () {
    let mut sink: () = val;
    return sink;
}
pub unsafe fn truncate_fp_1(mut val: ()) -> () {
    let mut sink: () = val;
    return sink;
}
pub unsafe fn truncate_fp_2(mut val: ()) -> () {
    let mut sink: () = val;
    return sink;
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
        let mut r: () = unimplemented!();
        let mut f: () = (unsafe { generate_canonical_3(&mut r) });
        unimplemented!();
    }
    {
        let mut r: () = unimplemented!();
        let mut f: () = (unsafe { generate_canonical_4(&mut r) });
        unimplemented!();
    }
    {
        let mut r: () = unimplemented!();
        let mut f: () = (unsafe { generate_canonical_5(&mut r) });
        unimplemented!();
    }
    {
        let mut r: () = unimplemented!();
        let mut f: () = (unsafe { generate_canonical_6(&mut r) });
        unimplemented!();
    }
    {
        let mut r: () = unimplemented!();
        let mut f: () = (unsafe { generate_canonical_7(&mut r) });
        unimplemented!();
    }
    {
        let mut r: () = unimplemented!();
        let mut f: () = (unsafe { generate_canonical_8(&mut r) });
        unimplemented!();
    }
    {
        let mut r: () = unimplemented!();
        let mut f: () = (unsafe { generate_canonical_9(&mut r) });
        unimplemented!();
    }
    {
        let mut r: () = unimplemented!();
        let mut f: () = (unsafe { generate_canonical_10(&mut r) });
        unimplemented!();
    }
    {
        let mut r: () = unimplemented!();
        let mut f: () = (unsafe { generate_canonical_11(&mut r) });
        unimplemented!();
    }
    {
        let mut r: () = unimplemented!();
        let mut f: () = (unsafe { generate_canonical_12(&mut r) });
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
