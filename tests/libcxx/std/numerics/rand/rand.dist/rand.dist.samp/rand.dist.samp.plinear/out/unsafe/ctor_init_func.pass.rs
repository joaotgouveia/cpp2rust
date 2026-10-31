extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f_0(mut x: f64) -> f64 {
    return ((x) * (2_f64));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut d: () = unimplemented!();
        let mut iv: Vec<f64> = unimplemented!();
        assert!(((iv.len()) == (2_usize)));
        assert!(((*(&mut iv).as_mut_ptr().add(0_usize as usize)) == (0_f64)));
        assert!(((*(&mut iv).as_mut_ptr().add(1_usize as usize)) == (1_f64)));
        let mut dn: Vec<f64> = unimplemented!();
        assert!(((dn.len()) == (2_usize)));
        assert!(((*(&mut dn).as_mut_ptr().add(0_usize as usize)) == (1_f64)));
        assert!(((*(&mut dn).as_mut_ptr().add(1_usize as usize)) == (1_f64)));
    }
    {
        let mut d: () = unimplemented!();
        let mut iv: Vec<f64> = unimplemented!();
        assert!(((iv.len()) == (2_usize)));
        assert!(((*(&mut iv).as_mut_ptr().add(0_usize as usize)) == (0_f64)));
        assert!(((*(&mut iv).as_mut_ptr().add(1_usize as usize)) == (1_f64)));
        let mut dn: Vec<f64> = unimplemented!();
        assert!(((dn.len()) == (2_usize)));
        assert!(((*(&mut dn).as_mut_ptr().add(0_usize as usize)) == (1_f64)));
        assert!(((*(&mut dn).as_mut_ptr().add(1_usize as usize)) == (1_f64)));
    }
    {
        let mut d: () = unimplemented!();
        let mut iv: Vec<f64> = unimplemented!();
        assert!(((iv.len()) == (2_usize)));
        assert!(((*(&mut iv).as_mut_ptr().add(0_usize as usize)) == (10_f64)));
        assert!(((*(&mut iv).as_mut_ptr().add(1_usize as usize)) == (12_f64)));
        let mut dn: Vec<f64> = unimplemented!();
        assert!(((dn.len()) == (2_usize)));
        assert!(((*(&mut dn).as_mut_ptr().add(0_usize as usize)) == ((20_f64) / (44_f64))));
        assert!(((*(&mut dn).as_mut_ptr().add(1_usize as usize)) == ((24_f64) / (44_f64))));
    }
    {
        let mut d: () = unimplemented!();
        let mut iv: Vec<f64> = unimplemented!();
        assert!(((iv.len()) == (3_usize)));
        assert!(((*(&mut iv).as_mut_ptr().add(0_usize as usize)) == (6_f64)));
        assert!(((*(&mut iv).as_mut_ptr().add(1_usize as usize)) == (10_f64)));
        assert!(((*(&mut iv).as_mut_ptr().add(2_usize as usize)) == (14_f64)));
        let mut dn: Vec<f64> = unimplemented!();
        assert!(((dn.len()) == (3_usize)));
        assert!(((*(&mut dn).as_mut_ptr().add(0_usize as usize)) == (0.075_f64)));
        assert!(((*(&mut dn).as_mut_ptr().add(1_usize as usize)) == (0.125_f64)));
        assert!(((*(&mut dn).as_mut_ptr().add(2_usize as usize)) == (0.175_f64)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
