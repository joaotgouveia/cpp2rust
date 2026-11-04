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
        let target: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"foo").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"foo$") },
                None,
            ),
        ));
        assert!(
            ({
                regex_match_0(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    None,
                )
            })
        );
        assert!(
            !({
                regex_match_0(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    Some(match_not_eol_1.with(|rc| *rc.borrow())),
                )
            })
        );
    }
    {
        let target: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"foo").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"foo") },
                None,
            ),
        ));
        assert!(
            ({
                regex_match_0(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    None,
                )
            })
        );
        assert!(
            ({
                regex_match_0(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    Some(match_not_eol_1.with(|rc| *rc.borrow())),
                )
            })
        );
    }
    {
        let target: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"refoo").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"foo$") },
                None,
            ),
        ));
        assert!(
            ({
                regex_search_2(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    None,
                )
            })
        );
        assert!(
            !({
                regex_search_2(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    Some(match_not_eol_1.with(|rc| *rc.borrow())),
                )
            })
        );
    }
    {
        let target: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"refoo").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"foo") },
                None,
            ),
        ));
        assert!(
            ({
                regex_search_2(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    None,
                )
            })
        );
        assert!(
            ({
                regex_search_2(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    Some(match_not_eol_1.with(|rc| *rc.borrow())),
                )
            })
        );
    }
    {
        let target: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"foo").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"$") },
                None,
            ),
        ));
        assert!(
            ({
                regex_search_2(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    None,
                )
            })
        );
        assert!(
            !({
                regex_search_2(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    Some(match_not_eol_1.with(|rc| *rc.borrow())),
                )
            })
        );
        let match_ : Value<std_match_results___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_sub_match___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char______ > = Rc::new(RefCell::new(<std_match_results___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_sub_match___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char______  >::default() )) ;
        assert!(
            ({
                regex_search_3(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    match_.as_pointer(),
                    re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*match_.borrow()).position(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*match_.borrow()).length(0_u64,) }) == 0_i64) }));
        assert!(
            !({
                regex_search_3(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    match_.as_pointer(),
                    re.as_pointer(),
                    Some(match_not_eol_1.with(|rc| *rc.borrow())),
                )
            })
        );
        assert!(({ (({ (*match_.borrow()).length(0_u64,) }) == 0_i64) }));
    }
    {
        let target: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"foo").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"$") },
                { Some(multiline_4.with(|rc| *rc.borrow())) },
            ),
        ));
        let match_ : Value<std_match_results___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_sub_match___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char______ > = Rc::new(RefCell::new(<std_match_results___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_sub_match___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char______  >::default() )) ;
        assert!(
            ({
                regex_search_3(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    match_.as_pointer(),
                    re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*match_.borrow()).position(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*match_.borrow()).length(0_u64,) }) == 0_i64) }));
        assert!(
            !({
                regex_search_3(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    match_.as_pointer(),
                    re.as_pointer(),
                    Some(match_not_eol_1.with(|rc| *rc.borrow())),
                )
            })
        );
        assert!(({ (({ (*match_.borrow()).length(0_u64,) }) == 0_i64) }));
    }
    {
        let target: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"foo").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"$") },
                None,
            ),
        ));
        assert!(
            !({
                regex_match_0(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    None,
                )
            })
        );
        assert!(
            !({
                regex_match_0(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    Some(match_not_eol_1.with(|rc| *rc.borrow())),
                )
            })
        );
    }
    {
        let target: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"^b*$") },
                None,
            ),
        ));
        assert!(
            !({
                regex_search_2(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    None,
                )
            })
        );
        assert!(
            !({
                regex_search_2(
                    (target.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                    re.as_pointer(),
                    Some(match_not_eol_1.with(|rc| *rc.borrow())),
                )
            })
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
