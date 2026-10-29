extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Width_enum = ();
pub const Width_enum_W_64: Width_enum = 0;
pub const Width_enum_W_32: Width_enum = 1;
pub const Width_enum_W_16: Width_enum = 2;
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub text: *const (),
    pub handle: (),
    pub signed_n: (),
    pub f: (),
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Sink {
    pub width: Width_enum,
    pub out: anon_0,
}
pub unsafe fn write_count_1(mut s: *mut Sink, mut count: ()) {
    'switch: {
        match { ((*s).width as ()) } {
            __v if __v == (Width_enum_W_64 as i32) => {
                (*((*s).out.handle as *mut ())) = count;
                break 'switch;
            }
            __v if __v == (Width_enum_W_32 as i32) => {
                (*((*s).out.handle as *mut ())) = (count as ());
                break 'switch;
            }
            __v if __v == (Width_enum_W_16 as i32) => {
                (*((*s).out.handle as *mut ())) = (count as ());
                break 'switch;
            }
            _ => {}
        }
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut buf64: () = 0_();
    let mut buf32: () = 0_();
    let mut buf16: () = 0_();
    let mut s: Sink = <Sink>::default();
    s.width = Width_enum_W_64;
    s.out.handle = ((&mut buf64 as *mut ()) as ());
    (unsafe { write_count_1((&mut s as *mut Sink), 1234605616436508552_()) });
    unimplemented!();
    s.width = Width_enum_W_32;
    s.out.handle = ((&mut buf32 as *mut ()) as ());
    (unsafe { write_count_1((&mut s as *mut Sink), 305419896_()) });
    unimplemented!();
    s.width = Width_enum_W_16;
    s.out.handle = ((&mut buf16 as *mut ()) as ());
    (unsafe { write_count_1((&mut s as *mut Sink), 4660_()) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
