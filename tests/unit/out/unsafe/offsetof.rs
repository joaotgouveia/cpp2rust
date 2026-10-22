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
    pub a: u8,
    pub b: u32,
    pub c: u16,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Frame {
    pub tag: u16,
    pub body: [(); 64],
}
impl Default for Frame {
    fn default() -> Self {
        unsafe {
            Frame {
                tag: 0_u16,
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
        a: 0_u8,
        b: 0_u32,
        c: 0_u16,
    };
    v.b = (3735928559_() as u32);
    let mut base: *mut () = ((&mut v as *mut Layout) as *mut ());
    let mut bp: *mut u32 =
        ((base.offset((::std::mem::offset_of!(Layout, b)) as isize)) as *mut u32);
    unimplemented!();
    (*((base.offset((::std::mem::offset_of!(Layout, b)) as isize)) as *mut u32)) = 305419896_u32;
    unimplemented!();
    let mut text: *const () = c"example-body".as_ptr();
    let mut len: () = (unsafe { strlen_0(text) }).wrapping_add(1_());
    let mut total: () = (::std::mem::offset_of!(Frame, body) as ()).wrapping_add(len);
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
