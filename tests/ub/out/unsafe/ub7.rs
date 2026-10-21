extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const ()) -> () {
    let mut begin: *const () = s;
    'loop_: while ((*s) != 0) {
        s.prefix_inc();
    }
    return ((s as usize - begin as usize) / ::std::mem::size_of::<()>()) as ();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let s: [(); 6] = [
        ('s' as ()),
        ('t' as ()),
        ('r' as ()),
        ('i' as ()),
        ('n' as ()),
        ('g' as ()),
    ];
    return (unsafe { strlen_0((&s[(unimplemented!()) as usize] as *const ())) });
}
pub unsafe fn __cpp2rust_init_globals() {}
