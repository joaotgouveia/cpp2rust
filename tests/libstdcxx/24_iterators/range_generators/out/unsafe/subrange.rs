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
        let mut foo: Vec<i32> = Vec::from([1, 2, 3]);
        let mut x: i32 = 123;
        unimplemented!();
        unimplemented!();
        x = 456;
        unimplemented!();
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut __tmp_0: () = (unsafe { yield_vector_0() });
    let __range1: *mut () = &mut __tmp_0;
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: i32 = (*unimplemented!());
        {
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                let __s: *mut ::std::fs::File = __r;
                libcc2rs::write_int(&mut *__s, x);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
            __s
        };
    }
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
