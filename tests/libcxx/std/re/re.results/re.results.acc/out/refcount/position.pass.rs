extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcdefghijk\0")));
    ({
        cpp2rust_assert_fail_1(
            ({
                let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"cd((e)fg)hi") },
                        None,
                    )),
                );
                regex_search_2(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ({ (*m.borrow()).position(None) }) } == {
                ({
                    distance_3((s.as_pointer() as Ptr<i8>), {
                        ((*m.borrow())[0_u64] as std_pair_const_char_ptr__const_char_ptr_)
                            .first
                            .clone()
                    })
                })
            }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ({ (*m.borrow()).position(Some(0_u64)) }) } == {
                ({
                    distance_3((s.as_pointer() as Ptr<i8>), {
                        ((*m.borrow())[0_u64] as std_pair_const_char_ptr__const_char_ptr_)
                            .first
                            .clone()
                    })
                })
            }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ({ (*m.borrow()).position(Some(1_u64)) }) } == {
                ({
                    distance_3((s.as_pointer() as Ptr<i8>), {
                        ((*m.borrow())[1_u64] as std_pair_const_char_ptr__const_char_ptr_)
                            .first
                            .clone()
                    })
                })
            }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ({ (*m.borrow()).position(Some(2_u64)) }) } == {
                ({
                    distance_3((s.as_pointer() as Ptr<i8>), {
                        ((*m.borrow())[2_u64] as std_pair_const_char_ptr__const_char_ptr_)
                            .first
                            .clone()
                    })
                })
            }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ({ (*m.borrow()).position(Some(3_u64)) }) } == {
                ({
                    distance_3((s.as_pointer() as Ptr<i8>), {
                        ((*m.borrow())[3_u64] as std_pair_const_char_ptr__const_char_ptr_)
                            .first
                            .clone()
                    })
                })
            }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ({ (*m.borrow()).position(Some(4_u64)) }) } == {
                ({
                    distance_3((s.as_pointer() as Ptr<i8>), {
                        ((*m.borrow())[4_u64] as std_pair_const_char_ptr__const_char_ptr_)
                            .first
                            .clone()
                    })
                })
            }),
        )
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
    ({ test_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
