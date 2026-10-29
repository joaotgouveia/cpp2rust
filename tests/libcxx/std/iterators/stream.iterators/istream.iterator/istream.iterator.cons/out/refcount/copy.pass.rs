extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_constexpr_0() -> () {
    let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return true;
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(1)]
pub struct throwing_copy_constructor {}
impl throwing_copy_constructor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<throwing_copy_constructor>) -> Self {
        Self {}
    }
}
impl Clone for throwing_copy_constructor {
    fn clone(&self) -> Self {
        let __src: Value<throwing_copy_constructor> =
            Rc::new(RefCell::new(throwing_copy_constructor {}));
        throwing_copy_constructor::copy_from(__src.as_pointer())
    }
}
impl Default for throwing_copy_constructor {
    fn default() -> Self {
        { throwing_copy_constructor::new() }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        const _: () = assert!(
            true,
            "std::is_nothrow_copy_constructible<std::istream_iterator<int>>::value"
        );;
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        const _: () = assert!(
            true,
            "!std::is_nothrow_copy_constructible<std::istream_iterator<throwing_copy_constructor>>::value"
        );;
    }
    {
        let inf: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut j: () = 0_();
        j = (unimplemented!().read());
        unimplemented!();
    }
    const _: () = assert!(true, "test_constexpr()");;
    return 0_();
}
pub trait throwing_copy_constructorImpl {
    fn copy_assign(&self, _a0: Ptr<throwing_copy_constructor>) -> Ptr<throwing_copy_constructor>;
}
impl throwing_copy_constructorImpl for Ptr<throwing_copy_constructor> {
    fn copy_assign(&self, _a0: Ptr<throwing_copy_constructor>) -> Ptr<throwing_copy_constructor> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
