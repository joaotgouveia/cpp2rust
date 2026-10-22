extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct MyContainer_int_ {
    vec_: (),
}
impl MyContainer_int_ {
    pub unsafe fn empty(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn size(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn back_4(&mut self) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn pop_back(&mut self) {
        unimplemented!();
        return;
    }
    pub unsafe fn push_back(&mut self, item: *const ()) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct MyContainer_char_ {
    vec_: (),
}
impl MyContainer_char_ {
    pub unsafe fn empty(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn size(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn back_4(&mut self) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn pop_back(&mut self) {
        unimplemented!();
        return;
    }
    pub unsafe fn push_back(&mut self, item: *const ()) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct MyContainer_float_ {
    vec_: (),
}
impl MyContainer_float_ {
    pub unsafe fn empty(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn size(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn back_4(&mut self) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn pop_back(&mut self) {
        unimplemented!();
        return;
    }
    pub unsafe fn push_back(&mut self, item: *const ()) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Boxed_short_ {
    pub value: (),
}
impl Boxed_short_ {
    pub unsafe fn twice(mut v: ()) -> () {
        return ((v) + (v));
    }
    pub unsafe fn plus(&self, mut other: ()) -> () {
        return ((self.value) + (other));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Boxed_int_ {
    pub value: (),
}
impl Boxed_int_ {
    pub unsafe fn twice(mut v: ()) -> () {
        return ((v) + (v));
    }
    pub unsafe fn plus(&self, mut other: ()) -> () {
        return ((self.value) + (other));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Boxed_long_ {
    pub value: (),
}
impl Boxed_long_ {
    pub unsafe fn twice(mut v: ()) -> () {
        return ((v) + (v));
    }
    pub unsafe fn plus(&self, mut other: ()) -> () {
        return ((self.value) + (other));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_int__Inner_int_ {
    pub t: (),
    pub u: (),
}
impl Outer_int__Inner_int_ {
    pub unsafe fn sum(&self) -> () {
        return ((self.t as ()) + (self.u as ()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_int_ {
    pub v: (),
}
impl Outer_int_ {
    pub unsafe fn with(&self, mut n: ()) -> Outer_int__Inner_int_ {
        return Outer_int__Inner_int_ { t: self.v, u: n };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_long__Inner_int_ {
    pub t: (),
    pub u: (),
}
impl Outer_long__Inner_int_ {
    pub unsafe fn sum(&self) -> () {
        return ((self.t) + (self.u as ()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_long__Inner_char_ {
    pub t: (),
    pub u: (),
}
impl Outer_long__Inner_char_ {
    pub unsafe fn sum(&self) -> () {
        return ((self.t) + (self.u));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_long_ {
    pub v: (),
}
impl Outer_long_ {
    pub unsafe fn with(&self, mut n: ()) -> Outer_long__Inner_int_ {
        return Outer_long__Inner_int_ { t: self.v, u: n };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Elem {
    pub i: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut ev: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut oi: Outer_int_ = Outer_int_ { v: 3_() };
    unimplemented!();
    let mut ol: Outer_long_ = Outer_long_ { v: 5_() };
    let mut ic: Outer_long__Inner_char_ = Outer_long__Inner_char_ {
        t: 6_(),
        u: ('a' as ()),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut bi: Boxed_int_ = Boxed_int_ { value: 4_() };
    unimplemented!();
    unimplemented!();
    let mut bl: Boxed_long_ = Boxed_long_ { value: 7_() };
    unimplemented!();
    let mut imc: MyContainer_int_ = <MyContainer_int_>::default();
    unimplemented!();
    (unsafe {
        let mut _item: () = 1_();
        MyContainer_int_::push_back(&mut imc, &mut _item)
    });
    unimplemented!();
    (unsafe { MyContainer_int_::pop_back(&mut imc) });
    unimplemented!();
    let mut cmc: MyContainer_char_ = <MyContainer_char_>::default();
    unimplemented!();
    (unsafe {
        let mut _item: () = ('a' as ());
        MyContainer_char_::push_back(&mut cmc, &mut _item)
    });
    unimplemented!();
    (unsafe { MyContainer_char_::pop_back(&mut cmc) });
    unimplemented!();
    let mut fmc: MyContainer_float_ = <MyContainer_float_>::default();
    unimplemented!();
    (unsafe {
        let mut _item: () = 1_f64;
        MyContainer_float_::push_back(&mut fmc, &mut _item)
    });
    unimplemented!();
    (unsafe { MyContainer_float_::pop_back(&mut fmc) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
