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
pub struct multiply {}
impl Callable2<i32, i32, i32> for multiply {
    fn call(&self, a1: i32, a2: i32) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            multiplyImpl::operator_call_1_int(__it, a1, a2)
        })
    }
}
impl CppMove for multiply {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct plus_one {}
impl Callable1<i32, i32> for plus_one {
    fn call(&self, a1: i32) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            plus_oneImpl::operator_call_1_int(__it, a1)
        })
    }
}
impl CppMove for plus_one {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_0() -> bool {
    let g: Value<std__Bind_multiply__int__std__Placeholder_1___> = Rc::new(RefCell::new(
        ({
            let ___f: Value<multiply> = Rc::new(RefCell::new(<multiply>::default()));
            let ___args_1: Value<i32> = Rc::new(RefCell::new(2));
            bind_1(
                ___f.as_pointer(),
                ___args_1.as_pointer(),
                _1_2.with(|v| v.as_pointer()),
            )
        }),
    ));
    assert!(
        (({
            let ___args: Value<i32> = Rc::new(RefCell::new(5));
            (*g.borrow_mut())(___args.as_pointer())
        }) == 10)
    );
    assert!(
        (({
            let ___args: Value<i32> = Rc::new(RefCell::new(5));
            ({
                let ___f: Value<plus_one> = Rc::new(RefCell::new(<plus_one>::default()));
                bind_3(___f.as_pointer(), g.as_pointer())
            })(___args.as_pointer())
        }) == 11)
    );
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait multiplyImpl {
    fn operator_call_1_int(&self, a: i32, b: i32) -> i32;
}
impl multiplyImpl for Ptr<multiply> {
    fn operator_call_1_int(&self, mut a: i32, mut b: i32) -> i32 {
        return (a * b);
    }
}
pub trait plus_oneImpl {
    fn operator_call_1_int(&self, a: i32) -> i32;
}
impl plus_oneImpl for Ptr<plus_one> {
    fn operator_call_1_int(&self, mut a: i32) -> i32 {
        return (a + 1);
    }
}
pub fn __cpp2rust_init_globals() {}
