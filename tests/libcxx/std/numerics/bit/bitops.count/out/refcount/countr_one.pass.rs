extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
pub type E1 = ();
pub const E1_rEd: E1 = 0;
pub type E2 = ();
pub const E2_red: E2 = 0;
pub fn test_1() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = ({ std_numeric_limits_unsigned_char_::max() });
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
    {}
    return true;
}
pub fn test_2() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = ({ std_numeric_limits_unsigned_short_::max() });
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
    {}
    return true;
}
pub fn test_3() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = ({ std_numeric_limits_unsigned_int_::max() });
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
    {}
    return true;
}
pub fn test_4() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = ({ std_numeric_limits_unsigned_long_::max() });
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
    {}
    return true;
}
pub fn test_5() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = ({ std_numeric_limits_unsigned_long_long_::max() });
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
    {}
    return true;
}
pub fn test_6() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = ({ std_numeric_limits_unsigned___int128_::max() });
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
    {
        let mut val: () = 128_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        val <<= 32_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        val <<= 60_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
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
        let lambda: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {})));
        const _: () = assert!(true, "!std::is_invocable_v<L, signed char>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, short>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, int>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, long>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, long long>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, __int128_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::int8_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::int16_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::int32_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::int64_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::intmax_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::intptr_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::ptrdiff_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, bool>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, char>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, wchar_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, char8_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, char16_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, char32_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, A>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, A*>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, E1>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, E2>");;
    }
    const _: () = assert!(true, "test<unsigned char>()");;
    const _: () = assert!(true, "test<unsigned short>()");;
    const _: () = assert!(true, "test<unsigned int>()");;
    const _: () = assert!(true, "test<unsigned long>()");;
    const _: () = assert!(true, "test<unsigned long long>()");;
    const _: () = assert!(true, "test<__uint128_t>()");;
    const _: () = assert!(true, "test<std::uint8_t>()");;
    const _: () = assert!(true, "test<std::uint16_t>()");;
    const _: () = assert!(true, "test<std::uint32_t>()");;
    const _: () = assert!(true, "test<std::uint64_t>()");;
    const _: () = assert!(true, "test<std::uintmax_t>()");;
    const _: () = assert!(true, "test<std::uintptr_t>()");;
    const _: () = assert!(true, "test<std::size_t>()");;
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    ({ test_6() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_4() });
    ({ test_4() });
    ({ test_4() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
