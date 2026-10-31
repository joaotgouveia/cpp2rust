extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
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
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 3));
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(Ptr::<i8>::from_string_literal(b"high").with_c_str(|__s| {
            (*unimplemented!().upgrade().deref())[..(*unimplemented!().upgrade().deref())
                .len()
                .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!().read()) == 5));
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<0>(t) == 2.718");;
        const _: () = assert!(true, "std::get<1>(t) == 5");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut e: Empty = (*unimplemented!().upgrade().deref()).clone();
        (&(e));
    }
    {
        let mut d: f64 = 1.5_f64;
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 1.5_f64));
        assert!(Ptr::<i8>::from_string_literal(b"high").with_c_str(|__s| {
            (*unimplemented!().upgrade().deref())[..(*unimplemented!().upgrade().deref())
                .len()
                .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!().read()) == 5));
        unimplemented!().write(2.5_f64);
        assert!(((unimplemented!().read()) == 2.5_f64));
        assert!(Ptr::<i8>::from_string_literal(b"high").with_c_str(|__s| {
            (*unimplemented!().upgrade().deref())[..(*unimplemented!().upgrade().deref())
                .len()
                .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!().read()) == 5));
        assert!((d == 2.5_f64));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
