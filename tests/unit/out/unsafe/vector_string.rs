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
unsafe fn main_0() -> i32 {
    let mut v1: Vec<Vec<libc::c_char>> = vec![{
        let s = c"a".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    }];
    assert!(
        (((*if 0_usize as usize >= (*((v1).last_mut().unwrap())).len() - 1 {
            panic!("out of bounds access")
        } else {
            &mut (*((v1).last_mut().unwrap()))[0_usize as usize]
        }) as i32)
            == (('a' as libc::c_char) as i32))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
