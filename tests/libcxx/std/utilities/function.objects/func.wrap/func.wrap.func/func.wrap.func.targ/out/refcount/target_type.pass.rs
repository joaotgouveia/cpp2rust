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
    data_: Box<[i32]>,
}
impl A {
    pub fn new() -> Self {
        let __this: Value<A> = Rc::new(RefCell::new(Self {
            data_: (0..10).map(|_| 0_i32).collect::<Box<[i32]>>(),
        }));
        let this: Ptr<A> = __this.as_pointer();
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            array_elem!(this, data_, i).write({ i });
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(_a0: Ptr<A>) -> Self {
        let __this: A = Self {
            data_: (0..10).map(|_| 0_i32).collect::<Box<[i32]>>(),
        };
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Callable1<i32, i32> for A {
    fn call(&self, a1: i32) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| AImpl::operator_call(__it, a1))
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
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn g_1(mut _a0: i32) -> i32 {
    return 0;
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
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    return 0;
}
pub trait AImpl {
    fn destructor(&self);
    fn operator_call(&self, i: i32) -> i32;
    fn foo(&self, _a0: i32) -> i32;
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn destructor(&self) {
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_call(&self, i: i32) -> i32 {
        let i: Value<i32> = Rc::new(RefCell::new(i));
        let mut j: i32 = 0;
        'loop_: while (j < 10) {
            (*i.borrow_mut()) += (array_elem!((*self), data_, j).read());
            j.prefix_inc();
        }
        return (*i.borrow());
    }
    fn foo(&self, mut _a0: i32) -> i32 {
        return 1;
    }
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        {
            ((array_field_ptr!((*self), data_)) as Ptr<i32>)
                .to_any()
                .memcpy(
                    &((array_field_ptr!(_a0, data_)) as Ptr<i32>).to_any(),
                    40_usize as usize,
                );
            ((array_field_ptr!((*self), data_)) as Ptr<i32>).to_any()
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
