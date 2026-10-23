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
pub struct ExplicitAllocator_int_ {}
impl ExplicitAllocator_std___shared_ptr_emplace_int__ExplicitAllocator_int___ {
    pub fn new_1(mut _a0: ExplicitAllocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitAllocator_std___shared_ptr_emplace_int__ExplicitAllocator_int___ {}
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
    let ptr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    &(*ptr.borrow_mut());
    return 0_();
}
pub trait ExplicitAllocator_int_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut ptr: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl ExplicitAllocator_int_Impl for Ptr<ExplicitAllocator_int_> {}
pub trait ExplicitAllocator_std___shared_ptr_emplace_int__ExplicitAllocator_int___Impl {
    fn allocate(&self, n: ()) -> Ptr<std___shared_ptr_emplace_int__ExplicitAllocator_int__>;
    fn deallocate(&self, ptr: Ptr<std___shared_ptr_emplace_int__ExplicitAllocator_int__>, n: ());
}
impl ExplicitAllocator_std___shared_ptr_emplace_int__ExplicitAllocator_int___Impl
    for Ptr<ExplicitAllocator_std___shared_ptr_emplace_int__ExplicitAllocator_int___>
{
    fn allocate(&self, mut n: ()) -> Ptr<std___shared_ptr_emplace_int__ExplicitAllocator_int__> {
        return unimplemented!();
    }
    fn deallocate(
        &self,
        mut ptr: Ptr<std___shared_ptr_emplace_int__ExplicitAllocator_int__>,
        mut n: (),
    ) {
        unimplemented!();
        return;
    }
}
pub fn __cpp2rust_init_globals() {}
