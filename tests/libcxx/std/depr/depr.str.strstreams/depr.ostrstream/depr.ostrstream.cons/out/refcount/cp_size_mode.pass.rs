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
        let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"123 4.5 dog\0")));
        let out: Value<std_ostrstream> = Rc::new(RefCell::new(std_ostrstream::new_1(
            { (buf.as_pointer() as Ptr<i8>) },
            { 0 },
            None,
        )));
        assert!(
            ({ (*out.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123 4.5 dog").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123 4.5 dog").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        let mut i: i32 = 321;
        let mut d: f64 = 5.5_f64;
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"cat").to_c_bytes();
            __bytes.push(0);
            __bytes
        }));
        write!(
            (out.as_pointer() as std_basic_ostream_char__std_char_traits_char__),
            "{:} {:} ",
            i,
            d,
        );
        (out.as_pointer() as std_basic_ostream_char__std_char_traits_char__).write_all(
            &([(&(*s.borrow())
                .iter()
                .take((*s.borrow()).len() - 1)
                .map(|&c| c as u8)
                .collect::<Vec<u8>>()[..] as &[u8])]
            .concat()),
        );
        write!(
            (out.as_pointer() as std_basic_ostream_char__std_char_traits_char__),
            "{:}",
            FnPtr::<
                fn(
                    Ptr<std_basic_ostream_char__std_char_traits_char__>,
                ) -> Ptr<std_basic_ostream_char__std_char_traits_char__>,
            >::new(ends_0),
        );
        assert!(
            ({ (*out.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"321 5.5 cat").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"321 5.5 cat").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
    }
    {
        let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
            b"123 4.5 dog\0\0\0\0\0\0\0\0\0\0\0\0",
        )));
        let out: Value<std_ostrstream> = Rc::new(RefCell::new(std_ostrstream::new_1(
            { (buf.as_pointer() as Ptr<i8>) },
            { 11 },
            { Some(app_1.with(|rc| *rc.borrow())) },
        )));
        assert!(
            ({ (*out.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123 4.5 dog").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes = Ptr::<i8>::from_string_literal(b"123 4.5 dog").to_c_bytes();
                __bytes.push(0);
                __bytes
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        let mut i: i32 = 321;
        let mut d: f64 = 5.5_f64;
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"cat").to_c_bytes();
            __bytes.push(0);
            __bytes
        }));
        write!(
            (out.as_pointer() as std_basic_ostream_char__std_char_traits_char__),
            "{:} {:} ",
            i,
            d,
        );
        (out.as_pointer() as std_basic_ostream_char__std_char_traits_char__).write_all(
            &([(&(*s.borrow())
                .iter()
                .take((*s.borrow()).len() - 1)
                .map(|&c| c as u8)
                .collect::<Vec<u8>>()[..] as &[u8])]
            .concat()),
        );
        write!(
            (out.as_pointer() as std_basic_ostream_char__std_char_traits_char__),
            "{:}",
            FnPtr::<
                fn(
                    Ptr<std_basic_ostream_char__std_char_traits_char__>,
                ) -> Ptr<std_basic_ostream_char__std_char_traits_char__>,
            >::new(ends_0),
        );
        assert!(
            ({ (*out.borrow()).str() }).with_c_str(|__s| {
                let mut __bytes =
                    Ptr::<i8>::from_string_literal(b"123 4.5 dog321 5.5 cat").to_c_bytes();
                __bytes.push(0);
                __bytes
            }[..{
                let mut __bytes =
                    Ptr::<i8>::from_string_literal(b"123 4.5 dog321 5.5 cat").to_c_bytes();
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
