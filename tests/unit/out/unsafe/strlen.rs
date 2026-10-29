extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut ptr: *mut ()) -> () {
    let mut count: () = 0_();
    'loop_: while ((*ptr.postfix_inc()) != ('\0' as ())) {
        count.prefix_inc();
    }
    return count;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut string: [(); 6] = [
        ('h' as ()),
        ('e' as ()),
        ('l' as ()),
        ('l' as ()),
        ('o' as ()),
        ('\0' as ()),
    ];
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
