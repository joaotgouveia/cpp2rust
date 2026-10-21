extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub n: (),
    #[offset(4)]
    #[byte_size(1)]
    pub name: Box<[()]>,
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                n: unimplemented!(),
                name: (0..1).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct E {
    #[offset(0)]
    pub id: (),
    #[offset(4)]
    pub w: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct T {
    #[offset(0)]
    pub n: (),
    #[offset(4)]
    pub cap: (),
    #[offset(8)]
    #[byte_size(8)]
    pub a: Box<[E]>,
}
impl Default for T {
    fn default() -> Self {
        {
            T {
                n: unimplemented!(),
                cap: unimplemented!(),
                a: (0..1).map(|_| <E>::default()).collect::<Box<[E]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut s: Ptr<S> = unimplemented!().reinterpret_cast::<S>();
    unimplemented!();
    ({
        memcpy_0(
            ((array_field_ptr!(s, name) as Ptr<()>) as Ptr<()>).to_any(),
            Ptr::<()>::from_string_literal(b"abcdefg").to_any(),
            8_(),
        )
    });
    field!(s, n).write(5_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut t: Ptr<T> = unimplemented!().reinterpret_cast::<T>();
    unimplemented!();
    field!(t, n).write(2_());
    field!(t, cap).write(2_());
    field!(array_elem!(t, a, unimplemented!()), id).write(10_());
    field!(array_elem!(t, a, 1_()), w).write(20_());
    field!(t, n).write(3_());
    unimplemented!();
    let mut tail: Ptr<E> = (t.offset((1_()) as isize)).reinterpret_cast::<E>();
    unimplemented!();
    field!(elem!(tail, unimplemented!()), id).write(30_());
    field!(t, cap).write(4_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
