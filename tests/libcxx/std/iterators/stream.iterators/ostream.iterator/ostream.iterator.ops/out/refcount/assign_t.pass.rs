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
        let outf : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
        let i: Value<OstreamIterator<i32>> = Rc::new(RefCell::new(OstreamIterator::new(
            outf.as_pointer(),
            Ptr::null(),
        )));
        (*i.borrow_mut()) = (2.4_f64 as i32);
        assert!(
            match (Rc::new(RefCell::new(({ (*outf.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_0,) => {
                    Ptr::<i8>::from_string_literal(b"2").with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let outf : Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__ > = Rc::new(RefCell::new(<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__  >::default() )) ;
        let i: Value<OstreamIterator<i32>> = Rc::new(RefCell::new(OstreamIterator::new(
            outf.as_pointer(),
            Ptr::<i8>::from_string_literal(b", "),
        )));
        (*i.borrow_mut()) = (2.4_f64 as i32);
        assert!(
            match (Rc::new(RefCell::new(({ (*outf.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_1,) => {
                    Ptr::<i8>::from_string_literal(b"2, ").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let outf : Value<std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ > = Rc::new(RefCell::new(<std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__  >::default() )) ;
        let i: Value<OstreamIterator<i32>> = Rc::new(RefCell::new(OstreamIterator::new(
            outf.as_pointer(),
            Ptr::null(),
        )));
        (*i.borrow_mut()) = (2.4_f64 as i32);
        assert!(
            match (Rc::new(RefCell::new(({ (*outf.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_2,) => {
                    Ptr::<i32>::from_string_literal(&[50 as i32, 0 as i32]).with_c_str(|__s| {
                        (*__tmp_2.upgrade().deref())
                            [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let outf : Value<std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ > = Rc::new(RefCell::new(<std_basic_ostringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__  >::default() )) ;
        let i: Value<OstreamIterator<i32>> = Rc::new(RefCell::new(OstreamIterator::new(
            outf.as_pointer(),
            Ptr::<i32>::from_string_literal(&[44 as i32, 32 as i32, 0 as i32]),
        )));
        (*i.borrow_mut()) = (2.4_f64 as i32);
        assert!(
            match (Rc::new(RefCell::new(({ (*outf.borrow()).str_16() }))).as_pointer(),) {
                (__tmp_3,) => {
                    Ptr::<i32>::from_string_literal(&[50 as i32, 44 as i32, 32 as i32, 0 as i32])
                        .with_c_str(|__s| {
                            (*__tmp_3.upgrade().deref())
                                [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        })
                }
            }
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
