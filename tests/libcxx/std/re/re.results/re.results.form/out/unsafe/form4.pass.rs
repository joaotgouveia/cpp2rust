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
        let mut m: () = unimplemented!();
        let s: [(); 12] = std::mem::transmute(*b"abcdefghijk\0");
        unimplemented!();
        let fmt: [(); 54] =
            std::mem::transmute(*b"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2\0");
        let mut out: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        let s: [(); 12] = std::mem::transmute(*b"abcdefghijk\0");
        unimplemented!();
        let fmt: [(); 54] =
            std::mem::transmute(*b"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2\0");
        let mut out: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        let s: [(); 12] = std::mem::transmute(*b"abcdefghijk\0");
        unimplemented!();
        let fmt: [(); 29] = std::mem::transmute(*b"match: &, m[1]: \\1, m[2]: \\2\0");
        let mut out: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        let s: [(); 12] = [
            97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (), 104 as (),
            105 as (), 106 as (), 107 as (), 0 as (),
        ];
        unimplemented!();
        let fmt: [(); 54] = [
            112 as (), 114 as (), 101 as (), 102 as (), 105 as (), 120 as (), 58 as (), 32 as (),
            36 as (), 96 as (), 44 as (), 32 as (), 109 as (), 97 as (), 116 as (), 99 as (),
            104 as (), 58 as (), 32 as (), 36 as (), 38 as (), 44 as (), 32 as (), 115 as (),
            117 as (), 102 as (), 102 as (), 105 as (), 120 as (), 58 as (), 32 as (), 36 as (),
            39 as (), 44 as (), 32 as (), 109 as (), 91 as (), 49 as (), 93 as (), 58 as (),
            32 as (), 36 as (), 49 as (), 44 as (), 32 as (), 109 as (), 91 as (), 50 as (),
            93 as (), 58 as (), 32 as (), 36 as (), 50 as (), 0 as (),
        ];
        let mut out: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        let s: [(); 12] = [
            97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (), 104 as (),
            105 as (), 106 as (), 107 as (), 0 as (),
        ];
        unimplemented!();
        let fmt: [(); 54] = [
            112 as (), 114 as (), 101 as (), 102 as (), 105 as (), 120 as (), 58 as (), 32 as (),
            36 as (), 96 as (), 44 as (), 32 as (), 109 as (), 97 as (), 116 as (), 99 as (),
            104 as (), 58 as (), 32 as (), 36 as (), 38 as (), 44 as (), 32 as (), 115 as (),
            117 as (), 102 as (), 102 as (), 105 as (), 120 as (), 58 as (), 32 as (), 36 as (),
            39 as (), 44 as (), 32 as (), 109 as (), 91 as (), 49 as (), 93 as (), 58 as (),
            32 as (), 36 as (), 49 as (), 44 as (), 32 as (), 109 as (), 91 as (), 50 as (),
            93 as (), 58 as (), 32 as (), 36 as (), 50 as (), 0 as (),
        ];
        let mut out: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        let s: [(); 12] = [
            97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (), 104 as (),
            105 as (), 106 as (), 107 as (), 0 as (),
        ];
        unimplemented!();
        let fmt: [(); 29] = [
            109 as (), 97 as (), 116 as (), 99 as (), 104 as (), 58 as (), 32 as (), 38 as (),
            44 as (), 32 as (), 109 as (), 91 as (), 49 as (), 93 as (), 58 as (), 32 as (),
            92 as (), 49 as (), 44 as (), 32 as (), 109 as (), 91 as (), 50 as (), 93 as (),
            58 as (), 32 as (), 92 as (), 50 as (), 0 as (),
        ];
        let mut out: () = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
