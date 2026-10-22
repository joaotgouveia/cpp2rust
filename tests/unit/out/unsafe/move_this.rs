extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Chain {
    pub v: (),
}
impl Chain {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Chain) -> Self {
        let mut this = Self {
            v: (((*o).v) + (100_())),
        };
        this
    }
    pub unsafe fn move_from(o: *mut Chain) -> Self {
        let mut this = Self {
            v: (((*o).v) + (1_())),
        };
        (*o).v = 0_();
        this
    }
    pub unsafe fn add_4(&mut self, mut n: ()) -> *mut Chain {
        self.v += n;
        return &mut (*(self as *mut Chain));
    }
    pub unsafe fn add_5(&mut self, mut n: ()) -> *mut Chain {
        self.v += n;
        return &mut (*unimplemented!());
    }
    pub unsafe fn take(&mut self) -> Chain {
        return Chain::move_from({ &mut (*unimplemented!()) });
    }
    pub unsafe fn copy(&self) -> Chain {
        return Chain::copy_from({ &(*(self as *const Chain)) });
    }
    pub unsafe fn self_(&mut self) -> *mut Chain {
        return &mut (*unimplemented!());
    }
}
impl Clone for Chain {
    fn clone(&self) -> Self {
        unsafe { Chain::copy_from(self as *const Chain) }
    }
}
pub unsafe fn consume_0(mut c: Chain) -> () {
    return c.v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: Chain = Chain::new({ 1_() });
    (unsafe { Chain::add_4(&mut (*(unsafe { Chain::add_4(&mut a, 1_()) })), 1_()) });
    unimplemented!();
    let mut b0: Chain = Chain::new({ 5_() });
    let mut b: Chain = Chain::move_from({
        (unsafe {
            Chain::add_5(
                &mut (*(unsafe { Chain::add_5(&mut (*unimplemented!()), 1_()) })),
                1_(),
            )
        })
    });
    unimplemented!();
    let mut c: Chain = (unsafe { Chain::take(&mut Chain::new({ 10_() })) });
    unimplemented!();
    let mut d: Chain = (unsafe { Chain::copy(&c) });
    unimplemented!();
    let mut g: Chain = Chain::new({ 20_() });
    unimplemented!();
    let mut e: Chain = Chain::new({ 30_() });
    let mut f: Chain = (unsafe { Chain::take(&mut (*unimplemented!())) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
