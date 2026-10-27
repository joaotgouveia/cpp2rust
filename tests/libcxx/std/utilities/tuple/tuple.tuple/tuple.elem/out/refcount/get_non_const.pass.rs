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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
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
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a: unimplemented!(),
            k: (({ get_0(field_ptr!(this, a)) }).read()),
            e: (*({ get_1(field_ptr!(this, a)) }).upgrade().deref()).clone(),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
pub fn getP_2() -> () {
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
        ({ get_3(t.as_pointer()) }).write(2_());
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ get_4(t.as_pointer()) }).write(4_());
        unimplemented!();
        unimplemented!();
    }
    {
        let mut d: () = 1.5_f64;
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ get_5(t.as_pointer()) }).write(2.5_f64);
        unimplemented!();
        ({ get_6(t.as_pointer()) }).write(4_());
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
pub fn __cpp2rust_init_globals() {}
