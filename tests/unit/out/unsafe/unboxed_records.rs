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
    pub b: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cookie {
    pub x: (),
    pub data: *mut (),
    pub in_: *const (),
    pub inner: Inner,
    pub arr: [(); 4],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Counter {
    pub n: (),
}
impl Counter {
    pub unsafe fn inc(&mut self) {
        self.n.prefix_inc();
    }
}
impl Default for Counter {
    fn default() -> Self {
        unsafe {
            Counter {
                n: unimplemented!(),
            }
        }
    }
}
pub unsafe fn set_0(mut p: *mut (), mut v: ()) {
    (*p) = v;
}
pub unsafe fn get_1(c: *const Cookie) -> () {
    return (*c).x;
}
pub unsafe fn first_2(mut p: *const ()) -> () {
    return (*p.offset((unimplemented!()) as isize));
}
pub unsafe fn consume_3(cookie: *const Cookie) -> () {
    let mut c: Cookie = (*cookie);
    let mut sum: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (4_())) {
        (*c.data.offset((i) as isize)) = ((*c.in_.offset((i) as isize)) * (c.x));
        sum += (*c.data.offset((i) as isize));
        i.prefix_inc();
    }
    c.x = sum;
    c.inner.a += c.x;
    c.arr[(1_()) as usize] = c.inner.a;
    (*c.data) = ((c.arr[(1_()) as usize]) + (unsafe { first_2(c.in_) }));
    (unsafe {
        let _p: *mut () = c.data.offset((1_()) as isize);
        let _v: () = (unsafe { first_2((c.data).cast_const()) });
        set_0(_p, _v)
    });
    return ((c.x) + (c.inner.b));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut data: [(); 4] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut in_: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    let mut cookie: Cookie = <Cookie>::default();
    cookie.x = 2_();
    cookie.data = data.as_mut_ptr();
    cookie.in_ = (in_.as_mut_ptr()).cast_const();
    cookie.inner.a = 1_();
    cookie.inner.b = 3_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut local: Inner = <Inner>::default();
    local.a = 4_();
    local.b = ((local.a) * (2_()));
    local.a.postfix_inc();
    unimplemented!();
    let mut field_addr: Inner = Inner {
        a: unimplemented!(),
        b: unimplemented!(),
    };
    (unsafe { set_0((&mut field_addr.b as *mut ()), 7_()) });
    unimplemented!();
    let mut counter: Counter = <Counter>::default();
    (unsafe { Counter::inc(&mut counter) });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
