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
        let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
            b"abc\0\0\0\0\0\0\0\0\0",
        )));
        let sb: Value<std_strstreambuf> = Rc::new(RefCell::new(std_strstreambuf::new_1(
            { (buf.as_pointer() as Ptr<i8>) },
            { (::std::mem::size_of::<[i8; 12]>() as i64) },
            { Some((buf.as_pointer() as Ptr<i8>)) },
        )));
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('1' as i8))
            }) == (('1' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"1bc").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"1bc").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('2' as i8))
            }) == (('2' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"12c").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"12c").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('3' as i8))
            }) == (('3' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('4' as i8))
            }) == (('4' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"1234").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"1234").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('5' as i8))
            }) == (('5' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"12345").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"12345").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('6' as i8))
            }) == (('6' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123456").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123456").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('7' as i8))
            }) == (('7' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('8' as i8))
            }) == (('8' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"12345678").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"12345678").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('9' as i8))
            }) == (('9' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123456789").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123456789").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('0' as i8))
            }) == (('0' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('1' as i8))
            }) == (('1' as i8) as i32))
        );
        assert!(
            ({ (*sb.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"12345678901").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"12345678901").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
