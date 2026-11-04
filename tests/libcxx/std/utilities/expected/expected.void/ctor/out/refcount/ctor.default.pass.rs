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
pub struct NoDefaultCtor {}
impl CppMove for NoDefaultCtor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    true,
    "std::is_nothrow_default_constructible_v<std::expected<void, int>>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_default_constructible_v<std::expected<void, NoDefaultCtor>>"
);
pub fn operator_eq_0(_a0: Ptr<MyInt>, _a1: Ptr<MyInt>) -> bool {
    return ({ _a0.with(|__s| __s.i) } == { _a1.with(|__s| __s.i) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct MyInt {
    #[offset(0)]
    pub i: i32,
}
impl CppEq for MyInt {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for MyInt {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_1() -> bool {
    {
        let e: Value<std_expected_void__int_> =
            Rc::new(RefCell::new(<std_expected_void__int_>::default()));
        assert!(({ ({ (*e.borrow()).has_value() }) }));
    }
    {
        let e: Value<std_expected_void__NoDefaultCtor_> =
            Rc::new(RefCell::new(<std_expected_void__NoDefaultCtor_>::default()));
        assert!(({ ({ (*e.borrow()).has_value() }) }));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
