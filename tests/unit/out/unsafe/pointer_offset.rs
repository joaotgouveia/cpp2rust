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
unsafe fn main_0() -> () {
    let mut out: () = 0_();
    let mut arr: [(); 5] = [1_(), 2_(), 3_(), 4_(), 0_()];
    let mut ptr: *const () = (&mut arr[(0_()) as usize] as *mut ()).cast_const();
    'loop_: while ((*ptr) != (0_())) {
        out += (*ptr);
        ptr.prefix_inc();
    }
    let mut ptr: *mut () = (&mut arr[(1_()) as usize] as *mut ());
    'loop_: while ((*ptr) != (4_())) {
        out += (*ptr);
        ptr.postfix_inc();
    }
    let mut ptr: *mut () = (&mut arr[(4_()) as usize] as *mut ());
    'loop_: while ((*ptr) != (1_())) {
        out += (*ptr);
        ptr.postfix_dec();
    }
    let mut ptr: *const () = (&mut arr[(3_()) as usize] as *mut ()).cast_const();
    'loop_: while ((*ptr) != (2_())) {
        out += (*ptr);
        ptr.prefix_dec();
    }
    let mut ptr: *mut () = (&mut arr[(0_()) as usize] as *mut ());
    'loop_: while ((*ptr) != (0_())) {
        out += (*ptr);
        ptr = ptr.offset((1_()) as isize);
    }
    let mut ptr: *mut () = (&mut arr[(0_()) as usize] as *mut ());
    let mut i: () = 0_();
    'loop_: while ((i) < (5_())) {
        out += (*ptr.offset((i) as isize));
        i.prefix_inc();
    }
    unimplemented!();
    let mut step: () = 2_();
    let mut offset: *mut () = arr.as_mut_ptr().offset((step) as isize);
    unimplemented!();
    offset = offset.offset(-((step) as isize));
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
