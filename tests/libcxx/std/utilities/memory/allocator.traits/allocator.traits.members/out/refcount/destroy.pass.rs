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
pub struct NoDestroy_CountDestructor_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoDestroy_Holder_NS_Incomplete__ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CountDestroy_CountDestructor_ {
    #[offset(0)]
    #[byte_size(8)]
    pub counter_: Ptr<()>,
}
impl CountDestroy_CountDestructor_ {
    pub fn new(mut counter: Ptr<()>) -> Self {
        Self {
            counter_: (counter).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct CountDestructor {
    #[offset(0)]
    #[byte_size(8)]
    pub counter_: Ptr<()>,
}
impl CountDestructor {
    pub fn new(mut counter: Ptr<()>) -> Self {
        Self {
            counter_: (counter).clone(),
        }
    }
}
pub fn test_4() -> () {
    {
        let destructors: Value<()> = Rc::new(RefCell::new(0_()));
        let mut alloc: NoDestroy_CountDestructor_ = <NoDestroy_CountDestructor_>::default();
        let mut pool: Ptr<CountDestructor> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut alloc: NoDestroy_Holder_NS_Incomplete__ptr_ =
            <NoDestroy_Holder_NS_Incomplete__ptr_>::default();
        let mut pool: Ptr<Ptr<Holder_NS_Incomplete_>> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let destroys_called: Value<()> = Rc::new(RefCell::new(0_()));
        let destructors_called: Value<()> = Rc::new(RefCell::new(0_()));
        let mut alloc: CountDestroy_CountDestructor_ =
            CountDestroy_CountDestructor_::new({ (destroys_called.as_pointer()) });
        let mut pool: Ptr<CountDestructor> = unimplemented!();
        unimplemented!();
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
pub trait CountDestroy_CountDestructor_Impl {
    fn allocate(&self, n: ()) -> Ptr<CountDestructor>;
    fn deallocate(&self, p: Ptr<CountDestructor>, n: ());
    fn destroy_4_struct_CountDestructor(&self, p: Ptr<CountDestructor>);
}
impl CountDestroy_CountDestructor_Impl for Ptr<CountDestroy_CountDestructor_> {
    fn allocate(&self, mut n: ()) -> Ptr<CountDestructor> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<CountDestructor>, mut n: ()) {
        unimplemented!();
        return;
    }
    fn destroy_4_struct_CountDestructor(&self, mut p: Ptr<CountDestructor>) {
        (*self)
            .with(|__s| __s.counter_.clone())
            .with_mut(|__v| __v.prefix_inc());
        ({ CountDestructorImpl::destructor(&p) });
    }
}
pub trait CountDestructorImpl {
    fn destructor(&self);
}
impl CountDestructorImpl for Ptr<CountDestructor> {
    fn destructor(&self) {
        (*self)
            .with(|__s| __s.counter_.clone())
            .with_mut(|__v| __v.prefix_inc());
    }
}
pub trait NoDestroy_CountDestructor_Impl {
    fn allocate(&self, n: ()) -> Ptr<CountDestructor>;
    fn deallocate(&self, p: Ptr<CountDestructor>, n: ());
}
impl NoDestroy_CountDestructor_Impl for Ptr<NoDestroy_CountDestructor_> {
    fn allocate(&self, mut n: ()) -> Ptr<CountDestructor> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<CountDestructor>, mut n: ()) {
        unimplemented!();
        return;
    }
}
pub trait NoDestroy_Holder_NS_Incomplete__ptr_Impl {
    fn allocate(&self, n: ()) -> Ptr<Ptr<Holder_NS_Incomplete_>>;
    fn deallocate(&self, p: Ptr<Ptr<Holder_NS_Incomplete_>>, n: ());
}
impl NoDestroy_Holder_NS_Incomplete__ptr_Impl for Ptr<NoDestroy_Holder_NS_Incomplete__ptr_> {
    fn allocate(&self, mut n: ()) -> Ptr<Ptr<Holder_NS_Incomplete_>> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<Holder_NS_Incomplete_>>, mut n: ()) {
        unimplemented!();
        return;
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
