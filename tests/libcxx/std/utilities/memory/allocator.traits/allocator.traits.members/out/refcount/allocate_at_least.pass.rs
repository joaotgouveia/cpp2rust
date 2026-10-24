extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct no_allocate_at_least_int_ {
    #[offset(0)]
    pub t: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct has_allocate_at_least_int_ {
    #[offset(0)]
    pub t1: (),
    #[offset(4)]
    pub t2: (),
}
pub fn test_0() -> () {
    {
        let alloc: Value<no_allocate_at_least_int_> =
            Rc::new(RefCell::new(<no_allocate_at_least_int_>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let alloc: Value<has_allocate_at_least_int_> =
            Rc::new(RefCell::new(<has_allocate_at_least_int_>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait has_allocate_at_least_int_Impl {
    fn allocate(&self, mut _a0: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut _a0: Ptr<()>, mut _a1: ()) {
        unimplemented!()
    }
    fn allocate_at_least(&self, _a0: ()) -> ();
}
impl has_allocate_at_least_int_Impl for Ptr<has_allocate_at_least_int_> {
    fn allocate_at_least(&self, mut _a0: ()) -> () {
        return unimplemented!();
    }
}
pub trait no_allocate_at_least_int_Impl {
    fn allocate(&self, _a0: ()) -> Ptr<()>;
    fn deallocate(&self, mut _a0: Ptr<()>, mut _a1: ()) {
        unimplemented!()
    }
}
impl no_allocate_at_least_int_Impl for Ptr<no_allocate_at_least_int_> {
    fn allocate(&self, mut _a0: ()) -> Ptr<()> {
        return (field_ptr!((*self), t));
    }
}
pub fn __cpp2rust_init_globals() {}
