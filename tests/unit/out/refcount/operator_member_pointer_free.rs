extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    #[byte_size(12)]
    pub data: Box<[()]>,
    #[offset(12)]
    #[byte_size(4)]
    pub inner: Inner,
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                data: (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                inner: <Inner>::default(),
            }
        }
    }
}
pub fn operator_deref_0(s: Ptr<S>) -> Ptr<Inner> {
    return field_ptr!(s, inner);
}
pub fn operator_addr_1(s: Ptr<S>) -> Ptr<()> {
    return ((array_field_ptr!(s, data) as Ptr<()>).offset((unimplemented!()) as isize));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S {
        data: Box::new([1_(), 2_(), 3_()]),
        inner: Inner { x: 9_() },
    }));
    unimplemented!();
    field!(
        ({
            let _s: Ptr<S> = s.as_pointer();
            operator_deref_0(_s)
        }),
        x
    )
    .write(10_());
    unimplemented!();
    let mut p: Ptr<()> = ({
        let _s: Ptr<S> = s.as_pointer();
        operator_addr_1(_s)
    });
    unimplemented!();
    p.write(5_());
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
