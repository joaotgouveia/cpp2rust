extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static deallocate_called_3: Value<(Value<Ptr<i32>>, Value<u64>)> =
        Rc::new(RefCell::new(unimplemented!()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct A1_int_ {
    #[offset(0)]
    id_: i32,
}
impl A1_int_ {
    pub fn new(id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| 0);
        Self { id_: id }
    }
}
impl Default for A1_int_ {
    fn default() -> Self {
        { A1_int_::new(None) }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct A2_int_ {
    #[offset(0)]
    id_: i32,
}
impl A2_int_ {
    pub fn new(id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| 0);
        Self { id_: id }
    }
}
impl Default for A2_int_ {
    fn default() -> Self {
        { A2_int_::new(None) }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct A3_int_ {
    #[offset(0)]
    id_: i32,
}
impl A3_int_ {
    pub fn new(id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| 0);
        Self { id_: id }
    }
}
impl Default for A3_int_ {
    fn default() -> Self {
        { A3_int_::new(None) }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!((unimplemented!()));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!((unimplemented!()));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!((unimplemented!()));
    }
    return 0;
}
pub trait A1_int_Impl {
    fn id(&self) -> i32 {
        unimplemented!()
    }
    fn copy_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<i32>, n: usize);
    fn max_size(&self) -> usize {
        unimplemented!()
    }
}
impl A1_int_Impl for Ptr<A1_int_> {
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        deallocate_called_3.with(|v| v.as_pointer()).write((
            Rc::new(RefCell::new((p).clone())),
            Rc::new(RefCell::new((n as u64))),
        ));
    }
}
pub trait A2_int_Impl {
    fn id(&self) -> i32 {
        unimplemented!()
    }
    fn copy_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut _a0: usize, mut hint: AnyPtr) -> Ptr<i32> {
        unimplemented!()
    }
}
impl A2_int_Impl for Ptr<A2_int_> {}
pub trait A3_int_Impl {
    fn id(&self) -> i32 {
        unimplemented!()
    }
    fn copy_assign(&self, a: Ptr<A3_int_>) -> Ptr<A3_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A3_int_>) -> Ptr<A3_int_> {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> A3_int_ {
        unimplemented!()
    }
}
impl A3_int_Impl for Ptr<A3_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = deallocate_called_3.with(|_| ());
}
