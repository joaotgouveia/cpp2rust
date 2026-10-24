extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(16)]
pub struct unary_counting_predicate_bool__ptr__int___int_ {
    #[offset(0)]
    #[byte_size(8)]
    p_: FnPtr<fn(()) -> ()>,
    #[offset(8)]
    count_: (),
}
impl unary_counting_predicate_bool__ptr__int___int_ {
    pub fn new(mut p: FnPtr<fn(()) -> ()>) -> Self {
        Self {
            p_: (p).clone(),
            count_: 0_(),
        }
    }
}
pub fn is5_0(mut i: ()) -> () {
    return (i == 5_());
}
pub fn call_pred_1(pred: unary_counting_predicate_bool__ptr__int___int_) -> () {
    let pred: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(pred));
    return ({
        let _a: Value<()> = Rc::new(RefCell::new(5_()));
        unary_counting_predicate_bool__ptr__int___int_Impl::operator_call(
            &pred.as_pointer(),
            _a.as_pointer(),
        )
    });
}
pub fn call_pred_2(pred: ()) -> () {
    let pred: Value<()> = Rc::new(RefCell::new(pred));
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct adl_A {}
pub fn ref_3(mut _a0: adl_A) {}
pub fn test_4() -> () {
    {
        let i: Value<()> = Rc::new(RefCell::new(0_()));
        let r1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let a: Value<adl_A> = Rc::new(RefCell::new(<adl_A>::default()));
        let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_4() });
    const _: () = assert!(true, "test()");;
    {
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(is5_0)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait unary_counting_predicate_bool__ptr__int___int_Impl {
    fn destructor(&self);
    fn operator_call(&self, a: Ptr<()>) -> ();
    fn count(&self) -> ();
    fn reset(&self) {
        unimplemented!()
    }
}
impl unary_counting_predicate_bool__ptr__int___int_Impl
    for Ptr<unary_counting_predicate_bool__ptr__int___int_>
{
    fn destructor(&self) {}
    fn operator_call(&self, a: Ptr<()>) -> () {
        field!((*self), count_).with_mut(|__v| __v.prefix_inc());
        return ({ (*self).with(|__s| __s.p_.clone()).call((a.read())) });
    }
    fn count(&self) -> () {
        return (*self).with(|__s| __s.count_);
    }
}
pub fn __cpp2rust_init_globals() {}
