extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_Base_ref__Base_ref_ {}
impl CppMove for test_macros_detail_is_same_Base_ref__Base_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_Base_refref__Base_refref_ {}
impl CppMove for test_macros_detail_is_same_Base_refref__Base_refref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_Base_ref__const_Base_ref_ {}
impl CppMove for test_macros_detail_is_same_const_Base_ref__const_Base_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_Base_refref__const_Base_refref_ {}
impl CppMove for test_macros_detail_is_same_const_Base_refref__const_Base_refref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Base {}
impl CppMove for Base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_4() -> bool {
    let ov: Value<std_ranges_owning_view_Base_> =
        Rc::new(RefCell::new(<std_ranges_owning_view_Base_>::default()));
    let b1: Ptr<Base> = ({ (*ov.borrow()).base_5() });
    let b2: Ptr<Base> = ({ (*ov.borrow()).base_7() });
    let b3: Ptr<Base> = ({ (*ov.borrow()).base_6() });
    let b4: Ptr<Base> = ({ (*ov.borrow()).base_8() });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(({ (b1).clone() } == { (b2).clone() }));
    assert!(({ (b1).clone() } == { (b3).clone() }));
    assert!(({ (b1).clone() } == { (b4).clone() }));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
pub trait BaseImpl {
    fn begin(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl BaseImpl for Ptr<Base> {}
pub fn __cpp2rust_init_globals() {}
