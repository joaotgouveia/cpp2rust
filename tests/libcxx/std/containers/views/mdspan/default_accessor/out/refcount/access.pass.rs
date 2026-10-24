extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct MinimalElementType {
    #[offset(0)]
    pub val: (),
}
impl MinimalElementType {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_int__10_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl ElementPool_int__10_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_int__10_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<()>::null(),
        }));
        let this: Ptr<ElementPool_int__10_> = __this.as_pointer();
        field!(this, ptr_).write(unimplemented!());
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_int__10_ {
    fn default() -> Self {
        { ElementPool_int__10_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_MinimalElementType__10_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<MinimalElementType>,
}
impl ElementPool_MinimalElementType__10_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_MinimalElementType__10_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<MinimalElementType>::null(),
        }));
        let this: Ptr<ElementPool_MinimalElementType__10_> = __this.as_pointer();
        field!(this, ptr_).write(unimplemented!());
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_MinimalElementType__10_ {
    fn default() -> Self {
        { ElementPool_MinimalElementType__10_::new() }
    }
}
pub fn test_access_0() {
    let data: Value<ElementPool_int__10_> = Rc::new(RefCell::new(ElementPool_int__10_::new()));
    let _dtor_data = ScopedDestructor::new(&data, |__p| __p.destructor());
    let mut ptr: Ptr<()> = ({ ElementPool_int__10_Impl::get_ptr(&data.as_pointer()) });
    let acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(acc.access(ptr, i)), typename std::default_accessor<T>::reference>"
        );;
        const _: () = assert!(true, "");;
        unimplemented!();
        i.postfix_inc();
    }
}
pub fn test_access_1() {
    let data: Value<ElementPool_int__10_> = Rc::new(RefCell::new(ElementPool_int__10_::new()));
    let _dtor_data = ScopedDestructor::new(&data, |__p| __p.destructor());
    let mut ptr: Ptr<()> = ({ ElementPool_int__10_Impl::get_ptr(&data.as_pointer()) });
    let acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(acc.access(ptr, i)), typename std::default_accessor<T>::reference>"
        );;
        const _: () = assert!(true, "");;
        unimplemented!();
        i.postfix_inc();
    }
}
pub fn test_access_2() {
    let data: Value<ElementPool_MinimalElementType__10_> =
        Rc::new(RefCell::new(ElementPool_MinimalElementType__10_::new()));
    let _dtor_data = ScopedDestructor::new(&data, |__p| __p.destructor());
    let mut ptr: Ptr<MinimalElementType> =
        ({ ElementPool_MinimalElementType__10_Impl::get_ptr(&data.as_pointer()) });
    let acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(acc.access(ptr, i)), typename std::default_accessor<T>::reference>"
        );;
        const _: () = assert!(true, "");;
        unimplemented!();
        i.postfix_inc();
    }
}
pub fn test_access_3() {
    let data: Value<ElementPool_MinimalElementType__10_> =
        Rc::new(RefCell::new(ElementPool_MinimalElementType__10_::new()));
    let _dtor_data = ScopedDestructor::new(&data, |__p| __p.destructor());
    let mut ptr: Ptr<MinimalElementType> =
        ({ ElementPool_MinimalElementType__10_Impl::get_ptr(&data.as_pointer()) });
    let acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(acc.access(ptr, i)), typename std::default_accessor<T>::reference>"
        );;
        const _: () = assert!(true, "");;
        unimplemented!();
        i.postfix_inc();
    }
}
pub fn test_4() -> () {
    ({ test_access_0() });
    ({ test_access_1() });
    ({ test_access_2() });
    ({ test_access_3() });
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
    return 0_();
}
pub trait ElementPool_MinimalElementType__10_Impl {
    fn get_ptr(&self) -> Ptr<MinimalElementType>;
    fn destructor(&self);
}
impl ElementPool_MinimalElementType__10_Impl for Ptr<ElementPool_MinimalElementType__10_> {
    fn get_ptr(&self) -> Ptr<MinimalElementType> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_int__10_Impl {
    fn get_ptr(&self) -> Ptr<()>;
    fn destructor(&self);
}
impl ElementPool_int__10_Impl for Ptr<ElementPool_int__10_> {
    fn get_ptr(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
