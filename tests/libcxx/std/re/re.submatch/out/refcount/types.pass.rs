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
        const _: () = assert!(true, "(std::is_same<SM::iterator, char*>::value)");;
        const _: () = assert!(true, "(std::is_same<SM::value_type, char>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<SM::difference_type, std::ptrdiff_t>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<SM::string_type, std::string>::value)");;
        const _: () = assert!(
            true,
            "(std::is_convertible<SM*, std::pair<char*, char*>*>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<std::csub_match, std::sub_match<const char*> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<std::ssub_match, std::sub_match<std::string::const_iterator> >::value)"
        );;
        let sm: Value<std_sub_match_char_ptr_> =
            Rc::new(RefCell::new(<std_sub_match_char_ptr_>::default()));
        (*({ ((*sm.borrow()) as (Value<*mut i8>, Value<*mut i8>)) })
            .0
            .borrow_mut()) = Ptr::<i8>::null();
        (*({ ((*sm.borrow()) as (Value<*mut i8>, Value<*mut i8>)) })
            .1
            .borrow_mut()) = Ptr::<i8>::null();
        (*sm.borrow_mut()).matched = false;
    }
    {
        const _: () = assert!(true, "(std::is_same<SM::iterator, wchar_t*>::value)");;
        const _: () = assert!(true, "(std::is_same<SM::value_type, wchar_t>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<SM::difference_type, std::ptrdiff_t>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<SM::string_type, std::wstring>::value)");;
        const _: () = assert!(
            true,
            "(std::is_convertible<SM*, std::pair<wchar_t*, wchar_t*>*>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<std::wcsub_match, std::sub_match<const wchar_t*> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<std::wssub_match, std::sub_match<std::wstring::const_iterator> >::value)"
        );;
        let sm: Value<std_sub_match_wchar_t_ptr_> =
            Rc::new(RefCell::new(<std_sub_match_wchar_t_ptr_>::default()));
        (*({ ((*sm.borrow()) as (Value<*mut i32>, Value<*mut i32>)) })
            .0
            .borrow_mut()) = Ptr::<i32>::null();
        (*({ ((*sm.borrow()) as (Value<*mut i32>, Value<*mut i32>)) })
            .1
            .borrow_mut()) = Ptr::<i32>::null();
        (*sm.borrow_mut()).matched = false;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
