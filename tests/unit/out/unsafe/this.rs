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
pub struct S {
    pub a_: (),
    pub self__: *mut S,
}
impl S {
    pub unsafe fn new_1(mut a: ()) -> Self {
        let mut this = Self {
            a_: a,
            self__: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_2(mut a: (), mut other: *mut S) -> Self {
        let mut this = Self {
            a_: a,
            self__: std::ptr::null_mut(),
        };
        if ((&raw mut this) == (other)) {
            this.self__ = std::ptr::null_mut();
        } else {
            this.self__ = other;
        }
        this
    }
    pub unsafe fn new_3(mut a: (), mut other: *const S) -> Self {
        let mut this = Self {
            a_: a,
            self__: std::ptr::null_mut(),
        };
        if (((&raw mut this).cast_const()) == (other)) {
            this.self__ = std::ptr::null_mut();
        }
        this
    }
    pub unsafe fn returns_this_reference(&mut self) -> *mut S {
        return &mut (*(self as *mut S));
    }
    pub unsafe fn returns_this_pointer(&mut self) -> *mut S {
        return (self as *mut S);
    }
    pub unsafe fn inc(&mut self) -> *mut S {
        self.a_.postfix_inc();
        return &mut (*(self as *mut S));
    }
    pub unsafe fn set_from_this(&mut self) {
        self.a_ = ((self.a_) + (1_()));
    }
    pub unsafe fn get(&mut self) -> () {
        return self.a_;
    }
    pub unsafe fn twice(&mut self) -> () {
        return ((unsafe { S::get(self) }) * (2_()));
    }
    pub unsafe fn link(&mut self) {
        self.self__ = (self as *mut S);
    }
    pub unsafe fn bump_me(&mut self) {
        (unsafe { bump_0((self as *mut S)) });
    }
    pub unsafe fn cref(&self) -> *const S {
        return &(*(self as *const S));
    }
    pub unsafe fn is(&self, mut o: *const S) -> () {
        return ((o) == (self as *const S));
    }
    pub unsafe fn destroy(&mut self) {
        {
            let __p = (self as *mut S);
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
    }
    pub unsafe fn reset(&mut self) {
        (*(self as *mut S)) = S::new_1({ 0_() });
    }
    pub unsafe fn copy_if_different_const(&mut self, mut other: *const S) -> () {
        if (((self as *mut S).cast_const()) == (other)) {
            return false;
        }
        self.a_ = (*other).a_;
        self.self__ = (*other).self__;
        return true;
    }
    pub unsafe fn copy_if_different(&mut self, mut other: *mut S) -> () {
        if ((self as *mut S) == (other)) {
            return false;
        }
        self.a_ = (*other).a_;
        self.self__ = (*other).self__;
        return true;
    }
}
pub unsafe fn bump_0(mut p: *mut S) {
    (*p).a_.postfix_inc();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D {
    pub a_: (),
}
impl D {
    pub unsafe fn new(mut a: ()) -> Self {
        let mut this = Self { a_: a };
        this.a_ *= 2_();
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S::new_1({ 1_() });
    let ref_: *mut S = (unsafe { S::returns_this_reference(&mut s) });
    (*ref_).a_.postfix_inc();
    unimplemented!();
    let mut ptr: *mut S = (unsafe { S::returns_this_pointer(&mut s) });
    (*ptr).a_.postfix_inc();
    unimplemented!();
    (unsafe { S::inc(&mut (*(unsafe { S::inc(&mut (*(unsafe { S::inc(&mut s) }))) }))) });
    unimplemented!();
    (unsafe { S::set_from_this(&mut s) });
    unimplemented!();
    unimplemented!();
    (unsafe { S::link(&mut s) });
    unimplemented!();
    (*s.self__).a_.postfix_inc();
    unimplemented!();
    (unsafe { S::bump_me(&mut s) });
    unimplemented!();
    let mut d: D = D::new({ 3_() });
    unimplemented!();
    let cr: *const S = (unsafe { S::cref(&s) });
    unimplemented!();
    let mut t: S = S::new_1({ 0_() });
    unimplemented!();
    unimplemented!();
    let mut p: *mut S = (Box::leak(Box::new(S::new_1({ 1_() }))) as *mut S);
    let mut q: *mut S = (unsafe { S::returns_this_pointer(&mut (*p)) });
    (*q).a_.postfix_inc();
    unimplemented!();
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut h: *mut S = (Box::leak(Box::new(S::new_1({ 5_() }))) as *mut S);
    (unsafe { S::destroy(&mut (*h)) });
    (unsafe { S::reset(&mut s) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut other: S = S::new_1({ 22_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut u: S = S::new_2({ 1_() }, { (&mut s as *mut S) });
    unimplemented!();
    let s_const: S = S::new_1({ 100_() });
    let mut u1: S = S::new_3({ 1_() }, { (&s_const as *const S) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
