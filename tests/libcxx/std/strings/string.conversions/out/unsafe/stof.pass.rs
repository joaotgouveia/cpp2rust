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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    assert!(((unimplemented!()) == (0_f32)));
    assert!(((unimplemented!()) == (0_f32)));
    assert!(((unimplemented!()) == (-10_i32 as f32)));
    assert!(((unimplemented!()) == (10_f32)));
    {
        let mut idx: usize = 0_usize;
        assert!(((unimplemented!()) == (10_f32)));
        assert!(((idx) == (2_usize)));
    }
    {
        let mut idx: usize = 0_usize;
        assert!(((unimplemented!()) == (f32::INFINITY)));
        assert!(((idx) == (3_usize)));
    }
    {
        let mut idx: usize = 0_usize;
        assert!(unimplemented!());
        assert!(((idx) == (3_usize)));
    }
    assert!(((unimplemented!()) == (0_f32)));
    assert!(((unimplemented!()) == (0_f32)));
    assert!(((unimplemented!() as f64) == (-10.5_f64)));
    assert!(((unimplemented!()) == (10_f32)));
    {
        let mut idx: usize = 0_usize;
        assert!(((unimplemented!()) == (10_f32)));
        assert!(((idx) == (2_usize)));
    }
    {
        let mut idx: usize = 0_usize;
        assert!(((unimplemented!()) == (f32::INFINITY)));
        assert!(((idx) == (3_usize)));
    }
    {
        let mut idx: usize = 0_usize;
        assert!(unimplemented!());
        assert!(((idx) == (3_usize)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
