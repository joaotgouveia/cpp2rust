extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_volatile_char_ptr__volatile_char_ptr_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_volatile_char_ptr__const_volatile_char_ptr_ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_volatile_int_ptr__volatile_int_ptr_ {}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_volatile_int_ptr__const_volatile_int_ptr_ {}
pub type anon_8 = u32;
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_double_ptr__double_ptr_ {}
pub type anon_9 = u32;
pub const anon_9_value: anon_9 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_double_ptr__const_double_ptr_ {}
pub type anon_10 = u32;
pub const anon_10_value: anon_10 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_volatile_double_ptr__volatile_double_ptr_ {}
pub type anon_11 = u32;
pub const anon_11_value: anon_11 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_volatile_double_ptr__const_volatile_double_ptr_ {}
const _: () = assert!(!(false), "!has_midpoint<std::nullptr_t>");
const _: () = assert!(!(false), "!has_midpoint<FuncPtr>");
const _: () = assert!(true, "");
const _: () = assert!(!(false), "!has_midpoint<void*>");
const _: () = assert!(!(false), "!has_midpoint<const void*>");
const _: () = assert!(!(false), "!has_midpoint<volatile void*>");
const _: () = assert!(!(false), "!has_midpoint<const volatile void*>");
pub unsafe fn check_12(
    mut base: *mut libc::c_char,
    mut i: isize,
    mut j: isize,
    mut expect: isize,
) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_13(
    mut base: *const libc::c_char,
    mut i: isize,
    mut j: isize,
    mut expect: isize,
) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_14(
    mut base: *mut libc::c_char,
    mut i: isize,
    mut j: isize,
    mut expect: isize,
) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_15(
    mut base: *const libc::c_char,
    mut i: isize,
    mut j: isize,
    mut expect: isize,
) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_16(mut base: *mut i32, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_17(
    mut base: *const i32,
    mut i: isize,
    mut j: isize,
    mut expect: isize,
) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_18(mut base: *mut i32, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_19(
    mut base: *const i32,
    mut i: isize,
    mut j: isize,
    mut expect: isize,
) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_20(mut base: *mut f64, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_21(
    mut base: *const f64,
    mut i: isize,
    mut j: isize,
    mut expect: isize,
) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_22(mut base: *mut f64, mut i: isize, mut j: isize, mut expect: isize) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn check_23(
    mut base: *const f64,
    mut i: isize,
    mut j: isize,
    mut expect: isize,
) -> bool {
    return ((unimplemented!()) == (base.offset((expect) as isize)));
}
pub unsafe fn test_pointer_24() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [libc::c_char; 20] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!((unsafe { check_12(array.as_mut_ptr(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_12(array.as_mut_ptr(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_12(array.as_mut_ptr(), 0_isize, 9_isize, 4_isize,) }));
    assert!((unsafe { check_12(array.as_mut_ptr(), 0_isize, 10_isize, 5_isize,) }));
    assert!((unsafe { check_12(array.as_mut_ptr(), 0_isize, 11_isize, 5_isize,) }));
    assert!((unsafe { check_12(array.as_mut_ptr(), 9_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_12(array.as_mut_ptr(), 10_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_12(array.as_mut_ptr(), 11_isize, 0_isize, 6_isize,) }));
    assert!((unsafe { check_12(array.as_mut_ptr(), 0_isize, 18_isize, 9_isize,) }));
    assert!((unsafe { check_12(array.as_mut_ptr(), 2_isize, 12_isize, 7_isize,) }));
    return true;
}
pub unsafe fn test_pointer_25() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [libc::c_char; 20] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!((unsafe { check_13((array.as_mut_ptr()).cast_const(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_13((array.as_mut_ptr()).cast_const(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_13((array.as_mut_ptr()).cast_const(), 0_isize, 9_isize, 4_isize,) }));
    assert!(
        (unsafe {
            check_13(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                10_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_13(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                11_isize,
                5_isize,
            )
        })
    );
    assert!((unsafe { check_13((array.as_mut_ptr()).cast_const(), 9_isize, 0_isize, 5_isize,) }));
    assert!(
        (unsafe {
            check_13(
                (array.as_mut_ptr()).cast_const(),
                10_isize,
                0_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_13(
                (array.as_mut_ptr()).cast_const(),
                11_isize,
                0_isize,
                6_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_13(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                18_isize,
                9_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_13(
                (array.as_mut_ptr()).cast_const(),
                2_isize,
                12_isize,
                7_isize,
            )
        })
    );
    return true;
}
pub unsafe fn test_pointer_26() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [libc::c_char; 20] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!((unsafe { check_14(array.as_mut_ptr(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_14(array.as_mut_ptr(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_14(array.as_mut_ptr(), 0_isize, 9_isize, 4_isize,) }));
    assert!((unsafe { check_14(array.as_mut_ptr(), 0_isize, 10_isize, 5_isize,) }));
    assert!((unsafe { check_14(array.as_mut_ptr(), 0_isize, 11_isize, 5_isize,) }));
    assert!((unsafe { check_14(array.as_mut_ptr(), 9_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_14(array.as_mut_ptr(), 10_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_14(array.as_mut_ptr(), 11_isize, 0_isize, 6_isize,) }));
    assert!((unsafe { check_14(array.as_mut_ptr(), 0_isize, 18_isize, 9_isize,) }));
    assert!((unsafe { check_14(array.as_mut_ptr(), 2_isize, 12_isize, 7_isize,) }));
    return true;
}
pub unsafe fn test_pointer_27() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [libc::c_char; 20] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!((unsafe { check_15((array.as_mut_ptr()).cast_const(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_15((array.as_mut_ptr()).cast_const(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_15((array.as_mut_ptr()).cast_const(), 0_isize, 9_isize, 4_isize,) }));
    assert!(
        (unsafe {
            check_15(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                10_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_15(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                11_isize,
                5_isize,
            )
        })
    );
    assert!((unsafe { check_15((array.as_mut_ptr()).cast_const(), 9_isize, 0_isize, 5_isize,) }));
    assert!(
        (unsafe {
            check_15(
                (array.as_mut_ptr()).cast_const(),
                10_isize,
                0_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_15(
                (array.as_mut_ptr()).cast_const(),
                11_isize,
                0_isize,
                6_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_15(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                18_isize,
                9_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_15(
                (array.as_mut_ptr()).cast_const(),
                2_isize,
                12_isize,
                7_isize,
            )
        })
    );
    return true;
}
pub unsafe fn test_pointer_28() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [i32; 20] = [
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
    ];
    assert!((unsafe { check_16(array.as_mut_ptr(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_16(array.as_mut_ptr(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_16(array.as_mut_ptr(), 0_isize, 9_isize, 4_isize,) }));
    assert!((unsafe { check_16(array.as_mut_ptr(), 0_isize, 10_isize, 5_isize,) }));
    assert!((unsafe { check_16(array.as_mut_ptr(), 0_isize, 11_isize, 5_isize,) }));
    assert!((unsafe { check_16(array.as_mut_ptr(), 9_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_16(array.as_mut_ptr(), 10_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_16(array.as_mut_ptr(), 11_isize, 0_isize, 6_isize,) }));
    assert!((unsafe { check_16(array.as_mut_ptr(), 0_isize, 18_isize, 9_isize,) }));
    assert!((unsafe { check_16(array.as_mut_ptr(), 2_isize, 12_isize, 7_isize,) }));
    return true;
}
pub unsafe fn test_pointer_29() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [i32; 20] = [
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
    ];
    assert!((unsafe { check_17((array.as_mut_ptr()).cast_const(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_17((array.as_mut_ptr()).cast_const(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_17((array.as_mut_ptr()).cast_const(), 0_isize, 9_isize, 4_isize,) }));
    assert!(
        (unsafe {
            check_17(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                10_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_17(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                11_isize,
                5_isize,
            )
        })
    );
    assert!((unsafe { check_17((array.as_mut_ptr()).cast_const(), 9_isize, 0_isize, 5_isize,) }));
    assert!(
        (unsafe {
            check_17(
                (array.as_mut_ptr()).cast_const(),
                10_isize,
                0_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_17(
                (array.as_mut_ptr()).cast_const(),
                11_isize,
                0_isize,
                6_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_17(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                18_isize,
                9_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_17(
                (array.as_mut_ptr()).cast_const(),
                2_isize,
                12_isize,
                7_isize,
            )
        })
    );
    return true;
}
pub unsafe fn test_pointer_30() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [i32; 20] = [
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
    ];
    assert!((unsafe { check_18(array.as_mut_ptr(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_18(array.as_mut_ptr(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_18(array.as_mut_ptr(), 0_isize, 9_isize, 4_isize,) }));
    assert!((unsafe { check_18(array.as_mut_ptr(), 0_isize, 10_isize, 5_isize,) }));
    assert!((unsafe { check_18(array.as_mut_ptr(), 0_isize, 11_isize, 5_isize,) }));
    assert!((unsafe { check_18(array.as_mut_ptr(), 9_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_18(array.as_mut_ptr(), 10_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_18(array.as_mut_ptr(), 11_isize, 0_isize, 6_isize,) }));
    assert!((unsafe { check_18(array.as_mut_ptr(), 0_isize, 18_isize, 9_isize,) }));
    assert!((unsafe { check_18(array.as_mut_ptr(), 2_isize, 12_isize, 7_isize,) }));
    return true;
}
pub unsafe fn test_pointer_31() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [i32; 20] = [
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
    ];
    assert!((unsafe { check_19((array.as_mut_ptr()).cast_const(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_19((array.as_mut_ptr()).cast_const(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_19((array.as_mut_ptr()).cast_const(), 0_isize, 9_isize, 4_isize,) }));
    assert!(
        (unsafe {
            check_19(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                10_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_19(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                11_isize,
                5_isize,
            )
        })
    );
    assert!((unsafe { check_19((array.as_mut_ptr()).cast_const(), 9_isize, 0_isize, 5_isize,) }));
    assert!(
        (unsafe {
            check_19(
                (array.as_mut_ptr()).cast_const(),
                10_isize,
                0_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_19(
                (array.as_mut_ptr()).cast_const(),
                11_isize,
                0_isize,
                6_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_19(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                18_isize,
                9_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_19(
                (array.as_mut_ptr()).cast_const(),
                2_isize,
                12_isize,
                7_isize,
            )
        })
    );
    return true;
}
pub unsafe fn test_pointer_32() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [f64; 20] = [
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
    ];
    assert!((unsafe { check_20(array.as_mut_ptr(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_20(array.as_mut_ptr(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_20(array.as_mut_ptr(), 0_isize, 9_isize, 4_isize,) }));
    assert!((unsafe { check_20(array.as_mut_ptr(), 0_isize, 10_isize, 5_isize,) }));
    assert!((unsafe { check_20(array.as_mut_ptr(), 0_isize, 11_isize, 5_isize,) }));
    assert!((unsafe { check_20(array.as_mut_ptr(), 9_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_20(array.as_mut_ptr(), 10_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_20(array.as_mut_ptr(), 11_isize, 0_isize, 6_isize,) }));
    assert!((unsafe { check_20(array.as_mut_ptr(), 0_isize, 18_isize, 9_isize,) }));
    assert!((unsafe { check_20(array.as_mut_ptr(), 2_isize, 12_isize, 7_isize,) }));
    return true;
}
pub unsafe fn test_pointer_33() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [f64; 20] = [
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
    ];
    assert!((unsafe { check_21((array.as_mut_ptr()).cast_const(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_21((array.as_mut_ptr()).cast_const(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_21((array.as_mut_ptr()).cast_const(), 0_isize, 9_isize, 4_isize,) }));
    assert!(
        (unsafe {
            check_21(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                10_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_21(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                11_isize,
                5_isize,
            )
        })
    );
    assert!((unsafe { check_21((array.as_mut_ptr()).cast_const(), 9_isize, 0_isize, 5_isize,) }));
    assert!(
        (unsafe {
            check_21(
                (array.as_mut_ptr()).cast_const(),
                10_isize,
                0_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_21(
                (array.as_mut_ptr()).cast_const(),
                11_isize,
                0_isize,
                6_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_21(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                18_isize,
                9_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_21(
                (array.as_mut_ptr()).cast_const(),
                2_isize,
                12_isize,
                7_isize,
            )
        })
    );
    return true;
}
pub unsafe fn test_pointer_34() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [f64; 20] = [
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
    ];
    assert!((unsafe { check_22(array.as_mut_ptr(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_22(array.as_mut_ptr(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_22(array.as_mut_ptr(), 0_isize, 9_isize, 4_isize,) }));
    assert!((unsafe { check_22(array.as_mut_ptr(), 0_isize, 10_isize, 5_isize,) }));
    assert!((unsafe { check_22(array.as_mut_ptr(), 0_isize, 11_isize, 5_isize,) }));
    assert!((unsafe { check_22(array.as_mut_ptr(), 9_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_22(array.as_mut_ptr(), 10_isize, 0_isize, 5_isize,) }));
    assert!((unsafe { check_22(array.as_mut_ptr(), 11_isize, 0_isize, 6_isize,) }));
    assert!((unsafe { check_22(array.as_mut_ptr(), 0_isize, 18_isize, 9_isize,) }));
    assert!((unsafe { check_22(array.as_mut_ptr(), 2_isize, 12_isize, 7_isize,) }));
    return true;
}
pub unsafe fn test_pointer_35() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut array: [f64; 20] = [
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
    ];
    assert!((unsafe { check_23((array.as_mut_ptr()).cast_const(), 0_isize, 0_isize, 0_isize,) }));
    assert!((unsafe { check_23((array.as_mut_ptr()).cast_const(), 1_isize, 1_isize, 1_isize,) }));
    assert!((unsafe { check_23((array.as_mut_ptr()).cast_const(), 0_isize, 9_isize, 4_isize,) }));
    assert!(
        (unsafe {
            check_23(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                10_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_23(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                11_isize,
                5_isize,
            )
        })
    );
    assert!((unsafe { check_23((array.as_mut_ptr()).cast_const(), 9_isize, 0_isize, 5_isize,) }));
    assert!(
        (unsafe {
            check_23(
                (array.as_mut_ptr()).cast_const(),
                10_isize,
                0_isize,
                5_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_23(
                (array.as_mut_ptr()).cast_const(),
                11_isize,
                0_isize,
                6_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_23(
                (array.as_mut_ptr()).cast_const(),
                0_isize,
                18_isize,
                9_isize,
            )
        })
    );
    assert!(
        (unsafe {
            check_23(
                (array.as_mut_ptr()).cast_const(),
                2_isize,
                12_isize,
                7_isize,
            )
        })
    );
    return true;
}
pub unsafe fn test_36() {
    assert!((unsafe { test_pointer_24() }));
    assert!((unsafe { test_pointer_25() }));
    assert!((unsafe { test_pointer_26() }));
    assert!((unsafe { test_pointer_27() }));
    const _: () = assert!(true, "test_pointer<T>()");;
    const _: () = assert!(true, "test_pointer<const T>()");;
}
pub unsafe fn test_37() {
    assert!((unsafe { test_pointer_28() }));
    assert!((unsafe { test_pointer_29() }));
    assert!((unsafe { test_pointer_30() }));
    assert!((unsafe { test_pointer_31() }));
    const _: () = assert!(true, "test_pointer<T>()");;
    const _: () = assert!(true, "test_pointer<const T>()");;
}
pub unsafe fn test_38() {
    assert!((unsafe { test_pointer_32() }));
    assert!((unsafe { test_pointer_33() }));
    assert!((unsafe { test_pointer_34() }));
    assert!((unsafe { test_pointer_35() }));
    const _: () = assert!(true, "test_pointer<T>()");;
    const _: () = assert!(true, "test_pointer<const T>()");;
}
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_36() });
    (unsafe { test_37() });
    (unsafe { test_38() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
