extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
        let buf1: Value<std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"testing").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__::new_1(
                    { __tmp_0.as_pointer() },
                    None,
                )
            }));
        let buf: Value<std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                <std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__>::default(),
            ));
        cpp_swap!(buf.as_pointer(), buf1.as_pointer());
        assert!(
            match (Rc::new(RefCell::new(({ (*buf.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_1,) => {
                    Ptr::<i8>::from_string_literal(b"testing").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(({ (*buf1.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_2,) => {
                    Ptr::<i8>::from_string_literal(b"").with_c_str(|__s| {
                        (*__tmp_2.upgrade().deref())
                            [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let buf1: Value<std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new({
                let __tmp_3: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"testing").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__::new_1(
                    { __tmp_3.as_pointer() },
                    { Some(8_i32) },
                )
            }));
        let buf: Value<std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                <std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__>::default(),
            ));
        cpp_swap!(buf.as_pointer(), buf1.as_pointer());
        assert!(
            match (Rc::new(RefCell::new(({ (*buf.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_4,) => {
                    Ptr::<i8>::from_string_literal(b"testing").with_c_str(|__s| {
                        (*__tmp_4.upgrade().deref())
                            [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(({ (*buf1.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_5,) => {
                    Ptr::<i8>::from_string_literal(b"").with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let buf1: Value<std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new({
                let __tmp_6: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"testing").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__::new_1(
                    { __tmp_6.as_pointer() },
                    { Some(16_i32) },
                )
            }));
        let buf: Value<std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                <std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__>::default(),
            ));
        cpp_swap!(buf.as_pointer(), buf1.as_pointer());
        assert!(
            match (Rc::new(RefCell::new(({ (*buf.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_7,) => {
                    Ptr::<i8>::from_string_literal(b"testing").with_c_str(|__s| {
                        (*__tmp_7.upgrade().deref())
                            [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(({ (*buf1.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_8,) => {
                    Ptr::<i8>::from_string_literal(b"").with_c_str(|__s| {
                        (*__tmp_8.upgrade().deref())
                            [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let buf1: Value<
            std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = Rc::new(RefCell::new({
            let __tmp_9: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i32>::from_string_literal(&[
                    116 as i32, 101 as i32, 115 as i32, 116 as i32, 105 as i32, 110 as i32,
                    103 as i32, 0 as i32,
                ])
                .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                { __tmp_9.as_pointer() },
                None,
            )
        }));
        let buf : Value<std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ > = Rc::new(RefCell::new(<std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__  >::default() )) ;
        cpp_swap!(buf.as_pointer(), buf1.as_pointer());
        assert!(
            match (Rc::new(RefCell::new(({ (*buf.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_10,) => {
                    Ptr::<i32>::from_string_literal(&[
                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 105 as i32, 110 as i32,
                        103 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_10.upgrade().deref())
                            [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(({ (*buf1.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_11,) => {
                    Ptr::<i32>::from_string_literal(&[0 as i32]).with_c_str(|__s| {
                        (*__tmp_11.upgrade().deref())
                            [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let buf1: Value<
            std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = Rc::new(RefCell::new({
            let __tmp_12: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i32>::from_string_literal(&[
                    116 as i32, 101 as i32, 115 as i32, 116 as i32, 105 as i32, 110 as i32,
                    103 as i32, 0 as i32,
                ])
                .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                { __tmp_12.as_pointer() },
                { Some(8_i32) },
            )
        }));
        let buf : Value<std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ > = Rc::new(RefCell::new(<std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__  >::default() )) ;
        cpp_swap!(buf.as_pointer(), buf1.as_pointer());
        assert!(
            match (Rc::new(RefCell::new(({ (*buf.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_13,) => {
                    Ptr::<i32>::from_string_literal(&[
                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 105 as i32, 110 as i32,
                        103 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_13.upgrade().deref())
                            [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(({ (*buf1.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_14,) => {
                    Ptr::<i32>::from_string_literal(&[0 as i32]).with_c_str(|__s| {
                        (*__tmp_14.upgrade().deref())
                            [..(*__tmp_14.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let buf1: Value<
            std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = Rc::new(RefCell::new({
            let __tmp_15: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i32>::from_string_literal(&[
                    116 as i32, 101 as i32, 115 as i32, 116 as i32, 105 as i32, 110 as i32,
                    103 as i32, 0 as i32,
                ])
                .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                { __tmp_15.as_pointer() },
                { Some(16_i32) },
            )
        }));
        let buf : Value<std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ > = Rc::new(RefCell::new(<std_basic_stringbuf_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__  >::default() )) ;
        cpp_swap!(buf.as_pointer(), buf1.as_pointer());
        assert!(
            match (Rc::new(RefCell::new(({ (*buf.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_16,) => {
                    Ptr::<i32>::from_string_literal(&[
                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 105 as i32, 110 as i32,
                        103 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_16.upgrade().deref())
                            [..(*__tmp_16.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(({ (*buf1.borrow()).str_17() }))).as_pointer(),) {
                (__tmp_17,) => {
                    Ptr::<i32>::from_string_literal(&[0 as i32]).with_c_str(|__s| {
                        (*__tmp_17.upgrade().deref())
                            [..(*__tmp_17.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
