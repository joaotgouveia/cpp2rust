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
    ({
        cpp2rust_assert_fail_0(
            (({
                let ___str: Value<
                    std_basic_string_char__std_char_traits_char___std_allocator_char__,
                > = Rc::new(RefCell::new(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"0") },
                        None,
                    ),
                ));
                stoull_1(___str.as_pointer(), None, None)
            }) == 0_u64),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            (({
                let ___str: Value<
                    std_basic_string_char__std_char_traits_char___std_allocator_char__,
                > = Rc::new(RefCell::new(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"-0") },
                        None,
                    ),
                ));
                stoull_1(___str.as_pointer(), None, None)
            }) == 0_u64),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            (({
                let ___str: Value<
                    std_basic_string_char__std_char_traits_char___std_allocator_char__,
                > = Rc::new(RefCell::new(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b" 10") },
                        None,
                    ),
                ));
                stoull_1(___str.as_pointer(), None, None)
            }) == 10_u64),
        )
    });
    {
        let idx: Value<u64> = Rc::new(RefCell::new(0_u64));
        ({
            cpp2rust_assert_fail_0(
                (({
                    let ___str: Value<
                        std_basic_string_char__std_char_traits_char___std_allocator_char__,
                    > = Rc::new(RefCell::new(
                        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                            { Ptr::<i8>::from_string_literal(b"10g") },
                            None,
                        ),
                    ));
                    stoull_1(___str.as_pointer(), Some((idx.as_pointer())), Some(16))
                }) == 16_u64),
            )
        });
        ({ cpp2rust_assert_fail_0(((*idx.borrow()) == 2_u64)) });
    }
    ({
        cpp2rust_assert_fail_0(
            (({
                let ___str : Value<std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ > = Rc::new(RefCell::new(std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { Ptr::<i32 >::from_string_literal(&[48 as i32 , 0 as i32 , ] )  } , None , )  ));
                stoull_2(___str.as_pointer(), None, None)
            }) == 0_u64),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            (({
                let ___str : Value<std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ > = Rc::new(RefCell::new(std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { Ptr::<i32 >::from_string_literal(&[45 as i32 , 48 as i32 , 0 as i32 , ] )  } , None , )  ));
                stoull_2(___str.as_pointer(), None, None)
            }) == 0_u64),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            (({
                let ___str : Value<std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ > = Rc::new(RefCell::new(std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { Ptr::<i32 >::from_string_literal(&[32 as i32 , 49 as i32 , 48 as i32 , 0 as i32 , ] )  } , None , )  ));
                stoull_2(___str.as_pointer(), None, None)
            }) == 10_u64),
        )
    });
    {
        let idx: Value<u64> = Rc::new(RefCell::new(0_u64));
        ({
            cpp2rust_assert_fail_0(
                (({
                    let ___str : Value<std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ > = Rc::new(RefCell::new(std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { Ptr::<i32 >::from_string_literal(&[49 as i32 , 48 as i32 , 103 as i32 , 0 as i32 , ] )  } , None , )  ));
                    stoull_2(___str.as_pointer(), Some((idx.as_pointer())), Some(16))
                }) == 16_u64),
            )
        });
        ({ cpp2rust_assert_fail_0(((*idx.borrow()) == 2_u64)) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
