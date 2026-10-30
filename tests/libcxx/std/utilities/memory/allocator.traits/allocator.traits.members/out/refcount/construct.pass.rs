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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_A0_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_A1_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_A2_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_Holder_NS_Incomplete__ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct B_A0_ {
    #[offset(0)]
    #[byte_size(8)]
    pub count_: Ptr<()>,
}
impl B_A0_ {
    pub fn new(count: Ptr<()>) -> Self {
        Self {
            count_: (count).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct B_A1_ {
    #[offset(0)]
    #[byte_size(8)]
    pub count_: Ptr<()>,
}
impl B_A1_ {
    pub fn new(count: Ptr<()>) -> Self {
        Self {
            count_: (count).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct B_A2_ {
    #[offset(0)]
    #[byte_size(8)]
    pub count_: Ptr<()>,
}
impl B_A2_ {
    pub fn new(count: Ptr<()>) -> Self {
        Self {
            count_: (count).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A0 {}
impl A0 {
    pub fn new(mut count: Ptr<()>) -> Self {
        let __this: A0 = Self {};
        count.with_mut(|__v| __v.prefix_inc());
        __this
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A1 {}
impl A1 {
    pub fn new(mut count: Ptr<()>, mut c: ()) -> Self {
        let __this: A1 = Self {};
        unimplemented!();
        count.with_mut(|__v| __v.prefix_inc());
        __this
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A2 {}
impl A2 {
    pub fn new(mut count: Ptr<()>, mut c: (), mut i: ()) -> Self {
        let __this: A2 = Self {};
        unimplemented!();
        unimplemented!();
        count.with_mut(|__v| __v.prefix_inc());
        __this
    }
}
pub fn test_4() -> () {
    {
        let A0_count: Value<()> = Rc::new(RefCell::new(0_()));
        let mut a: A_A0_ = <A_A0_>::default();
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut a0: Ptr<A0> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let A1_count: Value<()> = Rc::new(RefCell::new(0_()));
        let mut a: A_A1_ = <A_A1_>::default();
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut a1: Ptr<A1> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let A2_count: Value<()> = Rc::new(RefCell::new(0_()));
        let mut a: A_A2_ = <A_A2_>::default();
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut a2: Ptr<A2> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: A_Holder_NS_Incomplete__ptr_ = <A_Holder_NS_Incomplete__ptr_>::default();
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut vt: Ptr<Ptr<Holder_NS_Incomplete_>> = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let A0_count: Value<()> = Rc::new(RefCell::new(0_()));
        let b_construct: Value<()> = Rc::new(RefCell::new(0_()));
        let mut b: B_A0_ = B_A0_::new({ b_construct.as_pointer() });
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut a0: Ptr<A0> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let A1_count: Value<()> = Rc::new(RefCell::new(0_()));
        let b_construct: Value<()> = Rc::new(RefCell::new(0_()));
        let mut b: B_A1_ = B_A1_::new({ b_construct.as_pointer() });
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut a1: Ptr<A1> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let A2_count: Value<()> = Rc::new(RefCell::new(0_()));
        let b_construct: Value<()> = Rc::new(RefCell::new(0_()));
        let mut b: B_A2_ = B_A2_::new({ b_construct.as_pointer() });
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut a2: Ptr<A2> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
pub trait B_A0_Impl {
    fn construct_2_struct_A0_int_ptr(&self, p: Ptr<A0>, args: Ptr<Ptr<()>>);
}
impl B_A0_Impl for Ptr<B_A0_> {
    fn construct_2_struct_A0_int_ptr(&self, mut p: Ptr<A0>, args: Ptr<Ptr<()>>) {
        (*self)
            .with(|__s| __s.count_.clone())
            .with_mut(|__v| __v.prefix_inc());
        unimplemented!();
    }
}
pub trait B_A1_Impl {
    fn construct_2_struct_A1_int_ptr_char(&self, p: Ptr<A1>, args_1: Ptr<Ptr<()>>, args_2: Ptr<()>);
}
impl B_A1_Impl for Ptr<B_A1_> {
    fn construct_2_struct_A1_int_ptr_char(
        &self,
        mut p: Ptr<A1>,
        args_1: Ptr<Ptr<()>>,
        args_2: Ptr<()>,
    ) {
        (*self)
            .with(|__s| __s.count_.clone())
            .with_mut(|__v| __v.prefix_inc());
        unimplemented!();
    }
}
pub trait B_A2_Impl {
    fn construct_2_struct_A2_int_ptr_char_int(
        &self,
        p: Ptr<A2>,
        args_1: Ptr<Ptr<()>>,
        args_2: Ptr<()>,
        args_3: Ptr<()>,
    );
}
impl B_A2_Impl for Ptr<B_A2_> {
    fn construct_2_struct_A2_int_ptr_char_int(
        &self,
        mut p: Ptr<A2>,
        args_1: Ptr<Ptr<()>>,
        args_2: Ptr<()>,
        args_3: Ptr<()>,
    ) {
        (*self)
            .with(|__s| __s.count_.clone())
            .with_mut(|__v| __v.prefix_inc());
        unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
