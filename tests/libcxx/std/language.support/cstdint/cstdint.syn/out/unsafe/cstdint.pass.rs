extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    const _: () = assert!(true, "sizeof(std::int8_t)*CHAR_BIT == 8");;
    const _: () = assert!(true, "std::is_signed<std::int8_t>::value");;
    const _: () = assert!(true, "sizeof(std::int16_t)*CHAR_BIT == 16");;
    const _: () = assert!(true, "std::is_signed<std::int16_t>::value");;
    const _: () = assert!(true, "sizeof(std::int32_t)*CHAR_BIT == 32");;
    const _: () = assert!(true, "std::is_signed<std::int32_t>::value");;
    const _: () = assert!(true, "sizeof(std::int64_t)*CHAR_BIT == 64");;
    const _: () = assert!(true, "std::is_signed<std::int64_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint8_t)*CHAR_BIT == 8");;
    const _: () = assert!(true, "std::is_unsigned<std::uint8_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint16_t)*CHAR_BIT == 16");;
    const _: () = assert!(true, "std::is_unsigned<std::uint16_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint32_t)*CHAR_BIT == 32");;
    const _: () = assert!(true, "std::is_unsigned<std::uint32_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint64_t)*CHAR_BIT == 64");;
    const _: () = assert!(true, "std::is_unsigned<std::uint64_t>::value");;
    const _: () = assert!(true, "sizeof(std::int_least8_t)*CHAR_BIT >= 8");;
    const _: () = assert!(true, "std::is_signed<std::int_least8_t>::value");;
    const _: () = assert!(true, "sizeof(std::int_least16_t)*CHAR_BIT >= 16");;
    const _: () = assert!(true, "std::is_signed<std::int_least16_t>::value");;
    const _: () = assert!(true, "sizeof(std::int_least32_t)*CHAR_BIT >= 32");;
    const _: () = assert!(true, "std::is_signed<std::int_least32_t>::value");;
    const _: () = assert!(true, "sizeof(std::int_least64_t)*CHAR_BIT >= 64");;
    const _: () = assert!(true, "std::is_signed<std::int_least64_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint_least8_t)*CHAR_BIT >= 8");;
    const _: () = assert!(true, "std::is_unsigned<std::uint_least8_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint_least16_t)*CHAR_BIT >= 16");;
    const _: () = assert!(true, "std::is_unsigned<std::uint_least16_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint_least32_t)*CHAR_BIT >= 32");;
    const _: () = assert!(true, "std::is_unsigned<std::uint_least32_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint_least64_t)*CHAR_BIT >= 64");;
    const _: () = assert!(true, "std::is_unsigned<std::uint_least64_t>::value");;
    const _: () = assert!(true, "sizeof(std::int_fast8_t)*CHAR_BIT >= 8");;
    const _: () = assert!(true, "std::is_signed<std::int_fast8_t>::value");;
    const _: () = assert!(true, "sizeof(std::int_fast16_t)*CHAR_BIT >= 16");;
    const _: () = assert!(true, "std::is_signed<std::int_fast16_t>::value");;
    const _: () = assert!(true, "sizeof(std::int_fast32_t)*CHAR_BIT >= 32");;
    const _: () = assert!(true, "std::is_signed<std::int_fast32_t>::value");;
    const _: () = assert!(true, "sizeof(std::int_fast64_t)*CHAR_BIT >= 64");;
    const _: () = assert!(true, "std::is_signed<std::int_fast64_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint_fast8_t)*CHAR_BIT >= 8");;
    const _: () = assert!(true, "std::is_unsigned<std::uint_fast8_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint_fast16_t)*CHAR_BIT >= 16");;
    const _: () = assert!(true, "std::is_unsigned<std::uint_fast16_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint_fast32_t)*CHAR_BIT >= 32");;
    const _: () = assert!(true, "std::is_unsigned<std::uint_fast32_t>::value");;
    const _: () = assert!(true, "sizeof(std::uint_fast64_t)*CHAR_BIT >= 64");;
    const _: () = assert!(true, "std::is_unsigned<std::uint_fast64_t>::value");;
    const _: () = assert!(
        ((::std::mem::size_of::<()>()) >= (::std::mem::size_of::<()>())),
        "sizeof(std::intptr_t) >= sizeof(void*)"
    );;
    const _: () = assert!(true, "std::is_signed<std::intptr_t>::value");;
    const _: () = assert!(
        ((::std::mem::size_of::<usize>()) >= (::std::mem::size_of::<()>())),
        "sizeof(std::uintptr_t) >= sizeof(void*)"
    );;
    const _: () = assert!(true, "std::is_unsigned<std::uintptr_t>::value");;
    const _: () = assert!(
        ((::std::mem::size_of::<i64>()) >= (::std::mem::size_of::<()>())),
        "sizeof(std::intmax_t) >= sizeof(long long)"
    );;
    const _: () = assert!(true, "std::is_signed<std::intmax_t>::value");;
    const _: () = assert!(
        ((::std::mem::size_of::<u64>()) >= (::std::mem::size_of::<()>())),
        "sizeof(std::uintmax_t) >= sizeof(unsigned long long)"
    );;
    const _: () = assert!(true, "std::is_unsigned<std::uintmax_t>::value");;
    const _: () = assert!(true, "INT8_MIN == -128");;
    const _: () = assert!(true, "INT16_MIN == -32768");;
    const _: () = assert!(true, "INT32_MIN == -2147483647 - 1");;
    const _: () = assert!(true, "INT64_MIN == -9223372036854775807LL - 1");;
    const _: () = assert!(((127_()) == (127_())), "INT8_MAX == 127");;
    const _: () = assert!(((32767_()) == (32767_())), "INT16_MAX == 32767");;
    const _: () = assert!(
        ((2147483647_()) == (2147483647_())),
        "INT32_MAX == 2147483647"
    );;
    const _: () = assert!(
        ((9223372036854775807_()) == (9223372036854775807_())),
        "INT64_MAX == 9223372036854775807LL"
    );;
    const _: () = assert!(((255_()) == (255_())), "UINT8_MAX == 255");;
    const _: () = assert!(((65535_()) == (65535_())), "UINT16_MAX == 65535");;
    const _: () = assert!(
        ((4294967295_()) == (4294967295_())),
        "UINT32_MAX == 4294967295U"
    );;
    const _: () = assert!(
        ((18446744073709551615_()) == (18446744073709551615_())),
        "UINT64_MAX == 18446744073709551615ULL"
    );;
    const _: () = assert!(true, "INT_FAST8_MIN <= -128");;
    const _: () = assert!(true, "INT_FAST16_MIN <= -32768");;
    const _: () = assert!(true, "INT_FAST32_MIN <= -2147483647 - 1");;
    const _: () = assert!(true, "INT_FAST64_MIN <= -9223372036854775807LL - 1");;
    const _: () = assert!(((127_()) >= (127_())), "INT_FAST8_MAX >= 127");;
    const _: () = assert!(
        ((9223372036854775807_()) >= (32767_())),
        "INT_FAST16_MAX >= 32767"
    );;
    const _: () = assert!(
        ((9223372036854775807_()) >= (2147483647_())),
        "INT_FAST32_MAX >= 2147483647"
    );;
    const _: () = assert!(
        ((9223372036854775807_()) >= (9223372036854775807_())),
        "INT_FAST64_MAX >= 9223372036854775807LL"
    );;
    const _: () = assert!(((255_()) >= (255_())), "UINT_FAST8_MAX >= 255");;
    const _: () = assert!(
        ((18446744073709551615_()) >= (65535_())),
        "UINT_FAST16_MAX >= 65535"
    );;
    const _: () = assert!(
        ((18446744073709551615_()) >= (4294967295_())),
        "UINT_FAST32_MAX >= 4294967295U"
    );;
    const _: () = assert!(
        ((18446744073709551615_()) >= (18446744073709551615_())),
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
pub unsafe fn __cpp2rust_init_globals() {}
