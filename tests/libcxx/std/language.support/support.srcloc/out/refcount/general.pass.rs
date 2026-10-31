extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(
    true,
    "std::is_nothrow_move_constructible_v<std::source_location>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_move_assignable_v<std::source_location>"
);
const _: () = assert!(true, "std::is_nothrow_swappable_v<std::source_location>");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
thread_local!(
    pub static empty_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
const _: () = assert!(true, "empty.line() == 0");
const _: () = assert!(true, "empty.column() == 0");
const _: () = assert!(true, "empty.file_name()[0] == \'\\0\'");
const _: () = assert!(true, "empty.function_name()[0] == \'\\0\'");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
thread_local!(
    pub static line_1: Value<u32> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static column_2: Value<u32> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static file_3: Value<Ptr<i8>> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static function_4: Value<Ptr<i8>> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static cur_5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
const _: () = assert!(true, "cur.line() == 1000");
const _: () = assert!(true, "cur.column() > 0");
const _: () = assert!(
    true,
    "cur.file_name()[0] == \'s\' && cur.file_name()[1] == \'s\' && cur.file_name()[2] == \'\\0\'"
);
const _: () = assert!(true, "cur.function_name()[0] == \'\\0\'");
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
    let local: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({
            let mut __it1 = unimplemented!().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"ss").to_c_string_iterator();
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
    assert!(
        !(({
            let __needle = Ptr::<i8>::from_string_literal(b"main");
            let mut __p = unimplemented!();
            loop {
                let mut __h = __p.to_c_string_iterator();
                if __needle
                    .to_c_string_iterator()
                    .all(|__c| __h.next() == Some(__c))
                {
                    break __p;
                }
                if __p.read() == 0 {
                    break Ptr::null();
                }
                __p += 1;
            }
        })
        .is_null())
    );
    assert!((unimplemented!() == 2000_u32));
    assert!((unimplemented!() > 0_u32));
    let local2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({
            let mut __it1 = unimplemented!().to_c_string_iterator();
            let mut __it2 = unimplemented!().to_c_string_iterator();
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
    assert!(
        ({
            let mut __it1 = unimplemented!().to_c_string_iterator();
            let mut __it2 = unimplemented!().to_c_string_iterator();
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
    assert!((unimplemented!() == unimplemented!()));
    assert!((unimplemented!() == unimplemented!()));
    unimplemented!();
    assert!(
        ({
            let mut __it1 = unimplemented!().to_c_string_iterator();
            let mut __it2 = unimplemented!().to_c_string_iterator();
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
    assert!(
        ({
            let mut __it1 = unimplemented!().to_c_string_iterator();
            let mut __it2 = unimplemented!().to_c_string_iterator();
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
    assert!((unimplemented!() == unimplemented!()));
    assert!((unimplemented!() == unimplemented!()));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = empty_0.with(|_| ());
    let _ = line_1.with(|_| ());
    let _ = column_2.with(|_| ());
    let _ = file_3.with(|_| ());
    let _ = function_4.with(|_| ());
    let _ = cur_5.with(|_| ());
}
