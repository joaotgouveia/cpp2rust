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
        let f: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut i: ()| -> () {
                {
                    return (i + 3_());
                }
            },
        )));
        unimplemented!();
    }
    {
        let f: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(
            FnPtr::<fn(()) -> Ptr<()>>::new(|mut _a0: ()| -> Ptr<()> {
                {
                    return Ptr::<()>::null();
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
        let f: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(
            FnPtr::<fn(()) -> Ptr<()>>::new(|mut _a0: ()| -> Ptr<()> {
                {
                    return Ptr::<()>::null();
                }
            }),
        ));
        let g: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(
            FnPtr::<fn(()) -> Ptr<()>>::new(|mut _a0: ()| -> Ptr<()> {
                {
                    return Ptr::<()>::null();
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
                let was_called: Value<()> = Rc::new(RefCell::new(false));
                let f: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(lambda!(
                    {
                        let was_called: Ptr<()> = was_called.as_pointer();
                    },
                    |mut _a0: ()| -> Ptr<()> {
                        was_called.write(true);
                        return Ptr::<()>::null();
                    }
                )));
                unimplemented!();
                unimplemented!();
                const _: () = assert!(
                    true,
                    "std::is_void_v<decltype(std::invoke_r<CV_Void>(f, 3))>"
                );;
            },
            || {
                let was_called: Value<()> = Rc::new(RefCell::new(false));
                let f: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(lambda!(
                    {
                        let was_called: Ptr<()> = was_called.as_pointer();
                    },
                    |mut _a0: ()| -> Ptr<()> {
                        was_called.write(true);
                        return Ptr::<()>::null();
                    }
                )));
                unimplemented!();
                unimplemented!();
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
            let was_called: Value<()> = Rc::new(RefCell::new(false));
            let f: Value<FnPtr<fn(NonCopyable_2)>> = Rc::new(RefCell::new(lambda!(
                {
                    let was_called: Ptr<()> = was_called.as_pointer();
                },
                |mut _a0: NonCopyable_2| {
                    was_called.write(true);
                }
            )));
            unimplemented!();
            unimplemented!();
        }
        {
            let was_called: Value<()> = Rc::new(RefCell::new(false));
            let f: Value<FnPtr<fn(NonCopyable_2) -> ()>> = Rc::new(RefCell::new(lambda!(
                {
                    let was_called: Ptr<()> = was_called.as_pointer();
                },
                |mut _a0: NonCopyable_2| -> () {
                    was_called.write(true);
                    return 0_();
                }
            )));
            &(unimplemented!());
            unimplemented!();
        }
        {
            let was_called: Value<()> = Rc::new(RefCell::new(false));
            unimplemented!();
            unimplemented!();
        }
        {
            let was_called: Value<()> = Rc::new(RefCell::new(false));
            &(unimplemented!());
            unimplemented!();
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
        let mut result: () = unimplemented!();
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConversionNotNoexcept_1 {}
impl ConversionNotNoexcept_1 {
    pub fn new(mut _a0: Ptr<()>) -> Self {
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
    pub was_called: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct MoveOnlyIntFunction_4 {
    #[offset(0)]
    #[byte_size(8)]
    pub was_called: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait Convertible_5Impl {
    fn to_(&self) -> ();
}
impl Convertible_5Impl for Ptr<Convertible_5> {
    fn to_(&self) -> () {
        return 42_();
    }
}
pub trait MoveOnlyIntFunction_4Impl {
    fn operator_call(&self) -> ();
}
impl MoveOnlyIntFunction_4Impl for Ptr<MoveOnlyIntFunction_4> {
    fn operator_call(&self) -> () {
        (*self).with(|__s| __s.was_called.clone()).write(true);
        return 0_();
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
