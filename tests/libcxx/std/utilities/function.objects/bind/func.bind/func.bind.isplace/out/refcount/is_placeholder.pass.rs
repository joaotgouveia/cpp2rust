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
pub fn test_1(_a0: Ptr<std_placeholders___ph_1_>) {
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
pub fn test_2(_a0: Ptr<std_placeholders___ph_2_>) {
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
pub fn test_3(_a0: Ptr<std_placeholders___ph_3_>) {
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
pub fn test_4(_a0: Ptr<std_placeholders___ph_4_>) {
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
pub fn test_5(_a0: Ptr<std_placeholders___ph_5_>) {
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
pub fn test_6(_a0: Ptr<std_placeholders___ph_6_>) {
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
pub fn test_7(_a0: Ptr<std_placeholders___ph_7_>) {
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
pub fn test_8(_a0: Ptr<std_placeholders___ph_8_>) {
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
pub fn test_9(_a0: Ptr<std_placeholders___ph_9_>) {
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
pub fn test_10(_a0: Ptr<std_placeholders___ph_10_>) {
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
pub fn test_11(_a0: Ptr<()>) {
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
pub fn test_12(_a0: Ptr<()>) {
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
pub fn test_13(_a0: Ptr<()>) {
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
pub fn test_14(_a0: Ptr<C>) {
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
    ({ test_1(_1_15.with(|v| v.as_pointer())) });
    ({ test_2(_2_16.with(|v| v.as_pointer())) });
    ({ test_3(_3_17.with(|v| v.as_pointer())) });
    ({ test_4(_4_18.with(|v| v.as_pointer())) });
    ({ test_5(_5_19.with(|v| v.as_pointer())) });
    ({ test_6(_6_20.with(|v| v.as_pointer())) });
    ({ test_7(_7_21.with(|v| v.as_pointer())) });
    ({ test_8(_8_22.with(|v| v.as_pointer())) });
    ({ test_9(_9_23.with(|v| v.as_pointer())) });
    ({ test_10(_10_24.with(|v| v.as_pointer())) });
    ({
        let _arg0: Value<()> = Rc::new(RefCell::new(4_()));
        test_11(_arg0.as_pointer())
    });
    ({
        let _arg0: Value<()> = Rc::new(RefCell::new(5.5_f64));
        test_12(_arg0.as_pointer())
    });
    ({
        let _arg0: Value<()> = Rc::new(RefCell::new(('a' as ())));
        test_13(_arg0.as_pointer())
    });
    ({
        let _arg0: Value<C> = Rc::new(RefCell::new(<C>::default()));
        test_14(_arg0.as_pointer())
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
