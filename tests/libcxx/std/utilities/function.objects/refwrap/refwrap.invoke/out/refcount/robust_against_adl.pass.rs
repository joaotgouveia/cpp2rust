extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn no_args_0() -> Ptr<Holder_Incomplete_> {
    return Ptr::<Holder_Incomplete_>::null();
}
pub fn one_arg_1(p: Ptr<Holder_Incomplete_>) -> Ptr<Holder_Incomplete_> {
    let p: Value<Ptr<Holder_Incomplete_>> = Rc::new(RefCell::new(p));
    return (*p.borrow()).clone();
}
pub fn two_args_2(
    p: Ptr<Holder_Incomplete_>,
    mut _a1: Ptr<Holder_Incomplete_>,
) -> Ptr<Holder_Incomplete_> {
    let p: Value<Ptr<Holder_Incomplete_>> = Rc::new(RefCell::new(p));
    return (*p.borrow()).clone();
}
pub fn three_args_3(
    p: Ptr<Holder_Incomplete_>,
    mut _a1: Ptr<Holder_Incomplete_>,
    mut _a2: Ptr<Holder_Incomplete_>,
) -> Ptr<Holder_Incomplete_> {
    let p: Value<Ptr<Holder_Incomplete_>> = Rc::new(RefCell::new(p));
    return (*p.borrow()).clone();
}
pub fn one_arg_void_4(mut _a0: Ptr<Holder_Incomplete_>) {}
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
    let x: Value<Ptr<Holder_Incomplete_>> =
        Rc::new(RefCell::new(Ptr::<Holder_Incomplete_>::null()));
    let cx: Value<Ptr<Holder_Incomplete_>> =
        Rc::new(RefCell::new(Ptr::<Holder_Incomplete_>::null()));
    &({ ({ ref_5(FnPtr::<fn() -> Ptr<Holder_Incomplete_>>::new(no_args_0)) })() });
    &({
        ({
            ref_6(FnPtr::<
                fn(Ptr<Holder_Incomplete_>) -> Ptr<Holder_Incomplete_>,
            >::new(one_arg_1))
        })(x.as_pointer())
    });
    &({
        ({
            ref_6(FnPtr::<
                fn(Ptr<Holder_Incomplete_>) -> Ptr<Holder_Incomplete_>,
            >::new(one_arg_1))
        })(cx.as_pointer())
    });
    &({
        let ___args_0: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        let ___args_1: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        ({
            ref_7(FnPtr::<
                fn(Ptr<Holder_Incomplete_>, Ptr<Holder_Incomplete_>) -> Ptr<Holder_Incomplete_>,
            >::new(two_args_2))
        })(___args_0, ___args_1)
    });
    &({
        ({
            ref_7(FnPtr::<
                fn(Ptr<Holder_Incomplete_>, Ptr<Holder_Incomplete_>) -> Ptr<Holder_Incomplete_>,
            >::new(two_args_2))
        })(x.as_pointer(), cx.as_pointer())
    });
    &({
        ({
            ref_7(FnPtr::<
                fn(Ptr<Holder_Incomplete_>, Ptr<Holder_Incomplete_>) -> Ptr<Holder_Incomplete_>,
            >::new(two_args_2))
        })(cx.as_pointer(), x.as_pointer())
    });
    &({
        let ___args_0: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        let ___args_1: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        ({
            ref_7(FnPtr::<
                fn(Ptr<Holder_Incomplete_>, Ptr<Holder_Incomplete_>) -> Ptr<Holder_Incomplete_>,
            >::new(two_args_2))
        })(___args_0, ___args_1)
    });
    &({
        let ___args_0: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        let ___args_1: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        let ___args_2: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        ({
            ref_8(FnPtr::<
                fn(
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                ) -> Ptr<Holder_Incomplete_>,
            >::new(three_args_3))
        })(___args_0, ___args_1, ___args_2)
    });
    &({
        let ___args_0: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        let ___args_1: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        ({
            ref_8(FnPtr::<
                fn(
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                ) -> Ptr<Holder_Incomplete_>,
            >::new(three_args_3))
        })(___args_0, ___args_1, cx.as_pointer())
    });
    &({
        let ___args_0: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        let ___args_2: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        ({
            ref_8(FnPtr::<
                fn(
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                ) -> Ptr<Holder_Incomplete_>,
            >::new(three_args_3))
        })(___args_0, cx.as_pointer(), ___args_2)
    });
    &({
        let ___args_1: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        let ___args_2: Ptr<Ptr<Holder_Incomplete_>> = x.as_pointer();
        ({
            ref_8(FnPtr::<
                fn(
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                ) -> Ptr<Holder_Incomplete_>,
            >::new(three_args_3))
        })(cx.as_pointer(), ___args_1, ___args_2)
    });
    &({
        let ___args_1: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        let ___args_2: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        ({
            ref_8(FnPtr::<
                fn(
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                ) -> Ptr<Holder_Incomplete_>,
            >::new(three_args_3))
        })(x.as_pointer(), ___args_1, ___args_2)
    });
    &({
        let ___args_0: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        let ___args_2: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        ({
            ref_8(FnPtr::<
                fn(
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                ) -> Ptr<Holder_Incomplete_>,
            >::new(three_args_3))
        })(___args_0, x.as_pointer(), ___args_2)
    });
    &({
        let ___args_0: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        let ___args_1: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        ({
            ref_8(FnPtr::<
                fn(
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                ) -> Ptr<Holder_Incomplete_>,
            >::new(three_args_3))
        })(___args_0, ___args_1, x.as_pointer())
    });
    &({
        let ___args_0: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        let ___args_1: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        let ___args_2: Ptr<Ptr<Holder_Incomplete_>> = cx.as_pointer();
        ({
            ref_8(FnPtr::<
                fn(
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                    Ptr<Holder_Incomplete_>,
                ) -> Ptr<Holder_Incomplete_>,
            >::new(three_args_3))
        })(___args_0, ___args_1, ___args_2)
    });
    &({ ({ ref_9(FnPtr::<fn(Ptr<Holder_Incomplete_>)>::new(one_arg_void_4)) })(x.as_pointer()) });
    &({ ({ ref_9(FnPtr::<fn(Ptr<Holder_Incomplete_>)>::new(one_arg_void_4)) })(cx.as_pointer()) });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Holder_Incomplete_;
pub fn __cpp2rust_init_globals() {}
