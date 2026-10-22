extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct basic {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl basic {
    pub fn i(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn f(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for basic {
    fn default() -> Self {
        basic {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(1)]
pub struct empty {
    #[offset(0)]
    #[byte_size(1)]
    __bytes: Box<[u8]>,
}
impl empty {}
impl Default for empty {
    fn default() -> Self {
        empty {
            __bytes: Box::from([0u8; 1]),
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct underscores {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl underscores {
    pub fn __(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn ___(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for underscores {
    fn default() -> Self {
        underscores {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let u: Value<basic> = Rc::new(RefCell::new(<basic>::default()));
    let e: Value<empty> = Rc::new(RefCell::new(<empty>::default()));
    &(*e.borrow_mut());
    basic::i(u.as_pointer()).write(42_());
    unimplemented!();
    basic::f(u.as_pointer()).write(3.14_f32);
    unimplemented!();
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    ({
        let ___s: () = ((buf.as_pointer() as Ptr<()>) as Ptr<()>).to_any();
        let ___n: () = ::std::mem::size_of::<[(); 4]>();
        memset_0(___s, 0_(), ___n)
    });
    let mut ru: Ptr<basic> = (buf.as_pointer() as Ptr<()>).reinterpret_cast::<basic>();
    let mut pi: Ptr<()> = (basic::i(ru.clone())).clone();
    let mut pf: Ptr<()> = (basic::f(ru.clone())).clone();
    basic::i(ru.clone()).write(7_());
    unimplemented!();
    pi.write(1065353216_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let braced: Value<basic> = Rc::new(RefCell::new({
        let __u: Value<basic> = Rc::new(RefCell::new(<basic>::default()));
        basic::i(__u.as_pointer()).write(5_());
        Rc::try_unwrap(__u).ok().unwrap().into_inner()
    }));
    unimplemented!();
    let copied: Value<basic> = Rc::new(RefCell::new({
        let __u: Value<basic> = Rc::new(RefCell::new(<basic>::default()));
        basic::i(__u.as_pointer()).write(7_());
        Rc::try_unwrap(__u).ok().unwrap().into_inner()
    }));
    unimplemented!();
    {
        let anon_1: Value<anon_1> = Rc::new(RefCell::new(<anon_1>::default()));
        anon_1::ai(anon_1.as_pointer()).write(11_());
        unimplemented!();
        anon_1::ai(anon_1.as_pointer()).write(1065353216_());
        unimplemented!();
    }
    let us: Value<underscores> = Rc::new(RefCell::new(<underscores>::default()));
    underscores::__(us.as_pointer()).write(5_());
    unimplemented!();
    underscores::___(us.as_pointer()).write(2.5_f64);
    unimplemented!();
    let mut __: () = 1_();
    let mut ___: () = 2_();
    unimplemented!();
    return 0_();
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl anon_1 {
    pub fn ai(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn af(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
pub fn __cpp2rust_init_globals() {}
