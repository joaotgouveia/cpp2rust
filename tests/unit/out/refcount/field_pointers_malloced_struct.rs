extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct S {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
    #[offset(8)]
    pub c: (),
}
pub fn bump_0(mut s: Ptr<S>) -> () {
    {
        field!(s, b).with_mut(|__v| *__v = *__v + 10_())
    };
    return s.with(|__s| __s.b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut s: Ptr<S> = unimplemented!().reinterpret_cast::<S>();
    unimplemented!();
    field!(s, b).write(1_());
    let __rhs = ({ bump_0((s).clone()) });
    field!(s, a).write(__rhs);
    unimplemented!();
    unimplemented!();
    field!(s, a).write(1_());
    field!(s, b).write(2_());
    field!(s, c).write(unimplemented!());
    if ({ s.with(|__s| __s.a) } < { s.with(|__s| __s.b) })
        && (field!(s, c).with_mut(|__v| __v.postfix_inc()) == unimplemented!())
    {
        field!(s, a).write(5_());
    }
    unimplemented!();
    if ({ s.with(|__s| __s.a) } < { s.with(|__s| __s.b) })
        && (field!(s, c).with_mut(|__v| __v.postfix_inc()) == unimplemented!())
    {
        field!(s, a).write(6_());
    }
    unimplemented!();
    let mut x: () = ({ s.with(|__s| __s.a) } + {
        ({
            field!(s, b).write(3_());
            s.with(|__s| __s.b)
        })
    });
    unimplemented!();
    let mut y: () = unimplemented!();
    let __rhs = ({
        y = 99_();
        y
    });
    field!(s, c).write(__rhs);
    unimplemented!();
    let __rhs = ({ bump_0((s).clone()) });
    {
        field!(s, a).with_mut(|__v| *__v = *__v + __rhs)
    };
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
