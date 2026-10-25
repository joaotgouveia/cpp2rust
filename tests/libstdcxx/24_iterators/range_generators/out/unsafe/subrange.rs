extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn yield_vector_0() -> () {
    {
        let mut foo: () = unimplemented!();
        let mut x: () = 123_();
        (unsafe { (__promise as std___gen__Promise_erased_int_ref_).yield_value_2(&mut x) });
        unimplemented!();
        x = 456_();
        (unsafe { (__promise as std___gen__Promise_erased_int_ref_).yield_value_2(&mut x) });
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut __tmp_0: () = (unsafe { yield_vector_0() });
    let __range1: *mut () = &mut __tmp_0;
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: () = (*unimplemented!());
        unimplemented!();
    }
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
