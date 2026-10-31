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
    const _: () = assert!(
        true,
        "(std::is_base_of<std::exception, std::runtime_error>::value)"
    );;
    const _: () = assert!(true, "std::is_polymorphic<std::runtime_error>::value");;
    {
        let mut msg: Ptr<i8> = Ptr::<i8>::from_string_literal(b"runtime_error message");
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let mut __it1 = unimplemented!().to_c_string_iterator();
                let mut __it2 = msg.to_c_string_iterator();
                loop {
                    let __c1 = __it1.next();
                    let __c2 = __it2.next();
                    if __c1 != __c2 {
                        break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                    }
                    if __c1.is_none() {
                        break 0;
                    }
                }
            } == 0)
        );
        let e2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let mut __it1 = unimplemented!().to_c_string_iterator();
                let mut __it2 = msg.to_c_string_iterator();
                loop {
                    let __c1 = __it1.next();
                    let __c2 = __it2.next();
                    if __c1 != __c2 {
                        break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                    }
                    if __c1.is_none() {
                        break 0;
                    }
                }
            } == 0)
        );
        unimplemented!();
        assert!(
            ({
                let mut __it1 = unimplemented!().to_c_string_iterator();
                let mut __it2 = msg.to_c_string_iterator();
                loop {
                    let __c1 = __it1.next();
                    let __c2 = __it2.next();
                    if __c1 != __c2 {
                        break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                    }
                    if __c1.is_none() {
                        break 0;
                    }
                }
            } == 0)
        );
    }
    {
        let msg: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes =
                Ptr::<i8>::from_string_literal(b"another runtime_error message").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            unimplemented!().with_c_str(|__s| (*msg.borrow())
                [..(*msg.borrow()).len().saturating_sub(1)]
                == *__s)
        );
        let e2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            unimplemented!().with_c_str(|__s| (*msg.borrow())
                [..(*msg.borrow()).len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(
            unimplemented!().with_c_str(|__s| (*msg.borrow())
                [..(*msg.borrow()).len().saturating_sub(1)]
                == *__s)
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
