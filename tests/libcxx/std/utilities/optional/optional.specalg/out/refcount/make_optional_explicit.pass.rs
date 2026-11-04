extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static can_make_optional_explicit_impl_0: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_impl_1: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_impl_2: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_impl_3: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_impl_4: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_impl_5: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_impl_6: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_impl_7: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_8: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_9: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_10: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_11: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_12: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_13: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_14: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_15: Value<bool> = Rc::new(RefCell::new(false));
);
const _: () = assert!(true, "can_make_optional_explicit<int, long>");
const _: () = assert!(true, "can_make_optional_explicit<int, const long&>");
const _: () = assert!(true, "can_make_optional_explicit<std::string, const char*>");
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::string, const char* const&>"
);
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::unique_ptr<const int>, std::unique_ptr<int>>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::unique_ptr<const int>, const std::unique_ptr<int>>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::unique_ptr<const int>, std::unique_ptr<int>&>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::unique_ptr<const int>, const std::unique_ptr<int>&>"
);
pub fn test_ref_16() {
    let i: Value<i32> = Rc::new(RefCell::new(0));
    let opt: Value<std_optional_int_> =
        Rc::new(RefCell::new(({ make_optional_17(i.as_pointer()) })));
    const _: () = assert!(true, "");;
    assert!(({ (((*opt.borrow()).read()) == 0) }));
}
pub fn test_ref_18() {
    let i: Value<f64> = Rc::new(RefCell::new(0_f64));
    let opt: Value<std_optional_double_> =
        Rc::new(RefCell::new(({ make_optional_19(i.as_pointer()) })));
    const _: () = assert!(true, "");;
    assert!(({ (((*opt.borrow()).read()) == 0_f64) }));
}
pub fn test_20() -> bool {
    {
        let opt: Value<std_optional_int_> = Rc::new(RefCell::new(
            ({
                let ___args: Value<i8> = Rc::new(RefCell::new(('a' as i8)));
                make_optional_21(___args.as_pointer())
            }),
        ));
        const _: () = assert!(true, "*opt == int(\'a\')");;
    }
    let test_string: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        {
            let s: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            let opt: Value<
                std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char___,
            > = Rc::new(RefCell::new(
                ({ make_optional_22((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) }),
            ));
            assert!(
                ({
                    match (opt.as_pointer(),) {
                        (__tmp_0,) => Ptr::<i8>::from_string_literal(b"123").with_c_str(|__s| {
                            (*__tmp_0.upgrade().deref())
                                [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        }),
                    }
                })
            );
        }
        {
            let opt: Value<
                std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char___,
            > = Rc::new(RefCell::new(
                ({
                    let ___args_0: Value<u32> = Rc::new(RefCell::new(4_u32));
                    let ___args_1: Value<i8> = Rc::new(RefCell::new(('X' as i8)));
                    make_optional_23(___args_0.as_pointer(), ___args_1.as_pointer())
                }),
            ));
            assert!(
                ({
                    match (opt.as_pointer(),) {
                        (__tmp_1,) => Ptr::<i8>::from_string_literal(b"XXXX").with_c_str(|__s| {
                            (*__tmp_1.upgrade().deref())
                                [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        }),
                    }
                })
            );
        }
    })));
    {
        ({ (*test_string.borrow()).call() });
    }
    let test_unique_ptr: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        let s: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(3)))));
        let opt: Value<std_optional_std_unique_ptr_int__std_default_delete_int___> =
            Rc::new(RefCell::new(({ make_optional_24(s.as_pointer()) })));
        assert!(({ (((*(*opt.borrow()).upgrade().deref()).as_pointer().read()) == 3) }));
        assert!(({ (*s.borrow()).is_none() }));
    })));
    {
        ({ (*test_unique_ptr.borrow()).call() });
    }
    ({ test_ref_16() });
    ({ test_ref_18() });
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
    ({ test_20() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = can_make_optional_explicit_impl_0.with(|_| ());
    let _ = can_make_optional_explicit_impl_1.with(|_| ());
    let _ = can_make_optional_explicit_impl_2.with(|_| ());
    let _ = can_make_optional_explicit_impl_3.with(|_| ());
    let _ = can_make_optional_explicit_impl_4.with(|_| ());
    let _ = can_make_optional_explicit_impl_5.with(|_| ());
    let _ = can_make_optional_explicit_impl_6.with(|_| ());
    let _ = can_make_optional_explicit_impl_7.with(|_| ());
    let _ = can_make_optional_explicit_8.with(|_| ());
    let _ = can_make_optional_explicit_9.with(|_| ());
    let _ = can_make_optional_explicit_10.with(|_| ());
    let _ = can_make_optional_explicit_11.with(|_| ());
    let _ = can_make_optional_explicit_12.with(|_| ());
    let _ = can_make_optional_explicit_13.with(|_| ());
    let _ = can_make_optional_explicit_14.with(|_| ());
    let _ = can_make_optional_explicit_15.with(|_| ());
}
