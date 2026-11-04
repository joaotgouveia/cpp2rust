extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct My2 {}
impl CppMove for My2 {
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
    {
        let f: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut x: i32| -> i32 {
                return ((10 * x) + 9);
            })));
        let place: Value<My2> = Rc::new(RefCell::new(<My2>::default()));
        let bound: Value<std__Bind__lambda_at_specialization_pass_cpp_30_14___My2__> = Rc::new(
            RefCell::new(({ bind_0(f.as_pointer(), place.as_pointer()) })),
        );
        ({
            cpp2rust_assert_fail_1(
                (({
                    let ___args_0: Value<i32> = Rc::new(RefCell::new(7));
                    let ___args_1: Value<i32> = Rc::new(RefCell::new(8));
                    (*bound.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
                }) == 89),
            )
        });
    }
    {
        let f: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut x: i32| -> i32 {
                return ((10 * x) + 9);
            })));
        let place: Value<My2> = Rc::new(RefCell::new(<My2>::default()));
        let bound: Value<std__Bind__lambda_at_specialization_pass_cpp_36_14___My2__> = Rc::new(
            RefCell::new(({ bind_2(f.as_pointer(), place.as_pointer()) })),
        );
        ({
            cpp2rust_assert_fail_1(
                (({
                    let ___args_0: Value<i32> = Rc::new(RefCell::new(7));
                    let ___args_1: Value<i32> = Rc::new(RefCell::new(8));
                    (*bound.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
                }) == 89),
            )
        });
    }
    {
        let f: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut x: i32| -> i32 {
                return ((10 * x) + 9);
            })));
        let place: Value<My2> = Rc::new(RefCell::new(<My2>::default()));
        let bound: Value<std__Bind__lambda_at_specialization_pass_cpp_42_14___My2__> = Rc::new(
            RefCell::new(({ bind_3(f.as_pointer(), place.as_pointer()) })),
        );
        ({
            cpp2rust_assert_fail_1(
                (({
                    let ___args_0: Value<i32> = Rc::new(RefCell::new(7));
                    let ___args_1: Value<i32> = Rc::new(RefCell::new(8));
                    (*bound.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
                }) == 89),
            )
        });
    }
    {
        let f: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut x: i32| -> i32 {
                return ((10 * x) + 9);
            })));
        let place: Value<My2> = Rc::new(RefCell::new(<My2>::default()));
        let bound: Value<std__Bind__lambda_at_specialization_pass_cpp_48_14___My2__> = Rc::new(
            RefCell::new(({ bind_4(f.as_pointer(), place.as_pointer()) })),
        );
        ({
            cpp2rust_assert_fail_1(
                (({
                    let ___args_0: Value<i32> = Rc::new(RefCell::new(7));
                    let ___args_1: Value<i32> = Rc::new(RefCell::new(8));
                    (*bound.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
                }) == 89),
            )
        });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
