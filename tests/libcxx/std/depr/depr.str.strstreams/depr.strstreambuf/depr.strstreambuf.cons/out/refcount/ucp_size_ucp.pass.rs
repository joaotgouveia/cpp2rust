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
        let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(b"abcd\0")));
        let sb: Value<std_strstreambuf> = Rc::new(RefCell::new(std_strstreambuf::new_1(
            { (buf.as_pointer() as Ptr<u8>) },
            { (::std::mem::size_of::<[u8; 5]>() as i64) },
            None,
        )));
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).sgetc() })
                == (('a' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('b' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('c' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('d' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == 0)
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (-1_i32))
        );
    }
    {
        let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(b"abcd\0")));
        let sb: Value<std_strstreambuf> = Rc::new(RefCell::new(std_strstreambuf::new_1(
            { (buf.as_pointer() as Ptr<u8>) },
            { 0_i64 },
            None,
        )));
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).sgetc() })
                == (('a' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('b' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('c' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('d' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (-1_i32))
        );
    }
    {
        let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(b"abcd\0")));
        let sb: Value<std_strstreambuf> = Rc::new(RefCell::new(std_strstreambuf::new_1(
            { (buf.as_pointer() as Ptr<u8>) },
            { (::std::mem::size_of::<[u8; 5]>() as i64) },
            { Some((buf.as_pointer() as Ptr<u8>)) },
        )));
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).sgetc() })
                == (-1_i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('e' as i8))
            }) == (('e' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('f' as i8))
            }) == (('f' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('g' as i8))
            }) == (('g' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('h' as i8))
            }) == (('h' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('i' as i8))
            }) == (('i' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('j' as i8))
            }) == (-1_i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).sgetc() })
                == (('e' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('f' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('g' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('h' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('i' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (-1_i32))
        );
    }
    {
        let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(b"abcd\0")));
        let sb: Value<std_strstreambuf> = Rc::new(RefCell::new(std_strstreambuf::new_1(
            { (buf.as_pointer() as Ptr<u8>) },
            { 0_i64 },
            { Some((buf.as_pointer() as Ptr<u8>)) },
        )));
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).sgetc() })
                == (-1_i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('e' as i8))
            }) == (('e' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('f' as i8))
            }) == (('f' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('g' as i8))
            }) == (('g' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('h' as i8))
            }) == (('h' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('i' as i8))
            }) == (-1_i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).sgetc() })
                == (('e' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('f' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('g' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('h' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (-1_i32))
        );
    }
    {
        let buf: Value<Box<[u8]>> =
            Rc::new(RefCell::new(u8::array_from_literal(b"abcd\0\0\0\0\0\0")));
        let mut s: usize = (buf.as_pointer() as Ptr<u8>)
            .reinterpret_cast::<i8>()
            .to_c_string_iterator()
            .count();
        let sb: Value<std_strstreambuf> = Rc::new(RefCell::new(std_strstreambuf::new_1(
            { (buf.as_pointer() as Ptr<u8>) },
            { (((::std::mem::size_of::<[u8; 10]>() as u64).wrapping_sub((s as u64))) as i64) },
            { Some((buf.as_pointer() as Ptr<u8>).offset((s) as isize)) },
        )));
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).sgetc() })
                == (('a' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('b' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('c' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('d' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (-1_i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('e' as i8))
            }) == (('e' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('f' as i8))
            }) == (('f' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('g' as i8))
            }) == (('g' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('h' as i8))
            }) == (('h' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('i' as i8))
            }) == (('i' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('j' as i8))
            }) == (('j' as i8) as i32))
        );
        assert!(
            (({
                ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__)
                    .sputc(('j' as i8))
            }) == (-1_i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).sgetc() })
                == (('e' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('f' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('g' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('h' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('i' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (('j' as i8) as i32))
        );
        assert!(
            (({ ((*sb.borrow()) as std_basic_streambuf_char__std_char_traits_char__).snextc() })
                == (-1_i32))
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
