extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Layout {
    pub a: (),
    pub b: (),
    pub c: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Frame {
    pub tag: (),
    pub body: [(); 64],
}
impl Default for Frame {
    fn default() -> Self {
        unsafe {
            Frame {
                tag: unimplemented!(),
                body: [unimplemented!(); 64],
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v: Layout = Layout {
        a: unimplemented!(),
        b: unimplemented!(),
        c: unimplemented!(),
    };
    v.b = 3735928559_();
    let mut base: *mut () = ((&mut v as *mut Layout) as *mut ());
    let mut bp: *mut () = ((base.offset((::std::mem::offset_of!(Layout, b)) as isize)) as *mut ());
    unimplemented!();
    (*((base.offset((::std::mem::offset_of!(Layout, b)) as isize)) as *mut ())) = 305419896_();
    unimplemented!();
    let mut text: *const () = c"example-body".as_ptr();
    let mut len: () = (unimplemented!()).wrapping_add(1_());
    let mut total: () = (::std::mem::offset_of!(Frame, body) as ()).wrapping_add(len);
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
