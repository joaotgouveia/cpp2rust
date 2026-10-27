extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Rep {
    #[offset(0)]
    data_: (),
}
impl Rep {
    pub fn new_1() -> Self {
        Self { data_: -1_() }
    }
    pub fn new_2(mut i: ()) -> Self {
        Self { data_: i }
    }
}
impl std::cmp::PartialEq for Rep {
    fn eq(&self, other: &Self) -> bool {
        {
            RepImpl::operator_eq_4(
                &Rc::new(RefCell::new(Rep {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Rep {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Rep {}
impl Default for Rep {
    fn default() -> Self {
        { Rep::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotARep {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RepConstConvertibleLWG3050 {}
pub fn test_0() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let mut zero_rep: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut zero_rep: () = unimplemented!();
        const _: () = assert!(true, "D::zero().count() == zero_rep");;
    }
}
pub fn test_1() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let zero_rep: Value<Rep> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let zero_rep: Value<Rep> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "D::zero().count() == zero_rep");;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_0() });
    ({ test_1() });
    return 0_();
}
pub trait RepImpl {
    fn operator_eq_3(&self, i: ()) -> ();
    fn operator_eq_4(&self, r: Ptr<Rep>) -> ();
    fn operator_mul_assign(&self, x: Rep) -> Ptr<Rep>;
    fn operator_div_assign(&self, x: Rep) -> Ptr<Rep>;
}
impl RepImpl for Ptr<Rep> {
    fn operator_eq_3(&self, mut i: ()) -> () {
        return ((*self).with(|__s| __s.data_) == i);
    }
    fn operator_eq_4(&self, r: Ptr<Rep>) -> () {
        return ({ (*self).with(|__s| __s.data_) } == { r.with(|__s| __s.data_) });
    }
    fn operator_mul_assign(&self, mut x: Rep) -> Ptr<Rep> {
        {
            let __rhs = { x.data_ };
            field!((*self), data_).with_mut(|__v| *__v = *__v * __rhs)
        };
        return (*self).clone();
    }
    fn operator_div_assign(&self, mut x: Rep) -> Ptr<Rep> {
        {
            let __rhs = { x.data_ };
            field!((*self), data_).with_mut(|__v| *__v = *__v / __rhs)
        };
        return (*self).clone();
    }
}
pub trait RepConstConvertibleLWG3050Impl {
    fn to__2(&self) -> ();
}
impl RepConstConvertibleLWG3050Impl for Ptr<RepConstConvertibleLWG3050> {
    fn to__2(&self) -> () {
        return 2_();
    }
}
pub fn __cpp2rust_init_globals() {}
