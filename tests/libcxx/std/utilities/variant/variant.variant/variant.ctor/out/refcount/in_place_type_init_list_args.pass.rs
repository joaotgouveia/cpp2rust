extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_convertible_imp_0(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_imp_1(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_imp_2(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_imp_3(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_imp_4(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_imp_5(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_6() -> bool {
    return ({ test_convertible_imp_0(0_i64) });
}
pub fn test_convertible_7() -> bool {
    return ({ test_convertible_imp_1(0_i64) });
}
pub fn test_convertible_8() -> bool {
    return ({ test_convertible_imp_2(0_i64) });
}
pub fn test_convertible_9() -> bool {
    return ({ test_convertible_imp_3(0_i64) });
}
pub fn test_convertible_10() -> bool {
    return ({ test_convertible_imp_4(0_i64) });
}
pub fn test_convertible_11() -> bool {
    return ({ test_convertible_imp_5(0_i64) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Vec<i32  >  , new, value)]
#[byte_size(8)]
pub struct InitList {
    #[offset(0)]
    pub size: usize,
}
impl InitList {
    pub fn new(il: Vec<i32>) -> Self {
        let il: Value<Vec<i32>> = Rc::new(RefCell::new(il));
        Self {
            size: (*il.borrow()).len(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct InitListArg {
    #[offset(0)]
    pub size: usize,
    #[offset(8)]
    pub value: i32,
}
impl InitListArg {
    pub fn new(il: Vec<i32>, mut v: i32) -> Self {
        let il: Value<Vec<i32>> = Rc::new(RefCell::new(il));
        Self {
            size: (*il.borrow()).len(),
            value: v,
        }
    }
}
pub fn test_ctor_sfinae_12() {
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_type_t<InitList>, IL>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<InitList>, IL>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_type_t<InitList>, IL,\n                                         int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<InitList>, IL, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_type_t<InitListArg>, IL>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<InitListArg>, IL>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_type_t<InitListArg>,\n                                        IL, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<InitListArg>, IL, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_type_t<int>, IL>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<int>, IL>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_type_t<InitListArg>,\n                                         IL, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<InitListArg>, IL, int>()"
        );;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_ctor_basic_13() });
    ({ test_ctor_sfinae_12() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
