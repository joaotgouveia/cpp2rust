extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub bytes: [(); 1],
    pub aligner: (),
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct node {
    pub len: (),
    pub pos: (),
    pub x: anon_0,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut tail_size: () = 32_();
    let mut n: *mut node = (unimplemented!() as *mut node);
    (*n).len = tail_size;
    let mut i: () = 0_();
    'loop_: while ((((i) < (tail_size)) as ()) != 0) {
        (*(*n).x.bytes.as_mut_ptr().add((i) as usize)) = (((i) & (255_())) as ());
        i.postfix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((((i) < (tail_size)) as ()) != 0) {
        unimplemented!();
        i.postfix_inc();
    }
    let mut p: *mut () = ((*n).x.bytes.as_mut_ptr().add((10_()) as usize));
    unimplemented!();
    (*p) = 170_();
    unimplemented!();
    (*n).pos = 20_();
    let mut q: *mut () = ((*n).x.bytes.as_mut_ptr().add(((*n).pos) as usize));
    unimplemented!();
    (*q) = 187_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
