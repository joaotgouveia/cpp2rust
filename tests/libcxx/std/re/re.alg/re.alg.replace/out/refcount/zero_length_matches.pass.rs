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
    assert!(match (Rc::new(RefCell::new(
        ({
            let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
                std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"") },
                    None,
                ),
            ));
            regex_replace_0(
                Ptr::<i8>::from_string_literal(b"abc"),
                ___e.as_pointer(),
                Ptr::<i8>::from_string_literal(b"!"),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_0,) => {
            Ptr::<i8>::from_string_literal(b"!a!b!c!").with_c_str(|__s| {
                (*__tmp_0.upgrade().deref())[..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
                std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"X*") },
                    None,
                ),
            ));
            regex_replace_0(
                Ptr::<i8>::from_string_literal(b"abc"),
                ___e.as_pointer(),
                Ptr::<i8>::from_string_literal(b"!"),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_1,) => {
            Ptr::<i8>::from_string_literal(b"!a!b!c!").with_c_str(|__s| {
                (*__tmp_1.upgrade().deref())[..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
                std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"X{0,3}") },
                    None,
                ),
            ));
            regex_replace_0(
                Ptr::<i8>::from_string_literal(b"abc"),
                ___e.as_pointer(),
                Ptr::<i8>::from_string_literal(b"!"),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_2,) => {
            Ptr::<i8>::from_string_literal(b"!a!b!c!").with_c_str(|__s| {
                (*__tmp_2.upgrade().deref())[..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
                std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"") },
                    None,
                ),
            ));
            regex_replace_0(
                Ptr::<i8>::from_string_literal(b"abc"),
                ___e.as_pointer(),
                Ptr::<i8>::from_string_literal(b"[!]"),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_3,) => {
            Ptr::<i8>::from_string_literal(b"[!]a[!]b[!]c[!]").with_c_str(|__s| {
                (*__tmp_3.upgrade().deref())[..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
                std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"") },
                    None,
                ),
            ));
            regex_replace_0(
                Ptr::<i8>::from_string_literal(b"abc"),
                ___e.as_pointer(),
                Ptr::<i8>::from_string_literal(b""),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_4,) => {
            Ptr::<i8>::from_string_literal(b"abc").with_c_str(|__s| {
                (*__tmp_4.upgrade().deref())[..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
                std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"") },
                    None,
                ),
            ));
            regex_replace_0(
                Ptr::<i8>::from_string_literal(b""),
                ___e.as_pointer(),
                Ptr::<i8>::from_string_literal(b"!"),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_5,) => {
            Ptr::<i8>::from_string_literal(b"!").with_c_str(|__s| {
                (*__tmp_5.upgrade().deref())[..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(match (Rc::new(RefCell::new(
        ({
            let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
                std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"C*") },
                    None,
                ),
            ));
            regex_replace_0(
                Ptr::<i8>::from_string_literal(b"abCabCa"),
                ___e.as_pointer(),
                Ptr::<i8>::from_string_literal(b"!"),
                None,
            )
        })
    ))
    .as_pointer(),)
    {
        (__tmp_6,) => {
            Ptr::<i8>::from_string_literal(b"!a!b!!a!b!!a!").with_c_str(|__s| {
                (*__tmp_6.upgrade().deref())[..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
