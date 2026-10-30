extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn use_0() {
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
    &(::std::mem::size_of::<()>());
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
    const _: () = assert!(true, "sizeof(int8_t)*CHAR_BIT == 8");;
    const _: () = assert!(true, "std::is_signed<int8_t>::value");;
    const _: () = assert!(true, "sizeof(int16_t)*CHAR_BIT == 16");;
    const _: () = assert!(true, "std::is_signed<int16_t>::value");;
    const _: () = assert!(true, "sizeof(int32_t)*CHAR_BIT == 32");;
    const _: () = assert!(true, "std::is_signed<int32_t>::value");;
    const _: () = assert!(true, "sizeof(int64_t)*CHAR_BIT == 64");;
    const _: () = assert!(true, "std::is_signed<int64_t>::value");;
    const _: () = assert!(true, "sizeof(uint8_t)*CHAR_BIT == 8");;
    const _: () = assert!(true, "std::is_unsigned<uint8_t>::value");;
    const _: () = assert!(true, "sizeof(uint16_t)*CHAR_BIT == 16");;
    const _: () = assert!(true, "std::is_unsigned<uint16_t>::value");;
    const _: () = assert!(true, "sizeof(uint32_t)*CHAR_BIT == 32");;
    const _: () = assert!(true, "std::is_unsigned<uint32_t>::value");;
    const _: () = assert!(true, "sizeof(uint64_t)*CHAR_BIT == 64");;
    const _: () = assert!(true, "std::is_unsigned<uint64_t>::value");;
    const _: () = assert!(true, "sizeof(int_least8_t)*CHAR_BIT >= 8");;
    const _: () = assert!(true, "std::is_signed<int_least8_t>::value");;
    const _: () = assert!(true, "sizeof(int_least16_t)*CHAR_BIT >= 16");;
    const _: () = assert!(true, "std::is_signed<int_least16_t>::value");;
    const _: () = assert!(true, "sizeof(int_least32_t)*CHAR_BIT >= 32");;
    const _: () = assert!(true, "std::is_signed<int_least32_t>::value");;
    const _: () = assert!(true, "sizeof(int_least64_t)*CHAR_BIT >= 64");;
    const _: () = assert!(true, "std::is_signed<int_least64_t>::value");;
    const _: () = assert!(true, "sizeof(uint_least8_t)*CHAR_BIT >= 8");;
    const _: () = assert!(true, "std::is_unsigned<uint_least8_t>::value");;
    const _: () = assert!(true, "sizeof(uint_least16_t)*CHAR_BIT >= 16");;
    const _: () = assert!(true, "std::is_unsigned<uint_least16_t>::value");;
    const _: () = assert!(true, "sizeof(uint_least32_t)*CHAR_BIT >= 32");;
    const _: () = assert!(true, "std::is_unsigned<uint_least32_t>::value");;
    const _: () = assert!(true, "sizeof(uint_least64_t)*CHAR_BIT >= 64");;
    const _: () = assert!(true, "std::is_unsigned<uint_least64_t>::value");;
    const _: () = assert!(true, "sizeof(int_fast8_t)*CHAR_BIT >= 8");;
    const _: () = assert!(true, "std::is_signed<int_fast8_t>::value");;
    const _: () = assert!(true, "sizeof(int_fast16_t)*CHAR_BIT >= 16");;
    const _: () = assert!(true, "std::is_signed<int_fast16_t>::value");;
    const _: () = assert!(true, "sizeof(int_fast32_t)*CHAR_BIT >= 32");;
    const _: () = assert!(true, "std::is_signed<int_fast32_t>::value");;
    const _: () = assert!(true, "sizeof(int_fast64_t)*CHAR_BIT >= 64");;
    const _: () = assert!(true, "std::is_signed<int_fast64_t>::value");;
    const _: () = assert!(true, "sizeof(uint_fast8_t)*CHAR_BIT >= 8");;
    const _: () = assert!(true, "std::is_unsigned<uint_fast8_t>::value");;
    const _: () = assert!(true, "sizeof(uint_fast16_t)*CHAR_BIT >= 16");;
    const _: () = assert!(true, "std::is_unsigned<uint_fast16_t>::value");;
    const _: () = assert!(true, "sizeof(uint_fast32_t)*CHAR_BIT >= 32");;
    const _: () = assert!(true, "std::is_unsigned<uint_fast32_t>::value");;
    const _: () = assert!(true, "sizeof(uint_fast64_t)*CHAR_BIT >= 64");;
    const _: () = assert!(true, "std::is_unsigned<uint_fast64_t>::value");;
    const _: () = assert!(
        (::std::mem::size_of::<()>() >= 8usize),
        "sizeof(intptr_t) >= sizeof(void*)"
    );;
    const _: () = assert!(true, "std::is_signed<intptr_t>::value");;
    const _: () = assert!(
        (::std::mem::size_of::<()>() >= 8usize),
        "sizeof(uintptr_t) >= sizeof(void*)"
    );;
    const _: () = assert!(true, "std::is_unsigned<uintptr_t>::value");;
    const _: () = assert!(
        (::std::mem::size_of::<()>() >= ::std::mem::size_of::<()>()),
        "sizeof(intmax_t) >= sizeof(long long)"
    );;
    const _: () = assert!(true, "std::is_signed<intmax_t>::value");;
    const _: () = assert!(
        (::std::mem::size_of::<()>() >= ::std::mem::size_of::<()>()),
        "sizeof(uintmax_t) >= sizeof(unsigned long long)"
    );;
    const _: () = assert!(true, "std::is_unsigned<uintmax_t>::value");;
    const _: () = assert!(true, "INT8_MIN == -128");;
    const _: () = assert!(true, "INT16_MIN == -32768");;
    const _: () = assert!(true, "INT32_MIN == -2147483647 - 1");;
    const _: () = assert!(true, "INT64_MIN == -9223372036854775807LL - 1");;
    const _: () = assert!(((127_()) == 127_()), "INT8_MAX == 127");;
    const _: () = assert!(((32767_()) == 32767_()), "INT16_MAX == 32767");;
    const _: () = assert!(
        ((2147483647_()) == 2147483647_()),
        "INT32_MAX == 2147483647"
    );;
    const _: () = assert!(
        ((9223372036854775807_()) == 9223372036854775807_()),
        "INT64_MAX == 9223372036854775807LL"
    );;
    const _: () = assert!(((255_()) == 255_()), "UINT8_MAX == 255");;
    const _: () = assert!(((65535_()) == 65535_()), "UINT16_MAX == 65535");;
    const _: () = assert!(
        ((4294967295_()) == 4294967295_()),
        "UINT32_MAX == 4294967295U"
    );;
    const _: () = assert!(
        ((18446744073709551615_()) == 18446744073709551615_()),
        "UINT64_MAX == 18446744073709551615ULL"
    );;
    const _: () = assert!(true, "INT_FAST8_MIN <= -128");;
    const _: () = assert!(true, "INT_FAST16_MIN <= -32768");;
    const _: () = assert!(true, "INT_FAST32_MIN <= -2147483647 - 1");;
    const _: () = assert!(true, "INT_FAST64_MIN <= -9223372036854775807LL - 1");;
    const _: () = assert!(((127_()) >= 127_()), "INT_FAST8_MAX >= 127");;
    const _: () = assert!(
        ((9223372036854775807_()) >= 32767_()),
        "INT_FAST16_MAX >= 32767"
    );;
    const _: () = assert!(
        ((9223372036854775807_()) >= 2147483647_()),
        "INT_FAST32_MAX >= 2147483647"
    );;
    const _: () = assert!(
        ((9223372036854775807_()) >= 9223372036854775807_()),
        "INT_FAST64_MAX >= 9223372036854775807LL"
    );;
    const _: () = assert!(((255_()) >= 255_()), "UINT_FAST8_MAX >= 255");;
    const _: () = assert!(
        ((18446744073709551615_()) >= 65535_()),
        "UINT_FAST16_MAX >= 65535"
    );;
    const _: () = assert!(
        ((18446744073709551615_()) >= 4294967295_()),
        "UINT_FAST32_MAX >= 4294967295U"
    );;
    const _: () = assert!(
        ((18446744073709551615_()) >= 18446744073709551615_()),
        "UINT_FAST64_MAX >= 18446744073709551615ULL"
    );;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
