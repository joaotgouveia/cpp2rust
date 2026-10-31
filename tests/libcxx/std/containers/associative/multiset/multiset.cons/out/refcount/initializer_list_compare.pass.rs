extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(a: Ptr<test_less_int_>, b: Ptr<test_less_int_>) -> bool {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct test_less_int_ {
    #[offset(0)]
    pub data_: i32,
}
impl test_less_int_ {
    pub fn new_1(mut data: i32) -> Self {
        Self { data_: data }
    }
}
impl CppEq for test_less_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
pub fn test_1() -> bool {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == 6_()));
    assert!((unimplemented!() == 6_isize));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((((*i.borrow()).iter_deref().read()) == (1 as ())));
    assert!(((((*i.borrow_mut()).iter_inc()).clone().iter_deref().read()) == (2 as ())));
    assert!(((((*i.borrow_mut()).iter_inc()).clone().iter_deref().read()) == (3 as ())));
    assert!(((((*i.borrow_mut()).iter_inc()).clone().iter_deref().read()) == (4 as ())));
    assert!(((((*i.borrow_mut()).iter_inc()).clone().iter_deref().read()) == (5 as ())));
    assert!(((((*i.borrow_mut()).iter_inc()).clone().iter_deref().read()) == (6 as ())));
    assert!(
        ({
            let _a: Value<test_less_int_> = Rc::new(RefCell::new(unimplemented!()));
            let _b: Value<test_less_int_> = Rc::new(RefCell::new(test_less_int_::new_1({ 10 })));
            operator_eq_0(_a.as_pointer(), _b.as_pointer())
        })
    );
    return true;
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
    ({ test_1() });
    return 0;
}
pub trait test_less_int_Impl {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool;
}
impl test_less_int_Impl for Ptr<test_less_int_> {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool {
        return ({ (a.read()) } < { (b.read()) });
    }
}
pub fn __cpp2rust_init_globals() {}
