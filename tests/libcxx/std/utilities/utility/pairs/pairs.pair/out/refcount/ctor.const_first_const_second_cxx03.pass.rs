extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    data_: i32,
}
impl A {
    pub fn new(mut data: i32) -> Self {
        Self { data_: data }
    }
}
impl CppEq for A {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        AImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
        let p: Value<(Value<f32>, Value<Ptr<i16>>)> = Rc::new(RefCell::new({
            let __tmp_0: Value<f32> = Rc::new(RefCell::new(3.5_f32));
            let __tmp_1: Value<Ptr<i16>> = Rc::new(RefCell::new(Ptr::<i16>::null()));
            std_pair_float__short_ptr_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        assert!(({ ((*(*p.borrow()).0.borrow()) == 3.5_f32) }));
        assert!(({ (*(*p.borrow()).1.borrow()).is_null() }));
    }
    {
        let p: Value<(Value<A>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(A::new({ 1 }))),
            Rc::new(RefCell::new(2)),
        )));
        assert!(
            ({
                let _a: Value<A> = Rc::new(RefCell::new(A::new({ 1 })));
                AImpl::operator_eq(&(*p.borrow()).0.as_pointer(), _a.as_pointer())
            })
        );
        assert!(({ ((*(*p.borrow()).1.borrow()) == 2) }));
    }
    return 0;
}
pub trait AImpl {
    fn operator_eq(&self, a: Ptr<A>) -> bool;
}
impl AImpl for Ptr<A> {
    fn operator_eq(&self, a: Ptr<A>) -> bool {
        return ({ (*self).with(|__s| __s.data_) } == { a.with(|__s| __s.data_) });
    }
}
pub fn __cpp2rust_init_globals() {}
