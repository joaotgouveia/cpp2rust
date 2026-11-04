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
        assert!(
            (({
                let ___args: Value<i32> = Rc::new(RefCell::new(4));
                invoke_r_1(f.as_pointer(), ___args.as_pointer())
            }) == 7)
        );
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
                ({
                    let ___args: Value<i32> = Rc::new(RefCell::new(3));
                    invoke_r_3(f.as_pointer(), ___args.as_pointer())
                });
                assert!(({ (*was_called.borrow()) }));
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
                ({
                    let ___args: Value<i32> = Rc::new(RefCell::new(3));
                    invoke_r_4(f.as_pointer(), ___args.as_pointer())
                });
                assert!(({ (*was_called.borrow()) }));
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
            let f: Value<FnPtr<fn(NonCopyable_5)>> = Rc::new(RefCell::new(lambda!(
                {
                    let was_called: Ptr<bool> = was_called.as_pointer();
                },
                |mut _a0: NonCopyable_5| {
                    was_called.write(true);
                }
            )));
            ({
                let ___args: Value<NonCopyable_5> =
                    Rc::new(RefCell::new(<NonCopyable_5>::default()));
                invoke_r_6(f.as_pointer(), ___args.as_pointer())
            });
            assert!(({ (*was_called.borrow()) }));
        }
        {
            let was_called: Value<bool> = Rc::new(RefCell::new(false));
            let f: Value<FnPtr<fn(NonCopyable_5) -> i32>> = Rc::new(RefCell::new(lambda!(
                {
                    let was_called: Ptr<bool> = was_called.as_pointer();
                },
                |mut _a0: NonCopyable_5| -> i32 {
                    was_called.write(true);
                    return 0;
                }
            )));
            &({
                let ___args: Value<NonCopyable_5> =
                    Rc::new(RefCell::new(<NonCopyable_5>::default()));
                invoke_r_7(f.as_pointer(), ___args.as_pointer())
            });
            assert!(({ (*was_called.borrow()) }));
        }
        {
            let was_called: Value<bool> = Rc::new(RefCell::new(false));
            ({
                let ___fn: Value<MoveOnlyVoidFunction_8> =
                    Rc::new(RefCell::new(MoveOnlyVoidFunction_8 {
                        was_called: was_called.as_pointer(),
                    }));
                invoke_r_9(___fn.as_pointer())
            });
            assert!(({ (*was_called.borrow()) }));
        }
        {
            let was_called: Value<bool> = Rc::new(RefCell::new(false));
            &({
                let ___fn: Value<MoveOnlyIntFunction_10> =
                    Rc::new(RefCell::new(MoveOnlyIntFunction_10 {
                        was_called: was_called.as_pointer(),
                    }));
                invoke_r_11(___fn.as_pointer())
            });
            assert!(({ (*was_called.borrow()) }));
        }
    }
    {
        let f: Value<FnPtr<fn() -> Convertible_12>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> Convertible_12>::new(
                || -> Convertible_12 {
                    {
                        return <Convertible_12>::default();
                    }
                },
            )));
        let mut result: i32 = ({ invoke_r_13(f.as_pointer()) });
        assert!((result == 42));
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i8  >  , new, value)]
#[byte_size(1)]
pub struct ConversionNotNoexcept_2 {}
impl ConversionNotNoexcept_2 {
    pub fn new(mut _a0: Ptr<i8>) -> Self {
        Self {}
    }
}
impl CppMove for ConversionNotNoexcept_2 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct NonCopyable_5 {}
impl NonCopyable_5 {
    pub fn move_from(_a0: Ptr<NonCopyable_5>) -> Self {
        Self {}
    }
}
impl CppMove for NonCopyable_5 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct MoveOnlyVoidFunction_8 {
    #[offset(0)]
    #[byte_size(8)]
    pub was_called: Ptr<bool>,
}
impl Callable0<()> for MoveOnlyVoidFunction_8 {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            MoveOnlyVoidFunction_8Impl::operator_call(__it)
        })
    }
}
impl CppMove for MoveOnlyVoidFunction_8 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct MoveOnlyIntFunction_10 {
    #[offset(0)]
    #[byte_size(8)]
    pub was_called: Ptr<bool>,
}
impl Callable0<i32> for MoveOnlyIntFunction_10 {
    fn call(&self) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            MoveOnlyIntFunction_10Impl::operator_call(__it)
        })
    }
}
impl CppMove for MoveOnlyIntFunction_10 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i32, to_i32)]
#[byte_size(1)]
pub struct Convertible_12 {}
impl CppMove for Convertible_12 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait Convertible_12Impl {
    fn to_i32(&self) -> i32;
}
impl Convertible_12Impl for Ptr<Convertible_12> {
    fn to_i32(&self) -> i32 {
        return 42;
    }
}
pub trait MoveOnlyIntFunction_10Impl {
    fn operator_call(&self) -> i32;
}
impl MoveOnlyIntFunction_10Impl for Ptr<MoveOnlyIntFunction_10> {
    fn operator_call(&self) -> i32 {
        (*self).with(|__s| __s.was_called.clone()).write(true);
        return 0;
    }
}
pub trait MoveOnlyVoidFunction_8Impl {
    fn operator_call(&self);
}
impl MoveOnlyVoidFunction_8Impl for Ptr<MoveOnlyVoidFunction_8> {
    fn operator_call(&self) {
        (*self).with(|__s| __s.was_called.clone()).write(true);
    }
}
pub fn __cpp2rust_init_globals() {}
