extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int__const_int_ {}
pub fn test_1(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<T&>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T&>::value == Expected");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T&> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T&> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<T&> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T&> == Expected");;
}
pub fn test_2(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<T&>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T&>::value == Expected");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T&> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T&> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<T&> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T&> == Expected");;
}
pub fn test_3(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<T&>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T&>::value == Expected");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T&> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T&> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<T&> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T&> == Expected");;
}
pub fn test_4(_a0: Ptr<C>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<T&>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T&>::value == Expected");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T&> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T&> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<T&> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T&> == Expected");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct C {}
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
    ({ test_5(_1_6.with(|v| v.as_pointer())) });
    ({ test_7(_2_8.with(|v| v.as_pointer())) });
    ({ test_9(_3_10.with(|v| v.as_pointer())) });
    ({ test_11(_4_12.with(|v| v.as_pointer())) });
    ({ test_13(_5_14.with(|v| v.as_pointer())) });
    ({ test_15(_6_16.with(|v| v.as_pointer())) });
    ({ test_17(_7_18.with(|v| v.as_pointer())) });
    ({ test_19(_8_20.with(|v| v.as_pointer())) });
    ({ test_21(_9_22.with(|v| v.as_pointer())) });
    ({ test_23(_10_24.with(|v| v.as_pointer())) });
    ({
        let _arg0: Value<()> = Rc::new(RefCell::new(4_()));
        test_1(_arg0.as_pointer())
    });
    ({
        let _arg0: Value<()> = Rc::new(RefCell::new(5.5_f64));
        test_2(_arg0.as_pointer())
    });
    ({
        let _arg0: Value<()> = Rc::new(RefCell::new(('a' as ())));
        test_3(_arg0.as_pointer())
    });
    ({
        let _arg0: Value<C> = Rc::new(RefCell::new(<C>::default()));
        test_4(_arg0.as_pointer())
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
