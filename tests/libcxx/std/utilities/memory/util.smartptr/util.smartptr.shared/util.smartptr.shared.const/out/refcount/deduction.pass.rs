extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_shared_ptr_A___std_shared_ptr_A__ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_shared_ptr_const_A___std_shared_ptr_const_A__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct D {}
impl Callable1<Ptr<A>, ()> for D {
    fn call(&self, a1: Ptr<A>) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| DImpl::operator_call(__it, a1))
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let s0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let w: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!((unimplemented!() == 2_i64));
        assert!((unimplemented!() == 2_i64));
        assert!((unimplemented!() == unimplemented!()));
    }
    {
        let s0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let w: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!((unimplemented!() == 2_i64));
        assert!((unimplemented!() == 2_i64));
        assert!((unimplemented!() == unimplemented!()));
    }
    {
        let mut u: Option<Value<A>> = Ptr::alloc(<A>::default()).to_owned_opt();
        let mut uPointee: Ptr<A> = u.as_pointer();
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(u.is_none());
        assert!(({ unimplemented!() } == { (uPointee).clone() }));
    }
    {
        let mut u: Option<Value<A>> = Ptr::alloc(<A>::default()).to_owned_opt();
        let mut uPointee: Ptr<A> = u.as_pointer();
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(u.is_none());
        assert!(({ unimplemented!() } == { (uPointee).clone() }));
    }
    {
        let mut u: Option<Value<A>> = unimplemented!();
        let mut uPointee: Ptr<A> = u.as_pointer();
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(u.is_none());
        assert!(({ unimplemented!() } == { (uPointee).clone() }));
    }
    {
        let mut u: Option<Value<A>> = unimplemented!();
        let mut uPointee: Ptr<A> = u.as_pointer();
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(u.is_none());
        assert!(({ unimplemented!() } == { (uPointee).clone() }));
    }
    return 0;
}
pub trait DImpl {
    fn operator_call(&self, ptr: Ptr<A>);
}
impl DImpl for Ptr<D> {
    fn operator_call(&self, mut ptr: Ptr<A>) {
        ptr.delete();
    }
}
pub fn __cpp2rust_init_globals() {}
