extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static A_constructed_0: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    pub data_: (),
}
impl A {
    pub fn new(mut i: ()) -> Self {
        let __this: A = Self { data_: i };
        (*A_constructed_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(a: Ptr<A>) -> Self {
        let __this: A = Self {
            data_: a.with(|__s| __s.data_),
        };
        (*A_constructed_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        let __src: Value<A> = Rc::new(RefCell::new(A {
            data_: self.data_.clone(),
        }));
        A::copy_from(__src.as_pointer())
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
    let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it: Value<()> = Rc::new(RefCell::new(std_raw_storage_iterator_A_ptr__A_::new({
        (buffer.as_pointer()).reinterpret_cast::<A>()
    })));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 3_()) {
        unimplemented!();
        let mut ap: Ptr<A> = (buffer.as_pointer())
            .reinterpret_cast::<A>()
            .offset((i) as isize);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    return 0_();
}
pub trait AImpl {
    fn destructor(&self);
    fn operator_eq(&self, i: ()) -> ();
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn destructor(&self) {
        (*A_constructed_0.with(Value::clone).borrow_mut()).prefix_dec();
        field!((*self), data_).write(0_());
    }
    fn operator_eq(&self, mut i: ()) -> () {
        return ((*self).with(|__s| __s.data_) == i);
    }
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = A_constructed_0.with(|_| ());
}
