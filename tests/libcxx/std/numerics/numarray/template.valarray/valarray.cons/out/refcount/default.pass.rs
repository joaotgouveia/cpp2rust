extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct S {}
impl S {
    pub fn new() -> Self {
        let __this: S = Self {};
        ctor_called_0.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
thread_local!(
    pub static ctor_called_0: Value<bool> = Rc::new(RefCell::new(false));
);

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
        let v: Value<std_valarray_int_> = Rc::new(RefCell::new(<std_valarray_int_>::default()));
        assert!(({ (({ (*v.borrow()).size() }) == 0_usize) }));
    }
    {
        let v: Value<std_valarray_float_> = Rc::new(RefCell::new(<std_valarray_float_>::default()));
        assert!(({ (({ (*v.borrow()).size() }) == 0_usize) }));
    }
    {
        let v: Value<std_valarray_double_> =
            Rc::new(RefCell::new(<std_valarray_double_>::default()));
        assert!(({ (({ (*v.borrow()).size() }) == 0_usize) }));
    }
    {
        let v: Value<std_valarray_std_valarray_double__> =
            Rc::new(RefCell::new(<std_valarray_std_valarray_double__>::default()));
        assert!(({ (({ (*v.borrow()).size() }) == 0_usize) }));
    }
    {
        let v: Value<std_valarray_S_> = Rc::new(RefCell::new(<std_valarray_S_>::default()));
        assert!(({ (({ (*v.borrow()).size() }) == 0_usize) }));
        assert!(!(ctor_called_0.with(|rc| *rc.borrow())));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = ctor_called_0.with(|_| ());
}
