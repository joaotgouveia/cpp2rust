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
pub struct Inner {
    pub a: (),
    pub name: [(); 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Header {
    pub tag: (),
    pub size: (),
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub x: (),
    pub inner: Inner,
    pub items: [Inner; 3],
    pub v: (),
    pub cursor: *mut (),
    pub buf: [(); 4],
}
impl Outer {
    pub unsafe fn next(&mut self) -> () {
        return self.buf[(self.x.postfix_inc()) as usize];
    }
    pub unsafe fn sum(&self) -> () {
        return ((self.x) + (self.inner.a));
    }
    pub unsafe fn push(&mut self, mut k: ()) {
        unimplemented!();
    }
}
pub unsafe fn set_0(mut p: *mut (), mut value: ()) {
    (*p) = value;
}
pub unsafe fn bump_1(mut o: *mut Outer) -> () {
    (*o).x.postfix_inc();
    return 1_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut o: Outer = <Outer>::default();
    o.x = 1_();
    let mut px: *mut () = (&mut o.x as *mut ());
    (*px) = 2_();
    unimplemented!();
    let mut pa: *mut () = (&mut o.inner.a as *mut ());
    (unsafe {
        let _p: *mut () = pa;
        set_0(_p, 3_())
    });
    unimplemented!();
    let mut pi: *mut () = (&mut o.items[(1_()) as usize].a as *mut ());
    (*pi) = 4_();
    unimplemented!();
    unimplemented!();
    let mut name: *mut () = o.inner.name.as_mut_ptr();
    let mut i: () = 0_();
    'loop_: while ((i) < (3_())) {
        (*name.offset((i) as isize)) = (('a' as ()) + (i));
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    o.cursor = (&mut o.buf[(1_()) as usize] as *mut ());
    (*o.cursor) = 5_();
    (*o.cursor.postfix_inc()) += 1_();
    unimplemented!();
    o.x = 0_();
    let mut first: () = (unsafe { Outer::next(&mut o) });
    unimplemented!();
    o.x = ((o.inner.a) + (unsafe { bump_1((&mut o as *mut Outer)) }));
    unimplemented!();
    (unsafe {
        let _k: () = o.inner.a;
        Outer::push(&mut o, _k)
    });
    unimplemented!();
    unimplemented!();
    let mut y: () = 0_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut bytes: [(); 8] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut view: *mut Header = (bytes.as_mut_ptr() as *mut Header);
    (*view).tag = 16909060_();
    unimplemented!();
    (unsafe { set_0((&mut (*view).tag as *mut ()), 0_()) });
    unimplemented!();
    (*view).size = 1_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
