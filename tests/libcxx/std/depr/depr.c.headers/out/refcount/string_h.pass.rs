extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_void_ptr__void_ptr_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_void_ptr__const_void_ptr_ {}
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
        let mut s: usize = 0_usize;
        let mut vp: AnyPtr = Ptr::<()>::null();
        let mut vpc: AnyPtr = Ptr::<()>::null();
        let mut cp: Ptr<i8> = Ptr::<i8>::null();
        let mut cpc: Ptr<i8> = Ptr::<i8>::null();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let storage: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"hello world\0")));
        let mut s: Ptr<i8> = (storage.as_pointer() as Ptr<i8>);
        const _: () = assert!(true, "");;
        let mut res: Ptr<i8> = {
            let __s = (s).clone();
            let __t = (('l' as i8) as i32) as i8;
            match __s.to_c_string_iterator().position(|__c| __c == __t) {
                Some(__i) => __s.offset(__i),
                None => {
                    if __t == 0 {
                        __s.offset(__s.to_c_string_iterator().count())
                    } else {
                        Ptr::null()
                    }
                }
            }
        };
        assert!(({ (res).clone() } == { (s.offset((2) as isize)) }));
    }
    {
        let storage: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"hello world\0")));
        let mut s: Ptr<i8> = (storage.as_pointer() as Ptr<i8>);
        const _: () = assert!(true, "");;
        let mut res: Ptr<i8> = ({ strchr_6((s).clone(), (('l' as i8) as i32)) });
        assert!(({ (res).clone() } == { (s.offset((2) as isize)) }));
    }
    {
        let storage: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"hello world\0")));
        let mut s: Ptr<i8> = (storage.as_pointer() as Ptr<i8>);
        const _: () = assert!(true, "");;
        let mut res: Ptr<i8> = {
            let __s = (s).clone();
            let __set = Ptr::<i8>::from_string_literal(b"el");
            match __s
                .to_c_string_iterator()
                .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
            {
                Some(__i) => __s.offset(__i),
                None => Ptr::null(),
            }
        };
        assert!(({ (res).clone() } == { (s.offset((1) as isize)) }));
    }
    {
        let storage: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"hello world\0")));
        let mut s: Ptr<i8> = (storage.as_pointer() as Ptr<i8>);
        const _: () = assert!(true, "");;
        let mut res: Ptr<i8> = {
            let __s = (s).clone();
            let __set = Ptr::<i8>::from_string_literal(b"el");
            match __s
                .to_c_string_iterator()
                .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
            {
                Some(__i) => __s.offset(__i),
                None => Ptr::null(),
            }
        };
        assert!(({ (res).clone() } == { (s.offset((1) as isize)) }));
    }
    {
        let storage: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"hello world\0")));
        let mut s: Ptr<i8> = (storage.as_pointer() as Ptr<i8>);
        const _: () = assert!(true, "");;
        let mut res: Ptr<i8> = {
            let __s = (s).clone();
            let __t = (('l' as i8) as i32) as i8;
            match __s
                .to_c_string_iterator()
                .enumerate()
                .filter(|__e| __e.1 == __t)
                .last()
            {
                Some((__i, _)) => __s.offset(__i),
                None => {
                    if __t == 0 {
                        __s.offset(__s.to_c_string_iterator().count())
                    } else {
                        Ptr::null()
                    }
                }
            }
        };
        assert!(({ (res).clone() } == { (s.offset((9) as isize)) }));
    }
    {
        let storage: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"hello world\0")));
        let mut s: Ptr<i8> = (storage.as_pointer() as Ptr<i8>);
        const _: () = assert!(true, "");;
        let mut res: Ptr<i8> = {
            let __s = (s).clone();
            let __t = (('l' as i8) as i32) as i8;
            match __s
                .to_c_string_iterator()
                .enumerate()
                .filter(|__e| __e.1 == __t)
                .last()
            {
                Some((__i, _)) => __s.offset(__i),
                None => {
                    if __t == 0 {
                        __s.offset(__s.to_c_string_iterator().count())
                    } else {
                        Ptr::null()
                    }
                }
            }
        };
        assert!(({ (res).clone() } == { (s.offset((9) as isize)) }));
    }
    {
        let storage: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"hello world\0")));
        let mut count: usize = 11_usize;
        let mut s: AnyPtr = ((storage.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
        const _: () = assert!(true, "");;
        let mut res: AnyPtr = {
            let mut __p = s.reinterpret_cast::<u8>();
            let mut __i: usize = 0;
            loop {
                if __i == count {
                    break Ptr::<u8>::null().to_any();
                }
                if __p.read() == (('l' as i8) as i32) as u8 {
                    break __p.to_any();
                }
                __p += 1;
                __i += 1;
            }
        };
        assert!(
            ({ (res).clone() } == {
                (((storage.as_pointer() as Ptr<i8>).offset(2)) as Ptr<i8>).to_any()
            })
        );
    }
    {
        let storage: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"hello world\0")));
        let mut count: usize = 11_usize;
        let mut s: AnyPtr = ((storage.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
        const _: () = assert!(true, "");;
        let mut res: AnyPtr = ({ memchr_7((s).clone(), (('l' as i8) as i32), count) });
        assert!(
            ({ (res).clone() } == {
                (((storage.as_pointer() as Ptr<i8>).offset(2)) as Ptr<i8>).to_any()
            })
        );
    }
    {
        let storage: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"hello world\0")));
        let mut s: Ptr<i8> = (storage.as_pointer() as Ptr<i8>);
        const _: () = assert!(true, "");;
        let mut res: Ptr<i8> = {
            let __needle = Ptr::<i8>::from_string_literal(b"wor");
            let mut __p = (s).clone();
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
        };
        assert!(({ (res).clone() } == { ((storage.as_pointer() as Ptr<i8>).offset(6)) }));
    }
    {
        let storage: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"hello world\0")));
        let mut s: Ptr<i8> = (storage.as_pointer() as Ptr<i8>);
        const _: () = assert!(true, "");;
        let mut res: Ptr<i8> = {
            let __needle = Ptr::<i8>::from_string_literal(b"wor");
            let mut __p = (s).clone();
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
        };
        assert!(({ (res).clone() } == { ((storage.as_pointer() as Ptr<i8>).offset(6)) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
