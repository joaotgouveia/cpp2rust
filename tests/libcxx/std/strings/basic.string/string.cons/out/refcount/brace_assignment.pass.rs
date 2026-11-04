extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"hello world").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        (*s.borrow_mut()) = Default::default();
        assert!(({ (*s.borrow()).len() <= 1 }));
        const _: () = assert!(true, "");;
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"hello world").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        {
            s.as_pointer().write({
                let mut __v = Vec::with_capacity(2_usize as usize + 1);
                Ptr::<i8>::from_string_literal(b"abc")
                    .with_slice(2_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            });
            s.as_pointer()
        };
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"ab").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
        const _: () = assert!(true, "");;
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"hello world").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        {
            s.as_pointer().write({
                let mut __v = Vec::with_capacity(30_usize as usize + 1);
                Ptr::<i8>::from_string_literal(b"It\'sALongString!NoSSO!qwertyuiop")
                    .with_slice(30_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            });
            s.as_pointer()
        };
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"It\'sALongString!NoSSO!qwertyui").with_c_str(
                    |__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s,
                )
            })
        );
        const _: () = assert!(true, "");;
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(
                b"Hello world! Hello world! Hello world! Hello world! Hello world!",
            )
            .to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        {
            s.as_pointer().write({
                let mut __v = Vec::with_capacity(30_usize as usize + 1);
                Ptr::<i8>::from_string_literal(b"It\'sALongString!NoSSO!qwertyuiop")
                    .with_slice(30_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            });
            s.as_pointer()
        };
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"It\'sALongString!NoSSO!qwertyui").with_c_str(
                    |__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s,
                )
            })
        );
        const _: () = assert!(true, "");;
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(
                b"Hello world! Hello world! Hello world! Hello world! Hello world!",
            )
            .to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        {
            s.as_pointer().write({
                let mut __v = Vec::with_capacity(2_usize as usize + 1);
                Ptr::<i8>::from_string_literal(b"abc")
                    .with_slice(2_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            });
            s.as_pointer()
        };
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"ab").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
        const _: () = assert!(true, "");;
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(
                b"Hello world! Hello world! Hello world! Hello world! Hello world!",
            )
            .to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        {
            s.as_pointer().write({
                let mut __v = Vec::with_capacity(0_usize as usize + 1);
                Ptr::<i8>::from_string_literal(b"abc")
                    .with_slice(0_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            });
            s.as_pointer()
        };
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
        const _: () = assert!(true, "");;
    }
    return true;
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
