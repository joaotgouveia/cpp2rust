extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static allocated__0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static deallocated__1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct test_alloc_char__unsigned_int_ {}
impl test_alloc_char__unsigned_int_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for test_alloc_char__unsigned_int_ {
    fn default() -> Self {
        { test_alloc_char__unsigned_int_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct test_alloc_char__unsigned_long_ {}
impl test_alloc_char__unsigned_long_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for test_alloc_char__unsigned_long_ {
    fn default() -> Self {
        { test_alloc_char__unsigned_long_::new() }
    }
}
pub fn test_2() {
    let mut i: () = 1_();
    'loop_: while (i < 1000_()) {
        {
            let mut s: () = unimplemented!();
            &(s);
        }
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_3() {
    let mut i: () = 1_();
    'loop_: while (i < 1000_()) {
        {
            let mut s: () = unimplemented!();
            &(s);
        }
        unimplemented!();
        i.prefix_inc();
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
    ({ test_2() });
    ({ test_3() });
    ({ test_3() });
    return 0_();
}
pub trait test_alloc_char__unsigned_int_Impl {
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
}
impl test_alloc_char__unsigned_int_Impl for Ptr<test_alloc_char__unsigned_int_> {
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        allocated__0.with(|rc| {
            *rc.borrow_mut() = { (allocated__0.with(|rc| *rc.borrow())).wrapping_add(n) }
        });
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        deallocated__1.with(|rc| {
            *rc.borrow_mut() = { (deallocated__1.with(|rc| *rc.borrow())).wrapping_add(s) }
        });
        unimplemented!();
    }
}
pub trait test_alloc_char__unsigned_long_Impl {
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
}
impl test_alloc_char__unsigned_long_Impl for Ptr<test_alloc_char__unsigned_long_> {
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        allocated__0.with(|rc| {
            *rc.borrow_mut() = { (allocated__0.with(|rc| *rc.borrow())).wrapping_add(n) }
        });
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        deallocated__1.with(|rc| {
            *rc.borrow_mut() = { (deallocated__1.with(|rc| *rc.borrow())).wrapping_add(s) }
        });
        unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = allocated__0.with(|_| ());
    let _ = deallocated__1.with(|_| ());
}
