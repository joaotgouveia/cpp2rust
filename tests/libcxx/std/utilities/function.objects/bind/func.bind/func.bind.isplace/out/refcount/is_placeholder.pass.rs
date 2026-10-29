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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_2(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_3(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_4(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_5(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_6(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_7(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_8(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_9(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_10(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_11(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_12(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_13(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_14(_a0: Ptr<C>) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
    ({ test_1(unimplemented!()) });
    ({ test_2(unimplemented!()) });
    ({ test_3(unimplemented!()) });
    ({ test_4(unimplemented!()) });
    ({ test_5(unimplemented!()) });
    ({ test_6(unimplemented!()) });
    ({ test_7(unimplemented!()) });
    ({ test_8(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_10(unimplemented!()) });
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
