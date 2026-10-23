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
    {
        let ft: () = (unsafe { std_filesystem__FilesystemClock::now() });
        let mut st: () = (unsafe { std_filesystem__FilesystemClock::to_sys(&ft) });
        unimplemented!();
    }
    {
        let st: () = unimplemented!();
        let mut ft: () = (unsafe { std_filesystem__FilesystemClock::from_sys(&st) });
        unimplemented!();
    }
    {
        let file_now: () = (unsafe { std_filesystem__FilesystemClock::now() });
        let sys_now: () = unimplemented!();
        {
            let mut diff: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut diff: () = unimplemented!();
            unimplemented!();
        }
    }
    {
        let ft: () = (unsafe { std_filesystem__FilesystemClock::now() });
        let st: () = unimplemented!();
        let mut sys_diff: () = unimplemented!();
        let mut file_diff: () = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
