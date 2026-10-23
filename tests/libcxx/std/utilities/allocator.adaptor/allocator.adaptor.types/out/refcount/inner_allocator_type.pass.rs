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
pub struct A1_int_ {
    #[offset(0)]
    id_: (),
}
impl A1_int_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A2_int_ {
    #[offset(0)]
    id_: (),
}
impl A2_int_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A3_int_ {
    #[offset(0)]
    id_: (),
}
impl A3_int_ {}
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
    const _: () = assert!(
        true,
        "(std::is_same< std::scoped_allocator_adaptor<A1<int>>::inner_allocator_type,\n                               std::scoped_allocator_adaptor<A1<int>>>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< std::scoped_allocator_adaptor<A1<int>, A2<int>>::inner_allocator_type,\n                               std::scoped_allocator_adaptor<A2<int>>>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< std::scoped_allocator_adaptor<A1<int>, A2<int>, A3<int>>::inner_allocator_type,\n                               std::scoped_allocator_adaptor<A2<int>, A3<int>>>::value)"
    );;
    return 0_();
}
pub trait A1_int_Impl {
    fn id(&self) -> () {
        unimplemented!()
    }
    fn copy_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
}
impl A1_int_Impl for Ptr<A1_int_> {}
pub trait A2_int_Impl {
    fn id(&self) -> () {
        unimplemented!()
    }
    fn copy_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut _a0: (), mut hint: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl A2_int_Impl for Ptr<A2_int_> {}
pub trait A3_int_Impl {
    fn id(&self) -> () {
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
pub fn __cpp2rust_init_globals() {}
