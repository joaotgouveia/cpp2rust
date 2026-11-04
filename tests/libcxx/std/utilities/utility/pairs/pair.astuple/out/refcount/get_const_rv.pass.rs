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
        let p: Value<std_pair_std_unique_ptr_int__std_default_delete_int____short_> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<std_unique_ptr_int__std_default_delete_int__> =
                    Rc::new(RefCell::new(
                        std_unique_ptr_int__std_default_delete_int__::new_1({ Ptr::alloc(3) }),
                    ));
                let __tmp_1: Value<i16> = Rc::new(RefCell::new(4_i16));
                std_pair_std_unique_ptr_int__std_default_delete_int____short_::new_1(
                    { __tmp_0.as_pointer() },
                    { __tmp_1.as_pointer() },
                )
            }));
        const _: () = assert!(
            true,
            "std::is_same<const std::unique_ptr<int>&&, decltype(std::get<0>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(p)))");;
        let ptr: Ptr<std_unique_ptr_int__std_default_delete_int__> = ({ get_0(p.as_pointer()) });
        ({ cpp2rust_assert_fail_1((((*ptr.upgrade().deref()).read()) == 3)) });
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(42));
        let y: Value<i32> = Rc::new(RefCell::new(43));
        let p: Value<std_pair_int_ref__const_int_ref_> = Rc::new(RefCell::new(
            std_pair_int_ref__const_int_ref_::new_1({ x.as_pointer() }, { y.as_pointer() }),
        ));
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<0>(std::move(p)))>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<1>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(p)))");;
        const _: () = assert!(true, "noexcept(std::get<1>(std::move(p)))");;
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(42));
        let y: Value<i32> = Rc::new(RefCell::new(43));
        let p: Value<std_pair_int_refref__const_int_refref_> = Rc::new(RefCell::new(
            std_pair_int_refref__const_int_refref_::new_1({ x.as_pointer() }, { y.as_pointer() }),
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
        let p1: Value<std_pair_int__short_> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(3));
            let __tmp_3: Value<i16> = Rc::new(RefCell::new(4_i16));
            std_pair_int__short_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
        }));
        const _: () = assert!(true, "std::get<0>(std::move(p1)) == 3");;
        const _: () = assert!(true, "std::get<1>(std::move(p1)) == 4");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
