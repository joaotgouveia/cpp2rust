extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(p: Ptr<Vec<i8>>, mut mc: u32) {
    let r: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
        std_basic_regex_char__std_regex_traits_char__::new_1({ (p).clone() }, None),
    ));
    assert!(
        ({
            ((({ (*r.borrow()).flags() }) as u32) == (ECMAScript_1.with(|rc| *rc.borrow()) as u32))
        })
    );
    assert!(({ (({ (*r.borrow()).mark_count() }) == mc) }));
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
    ({
        let _p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\\(a\\)").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_0(_p.as_pointer(), 0_u32)
    });
    ({
        let _p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\\(a[bc]\\)").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_0(_p.as_pointer(), 0_u32)
    });
    ({
        let _p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\\(a\\([bc]\\)\\)").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_0(_p.as_pointer(), 0_u32)
    });
    ({
        let _p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"(a([bc]))").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_0(_p.as_pointer(), 2_u32)
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
