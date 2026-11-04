extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut fmt: std__Ios_Fmtflags, mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    ({ ((*ss.borrow()) as ::std::fs::File).setf_9(fmt, basefield_1.with(|rc| *rc.borrow())) });
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::showbase_refcount.call(__s)
        };
        write_int_refcount(
            &__s,
            (if value_2.with(|rc| *rc.borrow()) {
                i16::MIN
            } else {
                i16::MAX
            }),
        );
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_0,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_0.upgrade().deref())
                        [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_3(mut fmt: std__Ios_Fmtflags, mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    ({ ((*ss.borrow()) as ::std::fs::File).setf_9(fmt, basefield_1.with(|rc| *rc.borrow())) });
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::showbase_refcount.call(__s)
        };
        write_int_refcount(
            &__s,
            (if value_4.with(|rc| *rc.borrow()) {
                u16::MIN
            } else {
                u16::MAX
            }),
        );
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_1,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_1.upgrade().deref())
                        [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_5(mut fmt: std__Ios_Fmtflags, mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    ({ ((*ss.borrow()) as ::std::fs::File).setf_9(fmt, basefield_1.with(|rc| *rc.borrow())) });
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::showbase_refcount.call(__s)
        };
        write_int_refcount(
            &__s,
            (if value_2.with(|rc| *rc.borrow()) {
                i32::MIN
            } else {
                i32::MAX
            }),
        );
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_2,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_2.upgrade().deref())
                        [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_6(mut fmt: std__Ios_Fmtflags, mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    ({ ((*ss.borrow()) as ::std::fs::File).setf_9(fmt, basefield_1.with(|rc| *rc.borrow())) });
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::showbase_refcount.call(__s)
        };
        write_int_refcount(
            &__s,
            (if value_4.with(|rc| *rc.borrow()) {
                u32::MIN
            } else {
                u32::MAX
            }),
        );
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_3,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_3.upgrade().deref())
                        [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_7(mut fmt: std__Ios_Fmtflags, mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    ({ ((*ss.borrow()) as ::std::fs::File).setf_9(fmt, basefield_1.with(|rc| *rc.borrow())) });
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::showbase_refcount.call(__s)
        };
        write_int_refcount(
            &__s,
            (if value_2.with(|rc| *rc.borrow()) {
                i64::MIN
            } else {
                i64::MAX
            }),
        );
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_4,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_4.upgrade().deref())
                        [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_8(mut fmt: std__Ios_Fmtflags, mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    ({ ((*ss.borrow()) as ::std::fs::File).setf_9(fmt, basefield_1.with(|rc| *rc.borrow())) });
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::showbase_refcount.call(__s)
        };
        write_int_refcount(
            &__s,
            (if value_4.with(|rc| *rc.borrow()) {
                u64::MIN
            } else {
                u64::MAX
            }),
        );
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_5,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_5.upgrade().deref())
                        [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_9(mut fmt: std__Ios_Fmtflags, mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    ({ ((*ss.borrow()) as ::std::fs::File).setf_9(fmt, basefield_1.with(|rc| *rc.borrow())) });
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::showbase_refcount.call(__s)
        };
        write_int_refcount(
            &__s,
            (if value_2.with(|rc| *rc.borrow()) {
                i64::MIN
            } else {
                i64::MAX
            }),
        );
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_6,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_6.upgrade().deref())
                        [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_10(mut fmt: std__Ios_Fmtflags, mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    ({ ((*ss.borrow()) as ::std::fs::File).setf_9(fmt, basefield_1.with(|rc| *rc.borrow())) });
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::showbase_refcount.call(__s)
        };
        write_int_refcount(
            &__s,
            (if value_4.with(|rc| *rc.borrow()) {
                ({ std_numeric_limits_unsigned_long_long_::min() })
            } else {
                ({ std_numeric_limits_unsigned_long_long_::max() })
            }),
        );
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_7,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_7.upgrade().deref())
                        [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
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
    let mut o: std__Ios_Fmtflags = oct_11.with(|rc| *rc.borrow());
    let mut d: std__Ios_Fmtflags = dec_12.with(|rc| *rc.borrow());
    let mut x: std__Ios_Fmtflags = hex_13.with(|rc| *rc.borrow());
    ({ test_0(o, Ptr::<i8>::from_string_literal(b"0100000")) });
    ({ test_0(d, Ptr::<i8>::from_string_literal(b"-32768")) });
    ({ test_0(x, Ptr::<i8>::from_string_literal(b"0x8000")) });
    ({ test_3(o, Ptr::<i8>::from_string_literal(b"0177777")) });
    ({ test_3(d, Ptr::<i8>::from_string_literal(b"65535")) });
    ({ test_3(x, Ptr::<i8>::from_string_literal(b"0xffff")) });
    ({ test_5(o, Ptr::<i8>::from_string_literal(b"020000000000")) });
    ({ test_5(d, Ptr::<i8>::from_string_literal(b"-2147483648")) });
    ({ test_5(x, Ptr::<i8>::from_string_literal(b"0x80000000")) });
    ({ test_6(o, Ptr::<i8>::from_string_literal(b"037777777777")) });
    ({ test_6(d, Ptr::<i8>::from_string_literal(b"4294967295")) });
    ({ test_6(x, Ptr::<i8>::from_string_literal(b"0xffffffff")) });
    let mut long_is_32: bool = value_4.with(|rc| *rc.borrow());
    let mut long_is_64: bool = value_2.with(|rc| *rc.borrow());
    let mut long_long_is_64: bool = value_2.with(|rc| *rc.borrow());
    {
        {
            ({
                test_7(
                    o,
                    Ptr::<i8>::from_string_literal(b"01000000000000000000000"),
                )
            });
            ({ test_7(d, Ptr::<i8>::from_string_literal(b"-9223372036854775808")) });
            ({ test_7(x, Ptr::<i8>::from_string_literal(b"0x8000000000000000")) });
            ({
                test_8(
                    o,
                    Ptr::<i8>::from_string_literal(b"01777777777777777777777"),
                )
            });
            ({ test_8(d, Ptr::<i8>::from_string_literal(b"18446744073709551615")) });
            ({ test_8(x, Ptr::<i8>::from_string_literal(b"0xffffffffffffffff")) });
        }
    }
    {
        ({
            test_9(
                o,
                Ptr::<i8>::from_string_literal(b"01000000000000000000000"),
            )
        });
        ({ test_9(d, Ptr::<i8>::from_string_literal(b"-9223372036854775808")) });
        ({ test_9(x, Ptr::<i8>::from_string_literal(b"0x8000000000000000")) });
        ({
            test_10(
                o,
                Ptr::<i8>::from_string_literal(b"01777777777777777777777"),
            )
        });
        ({ test_10(d, Ptr::<i8>::from_string_literal(b"18446744073709551615")) });
        ({ test_10(x, Ptr::<i8>::from_string_literal(b"0xffffffffffffffff")) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
