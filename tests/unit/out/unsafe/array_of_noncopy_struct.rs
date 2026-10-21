extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct NonCopy {
    pub data: (),
    pub tag: (),
}
impl Default for NonCopy {
    fn default() -> Self {
        unsafe {
            NonCopy {
                data: unimplemented!(),
                tag: unimplemented!(),
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
    let mut arr: [NonCopy; 3] = std::array::from_fn::<_, 3, _>(|_| <NonCopy>::default());
    arr[(unimplemented!()) as usize].tag = 7_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
