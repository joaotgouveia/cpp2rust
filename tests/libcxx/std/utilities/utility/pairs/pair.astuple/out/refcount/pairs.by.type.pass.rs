extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> () {
    {
        let i: Value<()> = Rc::new(RefCell::new(1_()));
        let j: Value<()> = Rc::new(RefCell::new(2_()));
        {
            let p: Value<()> = Rc::new(RefCell::new(std_pair_int_ref__int_refref_::new(
                { i.as_pointer() },
                { (j.as_pointer()).reinterpret_cast::<()>() },
            )));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let cp: Value<()> = Rc::new(RefCell::new(std_pair_int_ref__int_refref_::new(
                { i.as_pointer() },
                { (j.as_pointer()).reinterpret_cast::<()>() },
            )));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let p: Value<()> = Rc::new(RefCell::new(std_pair_int_refref__int_ref_::new(
                { (i.as_pointer()).reinterpret_cast::<()>() },
                { j.as_pointer() },
            )));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let cp: Value<()> = Rc::new(RefCell::new(std_pair_int_refref__int_ref_::new(
                { (i.as_pointer()).reinterpret_cast::<()>() },
                { j.as_pointer() },
            )));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p1: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_1: Value<()> = Rc::new(RefCell::new(2_()));
            std_pair_int__const_int_::new({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        let i1: Ptr<()> = unimplemented!();
        let i2: Ptr<()> = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let y: Value<()> = Rc::new(RefCell::new(43_()));
        let p: Value<()> = Rc::new(RefCell::new(std_pair_int_ref__const_int_ref_::new(
            { x.as_pointer() },
            { y.as_pointer() },
        )));
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<int&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<int const&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&>(std::move(p)))");;
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let y: Value<()> = Rc::new(RefCell::new(43_()));
        let p: Value<()> = Rc::new(RefCell::new(std_pair_int_refref__const_int_refref_::new(
            { (x.as_pointer()).reinterpret_cast::<()>() },
            { y.as_pointer() },
        )));
        const _: () = assert!(
            true,
            "std::is_same<int&&, decltype(std::get<int&&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&&>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&&, decltype(std::get<int const&&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&&>(std::move(p)))");;
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_3: Value<()> = Rc::new(RefCell::new(2_()));
            std_pair_int__const_int_::new({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
        }));
        const _: () = assert!(true, "std::get<int>(std::move(p)) == 1");;
        const _: () = assert!(true, "std::get<const int>(std::move(p)) == 2");;
    }
    return true;
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    {
        let t: Value<()> = Rc::new(RefCell::new({
            let __tmp_4: Value<()> = Rc::new(RefCell::new(
                std_unique_ptr_int__std_default_delete_int__::new({ Ptr::alloc(4_()) }),
            ));
            let __tmp_5: Value<()> = Rc::new(RefCell::new(42_()));
            std_pair_std_unique_ptr_int__std_default_delete_int____int_::new(
                { __tmp_4.as_pointer() },
                { __tmp_5.as_pointer() },
            )
        }));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new({
            let __tmp_6: Value<()> = Rc::new(RefCell::new(
                std_unique_ptr_int__std_default_delete_int__::new({ Ptr::alloc(4_()) }),
            ));
            let __tmp_7: Value<()> = Rc::new(RefCell::new(42_()));
            std_pair_std_unique_ptr_int__std_default_delete_int____int_::new(
                { __tmp_6.as_pointer() },
                { __tmp_7.as_pointer() },
            )
        }));
        const _: () = assert!(
            true,
            "std::is_same<const upint&&, decltype(std::get<upint>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<upint>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<const int&&, decltype(std::get<int>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int>(std::move(t)))");;
        let p: Ptr<()> = unimplemented!();
        let i: Ptr<()> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
