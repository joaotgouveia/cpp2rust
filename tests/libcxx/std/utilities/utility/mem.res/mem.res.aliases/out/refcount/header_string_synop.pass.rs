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
pub struct constexpr_char_traits_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char16_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char32_t_ {}
pub fn test_string_typedef_0() {
    const _: () = assert!(true, "std::is_same<StdStr, PmrStr>::value");;
    const _: () = assert!(true, "std::is_same<PmrStr, PmrTypedef>::value");;
}
pub fn test_string_typedef_1() {
    const _: () = assert!(true, "std::is_same<StdStr, PmrStr>::value");;
    const _: () = assert!(true, "std::is_same<PmrStr, PmrTypedef>::value");;
}
pub fn test_string_typedef_2() {
    const _: () = assert!(true, "std::is_same<StdStr, PmrStr>::value");;
    const _: () = assert!(true, "std::is_same<PmrStr, PmrTypedef>::value");;
}
pub fn test_string_typedef_3() {
    const _: () = assert!(true, "std::is_same<StdStr, PmrStr>::value");;
    const _: () = assert!(true, "std::is_same<PmrStr, PmrTypedef>::value");;
}
pub fn test_basic_string_alias_4() {
    const _: () = assert!(true, "std::is_same<StdStr, PmrStr>::value");;
}
pub fn test_basic_string_alias_5() {
    const _: () = assert!(true, "std::is_same<StdStr, PmrStr>::value");;
}
pub fn test_basic_string_alias_6() {
    const _: () = assert!(true, "std::is_same<StdStr, PmrStr>::value");;
}
pub fn test_basic_string_alias_7() {
    const _: () = assert!(true, "std::is_same<StdStr, PmrStr>::value");;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        ({ test_string_typedef_0() });
        ({ test_string_typedef_1() });
        ({ test_string_typedef_2() });
        ({ test_string_typedef_3() });
    }
    {
        ({ test_basic_string_alias_4() });
        ({ test_basic_string_alias_5() });
        ({ test_basic_string_alias_6() });
        ({ test_basic_string_alias_7() });
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
