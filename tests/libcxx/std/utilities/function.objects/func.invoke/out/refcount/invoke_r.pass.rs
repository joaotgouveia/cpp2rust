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
        let f: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(
            |mut i: i32| -> i32 {
                {
                    return (i + 3);
                }
            },
        )));
        assert!((unimplemented!() == 7));
    }
    {
        let f: Value<FnPtr<fn(i32) -> Ptr<i8>>> = Rc::new(RefCell::new(
            FnPtr::<fn(i32) -> Ptr<i8>>::new(|mut _a0: i32| -> Ptr<i8> {
                {
                    return Ptr::<i8>::null();
                }
            }),
        ));
        const _: () = assert!(true, "can_invoke_r<char*, decltype(f), int>");;
        const _: () = assert!(true, "can_invoke_r<void*, decltype(f), int>");;
        const _: () = assert!(true, "can_invoke_r<void,  decltype(f), int>");;
        const _: () = assert!(!(false), "!can_invoke_r<char*, decltype(f), void*>");;
        const _: () = assert!(!(false), "!can_invoke_r<char*, decltype(f)>");;
        const _: () = assert!(!(false), "!can_invoke_r<int*,  decltype(f), int>");;
        const _: () = assert!(!(false), "!can_invoke_r<void,  decltype(f), void*>");;
    }
    {
        let f: Value<FnPtr<fn(i32) -> Ptr<i8>>> = Rc::new(RefCell::new(
            FnPtr::<fn(i32) -> Ptr<i8>>::new(|mut _a0: i32| -> Ptr<i8> {
                {
                    return Ptr::<i8>::null();
                }
            }),
        ));
        let g: Value<FnPtr<fn(i32) -> Ptr<i8>>> = Rc::new(RefCell::new(
            FnPtr::<fn(i32) -> Ptr<i8>>::new(|mut _a0: i32| -> Ptr<i8> {
                {
                    return Ptr::<i8>::null();
                }
            }),
        ));
        const _: () = assert!(true, "noexcept(std::invoke_r<char*>(f, 0))");;
        const _: () = assert!(true, "!noexcept(std::invoke_r<char*>(g, 0))");;
        const _: () = assert!(
            true,
            "!noexcept(std::invoke_r<ConversionNotNoexcept>(f, 0))"
        );;
        const _: () = assert!(
            true,
            "!noexcept(std::invoke_r<ConversionNotNoexcept>(g, 0))"
        );;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {},
            || {
                let was_called: Value<bool> = Rc::new(RefCell::new(false));
                let f: Value<FnPtr<fn(i32) -> Ptr<i8>>> = Rc::new(RefCell::new(lambda!(
                    {
                        let was_called: Ptr<bool> = was_called.as_pointer();
                    },
                    |mut _a0: i32| -> Ptr<i8> {
                        was_called.write(true);
                        return Ptr::<i8>::null();
                    }
                )));
                unimplemented!();
                assert!((*was_called.borrow()));
                const _: () = assert!(
                    true,
                    "std::is_void_v<decltype(std::invoke_r<CV_Void>(f, 3))>"
                );;
            },
            || {
                let was_called: Value<bool> = Rc::new(RefCell::new(false));
                let f: Value<FnPtr<fn(i32) -> Ptr<i8>>> = Rc::new(RefCell::new(lambda!(
                    {
                        let was_called: Ptr<bool> = was_called.as_pointer();
                    },
                    |mut _a0: i32| -> Ptr<i8> {
                        was_called.write(true);
                        return Ptr::<i8>::null();
                    }
                )));
                unimplemented!();
                assert!((*was_called.borrow()));
                const _: () = assert!(
                    true,
                    "std::is_void_v<decltype(std::invoke_r<CV_Void>(f, 3))>"
                );;
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
    }
    {
        {
            let was_called: Value<bool> = Rc::new(RefCell::new(false));
            let f: Value<FnPtr<fn(NonCopyable_2)>> = Rc::new(RefCell::new(lambda!(
                {
                    let was_called: Ptr<bool> = was_called.as_pointer();
                },
                |mut _a0: NonCopyable_2| {
                    was_called.write(true);
                }
            )));
            unimplemented!();
            assert!((*was_called.borrow()));
        }
        {
            let was_called: Value<bool> = Rc::new(RefCell::new(false));
            let f: Value<FnPtr<fn(NonCopyable_2) -> i32>> = Rc::new(RefCell::new(lambda!(
                {
                    let was_called: Ptr<bool> = was_called.as_pointer();
                },
                |mut _a0: NonCopyable_2| -> i32 {
                    was_called.write(true);
                    return 0;
                }
            )));
            &(unimplemented!());
            assert!((*was_called.borrow()));
        }
        {
            let was_called: Value<bool> = Rc::new(RefCell::new(false));
            unimplemented!();
            assert!((*was_called.borrow()));
        }
        {
            let was_called: Value<bool> = Rc::new(RefCell::new(false));
            &(unimplemented!());
            assert!((*was_called.borrow()));
        }
    }
    {
        let f: Value<FnPtr<fn() -> Convertible_5>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> Convertible_5>::new(
                || -> Convertible_5 {
                    {
                        return <Convertible_5>::default();
                    }
                },
            )));
        let mut result: i32 = unimplemented!();
        assert!((result == 42));
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i8  >  , new, value)]
#[byte_size(1)]
pub struct ConversionNotNoexcept_1 {}
impl ConversionNotNoexcept_1 {
    pub fn new(mut _a0: Ptr<i8>) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct NonCopyable_2 {}
impl NonCopyable_2 {
    pub fn move_from(_a0: Ptr<NonCopyable_2>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct MoveOnlyVoidFunction_3 {
    #[offset(0)]
    #[byte_size(8)]
    pub was_called: Ptr<bool>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct MoveOnlyIntFunction_4 {
    #[offset(0)]
    #[byte_size(8)]
    pub was_called: Ptr<bool>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i32, to_i32)]
#[byte_size(1)]
pub struct Convertible_5 {}
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
    return 0;
}
pub trait Convertible_5Impl {
    fn to_i32(&self) -> i32;
}
impl Convertible_5Impl for Ptr<Convertible_5> {
    fn to_i32(&self) -> i32 {
        return 42;
    }
}
pub trait MoveOnlyIntFunction_4Impl {
    fn operator_call(&self) -> i32;
}
impl MoveOnlyIntFunction_4Impl for Ptr<MoveOnlyIntFunction_4> {
    fn operator_call(&self) -> i32 {
        (*self).with(|__s| __s.was_called.clone()).write(true);
        return 0;
    }
}
pub trait MoveOnlyVoidFunction_3Impl {
    fn operator_call(&self);
}
impl MoveOnlyVoidFunction_3Impl for Ptr<MoveOnlyVoidFunction_3> {
    fn operator_call(&self) {
        (*self).with(|__s| __s.was_called.clone()).write(true);
    }
}
pub fn __cpp2rust_init_globals() {}
