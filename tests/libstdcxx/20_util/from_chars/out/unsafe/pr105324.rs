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
    let mut s: Vec<libc::c_char> = vec![('1' as libc::c_char); (512_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
    (*(&mut s).as_mut_ptr().add(1_usize as usize)) = ('.' as libc::c_char);
    let mut d: f64 = 0_f64;
    unimplemented!();
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
