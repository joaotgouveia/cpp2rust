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
pub struct test_macros_detail_is_same_const_bool__const_bool_ {}
impl CppMove for test_macros_detail_is_same_const_bool__const_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_1(_a0: Ptr<i32>) {
    const _: () = assert!(true, "std::is_bind_expression<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::is_bind_expression<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_bind_expression_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct C {}
impl CppMove for C {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    ({
        let _arg0: Value<std__Bind_C____> = Rc::new(RefCell::new(
            ({
                let ___f: Value<C> = Rc::new(RefCell::new(<C>::default()));
                bind_2(___f.as_pointer())
            }),
        ));
        test_3(_arg0.as_pointer())
    });
    ({
        let _arg0: Value<std__Bind_C__std__Placeholder_2___> = Rc::new(RefCell::new(
            ({
                let ___f: Value<C> = Rc::new(RefCell::new(<C>::default()));
                bind_4(___f.as_pointer(), _2_5.with(|v| v.as_pointer()))
            }),
        ));
        test_6(_arg0.as_pointer())
    });
    ({
        let _arg0: Value<std__Bind_result_int__C____> = Rc::new(RefCell::new(
            ({
                let ___f: Value<C> = Rc::new(RefCell::new(<C>::default()));
                bind_7(___f.as_pointer())
            }),
        ));
        test_8(_arg0.as_pointer())
    });
    ({
        let _arg0: Value<i32> = Rc::new(RefCell::new(1));
        test_1(_arg0.as_pointer())
    });
    ({ test_9(_2_5.with(|v| v.as_pointer())) });
    return 0;
}
pub trait CImpl {
    fn operator_call(&self, __args: Box<[VaArg]>) -> i32;
}
impl CImpl for Ptr<C> {
    fn operator_call(&self, __args: Box<[VaArg]>) -> i32 {
        let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
        return 0;
    }
}
pub fn __cpp2rust_init_globals() {}
