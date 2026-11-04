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
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcdefghijk\0")));
        assert!(
            ({
                let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"cd((e)fg)hi") },
                        None,
                    )),
                );
                regex_search_0(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
            b"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2\0",
        )));
        let mut out: Vec<i8> = ({ (*m.borrow()).format((fmt.as_pointer() as Ptr<i8>), None) });
        assert!(
            Ptr::<i8>::from_string_literal(
                b"prefix: ab, match: cdefghi, suffix: jk, m[1]: efg, m[2]: e"
            )
            .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcdefghijk\0")));
        assert!(
            ({
                let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"cd((e)fg)hi") },
                        None,
                    )),
                );
                regex_search_0(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
            b"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2\0",
        )));
        let mut out: Vec<i8> = ({
            (*m.borrow()).format(
                (fmt.as_pointer() as Ptr<i8>),
                Some(format_sed_1.with(|rc| *rc.borrow())),
            )
        });
        assert!(
            Ptr::<i8>::from_string_literal(
                b"prefix: $`, match: $cdefghi, suffix: $\', m[1]: $1, m[2]: $2"
            )
            .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcdefghijk\0")));
        assert!(
            ({
                let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"cd((e)fg)hi") },
                        None,
                    )),
                );
                regex_search_0(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
            b"match: &, m[1]: \\1, m[2]: \\2\0",
        )));
        let mut out: Vec<i8> = ({
            (*m.borrow()).format(
                (fmt.as_pointer() as Ptr<i8>),
                Some(format_sed_1.with(|rc| *rc.borrow())),
            )
        });
        assert!(
            Ptr::<i8>::from_string_literal(b"match: cdefghi, m[1]: efg, m[2]: e")
                .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___e: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                99 as i32, 100 as i32, 40 as i32, 40 as i32, 101 as i32, 41 as i32,
                                102 as i32, 103 as i32, 41 as i32, 104 as i32, 105 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_search_2(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
            32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
            116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32, 38 as i32,
            44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32,
            120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32,
            109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32,
            49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32,
            58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
        ])));
        let mut out: Vec<i32> = ({ (*m.borrow()).format((fmt.as_pointer() as Ptr<i32>), None) });
        assert!(
            Ptr::<i32>::from_string_literal(&[
                112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                32 as i32, 97 as i32, 98 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
                116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 99 as i32, 100 as i32,
                101 as i32, 102 as i32, 103 as i32, 104 as i32, 105 as i32, 44 as i32, 32 as i32,
                115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                32 as i32, 106 as i32, 107 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32,
                49 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32, 102 as i32, 103 as i32,
                44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32, 58 as i32,
                32 as i32, 101 as i32, 0 as i32,
            ])
            .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___e: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                99 as i32, 100 as i32, 40 as i32, 40 as i32, 101 as i32, 41 as i32,
                                102 as i32, 103 as i32, 41 as i32, 104 as i32, 105 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_search_2(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
            32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
            116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32, 38 as i32,
            44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32,
            120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32,
            109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32,
            49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32,
            58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
        ])));
        let mut out: Vec<i32> = ({
            (*m.borrow()).format(
                (fmt.as_pointer() as Ptr<i32>),
                Some(format_sed_1.with(|rc| *rc.borrow())),
            )
        });
        assert!(
            Ptr::<i32>::from_string_literal(&[
                112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
                116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32, 99 as i32,
                100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32, 105 as i32, 44 as i32,
                32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32, 120 as i32,
                58 as i32, 32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32, 109 as i32,
                91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32, 49 as i32,
                44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32, 58 as i32,
                32 as i32, 36 as i32, 50 as i32, 0 as i32,
            ])
            .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___e: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                99 as i32, 100 as i32, 40 as i32, 40 as i32, 101 as i32, 41 as i32,
                                102 as i32, 103 as i32, 41 as i32, 104 as i32, 105 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_search_2(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32,
            38 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32,
            58 as i32, 32 as i32, 92 as i32, 49 as i32, 44 as i32, 32 as i32, 109 as i32,
            91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 92 as i32, 50 as i32, 0 as i32,
        ])));
        let mut out: Vec<i32> = ({
            (*m.borrow()).format(
                (fmt.as_pointer() as Ptr<i32>),
                Some(format_sed_1.with(|rc| *rc.borrow())),
            )
        });
        assert!(
            Ptr::<i32>::from_string_literal(&[
                109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32,
                99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32, 105 as i32,
                44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32,
                32 as i32, 101 as i32, 102 as i32, 103 as i32, 44 as i32, 32 as i32, 109 as i32,
                91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32, 0 as i32,
            ])
            .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
