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
        let inf1: Value<std_basic_istringstream_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"abc").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                std_basic_istringstream_char__std_char_traits_char___std_allocator_char__::new_1(
                    { __tmp_0.as_pointer() },
                    None,
                )
            }));
        let inf2: Value<std_basic_istringstream_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new({
                let __tmp_1: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"def").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                std_basic_istringstream_char__std_char_traits_char___std_allocator_char__::new_1(
                    { __tmp_1.as_pointer() },
                    None,
                )
            }));
        let mut i1: Ptr<::std::fs::File> =
            std_istreambuf_iterator_char__std_char_traits_char__::new_1({ inf1.as_pointer() });
        let mut i2: Ptr<::std::fs::File> =
            std_istreambuf_iterator_char__std_char_traits_char__::new_1({ inf2.as_pointer() });
        let mut i3: Ptr<::std::fs::File> = Ptr::null();
        let mut i4: Ptr<::std::fs::File> = Ptr::null();
        let mut i5: Ptr<::std::fs::File> = Ptr::<::std::fs::File>::null();
        assert!((i1 == i1));
        assert!((i1 == i2));
        assert!(!(i1 == i3));
        assert!(!(i1 == i4));
        assert!(!(i1 == i5));
        assert!((i2 == i1));
        assert!((i2 == i2));
        assert!(!(i2 == i3));
        assert!(!(i2 == i4));
        assert!(!(i2 == i5));
        assert!(!(i3 == i1));
        assert!(!(i3 == i2));
        assert!((i3 == i3));
        assert!((i3 == i4));
        assert!((i3 == i5));
        assert!(!(i4 == i1));
        assert!(!(i4 == i2));
        assert!((i4 == i3));
        assert!((i4 == i4));
        assert!((i4 == i5));
        assert!(!(i5 == i1));
        assert!(!(i5 == i2));
        assert!((i5 == i3));
        assert!((i5 == i4));
        assert!((i5 == i5));
    }
    {
        let i1: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Ptr::null()));
        let i2: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            std_istreambuf_iterator_char__std_char_traits_char__::new_2({
                (*default_sentinel_0.with(Value::clone).borrow()).clone()
            }),
        ));
        assert!(({ (*i1.borrow()) == (*i2.borrow()) }));
        assert!(
            ({ (*i1.borrow()) == ({ (*default_sentinel_0.with(Value::clone).borrow()).clone() }) })
        );
        assert!(
            ({ (*i2.borrow()) == ({ (*default_sentinel_0.with(Value::clone).borrow()).clone() }) })
        );
        assert!(({ (*default_sentinel_0.with(Value::clone).borrow()).clone() }) == (*i1.borrow()));
        assert!(({ (*default_sentinel_0.with(Value::clone).borrow()).clone() }) == (*i2.borrow()));
        let stream: Value<
            std_basic_istringstream_char__std_char_traits_char___std_allocator_char__,
        > = Rc::new(RefCell::new({
            let __tmp_2: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b" 1 23").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            std_basic_istringstream_char__std_char_traits_char___std_allocator_char__::new_1(
                { __tmp_2.as_pointer() },
                None,
            )
        }));
        let i3: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            std_istreambuf_iterator_char__std_char_traits_char__::new_1({ stream.as_pointer() }),
        ));
        assert!(
            ({
                !((*i3.borrow()) == ({ (*default_sentinel_0.with(Value::clone).borrow()).clone() }))
            })
        );
        assert!(
            ({
                !(({ (*default_sentinel_0.with(Value::clone).borrow()).clone() }) == (*i3.borrow()))
            })
        );
    }
    {
        let inf1: Value<
            std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = Rc::new(RefCell::new({
            let __tmp_3: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes =
                    Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32])
                        .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { __tmp_3.as_pointer() } , None , )
        }));
        let inf2: Value<
            std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = Rc::new(RefCell::new({
            let __tmp_4: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i32>::from_string_literal(&[
                    100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ])
                .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { __tmp_4.as_pointer() } , None , )
        }));
        let mut i1: Ptr<::std::fs::File> =
            std_istreambuf_iterator_wchar_t__std_char_traits_wchar_t__::new_1({
                inf1.as_pointer()
            });
        let mut i2: Ptr<::std::fs::File> =
            std_istreambuf_iterator_wchar_t__std_char_traits_wchar_t__::new_1({
                inf2.as_pointer()
            });
        let mut i3: Ptr<::std::fs::File> = Ptr::null();
        let mut i4: Ptr<::std::fs::File> = Ptr::null();
        let mut i5: Ptr<::std::fs::File> = Ptr::<::std::fs::File>::null();
        assert!((i1 == i1));
        assert!((i1 == i2));
        assert!(!(i1 == i3));
        assert!(!(i1 == i4));
        assert!(!(i1 == i5));
        assert!((i2 == i1));
        assert!((i2 == i2));
        assert!(!(i2 == i3));
        assert!(!(i2 == i4));
        assert!(!(i2 == i5));
        assert!(!(i3 == i1));
        assert!(!(i3 == i2));
        assert!((i3 == i3));
        assert!((i3 == i4));
        assert!((i3 == i5));
        assert!(!(i4 == i1));
        assert!(!(i4 == i2));
        assert!((i4 == i3));
        assert!((i4 == i4));
        assert!((i4 == i5));
        assert!(!(i5 == i1));
        assert!(!(i5 == i2));
        assert!((i5 == i3));
        assert!((i5 == i4));
        assert!((i5 == i5));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
