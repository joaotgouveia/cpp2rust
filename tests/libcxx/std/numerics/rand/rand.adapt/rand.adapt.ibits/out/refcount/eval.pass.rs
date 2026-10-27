extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct rand1_unsigned_int__0__10_ {
    #[offset(0)]
    x_: (),
}
impl rand1_unsigned_int__0__10_ {
    pub fn min() -> () {
        return 0_();
    }
    pub fn max() -> () {
        return 10_();
    }
    pub fn new(sd: Option<()>) -> Self {
        let mut sd: () = sd.unwrap_or_else(|| 0_());
        let __this: Value<rand1_unsigned_int__0__10_> = Rc::new(RefCell::new(Self { x_: sd }));
        let this: Ptr<rand1_unsigned_int__0__10_> = __this.as_pointer();
        if (this.with(|__s| __s.x_) > 10_()) {
            field!(this, x_).write(10_());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for rand1_unsigned_int__0__10_ {
    fn default() -> Self {
        { rand1_unsigned_int__0__10_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct rand1_unsigned_int__0__100_ {
    #[offset(0)]
    x_: (),
}
impl rand1_unsigned_int__0__100_ {
    pub fn min() -> () {
        return 0_();
    }
    pub fn max() -> () {
        return 100_();
    }
    pub fn new(sd: Option<()>) -> Self {
        let mut sd: () = sd.unwrap_or_else(|| 0_());
        let __this: Value<rand1_unsigned_int__0__100_> = Rc::new(RefCell::new(Self { x_: sd }));
        let this: Ptr<rand1_unsigned_int__0__100_> = __this.as_pointer();
        if (this.with(|__s| __s.x_) > 100_()) {
            field!(this, x_).write(100_());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for rand1_unsigned_int__0__100_ {
    fn default() -> Self {
        { rand1_unsigned_int__0__100_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct rand1_unsigned_int__0__4294967295_ {
    #[offset(0)]
    x_: (),
}
impl rand1_unsigned_int__0__4294967295_ {
    pub fn min() -> () {
        return 0_();
    }
    pub fn max() -> () {
        return 4294967295_();
    }
    pub fn new(mut sd: ()) -> Self {
        let __this: Value<rand1_unsigned_int__0__4294967295_> =
            Rc::new(RefCell::new(Self { x_: sd }));
        let this: Ptr<rand1_unsigned_int__0__4294967295_> = __this.as_pointer();
        if (this.with(|__s| __s.x_) > 4294967295_()) {
            field!(this, x_).write(4294967295_());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct rand1_unsigned_int__2__3_ {
    #[offset(0)]
    x_: (),
}
impl rand1_unsigned_int__2__3_ {
    pub fn min() -> () {
        return 2_();
    }
    pub fn max() -> () {
        return 3_();
    }
    pub fn new(mut sd: ()) -> Self {
        let __this: Value<rand1_unsigned_int__2__3_> = Rc::new(RefCell::new(Self { x_: sd }));
        let this: Ptr<rand1_unsigned_int__2__3_> = __this.as_pointer();
        if (this.with(|__s| __s.x_) > 3_()) {
            field!(this, x_).write(3_());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn test1_0() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test2_1() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test3_2() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test4_3() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test5_4() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test6_5() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test7_6() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test8_7() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test1_0() });
    ({ test2_1() });
    ({ test3_2() });
    ({ test4_3() });
    ({ test5_4() });
    ({ test6_5() });
    ({ test7_6() });
    ({ test8_7() });
    return 0_();
}
pub trait rand1_unsigned_int__0__100_Impl {
    fn operator_call(&self) -> ();
}
impl rand1_unsigned_int__0__100_Impl for Ptr<rand1_unsigned_int__0__100_> {
    fn operator_call(&self) -> () {
        let r: Value<()> = Rc::new(RefCell::new((*self).with(|__s| __s.x_)));
        if ((*self).with(|__s| __s.x_) < 100_()) {
            field!((*self), x_).with_mut(|__v| __v.prefix_inc());
        } else {
            field!((*self), x_).write(0_());
        }
        return (*r.borrow());
    }
}
pub trait rand1_unsigned_int__0__10_Impl {
    fn operator_call(&self) -> ();
}
impl rand1_unsigned_int__0__10_Impl for Ptr<rand1_unsigned_int__0__10_> {
    fn operator_call(&self) -> () {
        let r: Value<()> = Rc::new(RefCell::new((*self).with(|__s| __s.x_)));
        if ((*self).with(|__s| __s.x_) < 10_()) {
            field!((*self), x_).with_mut(|__v| __v.prefix_inc());
        } else {
            field!((*self), x_).write(0_());
        }
        return (*r.borrow());
    }
}
pub trait rand1_unsigned_int__0__4294967295_Impl {
    fn operator_call(&self) -> ();
}
impl rand1_unsigned_int__0__4294967295_Impl for Ptr<rand1_unsigned_int__0__4294967295_> {
    fn operator_call(&self) -> () {
        let r: Value<()> = Rc::new(RefCell::new((*self).with(|__s| __s.x_)));
        if ((*self).with(|__s| __s.x_) < 4294967295_()) {
            field!((*self), x_).with_mut(|__v| __v.prefix_inc());
        } else {
            field!((*self), x_).write(0_());
        }
        return (*r.borrow());
    }
}
pub trait rand1_unsigned_int__2__3_Impl {
    fn operator_call(&self) -> ();
}
impl rand1_unsigned_int__2__3_Impl for Ptr<rand1_unsigned_int__2__3_> {
    fn operator_call(&self) -> () {
        let r: Value<()> = Rc::new(RefCell::new((*self).with(|__s| __s.x_)));
        if ((*self).with(|__s| __s.x_) < 3_()) {
            field!((*self), x_).with_mut(|__v| __v.prefix_inc());
        } else {
            field!((*self), x_).write(2_());
        }
        return (*r.borrow());
    }
}
pub fn __cpp2rust_init_globals() {}
