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
                    { ({ begin_0(phone_book.as_pointer()) }) },
                    { (phone_book.as_pointer() as Ptr<i8>).to_end() },
                    { phone_numbers.as_pointer() },
                    None,
                ),
            ));
        assert!(
            (*i.borrow())
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
            (__tmp_0,) => {
                Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| {
                    (*__tmp_0.upgrade().deref())
                        [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
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
        assert!(({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 10_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_1,) => {
                Ptr::<i8>::from_string_literal(b"555-2345").with_c_str(|__s| {
                    (*__tmp_1.upgrade().deref())
                        [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
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
        assert!(({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).size() }) == 1_u64) }));
        assert!(
            ({ (({ (*(*i.borrow()).iter_deref().upgrade().deref()).position(None,) }) == 20_i64) })
        );
        assert!(match (Rc::new(RefCell::new(
            ({ (*(*i.borrow()).iter_deref().upgrade().deref()).str(None,) })
        ))
        .as_pointer(),)
        {
            (__tmp_2,) => {
                Ptr::<i8>::from_string_literal(b"555-3456").with_c_str(|__s| {
                    (*__tmp_2.upgrade().deref())
                        [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
