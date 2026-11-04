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
        let inf: Value<std_basic_istringstream_char__std_char_traits_char___std_allocator_char__> =
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
        let i: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            std_istreambuf_iterator_char__std_char_traits_char__::new_1({ inf.as_pointer() }),
        ));
        assert!(({ (((*i.borrow()).iter_deref() as i32) == (('a' as i8) as i32)) }));
        assert!(
            (({
                i.as_pointer().with_mut(|__it| {
                    __it.iter_inc();
                });
                i.as_pointer()
            }
            .iter_deref() as i32)
                == (('b' as i8) as i32))
        );
        assert!(
            (({
                i.as_pointer().with_mut(|__it| {
                    __it.iter_inc();
                });
                i.as_pointer()
            }
            .iter_deref() as i32)
                == (('c' as i8) as i32))
        );
        assert!(
            {
                i.as_pointer().with_mut(|__it| {
                    __it.iter_inc();
                });
                i.as_pointer()
            } == Ptr::null()
        );
    }
    {
        let inf: Value<
            std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = Rc::new(RefCell::new({
            let __tmp_1: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes =
                    Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32])
                        .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { __tmp_1.as_pointer() } , None , )
        }));
        let i: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            std_istreambuf_iterator_wchar_t__std_char_traits_wchar_t__::new_1({ inf.as_pointer() }),
        ));
        assert!(({ (((*i.borrow()).iter_deref() as i32) == (97 as i32)) }));
        assert!(
            (({
                i.as_pointer().with_mut(|__it| {
                    __it.iter_inc();
                });
                i.as_pointer()
            }
            .iter_deref() as i32)
                == (98 as i32))
        );
        assert!(
            (({
                i.as_pointer().with_mut(|__it| {
                    __it.iter_inc();
                });
                i.as_pointer()
            }
            .iter_deref() as i32)
                == (99 as i32))
        );
        assert!(
            {
                i.as_pointer().with_mut(|__it| {
                    __it.iter_inc();
                });
                i.as_pointer()
            } == Ptr::null()
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
