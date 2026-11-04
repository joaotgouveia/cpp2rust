extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(f32, to_f32_1_float)]
#[cpp_into(f64, to_f64_1_double)]
#[byte_size(1)]
pub struct NastyConvertible {}
impl CppMove for NastyConvertible {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_0() {
    let nasty: Value<NastyConvertible> = Rc::new(RefCell::new(<NastyConvertible>::default()));
    let x: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    assert!(({ (({ (*x.borrow()).real_5() }) == 0_f32) }));
    assert!(({ (({ (*x.borrow()).imag_6() }) == 0_f32) }));
}
pub fn test_1() {
    let nasty: Value<NastyConvertible> = Rc::new(RefCell::new(<NastyConvertible>::default()));
    let x: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    assert!(({ (({ (*x.borrow()).real_5() }) == 0_f64) }));
    assert!(({ (({ (*x.borrow()).imag_6() }) == 0_f64) }));
}
pub fn test_2() {
    let nasty: Value<NastyConvertible> = Rc::new(RefCell::new(<NastyConvertible>::default()));
    let x: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    assert!(({ (({ (*x.borrow()).real_5() }) == 0_f64) }));
    assert!(({ (({ (*x.borrow()).imag_6() }) == 0_f64) }));
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
    ({ test_1() });
    ({ test_2() });
    return 0;
}
pub trait NastyConvertibleImpl {
    fn to_f32_1_float(&self) -> f32;
    fn to_f64_1_double(&self) -> f64;
    fn to_f64_1_long_double(&self) -> f64;
}
impl NastyConvertibleImpl for Ptr<NastyConvertible> {
    fn to_f32_1_float(&self) -> f32 {
        return 0_f32;
    }
    fn to_f64_1_double(&self) -> f64 {
        return 0_f64;
    }
    fn to_f64_1_long_double(&self) -> f64 {
        return 0_f64;
    }
}
pub fn __cpp2rust_init_globals() {}
