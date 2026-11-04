extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_ctor_sfinae_0() {
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
pub fn test_ctor_basic_1() {
    {
        let v: Value<std_variant_int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(42));
            std_variant_int_::new_1(
                { (*in_place_type_2.with(Value::clone).borrow()).clone() },
                { __tmp_0.as_pointer() },
            )
        }));
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v) == 42");;
    }
    {
        let v: Value<std_variant_int__long_> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(42));
            std_variant_int__long_::new_1(
                { (*in_place_type_3.with(Value::clone).borrow()).clone() },
                { __tmp_1.as_pointer() },
            )
        }));
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let v: Value<std_variant_int__const_int__long_> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(42));
            std_variant_int__const_int__long_::new_1(
                { (*in_place_type_4.with(Value::clone).borrow()).clone() },
                { __tmp_2.as_pointer() },
            )
        }));
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(42));
        let v: Value<std_variant_const_int__volatile_int__int_> = Rc::new(RefCell::new(
            std_variant_const_int__volatile_int__int_::new_1(
                { (*in_place_type_4.with(Value::clone).borrow()).clone() },
                { x.as_pointer() },
            ),
        ));
        assert!(({ (({ (*v.borrow()).index() }) == 0_usize) }));
        assert!(({ ((({ get_5(v.as_pointer(),) }).read()) == (*x.borrow())) }));
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(42));
        let v: Value<std_variant_const_int__volatile_int__int_> = Rc::new(RefCell::new(
            std_variant_const_int__volatile_int__int_::new_2(
                { (*in_place_type_6.with(Value::clone).borrow()).clone() },
                { x.as_pointer() },
            ),
        ));
        assert!(({ (({ (*v.borrow()).index() }) == 1_usize) }));
        assert!(({ ((({ get_7(v.as_pointer(),) }).read()) == (*x.borrow())) }));
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(42));
        let v: Value<std_variant_const_int__volatile_int__int_> = Rc::new(RefCell::new(
            std_variant_const_int__volatile_int__int_::new_3(
                { (*in_place_type_2.with(Value::clone).borrow()).clone() },
                { x.as_pointer() },
            ),
        ));
        assert!(({ (({ (*v.borrow()).index() }) == 2_usize) }));
        assert!(({ ((({ get_8(v.as_pointer(),) }).read()) == (*x.borrow())) }));
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
    ({ test_ctor_basic_1() });
    ({ test_ctor_sfinae_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
