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
    let mut l: () = unimplemented!();
    {
        let f: *const () = unimplemented!();
        let mut in_: Vec<libc::c_char> = std::slice::from_raw_parts(
            c" A\x07.a1".as_ptr(),
            libcc2rs::c_str_len(c" A\x07.a1".as_ptr()) + 1,
        )
        .to_vec();
        let mut v: Vec<i32> = (0..(in_.len() - 1) as usize)
            .map(|_| 0_i32)
            .collect::<Vec<_>>();
        assert!(
            ((unimplemented!())
                == ((in_.as_mut_ptr().offset((in_.len() - 1) as isize)).cast_const()))
        );
        assert!((((*(&mut v).as_mut_ptr().add(0_usize as usize)) as i32) == (32 as i32)));
        assert!((((*(&mut v).as_mut_ptr().add(1_usize as usize)) as i32) == (65 as i32)));
        assert!((((*(&mut v).as_mut_ptr().add(2_usize as usize)) as i32) == (7 as i32)));
        assert!((((*(&mut v).as_mut_ptr().add(3_usize as usize)) as i32) == (46 as i32)));
        assert!((((*(&mut v).as_mut_ptr().add(4_usize as usize)) as i32) == (97 as i32)));
        assert!((((*(&mut v).as_mut_ptr().add(5_usize as usize)) as i32) == (49 as i32)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
