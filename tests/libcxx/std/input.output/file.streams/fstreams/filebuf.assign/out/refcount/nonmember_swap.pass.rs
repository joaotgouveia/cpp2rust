extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_temp_file_name_0() -> Vec<i8> {
    let Name: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"libcxx.XXXXXX").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let mut FD: i32 = unimplemented!();
    if (FD == -1_i32) {
        unimplemented!();
        std::process::abort();
    }
    FdRegistry::close(FD);
    return ({ std::mem::take(&mut (*Name.borrow_mut())) });
}
pub fn glibc_version_less_than_1(mut version: Ptr<i8>) -> bool {
    let test_version: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __tmp2 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"glibc ").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        __tmp2.pop();
        version.with_c_str(|__s| __tmp2.extend_from_slice(__s));
        __tmp2.push(0_i8);
        __tmp2
    }));
    let mut n: usize = unimplemented!();
    let mut current_version: Ptr<i8> =
        Ptr::alloc_array((0..n).map(|_| 0_i8).collect::<Box<[i8]>>());
    unimplemented!();
    let result: Value<bool> = Rc::new(RefCell::new((unimplemented!() < 0)));
    current_version.delete();
    return (*result.borrow());
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
    let temp: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_0() })));
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        assert!((unimplemented!() == 3_()));
        unimplemented!();
        assert!((unimplemented!() == (('2' as i8) as i32)));
        let f2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!((unimplemented!() == (('2' as i8) as i32)));
    }
    unimplemented!();
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        assert!((unimplemented!() == 3_()));
        unimplemented!();
        assert!((unimplemented!() == (('2' as i8) as i32)));
        let f2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!((unimplemented!() == (('2' as i8) as i32)));
    }
    unimplemented!();
    {
        let tmpA: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_0() })));
        let tmpB: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_0() })));
        {
            let sa: Value<::std::fs::File> = Rc::new(RefCell::new(unimplemented!()));
            let sb: Value<::std::fs::File> = Rc::new(RefCell::new(unimplemented!()));
            {
                let __s = sa.as_pointer();
                let _ = Ptr::<i8>::from_string_literal(b"0123456789").with_c_str(|__cs| {
                    __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                });
                __s
            };
            {
                let __s = sb.as_pointer();
                let _ = Ptr::<i8>::from_string_literal(b"abcdefghij").with_c_str(|__cs| {
                    __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                });
                __s
            };
        }
        let f1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let f2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(!((unimplemented!()).is_null()));
        assert!(!((unimplemented!()).is_null()));
        assert!((unimplemented!() == (('0' as i8) as i32)));
        assert!((unimplemented!() == (('a' as i8) as i32)));
        unimplemented!();
        assert!((unimplemented!() == (('a' as i8) as i32)));
        assert!((unimplemented!() == (('0' as i8) as i32)));
        unimplemented!();
        unimplemented!();
    }
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        assert!((unimplemented!() == 3_()));
        unimplemented!();
        assert!((unimplemented!() == ((50 as i32) as ())));
        let f2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!((unimplemented!() == ((50 as i32) as ())));
    }
    unimplemented!();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
