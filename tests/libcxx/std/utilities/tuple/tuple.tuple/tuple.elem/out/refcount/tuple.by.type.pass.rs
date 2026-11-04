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
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 42));
        assert!(
            ({
                match (unimplemented!(),) {
                    (__tmp_0,) => Ptr::<i8>::from_string_literal(b"Hi").with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!((unimplemented!() == 1_f32));
        assert!((unimplemented!() == 2_f32));
    }
    {
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                match (unimplemented!(),) {
                    (__tmp_1,) => Ptr::<i8>::from_string_literal(b"Hi").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!((unimplemented!()));
    }
    {
        let p5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<int>(p5) == 1");;
        const _: () = assert!(true, "std::get<const int>(p5) == 2");;
    }
    {
        let p5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i1: Ptr<i32> = unimplemented!();
        let i2: Ptr<i32> = unimplemented!();
        assert!(((i1.read()) == 1));
        assert!(((i2.read()) == 2));
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut p: Option<Value<i32>> = unimplemented!().take();
        assert!(((p.as_pointer().read()) == 4));
        assert!((unimplemented!()).clone().is_none());
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Ptr<Option<Value<i32>>> = unimplemented!();
        assert!(({ (((*p.upgrade().deref()).as_pointer().read()) == 4) }));
        assert!(!((unimplemented!()).clone().is_none()));
    }
    {
        let mut x: i32 = 42;
        let y: Value<i32> = Rc::new(RefCell::new(43));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        let mut x: i32 = 42;
        let y: Value<i32> = Rc::new(RefCell::new(43));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<int>(std::move(t)) == 1");;
        const _: () = assert!(true, "std::get<const int>(std::move(t)) == 2");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
