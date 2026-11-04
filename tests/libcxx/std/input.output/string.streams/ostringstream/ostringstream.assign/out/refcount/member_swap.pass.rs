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
        let ss0: Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b" 123 456").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__::new_1(
                    { __tmp_0.as_pointer() },
                    None,
                )
            }));
        let ss : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
        ({ (*ss.borrow()).swap(ss0.as_pointer()) });
        assert!(({ !(({ (*ss.borrow()).rdbuf() }).is_null()) }));
        assert!(({ ({ ((*ss.borrow()) as std_basic_ios_char__std_char_traits_char__).good() }) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_1,) => {
                    Ptr::<i8>::from_string_literal(b" 123 456").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        let mut i: i32 = 234;
        {
            let __s = {
                let __s = {
                    let __s = (ss.as_pointer() as ::std::fs::File);
                    write_int_refcount(&__s, i);
                    __s
                };
                let _ = __s.write_all(&[(' ' as i8) as u8]);
                __s
            };
            write_int_refcount(&__s, 567);
            __s
        };
        assert!(
            match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_2,) => {
                    Ptr::<i8>::from_string_literal(b"234 5676").with_c_str(|__s| {
                        (*__tmp_2.upgrade().deref())
                            [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        {
            let __s = {
                let __s = {
                    let __s = (ss0.as_pointer() as ::std::fs::File);
                    write_int_refcount(&__s, i);
                    __s
                };
                let _ = __s.write_all(&[(' ' as i8) as u8]);
                __s
            };
            write_int_refcount(&__s, 567);
            __s
        };
        assert!(
            match (Rc::new(RefCell::new(({ (*ss0.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_3,) => {
                    Ptr::<i8>::from_string_literal(b"234 567").with_c_str(|__s| {
                        (*__tmp_3.upgrade().deref())
                            [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let ss0: Value<
            std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = Rc::new(RefCell::new({
            let __tmp_4: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i32>::from_string_literal(&[
                    32 as i32, 49 as i32, 50 as i32, 51 as i32, 32 as i32, 52 as i32, 53 as i32,
                    54 as i32, 0 as i32,
                ])
                .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { __tmp_4.as_pointer() } , None , )
        }));
        let ss : Value<std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ > = Rc::new(RefCell::new(<std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__  >::default() )) ;
        ({ (*ss.borrow()).swap(ss0.as_pointer()) });
        assert!(({ !(({ (*ss.borrow()).rdbuf() }).is_null()) }));
        assert!(
            ({ ({ ((*ss.borrow()) as std_basic_ios_wchar_t__std_char_traits_wchar_t__).good() }) })
        );
        assert!(
            match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_5,) => {
                    Ptr::<i32>::from_string_literal(&[
                        32 as i32, 49 as i32, 50 as i32, 51 as i32, 32 as i32, 52 as i32,
                        53 as i32, 54 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        let mut i: i32 = 234;
        {
            let __s = write!((ss.as_pointer() as ::std::fs::File), "{:} ", i,);
            write_int_refcount(&__s, 567);
            __s
        };
        assert!(
            match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_6,) => {
                    Ptr::<i32>::from_string_literal(&[
                        50 as i32, 51 as i32, 52 as i32, 32 as i32, 53 as i32, 54 as i32,
                        55 as i32, 54 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_6.upgrade().deref())
                            [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        {
            let __s = write!((ss0.as_pointer() as ::std::fs::File), "{:} ", i,);
            write_int_refcount(&__s, 567);
            __s
        };
        assert!(
            match (Rc::new(RefCell::new(({ (*ss0.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_7,) => {
                    Ptr::<i32>::from_string_literal(&[
                        50 as i32, 51 as i32, 52 as i32, 32 as i32, 53 as i32, 54 as i32,
                        55 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_7.upgrade().deref())
                            [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
