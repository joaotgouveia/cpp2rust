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
        let target: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"foo") },
                    None,
                ),
            ));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"foo$") },
                None,
            ),
        ));
        ({
            cpp2rust_assert_fail_0(({ regex_match_1(target.as_pointer(), re.as_pointer(), None) }))
        });
        ({
            cpp2rust_assert_fail_0(
                !({
                    regex_match_1(
                        target.as_pointer(),
                        re.as_pointer(),
                        Some(match_not_eol_2.with(|rc| *rc.borrow())),
                    )
                }),
            )
        });
    }
    {
        let target: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"foo") },
                    None,
                ),
            ));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"foo") },
                None,
            ),
        ));
        ({
            cpp2rust_assert_fail_0(({ regex_match_1(target.as_pointer(), re.as_pointer(), None) }))
        });
        ({
            cpp2rust_assert_fail_0(
                ({
                    regex_match_1(
                        target.as_pointer(),
                        re.as_pointer(),
                        Some(match_not_eol_2.with(|rc| *rc.borrow())),
                    )
                }),
            )
        });
    }
    {
        let target: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"refoo") },
                    None,
                ),
            ));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"foo$") },
                None,
            ),
        ));
        ({
            cpp2rust_assert_fail_0(({ regex_search_3(target.as_pointer(), re.as_pointer(), None) }))
        });
        ({
            cpp2rust_assert_fail_0(
                !({
                    regex_search_3(
                        target.as_pointer(),
                        re.as_pointer(),
                        Some(match_not_eol_2.with(|rc| *rc.borrow())),
                    )
                }),
            )
        });
    }
    {
        let target: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"refoo") },
                    None,
                ),
            ));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"foo") },
                None,
            ),
        ));
        ({
            cpp2rust_assert_fail_0(({ regex_search_3(target.as_pointer(), re.as_pointer(), None) }))
        });
        ({
            cpp2rust_assert_fail_0(
                ({
                    regex_search_3(
                        target.as_pointer(),
                        re.as_pointer(),
                        Some(match_not_eol_2.with(|rc| *rc.borrow())),
                    )
                }),
            )
        });
    }
    {
        let target: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"foo") },
                    None,
                ),
            ));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"$") },
                None,
            ),
        ));
        ({
            cpp2rust_assert_fail_0(({ regex_search_3(target.as_pointer(), re.as_pointer(), None) }))
        });
        ({
            cpp2rust_assert_fail_0(
                !({
                    regex_search_3(
                        target.as_pointer(),
                        re.as_pointer(),
                        Some(match_not_eol_2.with(|rc| *rc.borrow())),
                    )
                }),
            )
        });
        let match_ : Value<std_match_results___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_sub_match___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char______ > = Rc::new(RefCell::new(<std_match_results___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_sub_match___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char______  >::default() )) ;
        ({
            cpp2rust_assert_fail_0(
                ({
                    regex_search_4(
                        target.as_pointer(),
                        match_.as_pointer(),
                        re.as_pointer(),
                        None,
                    )
                }),
            )
        });
        ({ cpp2rust_assert_fail_0((({ (*match_.borrow()).position(0_u64) }) == 3_i64)) });
        ({ cpp2rust_assert_fail_0((({ (*match_.borrow()).length(0_u64) }) == 0_i64)) });
        ({
            cpp2rust_assert_fail_0(
                !({
                    regex_search_4(
                        target.as_pointer(),
                        match_.as_pointer(),
                        re.as_pointer(),
                        Some(match_not_eol_2.with(|rc| *rc.borrow())),
                    )
                }),
            )
        });
        ({ cpp2rust_assert_fail_0((({ (*match_.borrow()).length(0_u64) }) == 0_i64)) });
    }
    {
        let target: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"foo") },
                    None,
                ),
            ));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"$") },
                { Some(multiline_5.with(|rc| *rc.borrow())) },
            ),
        ));
        let match_ : Value<std_match_results___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_sub_match___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char______ > = Rc::new(RefCell::new(<std_match_results___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_sub_match___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char______  >::default() )) ;
        ({
            cpp2rust_assert_fail_0(
                ({
                    regex_search_4(
                        target.as_pointer(),
                        match_.as_pointer(),
                        re.as_pointer(),
                        None,
                    )
                }),
            )
        });
        ({ cpp2rust_assert_fail_0((({ (*match_.borrow()).position(0_u64) }) == 3_i64)) });
        ({ cpp2rust_assert_fail_0((({ (*match_.borrow()).length(0_u64) }) == 0_i64)) });
        ({
            cpp2rust_assert_fail_0(
                !({
                    regex_search_4(
                        target.as_pointer(),
                        match_.as_pointer(),
                        re.as_pointer(),
                        Some(match_not_eol_2.with(|rc| *rc.borrow())),
                    )
                }),
            )
        });
        ({ cpp2rust_assert_fail_0((({ (*match_.borrow()).length(0_u64) }) == 0_i64)) });
    }
    {
        let target: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"foo") },
                    None,
                ),
            ));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"$") },
                None,
            ),
        ));
        ({
            cpp2rust_assert_fail_0(!({ regex_match_1(target.as_pointer(), re.as_pointer(), None) }))
        });
        ({
            cpp2rust_assert_fail_0(
                !({
                    regex_match_1(
                        target.as_pointer(),
                        re.as_pointer(),
                        Some(match_not_eol_2.with(|rc| *rc.borrow())),
                    )
                }),
            )
        });
    }
    {
        let target: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"a") },
                    None,
                ),
            ));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"^b*$") },
                None,
            ),
        ));
        ({
            cpp2rust_assert_fail_0(
                !({ regex_search_3(target.as_pointer(), re.as_pointer(), None) }),
            )
        });
        ({
            cpp2rust_assert_fail_0(
                !({
                    regex_search_3(
                        target.as_pointer(),
                        re.as_pointer(),
                        Some(match_not_eol_2.with(|rc| *rc.borrow())),
                    )
                }),
            )
        });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
