extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_padding_0() {
    {
        let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(
                ),
            ));
        write!(
            (ss.as_pointer() as ::std::fs::File),
            "{:}{:10}{:}",
            FnPtr::<fn(Ptr<::std::fs::File>) -> Ptr<::std::fs::File>>::new(libcc2rs::left_refcount),
            ({ setfill_1(('!' as i8),) }),
            ({
                quoted_2(
                    Ptr::<i8>::from_string_literal(b"abc"),
                    Some(('`' as i8)),
                    None,
                )
            }),
        );
        assert!(
            match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_0,) => {
                    Ptr::<i8>::from_string_literal(b"`abc`!!!!!").with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(
                ),
            ));
        write!(
            (ss.as_pointer() as ::std::fs::File),
            "{:}{:10}{:}",
            FnPtr::<fn(Ptr<::std::fs::File>) -> Ptr<::std::fs::File>>::new(
                libcc2rs::right_refcount
            ),
            ({ setfill_1(('!' as i8),) }),
            ({
                quoted_2(
                    Ptr::<i8>::from_string_literal(b"abc"),
                    Some(('`' as i8)),
                    None,
                )
            }),
        );
        assert!(
            match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_1,) => {
                    Ptr::<i8>::from_string_literal(b"!!!!!`abc`").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
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
    ({ both_ways_3(Ptr::<i8>::from_string_literal(b"")) });
    ({ round_trip_4(Ptr::<i8>::from_string_literal(b"")) });
    ({ round_trip_ws_5(Ptr::<i8>::from_string_literal(b"")) });
    ({ round_trip_d_6(Ptr::<i8>::from_string_literal(b""), ('q' as i8)) });
    ({ round_trip_e_7(Ptr::<i8>::from_string_literal(b""), ('q' as i8)) });
    ({ round_trip_8(Ptr::<i32>::from_string_literal(&[0 as i32])) });
    ({ round_trip_ws_9(Ptr::<i32>::from_string_literal(&[0 as i32])) });
    ({ round_trip_d_10(Ptr::<i32>::from_string_literal(&[0 as i32]), ('q' as i8)) });
    ({ round_trip_e_11(Ptr::<i32>::from_string_literal(&[0 as i32]), ('q' as i8)) });
    ({ round_trip_4(Ptr::<i8>::from_string_literal(b"Hi")) });
    ({ round_trip_ws_5(Ptr::<i8>::from_string_literal(b"Hi")) });
    ({ round_trip_d_6(Ptr::<i8>::from_string_literal(b"Hi"), ('!' as i8)) });
    ({ round_trip_e_7(Ptr::<i8>::from_string_literal(b"Hi"), ('!' as i8)) });
    assert!(match (Rc::new(RefCell::new(
        ({
            quote_12(
                Ptr::<i8>::from_string_literal(b"Hi"),
                Some(('!' as i8)),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_2,) => {
            Ptr::<i8>::from_string_literal(b"!Hi!").with_c_str(|__s| {
                (*__tmp_2.upgrade().deref())[..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            quote_12(
                Ptr::<i8>::from_string_literal(b"Hi!"),
                Some(('!' as i8)),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_3,) => {
            Ptr::<i8>::from_string_literal(b"!Hi\\!!").with_c_str(|__s| {
                (*__tmp_3.upgrade().deref())[..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    ({
        round_trip_8(Ptr::<i32>::from_string_literal(&[
            72 as i32, 105 as i32, 0 as i32,
        ]))
    });
    ({
        round_trip_ws_9(Ptr::<i32>::from_string_literal(&[
            72 as i32, 105 as i32, 0 as i32,
        ]))
    });
    ({
        round_trip_d_10(
            Ptr::<i32>::from_string_literal(&[72 as i32, 105 as i32, 0 as i32]),
            ('!' as i8),
        )
    });
    ({
        round_trip_e_11(
            Ptr::<i32>::from_string_literal(&[72 as i32, 105 as i32, 0 as i32]),
            ('!' as i8),
        )
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            quote_13(
                Ptr::<i32>::from_string_literal(&[72 as i32, 105 as i32, 0 as i32]),
                Some(('!' as i8)),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_4,) => {
            Ptr::<i32>::from_string_literal(&[
                33 as i32, 72 as i32, 105 as i32, 33 as i32, 0 as i32,
            ])
            .with_c_str(|__s| {
                (*__tmp_4.upgrade().deref())[..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            quote_13(
                Ptr::<i32>::from_string_literal(&[72 as i32, 105 as i32, 33 as i32, 0 as i32]),
                Some(('!' as i8)),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_5,) => {
            Ptr::<i32>::from_string_literal(&[
                33 as i32, 72 as i32, 105 as i32, 92 as i32, 33 as i32, 33 as i32, 0 as i32,
            ])
            .with_c_str(|__s| {
                (*__tmp_5.upgrade().deref())[..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    ({ round_trip_4(Ptr::<i8>::from_string_literal(b"Hi Mom")) });
    ({ round_trip_ws_5(Ptr::<i8>::from_string_literal(b"Hi Mom")) });
    ({
        round_trip_8(Ptr::<i32>::from_string_literal(&[
            72 as i32, 105 as i32, 32 as i32, 77 as i32, 111 as i32, 109 as i32, 0 as i32,
        ]))
    });
    ({
        round_trip_ws_9(Ptr::<i32>::from_string_literal(&[
            72 as i32, 105 as i32, 32 as i32, 77 as i32, 111 as i32, 109 as i32, 0 as i32,
        ]))
    });
    assert!(match (Rc::new(RefCell::new(
        ({ quote_12(Ptr::<i8>::from_string_literal(b""), None, None,) })
    ))
    .as_pointer(),)
    {
        (__tmp_6,) => {
            Ptr::<i8>::from_string_literal(b"\"\"").with_c_str(|__s| {
                (*__tmp_6.upgrade().deref())[..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({ quote_12(Ptr::<i8>::from_string_literal(b"a"), None, None,) })
    ))
    .as_pointer(),)
    {
        (__tmp_7,) => {
            Ptr::<i8>::from_string_literal(b"\"a\"").with_c_str(|__s| {
                (*__tmp_7.upgrade().deref())[..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({ quote_13(Ptr::<i32>::from_string_literal(&[0 as i32,]), None, None,) })
    ))
    .as_pointer(),)
    {
        (__tmp_8,) => {
            Ptr::<i32>::from_string_literal(&[34 as i32, 34 as i32, 0 as i32]).with_c_str(|__s| {
                (*__tmp_8.upgrade().deref())[..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            quote_13(
                Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32]),
                None,
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_9,) => {
            Ptr::<i32>::from_string_literal(&[34 as i32, 97 as i32, 34 as i32, 0 as i32])
                .with_c_str(|__s| {
                    (*__tmp_9.upgrade().deref())
                        [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({ unquote_14(Ptr::<i8>::from_string_literal(b"\"abc"), None, None,) })
    ))
    .as_pointer(),)
    {
        (__tmp_10,) => {
            Ptr::<i8>::from_string_literal(b"abc").with_c_str(|__s| {
                (*__tmp_10.upgrade().deref())
                    [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            unquote_15(
                Ptr::<i32>::from_string_literal(&[
                    34 as i32, 97 as i32, 98 as i32, 99 as i32, 0 as i32,
                ]),
                None,
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_11,) => {
            Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32])
                .with_c_str(|__s| {
                    (*__tmp_11.upgrade().deref())
                        [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({ unquote_14(Ptr::<i8>::from_string_literal(b"abc"), None, None,) })
    ))
    .as_pointer(),)
    {
        (__tmp_12,) => {
            Ptr::<i8>::from_string_literal(b"abc").with_c_str(|__s| {
                (*__tmp_12.upgrade().deref())
                    [..(*__tmp_12.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({ unquote_14(Ptr::<i8>::from_string_literal(b"abc def"), None, None,) })
    ))
    .as_pointer(),)
    {
        (__tmp_13,) => {
            Ptr::<i8>::from_string_literal(b"abc").with_c_str(|__s| {
                (*__tmp_13.upgrade().deref())
                    [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            unquote_15(
                Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32]),
                None,
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_14,) => {
            Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32])
                .with_c_str(|__s| {
                    (*__tmp_14.upgrade().deref())
                        [..(*__tmp_14.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            unquote_15(
                Ptr::<i32>::from_string_literal(&[
                    97 as i32, 98 as i32, 99 as i32, 32 as i32, 100 as i32, 101 as i32, 102 as i32,
                    0 as i32,
                ]),
                None,
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_15,) => {
            Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32])
                .with_c_str(|__s| {
                    (*__tmp_15.upgrade().deref())
                        [..(*__tmp_15.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({ unquote_14(Ptr::<i8>::from_string_literal(b""), None, None,) })
    ))
    .as_pointer(),)
    {
        (__tmp_16,) => {
            Ptr::<i8>::from_string_literal(b"").with_c_str(|__s| {
                (*__tmp_16.upgrade().deref())
                    [..(*__tmp_16.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({ unquote_15(Ptr::<i32>::from_string_literal(&[0 as i32,]), None, None,) })
    ))
    .as_pointer(),)
    {
        (__tmp_17,) => {
            Ptr::<i32>::from_string_literal(&[0 as i32]).with_c_str(|__s| {
                (*__tmp_17.upgrade().deref())
                    [..(*__tmp_17.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    ({ test_padding_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
