extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut x: ()) -> () {
    return x;
}
pub unsafe fn foo_1(mut x: *mut ()) -> () {
    return (*x);
}
pub unsafe fn foo_2(mut x: *mut (), mut y: *mut ()) -> () {
    return ((*x) + (*y));
}
pub unsafe fn foo_3(mut x: *mut (), mut y: *mut (), z: *mut ()) -> () {
    return (((*x) + (*y)) + (*z));
}
pub unsafe fn bar_4(x: *mut ()) -> () {
    return (*x);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {}
impl Foo {
    pub unsafe fn foo_1(&self) {}
    pub unsafe fn foo_2(&mut self) {}
    pub unsafe fn method_3(&mut self, mut x: ()) {}
    pub unsafe fn method_4(&self, mut x: ()) {}
    pub unsafe fn method2_5(&self, mut x: (), mut y: ()) {}
    pub unsafe fn method2_6(&self, mut x: (), mut y: ()) {}
}
pub unsafe fn func_5(mut x: ()) -> () {
    return 1_();
}
pub unsafe fn func_6(mut x: *mut ()) -> () {
    return 1_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = 1_();
    let mut out: () = unimplemented!();
    out += (unsafe { foo_0(unimplemented!()) });
    out += (unsafe { foo_1((&mut x as *mut ())) });
    out += (unsafe { bar_4(&mut x) });
    out += (unsafe {
        let _x: *mut () = (&mut x as *mut ());
        let _y: *mut () = (&mut x as *mut ());
        let _z: *mut () = &mut x;
        foo_3(_x, _y, _z)
    });
    out += (unsafe {
        let _x: *mut () = (&mut x as *mut ());
        let _y: *mut () = (&mut x as *mut ());
        foo_2(_x, _y)
    });
    let mut bar: () = 5_();
    out +=
        (((bar) + (unsafe { foo_0(unimplemented!()) })) + (unsafe { foo_1((&mut x as *mut ())) }));
    let mut foo1: Foo = <Foo>::default();
    let foo2: Foo = <Foo>::default();
    (unsafe { Foo::foo_2(&mut foo1) });
    (unsafe { Foo::method_3(&mut foo1, 1_()) });
    (unsafe { Foo::foo_1(&foo2) });
    (unsafe { Foo::method_4(&foo2, 2_()) });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
