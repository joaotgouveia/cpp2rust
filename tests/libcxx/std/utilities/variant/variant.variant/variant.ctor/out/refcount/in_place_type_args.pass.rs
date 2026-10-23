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
pub fn test_ctor_sfinae_12() {
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_type_t<int>, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<int>, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_type_t<long>, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<long>, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_type_t<int *>, int *>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<int *>, int *>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_type_t<int>, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<int>, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_type_t<int>, int *>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<int>, int *>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_type_t<long long>, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_type_t<long long>, int>()"
        );;
    }
}
pub fn test_ctor_basic_13() {
    {
        let v: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(42_()));
            std_variant_int_::new(
                { (*in_place_type_14.with(Value::clone).borrow()).clone() },
                { __tmp_0.as_pointer() },
            )
        }));
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v) == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new({
            let __tmp_1: Value<()> = Rc::new(RefCell::new(42_()));
            std_variant_int__long_::new(
                { (*in_place_type_15.with(Value::clone).borrow()).clone() },
                { __tmp_1.as_pointer() },
            )
        }));
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<()> = Rc::new(RefCell::new(42_()));
            std_variant_int__const_int__long_::new(
                { (*in_place_type_16.with(Value::clone).borrow()).clone() },
                { __tmp_2.as_pointer() },
            )
        }));
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let v: Value<()> = Rc::new(RefCell::new(
            std_variant_const_int__volatile_int__int_::new_1(
                { (*in_place_type_16.with(Value::clone).borrow()).clone() },
                { x.as_pointer() },
            ),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let v: Value<()> = Rc::new(RefCell::new(
            std_variant_const_int__volatile_int__int_::new_2(
                { (*in_place_type_17.with(Value::clone).borrow()).clone() },
                { x.as_pointer() },
            ),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let v: Value<()> = Rc::new(RefCell::new(
            std_variant_const_int__volatile_int__int_::new_3(
                { (*in_place_type_14.with(Value::clone).borrow()).clone() },
                { x.as_pointer() },
            ),
        ));
        unimplemented!();
        unimplemented!();
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
