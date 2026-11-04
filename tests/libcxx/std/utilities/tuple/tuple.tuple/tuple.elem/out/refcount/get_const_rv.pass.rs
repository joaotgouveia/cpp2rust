extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    {
        let t: Value<std_tuple_int_> = Rc::new(RefCell::new(3));
        const _: () = assert!(
            true,
            "std::is_same<const int&&, decltype(std::get<0>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(t)))");;
        let i: Ptr<i32> = ({ get_0(t.as_pointer()) });
        assert!(((i.read()) == 3));
    }
    {
        let t: Value<
            std_tuple_std_basic_string_char__std_char_traits_char___std_allocator_char____int_,
        > = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
            std_tuple_std_basic_string_char__std_char_traits_char___std_allocator_char____int_ :: new_1 ( { Ptr::<i8 >::from_string_literal(b"high" ) } , { __tmp_0.as_pointer() } , )
        }));
        const _: () = assert!(
            true,
            "std::is_same<const std::string&&, decltype(std::get<0>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<const int&&, decltype(std::get<1>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<1>(std::move(t)))");;
        let s: Ptr<Vec<i8>> = ({ get_1(t.as_pointer()) }).reinterpret_cast::<Vec<i8>>();
        let i: Ptr<i32> = ({ get_2(t.as_pointer()) });
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"high").with_c_str(|__s| {
                    (*s.upgrade().deref())[..(*s.upgrade().deref()).len().saturating_sub(1)] == *__s
                })
            })
        );
        assert!(((i.read()) == 5));
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(42));
        let y: Value<i32> = Rc::new(RefCell::new(43));
        let p: Value<std_tuple_int_ref__const_int_ref_> = Rc::new(RefCell::new(
            std_tuple_int_ref__const_int_ref_::new_1({ x.as_pointer() }, { y.as_pointer() }),
        ));
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<0>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<1>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<1>(std::move(p)))");;
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(42));
        let y: Value<i32> = Rc::new(RefCell::new(43));
        let p: Value<std_tuple_int_refref__const_int_refref_> = Rc::new(RefCell::new(
            std_tuple_int_refref__const_int_refref_::new_1({ x.as_pointer() }, { y.as_pointer() }),
        ));
        const _: () = assert!(
            true,
            "std::is_same<int&&, decltype(std::get<0>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&&, decltype(std::get<1>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<1>(std::move(p)))");;
    }
    {
        let t: Value<std_tuple_double__int_> = Rc::new(RefCell::new({
            let __tmp_1: Value<f64> = Rc::new(RefCell::new(2.718_f64));
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(5));
            std_tuple_double__int_::new_1({ __tmp_1.as_pointer() }, { __tmp_2.as_pointer() })
        }));
        const _: () = assert!(true, "std::get<0>(std::move(t)) == 2.718");;
        const _: () = assert!(true, "std::get<1>(std::move(t)) == 5");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
