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
pub struct Counter_base {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Counter_int_ {
    #[offset(0)]
    data_: (),
}
impl Counter_int_ {
    pub fn new_1(data: Ptr<()>) -> Self {
        let __this: Counter_int_ = Self {
            data_: (data.read()),
        };
        (*gConstructed_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(rhs: Ptr<Counter_int_>) -> Self {
        let __this: Counter_int_ = Self {
            data_: rhs.with(|__s| __s.data_),
        };
        (*gConstructed_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for Counter_int_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        Counter_int_Impl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Counter_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        Counter_int_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Clone for Counter_int_ {
    fn clone(&self) -> Self {
        let __src: Value<Counter_int_> = Rc::new(RefCell::new(Counter_int_ {
            data_: self.data_.clone(),
        }));
        Counter_int_::copy_from(__src.as_pointer())
    }
}
thread_local!(
    pub static gConstructed_0: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn set_equal_1(set: Ptr<()>, other: ()) -> () {
    let other: Value<()> = Rc::new(RefCell::new(other));
    return unimplemented!();
}
pub fn set_equal_2(set: Ptr<()>, other: ()) -> () {
    let other: Value<()> = Rc::new(RefCell::new(other));
    return unimplemented!();
}
pub fn set_equal_3(set: Ptr<()>, other: ()) -> () {
    let other: Value<()> = Rc::new(RefCell::new(other));
    return unimplemented!();
}
pub fn set_equal_4(set: Ptr<()>, other: ()) -> () {
    let other: Value<()> = Rc::new(RefCell::new(other));
    return unimplemented!();
}
pub fn test_5() -> () {
    {
        let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let dst: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        unimplemented!();
    };
    {
        {
            let first: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let second: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let third: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        {
            let first: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let second: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let third: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
    }
    {
        let first: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let second: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
        }
        {
            let second: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
        }
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct comparator_6 {}
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
    ({ test_5() });
    return 0_();
}
pub trait Counter_int_Impl {
    fn copy_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn move_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn destructor(&self);
    fn get(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> ();
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> ();
}
impl Counter_int_Impl for Ptr<Counter_int_> {
    fn destructor(&self) {
        (*gConstructed_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> () {
        return ({ (*self).with(|__s| __s.data_) } == { x.with(|__s| __s.data_) });
    }
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> () {
        return ({ (*self).with(|__s| __s.data_) } < { x.with(|__s| __s.data_) });
    }
}
pub trait comparator_6Impl {
    fn operator_call(&self, lhs: Ptr<Counter_int_>, rhs: Ptr<Counter_int_>) -> ();
}
impl comparator_6Impl for Ptr<comparator_6> {
    fn operator_call(&self, lhs: Ptr<Counter_int_>, rhs: Ptr<Counter_int_>) -> () {
        return ({
            let _x: Ptr<Counter_int_> = (rhs).clone();
            Counter_int_Impl::operator_lt(&lhs, _x)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = gConstructed_0.with(|_| ());
}
