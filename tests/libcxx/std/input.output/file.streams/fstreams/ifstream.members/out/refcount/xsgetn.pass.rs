extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_unget_0(
    mut fb: Ptr<std_basic_filebuf_char__std_char_traits_char__>,
    mut expected: i8,
) {
    let mut c: i32 = ({ (*(fb as *mut ::std::fs::File).upgrade().deref()).sungetc() });
    if (c != (-1_i32)) {
        assert!((c == (expected as i32)));
        assert!(
            ({
                ({ ({ (*(fb as *mut ::std::fs::File).upgrade().deref()).sbumpc() }) } == {
                    (expected as i32)
                })
            })
        );
    }
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
    let stream_buffer: Value<Vec<i8>> = Rc::new(RefCell::new(
        (0..(10_usize) as usize).map(|_| 0_i8).collect::<Vec<_>>(),
    ));
    let fs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::open(Ptr::<i8>::from_string_literal(b"xsgetn.test.dat").to_string())
            .expect("Failed to open file"),
    ));
    let mut fb: Ptr<std_basic_filebuf_char__std_char_traits_char__> = (*fs.borrow()).clone();
    ({
        (*(fb as *mut ::std::fs::File).upgrade().deref()).pubsetbuf(stream_buffer.decay(), 10_i64)
    });
    assert!(
        ({
            (({ (*(fb as *mut ::std::fs::File).upgrade().deref()).sgetc() })
                == (('t' as i8) as i32))
        })
    );
    let test_buffer: Value<Vec<i8>> = Rc::new(RefCell::new(
        (0..(5_usize) as usize).map(|_| 0_i8).collect::<Vec<_>>(),
    ));
    test_buffer
        .decay()
        .offset(0_usize as isize)
        .write(('\0' as i8));
    {
        assert!(
            ({
                (({
                    (*(fb as *mut ::std::fs::File).upgrade().deref())
                        .sgetn(test_buffer.decay(), 5_i64)
                }) == 5_i64)
            })
        );
        assert!(match (Rc::new(RefCell::new({
            let mut __v = Vec::with_capacity(5_usize as usize + 1);
            test_buffer
                .decay()
                .with_slice(5_usize as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0_i8);
            __v
        }))
        .as_pointer(),)
        {
            (__tmp_0,) => {
                Ptr::<i8>::from_string_literal(b"this ").with_c_str(|__s| {
                    (*__tmp_0.upgrade().deref())
                        [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        ({ check_unget_0((fb).clone(), (' ' as i8)) });
    }
    {
        assert!(
            ({
                (({
                    (*(fb as *mut ::std::fs::File).upgrade().deref())
                        .sgetn(test_buffer.decay(), 5_i64)
                }) == 5_i64)
            })
        );
        assert!(match (Rc::new(RefCell::new({
            let mut __v = Vec::with_capacity(5_usize as usize + 1);
            test_buffer
                .decay()
                .with_slice(5_usize as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0_i8);
            __v
        }))
        .as_pointer(),)
        {
            (__tmp_1,) => {
                Ptr::<i8>::from_string_literal(b"is so").with_c_str(|__s| {
                    (*__tmp_1.upgrade().deref())
                        [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        ({ check_unget_0((fb).clone(), ('o' as i8)) });
    }
    {
        {
            let __a0 = 12_usize as usize;
            (*test_buffer.borrow_mut()).resize_with(__a0, || 0_i8)
        };
        assert!(
            ({
                (({
                    (*(fb as *mut ::std::fs::File).upgrade().deref())
                        .sgetn(test_buffer.decay(), 12_i64)
                }) == 12_i64)
            })
        );
        assert!(match (Rc::new(RefCell::new({
            let mut __v = Vec::with_capacity(12_usize as usize + 1);
            test_buffer
                .decay()
                .with_slice(12_usize as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0_i8);
            __v
        }))
        .as_pointer(),)
        {
            (__tmp_2,) => {
                Ptr::<i8>::from_string_literal(b"me random da").with_c_str(|__s| {
                    (*__tmp_2.upgrade().deref())
                        [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        ({ check_unget_0((fb).clone(), ('a' as i8)) });
    }
    {
        {
            let __a0 = 2_usize as usize;
            (*test_buffer.borrow_mut()).resize_with(__a0, || 0_i8)
        };
        assert!(
            ({
                (({
                    (*(fb as *mut ::std::fs::File).upgrade().deref())
                        .sgetn(test_buffer.decay(), 2_i64)
                }) == 2_i64)
            })
        );
        assert!(match (Rc::new(RefCell::new({
            let mut __v = Vec::with_capacity(2_usize as usize + 1);
            test_buffer
                .decay()
                .with_slice(2_usize as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0_i8);
            __v
        }))
        .as_pointer(),)
        {
            (__tmp_3,) => {
                Ptr::<i8>::from_string_literal(b"ta").with_c_str(|__s| {
                    (*__tmp_3.upgrade().deref())
                        [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        ({ check_unget_0((fb).clone(), ('a' as i8)) });
        {
            let __a0 = 12_usize as usize;
            (*test_buffer.borrow_mut()).resize_with(__a0, || 0_i8)
        };
        assert!(
            ({
                (({
                    (*(fb as *mut ::std::fs::File).upgrade().deref())
                        .sgetn(test_buffer.decay(), 12_i64)
                }) == 12_i64)
            })
        );
        assert!(match (Rc::new(RefCell::new({
            let mut __v = Vec::with_capacity(12_usize as usize + 1);
            test_buffer
                .decay()
                .with_slice(12_usize as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0_i8);
            __v
        }))
        .as_pointer(),)
        {
            (__tmp_4,) => {
                Ptr::<i8>::from_string_literal(b" to be able ").with_c_str(|__s| {
                    (*__tmp_4.upgrade().deref())
                        [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        ({ check_unget_0((fb).clone(), (' ' as i8)) });
    }
    {
        {
            let __a0 = 30_usize as usize;
            (*test_buffer.borrow_mut()).resize_with(__a0, || 0_i8)
        };
        assert!(
            ({
                (({
                    (*(fb as *mut ::std::fs::File).upgrade().deref())
                        .sgetn(test_buffer.decay(), 30_i64)
                }) == 24_i64)
            })
        );
        {
            let __a0 = 24_usize as usize;
            (*test_buffer.borrow_mut()).resize_with(__a0, || 0_i8)
        };
        assert!(match (Rc::new(RefCell::new({
            let mut __v = Vec::with_capacity(24_usize as usize + 1);
            test_buffer
                .decay()
                .with_slice(24_usize as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0_i8);
            __v
        }))
        .as_pointer(),)
        {
            (__tmp_5,) => {
                Ptr::<i8>::from_string_literal(b"to test buffer behaviour").with_c_str(|__s| {
                    (*__tmp_5.upgrade().deref())
                        [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        ({ check_unget_0((fb).clone(), ('r' as i8)) });
    }
    {
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..10).map(|_| 0_i8).collect::<Box<[i8]>>()));
        let ifs: Value<::std::fs::File> = Rc::new(RefCell::new(::std::fs::File::open("").unwrap()));
        let mut filebuf: Ptr<std_basic_filebuf_char__std_char_traits_char__> =
            (*ifs.borrow()).clone();
        assert!(({ !({ (*filebuf.upgrade().deref()).is_open() }) }));
        assert!(
            ({
                (({
                    let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                    let ___n: i64 = (::std::mem::size_of::<[i8; 10]>() as i64);
                    (*(filebuf as *mut ::std::fs::File).upgrade().deref()).sgetn(___s, ___n)
                }) == 0_i64)
            })
        );
        assert!(({ !({ (*filebuf.upgrade().deref()).is_open() }) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
