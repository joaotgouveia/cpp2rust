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
pub struct test_macros_detail_is_same_int__int_ {}
impl CppMove for test_macros_detail_is_same_int__int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_short__unsigned_short_ {}
impl CppMove for test_macros_detail_is_same_unsigned_short__unsigned_short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_long__long_long_ {}
impl CppMove for test_macros_detail_is_same_long_long__long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type e_default = i32;
pub const e_default_a: e_default = 0;
pub const e_default_b: e_default = 1;
pub const e_default_c: e_default = 2;
pub type e_ushort = u16;
pub const e_ushort_d: e_ushort = 10;
pub const e_ushort_e: e_ushort = 25;
pub const e_ushort_f: e_ushort = 50;
pub type e_longlong = i64;
pub const e_longlong_low: e_longlong = -9223372036854775808;
pub const e_longlong_high: e_longlong = 9223372036854775807;
pub type e_non_class = u32;
pub const e_non_class_enum_a: e_non_class = 10;
pub const e_non_class_enum_b: e_non_class = 11;
pub const e_non_class_enum_c: e_non_class = 12;
pub type e_int = i32;
pub const e_int_enum_min: e_int = -2147483648;
pub const e_int_enum_max: e_int = 2147483647;
pub type e_bool = u8;
pub const e_bool_f: e_bool = 0;
pub const e_bool_t: e_bool = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct WithBitfieldEnums {
    #[offset(0)]
    #[byte_size(4)]
    pub e1: e_default,
    #[offset(0)]
    #[byte_size(2)]
    pub e2: e_ushort,
    #[offset(1)]
    #[byte_size(1)]
    pub e3: e_bool,
}
impl CppMove for WithBitfieldEnums {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_3() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ cpp2rust_assert_fail_4((0 == ({ to_underlying_5(e_default_a) }))) });
    ({ cpp2rust_assert_fail_4((1 == ({ to_underlying_5(e_default_b) }))) });
    ({ cpp2rust_assert_fail_4((2 == ({ to_underlying_5(e_default_c) }))) });
    ({ cpp2rust_assert_fail_4((10 == (({ to_underlying_6(e_ushort_d) }) as i32))) });
    ({ cpp2rust_assert_fail_4((25 == (({ to_underlying_6(e_ushort_e) }) as i32))) });
    ({ cpp2rust_assert_fail_4((50 == (({ to_underlying_6(e_ushort_f) }) as i32))) });
    ({
        cpp2rust_assert_fail_4(
            (({ std_numeric_limits_long_long_::min() }) == ({ to_underlying_7(e_longlong_low) })),
        )
    });
    ({
        cpp2rust_assert_fail_4(
            (({ std_numeric_limits_long_long_::max() }) == ({ to_underlying_7(e_longlong_high) })),
        )
    });
    ({ cpp2rust_assert_fail_4((10_u32 == ({ to_underlying_8(e_non_class_enum_a) }))) });
    ({ cpp2rust_assert_fail_4((11_u32 == ({ to_underlying_8(e_non_class_enum_b) }))) });
    ({ cpp2rust_assert_fail_4((12_u32 == ({ to_underlying_8(e_non_class_enum_c) }))) });
    ({
        cpp2rust_assert_fail_4(
            (({ std_numeric_limits_int_::min() }) == ({ to_underlying_9(e_int_enum_min) })),
        )
    });
    ({
        cpp2rust_assert_fail_4(
            (({ std_numeric_limits_int_::max() }) == ({ to_underlying_9(e_int_enum_max) })),
        )
    });
    let mut bf: WithBitfieldEnums = <WithBitfieldEnums>::default();
    bf.e1 = ((3) as e_default);
    bf.e2 = e_ushort_e;
    bf.e3 = e_bool_t;
    ({ cpp2rust_assert_fail_4((3 == ({ to_underlying_5(bf.e1) }))) });
    ({ cpp2rust_assert_fail_4((25 == (({ to_underlying_6(bf.e2) }) as i32))) });
    ({ cpp2rust_assert_fail_4((1 == (({ to_underlying_10(bf.e3) }) as i32))) });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_3() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
