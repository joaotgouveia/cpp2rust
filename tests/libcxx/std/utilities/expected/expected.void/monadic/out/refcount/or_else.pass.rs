extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_val_types_0() {
    {
        let l: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {},
            |mut _a0: i32| -> std_expected_void__long_ {
                return <std_expected_void__long_>::default();
            }
        )));
        let v: Value<std_expected_void__int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
            std_unexpected_int_::new({ __tmp_0.as_pointer() })
        }));
        let val: Value<std_expected_void__long_> =
            Rc::new(RefCell::new(({ (*v.borrow()).or_else(l.as_pointer()) })));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
    {
        let l: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {},
            |mut _a0: i32| -> std_expected_void__long_ {
                return <std_expected_void__long_>::default();
            }
        )));
        let v: Value<std_expected_void__int_> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(1));
            std_unexpected_int_::new({ __tmp_1.as_pointer() })
        }));
        let val: Value<std_expected_void__long_> =
            Rc::new(RefCell::new(({ (*v.borrow()).or_else(l.as_pointer()) })));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
    {
        let l: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {},
            |mut _a0: i32| -> std_expected_void__long_ {
                return <std_expected_void__long_>::default();
            }
        )));
        let v: Value<std_expected_void__int_> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(1));
            std_unexpected_int_::new({ __tmp_2.as_pointer() })
        }));
        let val: Value<std_expected_void__long_> =
            Rc::new(RefCell::new(({ (*v.borrow()).or_else(l.as_pointer()) })));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
    {
        let l: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {},
            |mut _a0: i32| -> std_expected_void__long_ {
                return <std_expected_void__long_>::default();
            }
        )));
        let v: Value<std_expected_void__int_> = Rc::new(RefCell::new({
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(1));
            std_unexpected_int_::new({ __tmp_3.as_pointer() })
        }));
        let val: Value<std_expected_void__long_> =
            Rc::new(RefCell::new(({ (*v.borrow()).or_else(l.as_pointer()) })));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
}
pub fn test_fail_1() {
    let never_called: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut _a0: i32| -> std_expected_void__long_ {
            assert!(false);
            return {
                let __tmp_4: Value<i32> = Rc::new(RefCell::new(5));
                std_unexpected_long_::new({ __tmp_4.as_pointer() })
            };
        }
    )));
    {
        let v: Value<std_expected_void__int_> =
            Rc::new(RefCell::new(<std_expected_void__int_>::default()));
        let val: Value<std_expected_void__long_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).or_else(never_called.as_pointer()) }),
        ));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
    {
        let v: Value<std_expected_void__int_> =
            Rc::new(RefCell::new(<std_expected_void__int_>::default()));
        let val: Value<std_expected_void__long_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).or_else(never_called.as_pointer()) }),
        ));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
    {
        let v: Value<std_expected_void__int_> =
            Rc::new(RefCell::new(<std_expected_void__int_>::default()));
        let val: Value<std_expected_void__long_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).or_else(never_called.as_pointer()) }),
        ));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
    {
        let v: Value<std_expected_void__int_> =
            Rc::new(RefCell::new(<std_expected_void__int_>::default()));
        let val: Value<std_expected_void__long_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).or_else(never_called.as_pointer()) }),
        ));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
}
pub fn test_2() -> bool {
    ({ test_fail_1() });
    ({ test_val_types_0() });
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
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
