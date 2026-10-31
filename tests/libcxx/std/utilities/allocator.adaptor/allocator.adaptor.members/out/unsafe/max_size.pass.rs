extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut move_called_1: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct A1_int_ {
    id_: i32,
}
impl A1_int_ {
    pub unsafe fn new(mut id: i32) -> Self {
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn move_from(a: *mut A1_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A1_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_1)) = true;
        this
    }
    pub unsafe fn max_size(&self) -> usize {
        return (self.id_ as usize);
    }
}
pub static mut copy_called_4: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct A2_int_ {
    id_: i32,
}
impl A2_int_ {
    pub unsafe fn new(mut id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn copy_from(a: *const A2_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A2_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_4)) = true;
        this
    }
}
impl Clone for A2_int_ {
    fn clone(&self) -> Self {
        unsafe { A2_int_::copy_from(self as *const A2_int_) }
    }
}
impl Default for A2_int_ {
    fn default() -> Self {
        unsafe { A2_int_::new(None) }
    }
}
pub static mut copy_called_7: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct A3_int_ {
    id_: i32,
}
impl A3_int_ {
    pub unsafe fn new(mut id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn copy_from(a: *const A3_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A3_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_7)) = true;
        this
    }
}
impl Clone for A3_int_ {
    fn clone(&self) -> Self {
        unsafe { A3_int_::copy_from(self as *const A3_int_) }
    }
}
impl Default for A3_int_ {
    fn default() -> Self {
        unsafe { A3_int_::new(None) }
    }
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let a: () = unimplemented!();
        assert!(((unimplemented!()) == (100_())));
    }
    {
        let a: () = unimplemented!();
        assert!(((unimplemented!()) == (20_())));
    }
    {
        let a: () = unimplemented!();
        assert!(((unimplemented!()) == (200_())));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const move_called_1);
    std::cell::LazyCell::force(&*&raw const copy_called_4);
    std::cell::LazyCell::force(&*&raw const copy_called_7);
}
