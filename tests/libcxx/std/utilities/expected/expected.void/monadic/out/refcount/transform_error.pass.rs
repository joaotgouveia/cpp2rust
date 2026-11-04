extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NonCopy {
    #[offset(0)]
    pub value: i32,
}
impl NonCopy {
    pub fn new(mut val: i32) -> Self {
        Self { value: val }
    }
}
pub fn test_val_types_0() {
    {
        let l: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> i32 {
                return 1;
            })));
        let v: Value<std_expected_void__int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(2));
            std_unexpected_int_::new({ __tmp_0.as_pointer() })
        }));
        let val: Value<std_expected_void__int_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).transform_error(l.as_pointer()) }),
        ));
        assert!(({ ((({ (*val.borrow()).error_28() }).read()) == 1) }));
    }
    {
        let l: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> i32 {
                return 1;
            })));
        let v: Value<std_expected_void__int_> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
            std_unexpected_int_::new({ __tmp_1.as_pointer() })
        }));
        let val: Value<std_expected_void__int_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).transform_error(l.as_pointer()) }),
        ));
        assert!(({ ((({ (*val.borrow()).error_28() }).read()) == 1) }));
    }
    {
        let l: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> i32 {
                return 1;
            })));
        let v: Value<std_expected_void__int_> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(2));
            std_unexpected_int_::new({ __tmp_2.as_pointer() })
        }));
        let val: Value<std_expected_void__int_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).transform_error(l.as_pointer()) }),
        ));
        assert!(({ ((({ (*val.borrow()).error_28() }).read()) == 1) }));
    }
    {
        let l: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> i32 {
                return 1;
            })));
        let v: Value<std_expected_void__int_> = Rc::new(RefCell::new({
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(2));
            std_unexpected_int_::new({ __tmp_3.as_pointer() })
        }));
        let val: Value<std_expected_void__int_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).transform_error(l.as_pointer()) }),
        ));
        assert!(({ ((({ (*val.borrow()).error_28() }).read()) == 1) }));
    }
}
pub fn test_fail_1() {
    {
        let l: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> i32 {
                assert!(false);
                return 0;
            })));
        let v: Value<std_expected_void__int_> =
            Rc::new(RefCell::new(<std_expected_void__int_>::default()));
        let val: Value<std_expected_void__int_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).transform_error(l.as_pointer()) }),
        ));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
    {
        let l: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> i32 {
                assert!(false);
                return 0;
            })));
        let v: Value<std_expected_void__int_> =
            Rc::new(RefCell::new(<std_expected_void__int_>::default()));
        let val: Value<std_expected_void__int_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).transform_error(l.as_pointer()) }),
        ));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
    {
        let l: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> i32 {
                assert!(false);
                return 0;
            })));
        let v: Value<std_expected_void__int_> =
            Rc::new(RefCell::new(<std_expected_void__int_>::default()));
        let val: Value<std_expected_void__int_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).transform_error(l.as_pointer()) }),
        ));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
    {
        let l: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> i32 {
                assert!(false);
                return 0;
            })));
        let v: Value<std_expected_void__int_> =
            Rc::new(RefCell::new(<std_expected_void__int_>::default()));
        let val: Value<std_expected_void__int_> = Rc::new(RefCell::new(
            ({ (*v.borrow()).transform_error(l.as_pointer()) }),
        ));
        assert!(({ ({ (*val.borrow()).has_value() }) }));
    }
}
pub fn test_direct_non_list_init_2() {
    let x: Value<FnPtr<fn(i32) -> NonCopy>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> NonCopy>::new(|mut i: i32| -> NonCopy {
            {
                return NonCopy::new({ i });
            }
        }),
    ));
    let v: Value<std_expected_void__int_> = Rc::new(RefCell::new({
        let __tmp_4: Value<i32> = Rc::new(RefCell::new(2));
        std_unexpected_int_::new({ __tmp_4.as_pointer() })
    }));
    let nv: Value<std_expected_void__NonCopy_> = Rc::new(RefCell::new(
        ({ (*v.borrow()).transform_error(x.as_pointer()) }),
    ));
    assert!(({ (({ (*nv.borrow()).error_28() }).with(|__s| __s.value) == 2) }));
}
pub fn test_3() -> bool {
    ({ test_fail_1() });
    ({ test_val_types_0() });
    ({ test_direct_non_list_init_2() });
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
    ({ test_3() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
