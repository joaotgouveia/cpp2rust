extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_convertible_imp_0(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_1(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_2(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_3(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_4(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_5(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_6() -> () {
    return ({ test_convertible_imp_0(0_()) });
}
pub fn test_convertible_7() -> () {
    return ({ test_convertible_imp_1(0_()) });
}
pub fn test_convertible_8() -> () {
    return ({ test_convertible_imp_2(0_()) });
}
pub fn test_convertible_9() -> () {
    return ({ test_convertible_imp_3(0_()) });
}
pub fn test_convertible_10() -> () {
    return ({ test_convertible_imp_4(0_()) });
}
pub fn test_convertible_11() -> () {
    return ({ test_convertible_imp_5(0_()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct InitList {
    #[offset(0)]
    pub size: (),
}
impl InitList {
    pub fn new(il: ()) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            size: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct InitListArg {
    #[offset(0)]
    pub size: (),
    #[offset(8)]
    pub value: (),
}
impl InitListArg {
    pub fn new(il: (), mut v: ()) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            size: unimplemented!(),
            value: v,
        }
    }
}
pub fn test_ctor_sfinae_12() {
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_index_t<0>, IL>::value"
        );;
        const _: () = assert!(true, "!test_convertible<V, std::in_place_index_t<0>, IL>()");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<0>, IL, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_index_t<0>, IL, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<1>, IL>::value"
        );;
        const _: () = assert!(true, "!test_convertible<V, std::in_place_index_t<1>, IL>()");;
    }
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_index_t<1>, IL, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_index_t<1>, IL, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<2>, IL>::value"
        );;
        const _: () = assert!(true, "!test_convertible<V, std::in_place_index_t<2>, IL>()");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<3>, IL>::value"
        );;
        const _: () = assert!(true, "!test_convertible<V, std::in_place_index_t<3>, IL>()");;
    }
}
pub fn test_ctor_basic_13() {
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v).size == 3");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v.index() == 2");;
        const _: () = assert!(true, "std::get<2>(v).size == 3");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v).size == 4");;
        const _: () = assert!(true, "std::get<1>(v).value == 42");;
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
    ({ test_ctor_basic_13() });
    ({ test_ctor_sfinae_12() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
