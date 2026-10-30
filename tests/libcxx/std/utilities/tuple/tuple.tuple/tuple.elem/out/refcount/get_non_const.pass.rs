extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(12)]
pub struct S {
    #[offset(0)]
    #[byte_size(4)]
    pub a: (),
    #[offset(4)]
    pub k: (),
    #[offset(8)]
    #[byte_size(1)]
    pub e: Empty,
}
impl S {
    pub fn new() -> Self {
        Self {
            a: unimplemented!(),
            k: (unimplemented!().read()),
            e: (*unimplemented!().upgrade().deref()).clone(),
        }
    }
    pub fn move_from(_a0: Ptr<S>) -> Self {
        Self {
            a: unimplemented!(),
            k: { (*_a0.upgrade().deref()).k },
            e: { (*_a0.upgrade().deref()).e.clone() },
        }
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
pub fn getP_0() -> () {
    return unimplemented!();
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
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!().write(2_());
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!().write(4_());
        unimplemented!();
        unimplemented!();
    }
    {
        let mut d: () = 1.5_f64;
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!().write(2.5_f64);
        unimplemented!();
        unimplemented!().write(4_());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(
            true,
            "std::get<0> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 0"
        );;
        const _: () = assert!(
            true,
            "std::get<1> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 1"
        );;
        const _: () = assert!(
            true,
            "std::get<2> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 2"
        );;
        const _: () = assert!(
            true,
            "std::get<3> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 3"
        );;
        const _: () = assert!(true, "S().k == 1");;
        const _: () = assert!(true, "std::get<1>(getP()) == 4");;
    }
    return 0_();
}
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        unimplemented!();
        field!((*self), k).write({ { (*_a0.upgrade().deref()).k } });
        field!((*self), e).write({ { (*_a0.upgrade().deref()).e.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
