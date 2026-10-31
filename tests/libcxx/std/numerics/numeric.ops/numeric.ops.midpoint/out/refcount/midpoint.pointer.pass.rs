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
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_volatile_char_ptr__volatile_char_ptr_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_volatile_char_ptr__const_volatile_char_ptr_ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_volatile_int_ptr__volatile_int_ptr_ {}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_volatile_int_ptr__const_volatile_int_ptr_ {}
pub type anon_8 = u32;
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double_ptr__double_ptr_ {}
pub type anon_9 = u32;
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_double_ptr__const_double_ptr_ {}
pub type anon_10 = u32;
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_volatile_double_ptr__volatile_double_ptr_ {}
pub type anon_11 = u32;
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_volatile_double_ptr__const_volatile_double_ptr_ {}
const _: () = assert!(!(false), "!has_midpoint<std::nullptr_t>");
const _: () = assert!(!(false), "!has_midpoint<FuncPtr>");
const _: () = assert!(true, "");
const _: () = assert!(!(false), "!has_midpoint<void*>");
const _: () = assert!(!(false), "!has_midpoint<const void*>");
const _: () = assert!(!(false), "!has_midpoint<volatile void*>");
const _: () = assert!(!(false), "!has_midpoint<const volatile void*>");
pub fn check_12(mut base: Ptr<i8>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_13(mut base: Ptr<i8>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_14(mut base: Ptr<i8>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_15(mut base: Ptr<i8>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_16(mut base: Ptr<i32>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_17(mut base: Ptr<i32>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_18(mut base: Ptr<i32>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_19(mut base: Ptr<i32>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_20(mut base: Ptr<f64>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_21(mut base: Ptr<f64>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_22(mut base: Ptr<f64>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn check_23(mut base: Ptr<f64>, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ({ unimplemented!() } == { base.offset((expect) as isize) });
}
pub fn test_pointer_24() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
    ])));
    assert!(({ check_12((array.as_pointer() as Ptr::<i8>), 0_isize, 0_isize, 0_isize,) }));
    assert!(({ check_12((array.as_pointer() as Ptr::<i8>), 1_isize, 1_isize, 1_isize,) }));
    assert!(({ check_12((array.as_pointer() as Ptr::<i8>), 0_isize, 9_isize, 4_isize,) }));
    assert!(
        ({
            check_12((array.as_pointer() as Ptr<i8>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_12((array.as_pointer() as Ptr<i8>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(({ check_12((array.as_pointer() as Ptr::<i8>), 9_isize, 0_isize, 5_isize,) }));
    assert!(
        ({
            check_12((array.as_pointer() as Ptr<i8>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_12((array.as_pointer() as Ptr<i8>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_12((array.as_pointer() as Ptr<i8>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_12((array.as_pointer() as Ptr<i8>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_25() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
    ])));
    assert!(({ check_13((array.as_pointer() as Ptr::<i8>), 0_isize, 0_isize, 0_isize,) }));
    assert!(({ check_13((array.as_pointer() as Ptr::<i8>), 1_isize, 1_isize, 1_isize,) }));
    assert!(({ check_13((array.as_pointer() as Ptr::<i8>), 0_isize, 9_isize, 4_isize,) }));
    assert!(
        ({
            check_13((array.as_pointer() as Ptr<i8>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_13((array.as_pointer() as Ptr<i8>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(({ check_13((array.as_pointer() as Ptr::<i8>), 9_isize, 0_isize, 5_isize,) }));
    assert!(
        ({
            check_13((array.as_pointer() as Ptr<i8>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_13((array.as_pointer() as Ptr<i8>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_13((array.as_pointer() as Ptr<i8>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_13((array.as_pointer() as Ptr<i8>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_26() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
    ])));
    assert!(({ check_14((array.as_pointer() as Ptr::<i8>), 0_isize, 0_isize, 0_isize,) }));
    assert!(({ check_14((array.as_pointer() as Ptr::<i8>), 1_isize, 1_isize, 1_isize,) }));
    assert!(({ check_14((array.as_pointer() as Ptr::<i8>), 0_isize, 9_isize, 4_isize,) }));
    assert!(
        ({
            check_14((array.as_pointer() as Ptr<i8>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_14((array.as_pointer() as Ptr<i8>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(({ check_14((array.as_pointer() as Ptr::<i8>), 9_isize, 0_isize, 5_isize,) }));
    assert!(
        ({
            check_14((array.as_pointer() as Ptr<i8>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_14((array.as_pointer() as Ptr<i8>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_14((array.as_pointer() as Ptr<i8>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_14((array.as_pointer() as Ptr<i8>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_27() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
    ])));
    assert!(({ check_15((array.as_pointer() as Ptr::<i8>), 0_isize, 0_isize, 0_isize,) }));
    assert!(({ check_15((array.as_pointer() as Ptr::<i8>), 1_isize, 1_isize, 1_isize,) }));
    assert!(({ check_15((array.as_pointer() as Ptr::<i8>), 0_isize, 9_isize, 4_isize,) }));
    assert!(
        ({
            check_15((array.as_pointer() as Ptr<i8>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_15((array.as_pointer() as Ptr<i8>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(({ check_15((array.as_pointer() as Ptr::<i8>), 9_isize, 0_isize, 5_isize,) }));
    assert!(
        ({
            check_15((array.as_pointer() as Ptr<i8>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_15((array.as_pointer() as Ptr<i8>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_15((array.as_pointer() as Ptr<i8>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_15((array.as_pointer() as Ptr<i8>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_28() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
    ])));
    assert!(
        ({
            check_16((array.as_pointer() as Ptr<i32>), 0_isize, 0_isize, 0_isize)
        })
    );
    assert!(
        ({
            check_16((array.as_pointer() as Ptr<i32>), 1_isize, 1_isize, 1_isize)
        })
    );
    assert!(
        ({
            check_16((array.as_pointer() as Ptr<i32>), 0_isize, 9_isize, 4_isize)
        })
    );
    assert!(
        ({
            check_16((array.as_pointer() as Ptr<i32>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_16((array.as_pointer() as Ptr<i32>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_16((array.as_pointer() as Ptr<i32>), 9_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_16((array.as_pointer() as Ptr<i32>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_16((array.as_pointer() as Ptr<i32>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_16((array.as_pointer() as Ptr<i32>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_16((array.as_pointer() as Ptr<i32>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_29() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
    ])));
    assert!(
        ({
            check_17((array.as_pointer() as Ptr<i32>), 0_isize, 0_isize, 0_isize)
        })
    );
    assert!(
        ({
            check_17((array.as_pointer() as Ptr<i32>), 1_isize, 1_isize, 1_isize)
        })
    );
    assert!(
        ({
            check_17((array.as_pointer() as Ptr<i32>), 0_isize, 9_isize, 4_isize)
        })
    );
    assert!(
        ({
            check_17((array.as_pointer() as Ptr<i32>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_17((array.as_pointer() as Ptr<i32>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_17((array.as_pointer() as Ptr<i32>), 9_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_17((array.as_pointer() as Ptr<i32>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_17((array.as_pointer() as Ptr<i32>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_17((array.as_pointer() as Ptr<i32>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_17((array.as_pointer() as Ptr<i32>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_30() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
    ])));
    assert!(
        ({
            check_18((array.as_pointer() as Ptr<i32>), 0_isize, 0_isize, 0_isize)
        })
    );
    assert!(
        ({
            check_18((array.as_pointer() as Ptr<i32>), 1_isize, 1_isize, 1_isize)
        })
    );
    assert!(
        ({
            check_18((array.as_pointer() as Ptr<i32>), 0_isize, 9_isize, 4_isize)
        })
    );
    assert!(
        ({
            check_18((array.as_pointer() as Ptr<i32>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_18((array.as_pointer() as Ptr<i32>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_18((array.as_pointer() as Ptr<i32>), 9_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_18((array.as_pointer() as Ptr<i32>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_18((array.as_pointer() as Ptr<i32>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_18((array.as_pointer() as Ptr<i32>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_18((array.as_pointer() as Ptr<i32>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_31() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
    ])));
    assert!(
        ({
            check_19((array.as_pointer() as Ptr<i32>), 0_isize, 0_isize, 0_isize)
        })
    );
    assert!(
        ({
            check_19((array.as_pointer() as Ptr<i32>), 1_isize, 1_isize, 1_isize)
        })
    );
    assert!(
        ({
            check_19((array.as_pointer() as Ptr<i32>), 0_isize, 9_isize, 4_isize)
        })
    );
    assert!(
        ({
            check_19((array.as_pointer() as Ptr<i32>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_19((array.as_pointer() as Ptr<i32>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_19((array.as_pointer() as Ptr<i32>), 9_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_19((array.as_pointer() as Ptr<i32>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_19((array.as_pointer() as Ptr<i32>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_19((array.as_pointer() as Ptr<i32>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_19((array.as_pointer() as Ptr<i32>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_32() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
    ])));
    assert!(
        ({
            check_20((array.as_pointer() as Ptr<f64>), 0_isize, 0_isize, 0_isize)
        })
    );
    assert!(
        ({
            check_20((array.as_pointer() as Ptr<f64>), 1_isize, 1_isize, 1_isize)
        })
    );
    assert!(
        ({
            check_20((array.as_pointer() as Ptr<f64>), 0_isize, 9_isize, 4_isize)
        })
    );
    assert!(
        ({
            check_20((array.as_pointer() as Ptr<f64>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_20((array.as_pointer() as Ptr<f64>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_20((array.as_pointer() as Ptr<f64>), 9_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_20((array.as_pointer() as Ptr<f64>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_20((array.as_pointer() as Ptr<f64>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_20((array.as_pointer() as Ptr<f64>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_20((array.as_pointer() as Ptr<f64>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_33() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
    ])));
    assert!(
        ({
            check_21((array.as_pointer() as Ptr<f64>), 0_isize, 0_isize, 0_isize)
        })
    );
    assert!(
        ({
            check_21((array.as_pointer() as Ptr<f64>), 1_isize, 1_isize, 1_isize)
        })
    );
    assert!(
        ({
            check_21((array.as_pointer() as Ptr<f64>), 0_isize, 9_isize, 4_isize)
        })
    );
    assert!(
        ({
            check_21((array.as_pointer() as Ptr<f64>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_21((array.as_pointer() as Ptr<f64>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_21((array.as_pointer() as Ptr<f64>), 9_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_21((array.as_pointer() as Ptr<f64>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_21((array.as_pointer() as Ptr<f64>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_21((array.as_pointer() as Ptr<f64>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_21((array.as_pointer() as Ptr<f64>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_34() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
    ])));
    assert!(
        ({
            check_22((array.as_pointer() as Ptr<f64>), 0_isize, 0_isize, 0_isize)
        })
    );
    assert!(
        ({
            check_22((array.as_pointer() as Ptr<f64>), 1_isize, 1_isize, 1_isize)
        })
    );
    assert!(
        ({
            check_22((array.as_pointer() as Ptr<f64>), 0_isize, 9_isize, 4_isize)
        })
    );
    assert!(
        ({
            check_22((array.as_pointer() as Ptr<f64>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_22((array.as_pointer() as Ptr<f64>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_22((array.as_pointer() as Ptr<f64>), 9_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_22((array.as_pointer() as Ptr<f64>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_22((array.as_pointer() as Ptr<f64>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_22((array.as_pointer() as Ptr<f64>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_22((array.as_pointer() as Ptr<f64>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_pointer_35() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
    ])));
    assert!(
        ({
            check_23((array.as_pointer() as Ptr<f64>), 0_isize, 0_isize, 0_isize)
        })
    );
    assert!(
        ({
            check_23((array.as_pointer() as Ptr<f64>), 1_isize, 1_isize, 1_isize)
        })
    );
    assert!(
        ({
            check_23((array.as_pointer() as Ptr<f64>), 0_isize, 9_isize, 4_isize)
        })
    );
    assert!(
        ({
            check_23((array.as_pointer() as Ptr<f64>), 0_isize, 10_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_23((array.as_pointer() as Ptr<f64>), 0_isize, 11_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_23((array.as_pointer() as Ptr<f64>), 9_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_23((array.as_pointer() as Ptr<f64>), 10_isize, 0_isize, 5_isize)
        })
    );
    assert!(
        ({
            check_23((array.as_pointer() as Ptr<f64>), 11_isize, 0_isize, 6_isize)
        })
    );
    assert!(
        ({
            check_23((array.as_pointer() as Ptr<f64>), 0_isize, 18_isize, 9_isize)
        })
    );
    assert!(
        ({
            check_23((array.as_pointer() as Ptr<f64>), 2_isize, 12_isize, 7_isize)
        })
    );
    return true;
}
pub fn test_36() {
    assert!(({ test_pointer_24() }));
    assert!(({ test_pointer_25() }));
    assert!(({ test_pointer_26() }));
    assert!(({ test_pointer_27() }));
    const _: () = assert!(true, "test_pointer<T>()");;
    const _: () = assert!(true, "test_pointer<const T>()");;
}
pub fn test_37() {
    assert!(({ test_pointer_28() }));
    assert!(({ test_pointer_29() }));
    assert!(({ test_pointer_30() }));
    assert!(({ test_pointer_31() }));
    const _: () = assert!(true, "test_pointer<T>()");;
    const _: () = assert!(true, "test_pointer<const T>()");;
}
pub fn test_38() {
    assert!(({ test_pointer_32() }));
    assert!(({ test_pointer_33() }));
    assert!(({ test_pointer_34() }));
    assert!(({ test_pointer_35() }));
    const _: () = assert!(true, "test_pointer<T>()");;
    const _: () = assert!(true, "test_pointer<const T>()");;
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
    ({ test_36() });
    ({ test_37() });
    ({ test_38() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
