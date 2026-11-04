extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn validate_prefixes_0(
    empty_matching_pattern: Ptr<std_basic_regex_char__std_regex_traits_char__>,
) {
    let source: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let i: Value<std_regex_iterator_const_char_ptr__char__std_regex_traits_char__> =
        Rc::new(RefCell::new(
            std_regex_iterator_const_char_ptr__char__std_regex_traits_char__::new_1(
                { (source.as_pointer() as Ptr<i8>) },
                { (source.as_pointer() as Ptr<i8>).offset((3) as isize) },
                { (empty_matching_pattern).clone() },
                None,
            ),
        ));
    assert!(
        ({
            !(({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                .with(|__s| __s.matched))
        })
    );
    assert!(
        ({
            (({
                (*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                    .upgrade()
                    .deref())
                .length()
            }) == 0_i64)
        })
    );
    assert!(
        ({
            ({
                (*({
                    ((*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                        .upgrade()
                        .deref()) as (Value<*const i8>, Value<*const i8>))
                })
                .0
                .borrow())
                .clone()
            } == { (source.as_pointer() as Ptr<i8>) })
        })
    );
    assert!(
        ({
            ({
                (*({
                    ((*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                        .upgrade()
                        .deref()) as (Value<*const i8>, Value<*const i8>))
                })
                .1
                .borrow())
                .clone()
            } == { (source.as_pointer() as Ptr<i8>) })
        })
    );
    {
        i.as_pointer().with_mut(|__it| {
            __it.iter_inc();
        });
        i.as_pointer()
    };
    assert!(
        ({
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() }).with(|__s| __s.matched)
        })
    );
    assert!(
        ({
            (({
                (*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                    .upgrade()
                    .deref())
                .length()
            }) == 1_i64)
        })
    );
    assert!(
        ({
            ({
                (*({
                    ((*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                        .upgrade()
                        .deref()) as (Value<*const i8>, Value<*const i8>))
                })
                .0
                .borrow())
                .clone()
            } == { (source.as_pointer() as Ptr<i8>) })
        })
    );
    assert!(
        ({
            ({
                (*({
                    ((*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                        .upgrade()
                        .deref()) as (Value<*const i8>, Value<*const i8>))
                })
                .1
                .borrow())
                .clone()
            } == { (source.as_pointer() as Ptr<i8>).offset((1) as isize) })
        })
    );
    assert!(match (Rc::new(RefCell::new(
        ({
            (*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                .upgrade()
                .deref())
            .str()
        })
    ))
    .as_pointer(),)
    {
        (__tmp_0,) => {
            Ptr::<i8>::from_string_literal(b"a").with_c_str(|__s| {
                (*__tmp_0.upgrade().deref())[..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    {
        i.as_pointer().with_mut(|__it| {
            __it.iter_inc();
        });
        i.as_pointer()
    };
    assert!(
        ({
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() }).with(|__s| __s.matched)
        })
    );
    assert!(
        ({
            (({
                (*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                    .upgrade()
                    .deref())
                .length()
            }) == 1_i64)
        })
    );
    assert!(
        ({
            ({
                (*({
                    ((*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                        .upgrade()
                        .deref()) as (Value<*const i8>, Value<*const i8>))
                })
                .0
                .borrow())
                .clone()
            } == { (source.as_pointer() as Ptr<i8>).offset((1) as isize) })
        })
    );
    assert!(
        ({
            ({
                (*({
                    ((*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                        .upgrade()
                        .deref()) as (Value<*const i8>, Value<*const i8>))
                })
                .1
                .borrow())
                .clone()
            } == { (source.as_pointer() as Ptr<i8>).offset((2) as isize) })
        })
    );
    assert!(match (Rc::new(RefCell::new(
        ({
            (*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                .upgrade()
                .deref())
            .str()
        })
    ))
    .as_pointer(),)
    {
        (__tmp_1,) => {
            Ptr::<i8>::from_string_literal(b"b").with_c_str(|__s| {
                (*__tmp_1.upgrade().deref())[..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    {
        i.as_pointer().with_mut(|__it| {
            __it.iter_inc();
        });
        i.as_pointer()
    };
    assert!(
        ({
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() }).with(|__s| __s.matched)
        })
    );
    assert!(
        ({
            (({
                (*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                    .upgrade()
                    .deref())
                .length()
            }) == 1_i64)
        })
    );
    assert!(
        ({
            ({
                (*({
                    ((*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                        .upgrade()
                        .deref()) as (Value<*const i8>, Value<*const i8>))
                })
                .0
                .borrow())
                .clone()
            } == { (source.as_pointer() as Ptr<i8>).offset((2) as isize) })
        })
    );
    assert!(
        ({
            ({
                (*({
                    ((*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                        .upgrade()
                        .deref()) as (Value<*const i8>, Value<*const i8>))
                })
                .1
                .borrow())
                .clone()
            } == { (source.as_pointer() as Ptr<i8>).offset((3) as isize) })
        })
    );
    assert!(match (Rc::new(RefCell::new(
        ({
            (*({ (*(*i.borrow()).iter_deref().upgrade().deref()).prefix() })
                .upgrade()
                .deref())
            .str()
        })
    ))
    .as_pointer(),)
    {
        (__tmp_2,) => {
            Ptr::<i8>::from_string_literal(b"c").with_c_str(|__s| {
                (*__tmp_2.upgrade().deref())[..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    {
        i.as_pointer().with_mut(|__it| {
            __it.iter_inc();
        });
        i.as_pointer()
    };
    assert!(
        (*i.borrow())
            == <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
    );
}
pub fn test_prefix_adjustment_1() {
    ({
        let _empty_matching_pattern: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
            RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"") },
                None,
            )),
        );
        validate_prefixes_0(_empty_matching_pattern.as_pointer())
    });
    ({
        let _empty_matching_pattern: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
            RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"X*") },
                None,
            )),
        );
        validate_prefixes_0(_empty_matching_pattern.as_pointer())
    });
    ({
        let _empty_matching_pattern: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
            RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"X{0,3}") },
                None,
            )),
        );
        validate_prefixes_0(_empty_matching_pattern.as_pointer())
    });
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
        let phone_numbers: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
            RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"\\d{3}-\\d{4}") },
                None,
            )),
        );
        let phone_book: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
            b"555-1234, 555-2345, 555-3456\0",
        )));
        let i: Value<std_regex_iterator_const_char_ptr__char__std_regex_traits_char__> =
            Rc::new(RefCell::new(
                std_regex_iterator_const_char_ptr__char__std_regex_traits_char__::new_1(
                    { ({ begin_2(phone_book.as_pointer()) }) },
                    { (phone_book.as_pointer() as Ptr<i8>).to_end() },
                    { phone_numbers.as_pointer() },
                    None,
                ),
            ));
        let i2: Value<std_regex_iterator_const_char_ptr__char__std_regex_traits_char__> =
            Rc::new(RefCell::new((*i.borrow()).clone()));
        assert!(
            (*i.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(
            (*i2.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 0_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_3,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_3.upgrade().deref())
                        [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 0_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i2.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_4,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_4.upgrade().deref())
                        [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        (*i.borrow_mut()).iter_post_inc();
        assert!(
            (*i.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(
            (*i2.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 10_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_5,) => {
                Ptr::<i8>::from_string_literal(b"555-2345").with_c_str(|__s| {
                    (*__tmp_5.upgrade().deref())
                        [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 0_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i2.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_6,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_6.upgrade().deref())
                        [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        (*i.borrow_mut()).iter_post_inc();
        assert!(
            (*i.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(
            (*i2.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 20_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_7,) => {
                Ptr::<i8>::from_string_literal(b"555-3456").with_c_str(|__s| {
                    (*__tmp_7.upgrade().deref())
                        [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 0_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i2.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_8,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_8.upgrade().deref())
                        [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        (*i.borrow_mut()).iter_post_inc();
        assert!(
            (*i.borrow())
                == <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(
            (*i2.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 0_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i2.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_9,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_9.upgrade().deref())
                        [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
    }
    {
        let phone_numbers: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
            RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"\\d{3}-\\d{4}") },
                None,
            )),
        );
        let phone_book: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
            b"555-1234, 555-2345, 555-3456\0",
        )));
        let i: Value<std_regex_iterator_const_char_ptr__char__std_regex_traits_char__> =
            Rc::new(RefCell::new(
                std_regex_iterator_const_char_ptr__char__std_regex_traits_char__::new_1(
                    { ({ begin_2(phone_book.as_pointer()) }) },
                    { (phone_book.as_pointer() as Ptr<i8>).to_end() },
                    { phone_numbers.as_pointer() },
                    None,
                ),
            ));
        let i2: Value<std_regex_iterator_const_char_ptr__char__std_regex_traits_char__> =
            Rc::new(RefCell::new((*i.borrow()).clone()));
        assert!(
            (*i.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(
            (*i2.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 0_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_10,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_10.upgrade().deref())
                        [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 0_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i2.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_11,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_11.upgrade().deref())
                        [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(
            (*i.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(
            (*i2.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 10_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_12,) => {
                Ptr::<i8>::from_string_literal(b"555-2345").with_c_str(|__s| {
                    (*__tmp_12.upgrade().deref())
                        [..(*__tmp_12.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 0_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i2.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_13,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_13.upgrade().deref())
                        [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(
            (*i.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(
            (*i2.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 20_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_14,) => {
                Ptr::<i8>::from_string_literal(b"555-3456").with_c_str(|__s| {
                    (*__tmp_14.upgrade().deref())
                        [..(*__tmp_14.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 0_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i2.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_15,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_15.upgrade().deref())
                        [..(*__tmp_15.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(
            (*i.borrow())
                == <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(
            (*i2.borrow())
                != <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()
        );
        assert!(({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i2.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 0_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i2.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_16,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_16.upgrade().deref())
                        [..(*__tmp_16.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
    }
    {
        let rex: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b".*") },
                None,
            ),
        ));
        let foo: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"foo\0")));
        let i: Value<std_regex_iterator_const_char_ptr__char__std_regex_traits_char__> =
            Rc::new(RefCell::new(
                std_regex_iterator_const_char_ptr__char__std_regex_traits_char__::new_1(
                    { ({ begin_3(foo.as_pointer()) }) },
                    {
                        (foo.as_pointer() as Ptr<i8>)
                            .to_end()
                            .offset(-((1) as isize))
                    },
                    { rex.as_pointer() },
                    None,
                ),
            ));
        let e: Value<std_regex_iterator_const_char_ptr__char__std_regex_traits_char__> =
            Rc::new(RefCell::new(
                <std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default(),
            ));
        assert!((*i.borrow()) != (*e.borrow()));
        assert!(({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_17,) => {
                Ptr::<i8>::from_string_literal(b"foo").with_c_str(|__s| {
                    (*__tmp_17.upgrade().deref())
                        [..(*__tmp_17.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!((*i.borrow()) != (*e.borrow()));
        assert!(({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_18,) => {
                Ptr::<i8>::from_string_literal(b"").with_c_str(|__s| {
                    (*__tmp_18.upgrade().deref())
                        [..(*__tmp_18.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (*i.borrow()) == (*e.borrow()) }));
    }
    ({ test_prefix_adjustment_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
