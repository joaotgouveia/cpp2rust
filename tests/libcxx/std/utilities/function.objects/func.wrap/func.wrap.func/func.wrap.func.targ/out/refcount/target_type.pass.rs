extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(40)]
pub struct A {
    #[offset(0)]
    #[byte_size(40)]
    data_: Box<[()]>,
}
impl A {
    pub fn new() -> Self {
        let __this: Value<A> = Rc::new(RefCell::new(Self {
            data_: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<A> = __this.as_pointer();
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        let mut i: () = 0_();
        'loop_: while (i < 10_()) {
            array_elem!(this, data_, i).write({ i });
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(_a0: Ptr<A>) -> Self {
        let __this: A = Self {
            data_: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        };
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
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
impl Default for A {
    fn default() -> Self {
        { A::new() }
    }
}
thread_local!(
    pub static count_0: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn g_1(mut _a0: ()) -> () {
    return 0_();
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
        let f: Value<()> = Rc::new(RefCell::new(std_function_int__int__::new_1({ A::new() })));
        unimplemented!();
    }
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return 0_();
}
pub trait AImpl {
    fn destructor(&self);
    fn operator_call(&self, i: ()) -> ();
    fn foo(&self, _a0: ()) -> ();
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn destructor(&self) {
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_call(&self, i: ()) -> () {
        let i: Value<()> = Rc::new(RefCell::new(i));
        let mut j: () = 0_();
        'loop_: while (j < 10_()) {
            (*i.borrow_mut()) += (array_elem!((*self), data_, j).read());
            j.prefix_inc();
        }
        return (*i.borrow());
    }
    fn foo(&self, mut _a0: ()) -> () {
        return 1_();
    }
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
