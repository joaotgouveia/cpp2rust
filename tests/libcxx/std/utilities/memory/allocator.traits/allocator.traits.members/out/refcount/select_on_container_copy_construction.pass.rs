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
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A_int_ {
    #[offset(0)]
    pub id: i32,
}
impl A_int_ {
    pub fn new(i: Option<i32>) -> Self {
        let mut i: i32 = i.unwrap_or_else(|| 0);
        Self { id: i }
    }
}
impl Default for A_int_ {
    fn default() -> Self {
        { A_int_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A_Holder_NS_Incomplete__ptr_ {
    #[offset(0)]
    pub id: i32,
}
impl A_Holder_NS_Incomplete__ptr_ {
    pub fn new(i: Option<i32>) -> Self {
        let mut i: i32 = i.unwrap_or_else(|| 0);
        Self { id: i }
    }
}
impl Default for A_Holder_NS_Incomplete__ptr_ {
    fn default() -> Self {
        { A_Holder_NS_Incomplete__ptr_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct B_int_ {
    #[offset(0)]
    pub id: i32,
}
impl B_int_ {
    pub fn new(i: Option<i32>) -> Self {
        let mut i: i32 = i.unwrap_or_else(|| 0);
        Self { id: i }
    }
}
impl Default for B_int_ {
    fn default() -> Self {
        { B_int_::new(None) }
    }
}
pub fn test_4() -> bool {
    {
        let mut a: A_int_ = A_int_::new(None);
        assert!(({ unimplemented!().id } == 0));
    }
    {
        let mut a: A_int_ = A_int_::new({ Some(0) });
        assert!(({ unimplemented!().id } == 0));
    }
    {
        let mut a: A_Holder_NS_Incomplete__ptr_ = A_Holder_NS_Incomplete__ptr_::new(None);
        assert!(({ unimplemented!().id } == 0));
    }
    {
        let mut b: B_int_ = B_int_::new(None);
        assert!(({ unimplemented!().id } == 100));
    }
    {
        let mut b: B_int_ = B_int_::new({ Some(0) });
        assert!(({ unimplemented!().id } == 100));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_4() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait B_int_Impl {
    fn select_on_container_copy_construction(&self) -> B_int_;
}
impl B_int_Impl for Ptr<B_int_> {
    fn select_on_container_copy_construction(&self) -> B_int_ {
        return B_int_::new({ Some(100) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
