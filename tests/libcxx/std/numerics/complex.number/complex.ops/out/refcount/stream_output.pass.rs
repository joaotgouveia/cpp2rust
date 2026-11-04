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
        let c: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
        let os : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
        write!(os.as_pointer(), "{:}", (*c.borrow_mut()),);
        assert!(
            match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_0,) => {
                    Ptr::<i8>::from_string_literal(b"(1,2)").with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let mut width: i32 = 0;
        'loop_: while (width <= 5) {
            let c: Value<std_complex_double_> =
                Rc::new(RefCell::new(<std_complex_double_>::default()));
            let os : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
            ({ ((*os.borrow()) as ::std::fs::File).width_14((width as i64)) });
            ({
                ((*os.borrow()) as std_basic_ios_char__std_char_traits_char__).fill_21(('_' as i8))
            });
            write!(os.as_pointer(), "{:}", (*c.borrow_mut()),);
            assert!(({ (({ ((*os.borrow()) as ::std::fs::File).width_13() }) == 0_i64) }));
            assert!(
                match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                    (__tmp_1,) => {
                        Ptr::<i8>::from_string_literal(b"(1,2)").with_c_str(|__s| {
                            (*__tmp_1.upgrade().deref())
                                [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                    }
                }
            );
            width.prefix_inc();
        }
        {
            let c: Value<std_complex_double_> =
                Rc::new(RefCell::new(<std_complex_double_>::default()));
            let os : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
            ({ ((*os.borrow()) as ::std::fs::File).width_14(6_i64) });
            ({
                ((*os.borrow()) as std_basic_ios_char__std_char_traits_char__).fill_21(('_' as i8))
            });
            write!(os.as_pointer(), "{:}", (*c.borrow_mut()),);
            assert!(({ (({ ((*os.borrow()) as ::std::fs::File).width_13() }) == 0_i64) }));
            assert!(
                match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                    (__tmp_2,) => {
                        Ptr::<i8>::from_string_literal(b"_(1,2)").with_c_str(|__s| {
                            (*__tmp_2.upgrade().deref())
                                [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                    }
                }
            );
        }
        {
            let c: Value<std_complex_double_> =
                Rc::new(RefCell::new(<std_complex_double_>::default()));
            let os : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
            ({ ((*os.borrow()) as ::std::fs::File).width_14(7_i64) });
            ({
                ((*os.borrow()) as std_basic_ios_char__std_char_traits_char__).fill_21(('_' as i8))
            });
            write!(os.as_pointer(), "{:}", (*c.borrow_mut()),);
            assert!(({ (({ ((*os.borrow()) as ::std::fs::File).width_13() }) == 0_i64) }));
            assert!(
                match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                    (__tmp_3,) => {
                        Ptr::<i8>::from_string_literal(b"__(1,2)").with_c_str(|__s| {
                            (*__tmp_3.upgrade().deref())
                                [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                    }
                }
            );
        }
        {
            let c: Value<std_complex_double_> =
                Rc::new(RefCell::new(<std_complex_double_>::default()));
            let os : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
            ({ ((*os.borrow()) as ::std::fs::File).width_14(8_i64) });
            ({
                ((*os.borrow()) as std_basic_ios_char__std_char_traits_char__).fill_21(('_' as i8))
            });
            write!(os.as_pointer(), "{:}", (*c.borrow_mut()),);
            assert!(({ (({ ((*os.borrow()) as ::std::fs::File).width_13() }) == 0_i64) }));
            assert!(
                match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                    (__tmp_4,) => {
                        Ptr::<i8>::from_string_literal(b"___(1,2)").with_c_str(|__s| {
                            (*__tmp_4.upgrade().deref())
                                [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                    }
                }
            );
        }
        {
            let c: Value<std_complex_double_> =
                Rc::new(RefCell::new(<std_complex_double_>::default()));
            let os : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
            ({ ((*os.borrow()) as ::std::fs::File).width_14(8_i64) });
            ({
                ((*os.borrow()) as std_basic_ios_char__std_char_traits_char__).fill_21(('_' as i8))
            });
            write!(os.as_pointer(), "{:}", (*c.borrow_mut()),);
            assert!(({ (({ ((*os.borrow()) as ::std::fs::File).width_13() }) == 0_i64) }));
            {
                let __s = os.as_pointer();
                let _ = Ptr::<i8>::from_string_literal(b"hello").with_c_str(|__cs| {
                    __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                });
                __s
            };
            assert!(
                match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                    (__tmp_5,) => {
                        Ptr::<i8>::from_string_literal(b"___(1,2)hello").with_c_str(|__s| {
                            (*__tmp_5.upgrade().deref())
                                [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                    }
                }
            );
        }
        {
            let c: Value<std_complex_double_> =
                Rc::new(RefCell::new(<std_complex_double_>::default()));
            let os : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
            ({ ((*os.borrow()) as ::std::fs::File).width_14(4_i64) });
            ({
                ((*os.borrow()) as std_basic_ios_char__std_char_traits_char__).fill_21(('_' as i8))
            });
            write!(os.as_pointer(), "{:}", (*c.borrow_mut()),);
            assert!(({ (({ ((*os.borrow()) as ::std::fs::File).width_13() }) == 0_i64) }));
            assert!(
                match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                    (__tmp_6,) => {
                        Ptr::<i8>::from_string_literal(b"(123,456)").with_c_str(|__s| {
                            (*__tmp_6.upgrade().deref())
                                [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                    }
                }
            );
        }
        {
            let c: Value<std_complex_double_> =
                Rc::new(RefCell::new(<std_complex_double_>::default()));
            let os : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
            ({ ((*os.borrow()) as ::std::fs::File).width_14(12_i64) });
            ({
                ((*os.borrow()) as std_basic_ios_char__std_char_traits_char__).fill_21(('_' as i8))
            });
            write!(os.as_pointer(), "{:}", (*c.borrow_mut()),);
            assert!(({ (({ ((*os.borrow()) as ::std::fs::File).width_13() }) == 0_i64) }));
            assert!(
                match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                    (__tmp_7,) => {
                        Ptr::<i8>::from_string_literal(b"___(123,456)").with_c_str(|__s| {
                            (*__tmp_7.upgrade().deref())
                                [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                    }
                }
            );
            {
                let __s = os.as_pointer();
                let _ = Ptr::<i8>::from_string_literal(b"hello").with_c_str(|__cs| {
                    __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                });
                __s
            };
            assert!(
                match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                    (__tmp_8,) => {
                        Ptr::<i8>::from_string_literal(b"___(123,456)hello").with_c_str(|__s| {
                            (*__tmp_8.upgrade().deref())
                                [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                    }
                }
            );
        }
        {
            let c: Value<std_complex_double_> =
                Rc::new(RefCell::new(<std_complex_double_>::default()));
            let os : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
            ({ ((*os.borrow()) as ::std::fs::File).width_14(12_i64) });
            ({
                ((*os.borrow()) as std_basic_ios_char__std_char_traits_char__).fill_21(('_' as i8))
            });
            write!(
                (os.as_pointer() as ::std::fs::File),
                "{:}{:}",
                FnPtr::<fn(Ptr<::std::fs::File>) -> Ptr<::std::fs::File>>::new(
                    libcc2rs::left_refcount
                ),
                (*c.borrow_mut()),
            );
            assert!(({ (({ ((*os.borrow()) as ::std::fs::File).width_13() }) == 0_i64) }));
            assert!(
                match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                    (__tmp_9,) => {
                        Ptr::<i8>::from_string_literal(b"(123,456)___").with_c_str(|__s| {
                            (*__tmp_9.upgrade().deref())
                                [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                    }
                }
            );
            {
                let __s = os.as_pointer();
                let _ = Ptr::<i8>::from_string_literal(b"xy").with_c_str(|__cs| {
                    __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                });
                __s
            };
            assert!(
                match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
                    (__tmp_10,) => {
                        Ptr::<i8>::from_string_literal(b"(123,456)___xy").with_c_str(|__s| {
                            (*__tmp_10.upgrade().deref())
                                [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                    }
                }
            );
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
