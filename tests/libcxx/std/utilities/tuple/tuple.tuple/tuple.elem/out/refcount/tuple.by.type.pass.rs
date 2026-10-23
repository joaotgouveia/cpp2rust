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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let t1: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(42_()));
            let __tmp_1: Value<()> = Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<()>::from_string_literal(b"Hi") },
                    None,
                ),
            ));
            let __tmp_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_tuple_int__std_string__std_complex_float__::new(
                { __tmp_0.as_pointer() },
                { __tmp_1.as_pointer() },
                { __tmp_2.as_pointer() },
            )
        }));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t2: Value<()> = Rc::new(RefCell::new({
            let __tmp_3: Value<()> = Rc::new(RefCell::new(42_()));
            let __tmp_4: Value<()> = Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<()>::from_string_literal(b"Hi") },
                    None,
                ),
            ));
            let __tmp_5: Value<()> = Rc::new(RefCell::new(23_()));
            let __tmp_6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_tuple_int__std_string__int__std_complex_float__::new(
                { __tmp_3.as_pointer() },
                { __tmp_4.as_pointer() },
                { __tmp_5.as_pointer() },
                { __tmp_6.as_pointer() },
            )
        }));
        unimplemented!();
        unimplemented!();
    }
    {
        let p5: Value<()> = Rc::new(RefCell::new({
            let __tmp_7: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_8: Value<()> = Rc::new(RefCell::new(2_()));
            let __tmp_9: Value<()> = Rc::new(RefCell::new(3.4_f64));
            let __tmp_10: Value<()> = Rc::new(RefCell::new(5.6_f64));
            std_tuple_int__const_int__double__double_::new(
                { __tmp_7.as_pointer() },
                { __tmp_8.as_pointer() },
                { __tmp_9.as_pointer() },
                { __tmp_10.as_pointer() },
            )
        }));
        const _: () = assert!(true, "std::get<int>(p5) == 1");;
        const _: () = assert!(true, "std::get<const int>(p5) == 2");;
    }
    {
        let p5: Value<()> = Rc::new(RefCell::new({
            let __tmp_11: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_12: Value<()> = Rc::new(RefCell::new(2_()));
            let __tmp_13: Value<()> = Rc::new(RefCell::new(3.4_f64));
            let __tmp_14: Value<()> = Rc::new(RefCell::new(5.6_f64));
            std_tuple_int__const_int__double__double_::new(
                { __tmp_11.as_pointer() },
                { __tmp_12.as_pointer() },
                { __tmp_13.as_pointer() },
                { __tmp_14.as_pointer() },
            )
        }));
        let i1: Ptr<()> = ({ get_0(p5.as_pointer()) });
        let i2: Ptr<()> = ({ get_1(p5.as_pointer()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(
            std_unique_ptr_int__std_default_delete_int__::new({ Ptr::alloc(4_()) }),
        ));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(
            std_unique_ptr_int__std_default_delete_int__::new({ Ptr::alloc(4_()) }),
        ));
        let p: Ptr<()> = ({ get_2(t.as_pointer()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let y: Value<()> = Rc::new(RefCell::new(43_()));
        let t: Value<()> = Rc::new(RefCell::new(std_tuple_int_ref__const_int_ref_::new(
            { x.as_pointer() },
            { y.as_pointer() },
        )));
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<int&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<int const&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&>(std::move(t)))");;
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let y: Value<()> = Rc::new(RefCell::new(43_()));
        let t: Value<()> = Rc::new(RefCell::new(std_tuple_int_refref__const_int_refref_::new(
            { (x.as_pointer()).reinterpret_cast::<()>() },
            { (y.as_pointer()).reinterpret_cast::<()>() },
        )));
        const _: () = assert!(
            true,
            "std::is_same<int&&, decltype(std::get<int&&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&&>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&&, decltype(std::get<int const&&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&&>(std::move(t)))");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new({
            let __tmp_15: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_16: Value<()> = Rc::new(RefCell::new(2_()));
            let __tmp_17: Value<()> = Rc::new(RefCell::new(3.4_f64));
            let __tmp_18: Value<()> = Rc::new(RefCell::new(5.6_f64));
            std_tuple_int__const_int__double__double_::new(
                { __tmp_15.as_pointer() },
                { __tmp_16.as_pointer() },
                { __tmp_17.as_pointer() },
                { __tmp_18.as_pointer() },
            )
        }));
        const _: () = assert!(true, "std::get<int>(std::move(t)) == 1");;
        const _: () = assert!(true, "std::get<const int>(std::move(t)) == 2");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
