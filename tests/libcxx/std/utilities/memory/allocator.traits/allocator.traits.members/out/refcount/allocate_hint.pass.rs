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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A_int_ {
    #[offset(0)]
    pub storage: (),
}
impl A_int_ {
    pub fn new() -> Self {
        Self {
            storage: unimplemented!(),
        }
    }
}
impl Default for A_int_ {
    fn default() -> Self {
        { A_int_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct A_Holder_NS_Incomplete__ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub storage: Ptr<Holder_NS_Incomplete_>,
}
impl A_Holder_NS_Incomplete__ptr_ {
    pub fn new() -> Self {
        Self {
            storage: Ptr::<Holder_NS_Incomplete_>::null(),
        }
    }
}
impl Default for A_Holder_NS_Incomplete__ptr_ {
    fn default() -> Self {
        { A_Holder_NS_Incomplete__ptr_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct B_int_ {
    #[offset(0)]
    pub storage: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct B_Holder_NS_Incomplete__ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub storage: Ptr<Holder_NS_Incomplete_>,
}
pub fn test_4() -> () {
    {
        let a: Value<A_int_> = Rc::new(RefCell::new(A_int_::new()));
        unimplemented!();
    }
    {
        let a: Value<A_Holder_NS_Incomplete__ptr_> =
            Rc::new(RefCell::new(A_Holder_NS_Incomplete__ptr_::new()));
        unimplemented!();
    }
    {
        let b: Value<B_int_> = Rc::new(RefCell::new(<B_int_>::default()));
        unimplemented!();
    }
    {
        let b: Value<B_Holder_NS_Incomplete__ptr_> =
            Rc::new(RefCell::new(<B_Holder_NS_Incomplete__ptr_>::default()));
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
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Holder_NS_Incomplete_;
pub trait A_Holder_NS_Incomplete__ptr_Impl {
    fn allocate(&self, n: ()) -> Ptr<Ptr<Holder_NS_Incomplete_>>;
}
impl A_Holder_NS_Incomplete__ptr_Impl for Ptr<A_Holder_NS_Incomplete__ptr_> {
    fn allocate(&self, mut n: ()) -> Ptr<Ptr<Holder_NS_Incomplete_>> {
        unimplemented!();
        return (field_ptr!((*self), storage));
    }
}
pub trait A_int_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
}
impl A_int_Impl for Ptr<A_int_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!();
        return (field_ptr!((*self), storage));
    }
}
pub trait B_Holder_NS_Incomplete__ptr_Impl {
    fn allocate(&self, n: (), p: ()) -> Ptr<Ptr<Holder_NS_Incomplete_>>;
}
impl B_Holder_NS_Incomplete__ptr_Impl for Ptr<B_Holder_NS_Incomplete__ptr_> {
    fn allocate(&self, mut n: (), mut p: ()) -> Ptr<Ptr<Holder_NS_Incomplete_>> {
        unimplemented!();
        unimplemented!();
        return (field_ptr!((*self), storage));
    }
}
pub trait B_int_Impl {
    fn allocate(&self, n: (), p: ()) -> Ptr<()>;
}
impl B_int_Impl for Ptr<B_int_> {
    fn allocate(&self, mut n: (), mut p: ()) -> Ptr<()> {
        unimplemented!();
        unimplemented!();
        return (field_ptr!((*self), storage));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
