extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    {
        let i: Value<i32> = Rc::new(RefCell::new(1));
        let j: Value<i32> = Rc::new(RefCell::new(2));
        {
            let p: Value<(Value<Ptr<i32>>, Value<Ptr<i32>>)> = Rc::new(RefCell::new((
                Rc::new(RefCell::new((*i.borrow()))),
                Rc::new(RefCell::new((*j.borrow()))),
            )));
            assert!(((unimplemented!()) == (i.as_pointer())));
            assert!(((unimplemented!()) == (j.as_pointer())));
            assert!(((unimplemented!()) == (i.as_pointer())));
            assert!(((unimplemented!().read()) == 2));
            let cp: Value<(Value<Ptr<i32>>, Value<Ptr<i32>>)> = Rc::new(RefCell::new((
                Rc::new(RefCell::new((*i.borrow()))),
                Rc::new(RefCell::new((*j.borrow()))),
            )));
            assert!(((unimplemented!()) == (i.as_pointer())));
            assert!(((unimplemented!()) == (j.as_pointer())));
            assert!(((unimplemented!()) == (i.as_pointer())));
            assert!(((unimplemented!().read()) == 2));
        }
        {
            let p: Value<(Value<Ptr<i32>>, Value<Ptr<i32>>)> = Rc::new(RefCell::new((
                Rc::new(RefCell::new((*i.borrow()))),
                Rc::new(RefCell::new((*j.borrow()))),
            )));
            assert!(((unimplemented!()) == (j.as_pointer())));
            assert!(((unimplemented!()) == (i.as_pointer())));
            assert!(((unimplemented!()) == (j.as_pointer())));
            assert!(((unimplemented!().read()) == 1));
            let cp: Value<(Value<Ptr<i32>>, Value<Ptr<i32>>)> = Rc::new(RefCell::new((
                Rc::new(RefCell::new((*i.borrow()))),
                Rc::new(RefCell::new((*j.borrow()))),
            )));
            assert!(((unimplemented!()) == (j.as_pointer())));
            assert!(((unimplemented!()) == (i.as_pointer())));
            assert!(((unimplemented!()) == (j.as_pointer())));
            assert!(((unimplemented!().read()) == 1));
        }
    }
    {
        let t1: Value<(Value<i32>, Value<()>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(42)),
            Rc::new(RefCell::new(unimplemented!())),
        )));
        assert!(((unimplemented!().read()) == 42));
        assert!((unimplemented!() == 1_f32));
        assert!((unimplemented!() == 2_f32));
    }
    {
        let p1: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(1)),
            Rc::new(RefCell::new(2)),
        )));
        let i1: Ptr<i32> = unimplemented!();
        let i2: Ptr<i32> = unimplemented!();
        assert!(((i1.read()) == 1));
        assert!(((i2.read()) == 2));
    }
    {
        let mut x: i32 = 42;
        let mut y: i32 = 43;
        let p: Value<(Value<Ptr<i32>>, Value<Ptr<i32>>)> = Rc::new(RefCell::new(unimplemented!()));
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
        let x: Value<i32> = Rc::new(RefCell::new(42));
        let y: Value<i32> = Rc::new(RefCell::new(43));
        let p: Value<(Value<Ptr<i32>>, Value<Ptr<i32>>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new((*x.borrow()))),
            Rc::new(RefCell::new((*y.borrow()))),
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
        let p: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(1)),
            Rc::new(RefCell::new(2)),
        )));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    {
        let t: Value<(Value<Option<Value<i32>>>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(Ptr::alloc(4).to_owned_opt())),
            Rc::new(RefCell::new(42)),
        )));
        let mut p: Option<Value<i32>> = unimplemented!().take();
        assert!(((p.as_pointer().read()) == 4));
        assert!((unimplemented!()).clone().is_none());
    }
    {
        let t: Value<(Value<Option<Value<i32>>>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(Ptr::alloc(4).to_owned_opt())),
            Rc::new(RefCell::new(42)),
        )));
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
        let p: Ptr<Option<Value<i32>>> = unimplemented!();
        let i: Ptr<i32> = unimplemented!();
        assert!((((*p.upgrade().deref()).as_pointer().read()) == 4));
        assert!(((i.read()) == 42));
        assert!(!((unimplemented!()).clone().is_none()));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
