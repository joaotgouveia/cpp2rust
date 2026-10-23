extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static pointer_count_0: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct Pointer_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr_: Ptr<()>,
}
impl Pointer_int_ {
    pub fn new_1() -> Self {
        let __this: Pointer_int_ = Self {
            ptr_: Ptr::<()>::null(),
        };
        (*pointer_count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_2(mut ptr: Ptr<()>) -> Self {
        let __this: Pointer_int_ = Self {
            ptr_: (ptr).clone(),
        };
        (*pointer_count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(other: Ptr<Pointer_int_>) -> Self {
        let __this: Pointer_int_ = Self {
            ptr_: other.with(|__s| __s.ptr_.clone()),
        };
        (*pointer_count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Pointer_int_ {
    fn clone(&self) -> Self {
        let __src: Value<Pointer_int_> = Rc::new(RefCell::new(Pointer_int_ {
            ptr_: self.ptr_.clone(),
        }));
        Pointer_int_::copy_from(__src.as_pointer())
    }
}
impl Default for Pointer_int_ {
    fn default() -> Self {
        { Pointer_int_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Deleter_int_ {}
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
        let ptr: Value<()> = Rc::new(RefCell::new(std_unique_ptr_int__Deleter_int__::new({
            Pointer_int_::new_2({ Ptr::alloc(unimplemented!()) })
        })));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub trait Deleter_int_Impl {
    fn operator_call(&self, ptr: Pointer_int_);
}
impl Deleter_int_Impl for Ptr<Deleter_int_> {
    fn operator_call(&self, ptr: Pointer_int_) {
        let ptr: Value<Pointer_int_> = Rc::new(RefCell::new(ptr));
        { (*ptr.borrow()).ptr_.clone() }.delete();
    }
}
pub trait Pointer_int_Impl {
    fn copy_assign(&self, other: Ptr<Pointer_int_>) -> Ptr<Pointer_int_>;
    fn destructor(&self);
    fn operator_assign_7(&self, mut _a0: ()) -> Ptr<Pointer_int_> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn to_pmut(&self) -> Ptr<()>;
}
impl Pointer_int_Impl for Ptr<Pointer_int_> {
    fn copy_assign(&self, other: Ptr<Pointer_int_>) -> Ptr<Pointer_int_> {
        field!((*self), ptr_).write({ other.with(|__s| __s.ptr_.clone()) });
        return (*self).clone();
    }
    fn destructor(&self) {
        (*pointer_count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn to_pmut(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = pointer_count_0.with(|_| ());
}
