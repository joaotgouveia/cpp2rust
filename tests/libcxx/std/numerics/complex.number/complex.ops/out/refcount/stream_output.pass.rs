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
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(
            Ptr::<i8>::from_string_literal(b"(1,2)").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    }
    {
        let mut width: i32 = 0;
        'loop_: while (width <= 5) {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!((unimplemented!() == 0_()));
            assert!(
                Ptr::<i8>::from_string_literal(b"(1,2)").with_c_str(|__s| unimplemented!()
                    [..unimplemented!().len().saturating_sub(1)]
                    == *__s)
            );
            width.prefix_inc();
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!((unimplemented!() == 0_()));
            assert!(
                Ptr::<i8>::from_string_literal(b"_(1,2)").with_c_str(|__s| unimplemented!()
                    [..unimplemented!().len().saturating_sub(1)]
                    == *__s)
            );
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!((unimplemented!() == 0_()));
            assert!(
                Ptr::<i8>::from_string_literal(b"__(1,2)").with_c_str(|__s| unimplemented!()
                    [..unimplemented!().len().saturating_sub(1)]
                    == *__s)
            );
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!((unimplemented!() == 0_()));
            assert!(
                Ptr::<i8>::from_string_literal(b"___(1,2)").with_c_str(|__s| unimplemented!()
                    [..unimplemented!().len().saturating_sub(1)]
                    == *__s)
            );
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!((unimplemented!() == 0_()));
            {
                let __s = os.as_pointer();
                let _ = Ptr::<i8>::from_string_literal(b"hello").with_c_str(|__cs| {
                    __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                });
                __s
            };
            assert!(Ptr::<i8>::from_string_literal(b"___(1,2)hello").with_c_str(
                |__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s
            ));
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!((unimplemented!() == 0_()));
            assert!(Ptr::<i8>::from_string_literal(b"(123,456)").with_c_str(
                |__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s
            ));
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!((unimplemented!() == 0_()));
            assert!(Ptr::<i8>::from_string_literal(b"___(123,456)").with_c_str(
                |__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s
            ));
            {
                let __s = os.as_pointer();
                let _ = Ptr::<i8>::from_string_literal(b"hello").with_c_str(|__cs| {
                    __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                });
                __s
            };
            assert!(
                Ptr::<i8>::from_string_literal(b"___(123,456)hello").with_c_str(
                    |__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s
                )
            );
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!((unimplemented!() == 0_()));
            assert!(Ptr::<i8>::from_string_literal(b"(123,456)___").with_c_str(
                |__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s
            ));
            {
                let __s = os.as_pointer();
                let _ = Ptr::<i8>::from_string_literal(b"xy").with_c_str(|__cs| {
                    __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                });
                __s
            };
            assert!(
                Ptr::<i8>::from_string_literal(b"(123,456)___xy").with_c_str(
                    |__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s
                )
            );
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
