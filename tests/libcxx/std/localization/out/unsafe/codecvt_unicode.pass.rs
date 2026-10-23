extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_ok {
    pub in_size: (),
    pub out_size: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_partial {
    pub in_size: (),
    pub out_size: (),
    pub expected_in_next: (),
    pub expected_out_next: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_error_unsigned_char_ {
    pub in_size: (),
    pub out_size: (),
    pub expected_in_next: (),
    pub expected_out_next: (),
    pub replace_char: (),
    pub replace_pos: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_error_char16_t_ {
    pub in_size: (),
    pub out_size: (),
    pub expected_in_next: (),
    pub expected_out_next: (),
    pub replace_char: (),
    pub replace_pos: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_error_char32_t_ {
    pub in_size: (),
    pub out_size: (),
    pub expected_in_next: (),
    pub expected_out_next: (),
    pub replace_char: (),
    pub replace_pos: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_error_wchar_t_ {
    pub in_size: (),
    pub out_size: (),
    pub expected_in_next: (),
    pub expected_out_next: (),
    pub replace_char: (),
    pub replace_pos: (),
}
pub unsafe fn utf8_to_utf32_in_ok_0(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 4_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf32_in_ok_1(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 4_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf32_in_ok_2(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 4_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf32_in_partial_3(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 16] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf32_in_partial_4(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 16] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf32_in_partial_5(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 16] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf32_in_error_6(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [('b' as ()), 1096_(), 55040_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 255_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 170_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 170_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 170_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 170_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: ('z' as ()),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 160_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 172_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 176_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 188_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 192_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 193_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 224_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 240_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 245_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 154_(),
            replace_pos: 7_(),
        },
    ];
    let mut it: *mut test_offsets_error_unsigned_char_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_unsigned_char_ = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf32_in_error_7(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [('b' as ()), 1096_(), 55040_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 255_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 170_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 170_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 170_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 170_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: ('z' as ()),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 160_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 172_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 176_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 188_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 192_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 193_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 224_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 240_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 245_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 154_(),
            replace_pos: 7_(),
        },
    ];
    let mut it: *mut test_offsets_error_unsigned_char_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_unsigned_char_ = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf32_in_error_8(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [('b' as ()), 1096_(), 55040_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 255_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 170_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 170_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 170_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 170_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: ('z' as ()),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 160_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 172_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 176_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 188_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 192_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 193_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 224_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 240_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 245_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 154_(),
            replace_pos: 7_(),
        },
    ];
    let mut it: *mut test_offsets_error_unsigned_char_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_unsigned_char_ = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf32_in_9(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_ok_0(cvt) });
    (unsafe { utf8_to_utf32_in_partial_3(cvt) });
    (unsafe { utf8_to_utf32_in_error_6(cvt) });
}
pub unsafe fn utf8_to_utf32_in_10(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_ok_1(cvt) });
    (unsafe { utf8_to_utf32_in_partial_4(cvt) });
    (unsafe { utf8_to_utf32_in_error_7(cvt) });
}
pub unsafe fn utf8_to_utf32_in_11(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_ok_2(cvt) });
    (unsafe { utf8_to_utf32_in_partial_5(cvt) });
    (unsafe { utf8_to_utf32_in_error_8(cvt) });
}
pub unsafe fn utf32_to_utf8_out_ok_12(cvt: *const ()) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 10_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf8_out_ok_13(cvt: *const ()) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 10_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf8_out_ok_14(cvt: *const ()) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 10_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf8_out_partial_15(cvt: *const ()) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 10] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf8_out_partial_16(cvt: *const ()) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 10] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf8_out_partial_17(cvt: *const ()) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 10] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf8_out_error_18(cvt: *const ()) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_char32_t_; 8] = [
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 1114112_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 1114112_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 1114112_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 1114112_(),
            replace_pos: 3_(),
        },
    ];
    let mut it: *mut test_offsets_error_char32_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char32_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf8_out_error_19(cvt: *const ()) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_wchar_t_; 8] = [
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 1114112_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 1114112_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 1114112_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 1114112_(),
            replace_pos: 3_(),
        },
    ];
    let mut it: *mut test_offsets_error_wchar_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_wchar_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf8_out_error_20(cvt: *const ()) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_char32_t_; 8] = [
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 1114112_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 1114112_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 1114112_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 1114112_(),
            replace_pos: 3_(),
        },
    ];
    let mut it: *mut test_offsets_error_char32_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char32_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf8_out_21(cvt: *const ()) {
    (unsafe { utf32_to_utf8_out_ok_12(cvt) });
    (unsafe { utf32_to_utf8_out_partial_15(cvt) });
    (unsafe { utf32_to_utf8_out_error_18(cvt) });
}
pub unsafe fn utf32_to_utf8_out_22(cvt: *const ()) {
    (unsafe { utf32_to_utf8_out_ok_13(cvt) });
    (unsafe { utf32_to_utf8_out_partial_16(cvt) });
    (unsafe { utf32_to_utf8_out_error_19(cvt) });
}
pub unsafe fn utf32_to_utf8_out_23(cvt: *const ()) {
    (unsafe { utf32_to_utf8_out_ok_14(cvt) });
    (unsafe { utf32_to_utf8_out_partial_17(cvt) });
    (unsafe { utf32_to_utf8_out_error_20(cvt) });
}
pub unsafe fn test_utf8_utf32_cvt_24(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_9(cvt) });
    (unsafe { utf32_to_utf8_out_21(cvt) });
}
pub unsafe fn test_utf8_utf32_cvt_25(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_10(cvt) });
    (unsafe { utf32_to_utf8_out_22(cvt) });
}
pub unsafe fn test_utf8_utf32_cvt_26(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_11(cvt) });
    (unsafe { utf32_to_utf8_out_23(cvt) });
}
pub unsafe fn utf8_to_utf16_in_ok_27(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 5_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 6]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_ok_28(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 5_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 6]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_ok_29(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 5_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 6]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_ok_30(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 5_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 6]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_partial_31(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 20] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_partial_32(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 20] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_partial_33(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 20] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_partial_34(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 20] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_error_35(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 55040_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 255_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 170_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 170_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 170_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 170_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: ('z' as ()),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 160_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 172_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 176_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 188_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 192_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 193_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 224_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 240_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 245_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 154_(),
            replace_pos: 7_(),
        },
    ];
    let mut it: *mut test_offsets_error_unsigned_char_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_unsigned_char_ = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_error_36(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 55040_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 255_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 170_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 170_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 170_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 170_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: ('z' as ()),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 160_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 172_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 176_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 188_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 192_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 193_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 224_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 240_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 245_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 154_(),
            replace_pos: 7_(),
        },
    ];
    let mut it: *mut test_offsets_error_unsigned_char_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_unsigned_char_ = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_error_37(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 55040_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 255_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 170_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 170_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 170_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 170_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: ('z' as ()),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 160_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 172_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 176_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 188_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 192_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 193_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 224_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 240_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 245_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 154_(),
            replace_pos: 7_(),
        },
    ];
    let mut it: *mut test_offsets_error_unsigned_char_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_unsigned_char_ = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_error_38(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 55040_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 255_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 170_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 170_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 170_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 170_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: ('z' as ()),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 160_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 172_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 176_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 188_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 192_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 193_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 224_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 240_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 245_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 154_(),
            replace_pos: 7_(),
        },
    ];
    let mut it: *mut test_offsets_error_unsigned_char_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_unsigned_char_ = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_utf16_in_39(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_ok_27(cvt) });
    (unsafe { utf8_to_utf16_in_partial_31(cvt) });
    (unsafe { utf8_to_utf16_in_error_35(cvt) });
}
pub unsafe fn utf8_to_utf16_in_40(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_ok_28(cvt) });
    (unsafe { utf8_to_utf16_in_partial_32(cvt) });
    (unsafe { utf8_to_utf16_in_error_36(cvt) });
}
pub unsafe fn utf8_to_utf16_in_41(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_ok_29(cvt) });
    (unsafe { utf8_to_utf16_in_partial_33(cvt) });
    (unsafe { utf8_to_utf16_in_error_37(cvt) });
}
pub unsafe fn utf8_to_utf16_in_42(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_ok_30(cvt) });
    (unsafe { utf8_to_utf16_in_partial_34(cvt) });
    (unsafe { utf8_to_utf16_in_error_38(cvt) });
}
pub unsafe fn utf16_to_utf8_out_ok_43(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 5_(),
            out_size: 10_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_ok_44(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 5_(),
            out_size: 10_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_ok_45(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 5_(),
            out_size: 10_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_ok_46(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 5_(),
            out_size: 10_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_partial_47(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 15] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_partial_48(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 15] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_partial_49(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 15] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_partial_50(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 15] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_error_51(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_char16_t_; 17] = [
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
    ];
    let mut it: *mut test_offsets_error_char16_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char16_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_error_52(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_char32_t_; 17] = [
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
    ];
    let mut it: *mut test_offsets_error_char32_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char32_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_error_53(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_wchar_t_; 17] = [
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
    ];
    let mut it: *mut test_offsets_error_wchar_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_wchar_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_error_54(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_char16_t_; 17] = [
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
    ];
    let mut it: *mut test_offsets_error_char16_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char16_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf8_out_55(cvt: *const ()) {
    (unsafe { utf16_to_utf8_out_ok_43(cvt) });
    (unsafe { utf16_to_utf8_out_partial_47(cvt) });
    (unsafe { utf16_to_utf8_out_error_51(cvt) });
}
pub unsafe fn utf16_to_utf8_out_56(cvt: *const ()) {
    (unsafe { utf16_to_utf8_out_ok_44(cvt) });
    (unsafe { utf16_to_utf8_out_partial_48(cvt) });
    (unsafe { utf16_to_utf8_out_error_52(cvt) });
}
pub unsafe fn utf16_to_utf8_out_57(cvt: *const ()) {
    (unsafe { utf16_to_utf8_out_ok_45(cvt) });
    (unsafe { utf16_to_utf8_out_partial_49(cvt) });
    (unsafe { utf16_to_utf8_out_error_53(cvt) });
}
pub unsafe fn utf16_to_utf8_out_58(cvt: *const ()) {
    (unsafe { utf16_to_utf8_out_ok_46(cvt) });
    (unsafe { utf16_to_utf8_out_partial_50(cvt) });
    (unsafe { utf16_to_utf8_out_error_54(cvt) });
}
pub unsafe fn test_utf8_utf16_cvt_59(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_39(cvt) });
    (unsafe { utf16_to_utf8_out_55(cvt) });
}
pub unsafe fn test_utf8_utf16_cvt_60(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_40(cvt) });
    (unsafe { utf16_to_utf8_out_56(cvt) });
}
pub unsafe fn test_utf8_utf16_cvt_61(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_41(cvt) });
    (unsafe { utf16_to_utf8_out_57(cvt) });
}
pub unsafe fn test_utf8_utf16_cvt_62(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_42(cvt) });
    (unsafe { utf16_to_utf8_out_58(cvt) });
}
pub unsafe fn utf8_to_ucs2_in_ok_63(cvt: *const ()) {
    let input: [(); 7] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\0");
    let expected: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 7");;
    const _: () = assert!(true, "array_size(expected) == 4");;
    let mut in_: [(); 7] = [unimplemented!(); 7];
    let mut exp: [(); 4] = [unimplemented!(); 4];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 4] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 3]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_ucs2_in_partial_64(cvt: *const ()) {
    let input: [(); 7] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\0");
    let expected: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 7");;
    const _: () = assert!(true, "array_size(expected) == 4");;
    let mut in_: [(); 7] = [unimplemented!(); 7];
    let mut exp: [(); 4] = [unimplemented!(); 4];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 9] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 3]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_ucs2_in_error_65(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [('b' as ()), 1096_(), 55040_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 11");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_unsigned_char_; 46] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 255_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 170_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 170_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 170_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 170_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: ('z' as ()),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 160_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 172_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 176_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 188_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 192_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 193_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 224_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 240_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 245_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 154_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 7_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
    ];
    let mut it: *mut test_offsets_error_unsigned_char_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_unsigned_char_ = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf8_to_ucs2_in_66(cvt: *const ()) {
    (unsafe { utf8_to_ucs2_in_ok_63(cvt) });
    (unsafe { utf8_to_ucs2_in_partial_64(cvt) });
    (unsafe { utf8_to_ucs2_in_error_65(cvt) });
}
pub unsafe fn ucs2_to_utf8_out_ok_67(cvt: *const ()) {
    let input: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    let expected: [(); 7] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 4");;
    const _: () = assert!(true, "array_size(expected) == 7");;
    let mut in_: [(); 4] = [unimplemented!(); 4];
    let mut exp: [(); 7] = [unimplemented!(); 7];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_ok; 4] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 6]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn ucs2_to_utf8_out_partial_68(cvt: *const ()) {
    let input: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    let expected: [(); 7] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 4");;
    const _: () = assert!(true, "array_size(expected) == 7");;
    let mut in_: [(); 4] = [unimplemented!(); 4];
    let mut exp: [(); 7] = [unimplemented!(); 7];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_partial; 6] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 6]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn ucs2_to_utf8_out_error_69(cvt: *const ()) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offsets: [test_offsets_error_char16_t_; 25] = [
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
    ];
    let mut it: *mut test_offsets_error_char16_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char16_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn ucs2_to_utf8_out_70(cvt: *const ()) {
    (unsafe { ucs2_to_utf8_out_ok_67(cvt) });
    (unsafe { ucs2_to_utf8_out_partial_68(cvt) });
    (unsafe { ucs2_to_utf8_out_error_69(cvt) });
}
pub unsafe fn test_utf8_ucs2_cvt_71(cvt: *const ()) {
    (unsafe { utf8_to_ucs2_in_66(cvt) });
    (unsafe { ucs2_to_utf8_out_70(cvt) });
}
pub type utf16_endianess = ();
pub const utf16_endianess_utf16_big_endian: utf16_endianess = 0;
pub const utf16_endianess_utf16_little_endian: utf16_endianess = 1;
pub unsafe fn utf16_to_bytes_72(
    mut f: *const (),
    mut l: *const (),
    mut o: *mut (),
    mut e: utf16_endianess,
) -> *mut () {
    if ((e as ()) == (utf16_endianess_utf16_big_endian as ())) {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = (((*f) >> (8_())) & (255_()));
            (*o.postfix_inc()) = ((*f) & (255_()));
            f.prefix_inc();
        }
    } else {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = ((*f) & (255_()));
            (*o.postfix_inc()) = (((*f) >> (8_())) & (255_()));
            f.prefix_inc();
        }
    }
    return o;
}
pub unsafe fn utf16_to_bytes_73(
    mut f: *mut (),
    mut l: *mut (),
    mut o: *mut (),
    mut e: utf16_endianess,
) -> *mut () {
    if ((e as ()) == (utf16_endianess_utf16_big_endian as ())) {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = (((*f) >> (8_())) & (255_()));
            (*o.postfix_inc()) = ((*f) & (255_()));
            f.prefix_inc();
        }
    } else {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = ((*f) & (255_()));
            (*o.postfix_inc()) = (((*f) >> (8_())) & (255_()));
            f.prefix_inc();
        }
    }
    return o;
}
pub unsafe fn utf16_to_utf32_in_ok_74(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 12] = [unimplemented!(); 12];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 4_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf32_in_ok_75(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 12] = [unimplemented!(); 12];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 4_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf32_in_partial_76(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 12] = [unimplemented!(); 12];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    unimplemented!();
    let mut offsets: [test_offsets_partial; 16] = [
        test_offsets_partial {
            in_size: 2_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 1_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 1_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 2_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf32_in_partial_77(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut in_: [(); 12] = [unimplemented!(); 12];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    unimplemented!();
    let mut offsets: [test_offsets_partial; 16] = [
        test_offsets_partial {
            in_size: 2_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 1_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 1_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 2_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf32_in_error_78(cvt: *const (), mut endianess: utf16_endianess) {
    let mut input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    let mut offsets: [test_offsets_error_char16_t_; 17] = [
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
    ];
    let mut it: *mut test_offsets_error_char16_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char16_t_ = (*it);
        let mut in_: [(); 12] = [unimplemented!(); 12];
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = input[(t.replace_pos) as usize];
        input[(t.replace_pos) as usize] = t.replace_char;
        (unsafe {
            let _f: *mut () = unimplemented!();
            let _l: *mut () = unimplemented!();
            utf16_to_bytes_73(_f, _l, unimplemented!(), endianess)
        });
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        input[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_utf32_in_error_79(cvt: *const (), mut endianess: utf16_endianess) {
    let mut input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 5");;
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    let mut offsets: [test_offsets_error_char16_t_; 17] = [
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
    ];
    let mut it: *mut test_offsets_error_char16_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char16_t_ = (*it);
        let mut in_: [(); 12] = [unimplemented!(); 12];
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = input[(t.replace_pos) as usize];
        input[(t.replace_pos) as usize] = t.replace_char;
        (unsafe {
            let _f: *mut () = unimplemented!();
            let _l: *mut () = unimplemented!();
            utf16_to_bytes_73(_f, _l, unimplemented!(), endianess)
        });
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        input[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf16_out_ok_80(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 12] = [unimplemented!(); 12];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 4_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 10_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf16_out_ok_81(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 12] = [unimplemented!(); 12];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 4_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 10_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf16_out_partial_82(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 12] = [unimplemented!(); 12];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_partial; 10] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 1_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 3_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf16_out_partial_83(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 12] = [unimplemented!(); 12];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_partial; 10] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 1_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 3_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf16_out_error_84(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 12] = [unimplemented!(); 12];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_error_char32_t_; 8] = [
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 1114112_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
            replace_char: 1114112_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
            replace_char: 1114112_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char32_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 1114112_(),
            replace_pos: 3_(),
        },
    ];
    let mut it: *mut test_offsets_error_char32_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char32_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn utf32_to_utf16_out_error_85(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 5] = [('b' as ()), 1096_(), 43690_(), 1092266_(), 0_()];
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 5");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 12] = [unimplemented!(); 12];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_error_wchar_t_; 8] = [
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 1114112_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
            replace_char: 1114112_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
            replace_char: 1114112_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 1114112_(),
            replace_pos: 3_(),
        },
    ];
    let mut it: *mut test_offsets_error_wchar_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_wchar_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn test_utf16_utf32_cvt_86(cvt: *const (), mut endianess: utf16_endianess) {
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_ok_74(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_partial_76(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_error_78(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_ok_80(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_partial_82(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_error_84(_cvt, _endianess)
    });
}
pub unsafe fn test_utf16_utf32_cvt_87(cvt: *const (), mut endianess: utf16_endianess) {
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_ok_75(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_partial_77(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_error_79(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_ok_81(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_partial_83(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_error_85(_cvt, _endianess)
    });
}
pub unsafe fn utf16_to_ucs2_in_ok_88(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    let expected: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 4");;
    const _: () = assert!(true, "array_size(expected) == 4");;
    let mut in_: [(); 8] = [unimplemented!(); 8];
    let mut exp: [(); 4] = [unimplemented!(); 4];
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    unimplemented!();
    let mut offsets: [test_offsets_ok; 4] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 3]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 4]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_ucs2_in_partial_89(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    let expected: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 4");;
    const _: () = assert!(true, "array_size(expected) == 4");;
    let mut in_: [(); 8] = [unimplemented!(); 8];
    let mut exp: [(); 4] = [unimplemented!(); 4];
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    unimplemented!();
    let mut offsets: [test_offsets_partial; 9] = [
        test_offsets_partial {
            in_size: 2_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 1_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 1_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 2_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 3]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        it.prefix_inc();
    }
}
pub unsafe fn utf16_to_ucs2_in_error_90(cvt: *const (), mut endianess: utf16_endianess) {
    let mut input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    let mut offsets: [test_offsets_error_char16_t_; 23] = [
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 6_(),
            out_size: 3_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
    ];
    let mut it: *mut test_offsets_error_char16_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char16_t_ = (*it);
        let mut in_: [(); 12] = [unimplemented!(); 12];
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = input[(t.replace_pos) as usize];
        input[(t.replace_pos) as usize] = t.replace_char;
        (unsafe {
            let _f: *mut () = unimplemented!();
            let _l: *mut () = unimplemented!();
            utf16_to_bytes_73(_f, _l, unimplemented!(), endianess)
        });
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 5]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        state = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        input[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn ucs2_to_utf16_out_ok_91(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    let expected: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 4");;
    const _: () = assert!(true, "array_size(expected) == 4");;
    let mut in_: [(); 4] = [unimplemented!(); 4];
    let mut exp: [(); 8] = [unimplemented!(); 8];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_ok; 4] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 4_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
    ];
    let mut it: *mut test_offsets_ok = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_ok = (*it);
        let mut out: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.out_size)
            < ((::std::mem::size_of::<[(); 6]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn ucs2_to_utf16_out_partial_92(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    let expected: [(); 4] = [('b' as ()), 1096_(), 43690_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 4");;
    const _: () = assert!(true, "array_size(expected) == 4");;
    let mut in_: [(); 4] = [unimplemented!(); 4];
    let mut exp: [(); 8] = [unimplemented!(); 8];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_partial; 6] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 1_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 3_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
        },
    ];
    let mut it: *mut test_offsets_partial = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_partial = (*it);
        let mut out: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 6]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        it.prefix_inc();
    }
}
pub unsafe fn ucs2_to_utf16_out_error_93(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    let expected: [(); 6] = [('b' as ()), 1096_(), 43690_(), 56298_(), 57002_(), 0_()];
    const _: () = assert!(true, "array_size(input) == 6");;
    const _: () = assert!(true, "array_size(expected) == 6");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 12] = [unimplemented!(); 12];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_72(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_error_char16_t_; 23] = [
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 3_(),
            out_size: 6_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 5_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: ('b' as ()),
            replace_pos: 0_(),
        },
    ];
    let mut it: *mut test_offsets_error_char16_t_ = unimplemented!();
    'loop_: while ((it) != (unimplemented!())) {
        let mut t: test_offsets_error_char16_t_ = (*it);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = std_codecvt_base_result_ok;
        res = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if ((t.expected_out_next)
            < ((::std::mem::size_of::<[(); 10]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()))))
        {
            unimplemented!();
        }
        in_[(t.replace_pos) as usize] = old_char;
        it.prefix_inc();
    }
}
pub unsafe fn test_utf16_ucs2_cvt_94(cvt: *const (), mut endianess: utf16_endianess) {
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_ucs2_in_ok_88(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_ucs2_in_partial_89(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_ucs2_in_error_90(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        ucs2_to_utf16_out_ok_91(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        ucs2_to_utf16_out_partial_92(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        ucs2_to_utf16_out_error_93(_cvt, _endianess)
    });
}
pub unsafe fn test_utf8_utf32_codecvts_95() {
    let loc_c: *const () = unimplemented!();
    unimplemented!();
    let cvt: *const () = (unsafe { use_facet_96(loc_c) });
    (unsafe { test_utf8_utf32_cvt_24(cvt) });
    let mut cvt2: () = unimplemented!();
    (unsafe { test_utf8_utf32_cvt_24(&cvt2) });
    let mut cvt3: () = unimplemented!();
    (unsafe { test_utf8_utf32_cvt_25(&cvt3) });
    unimplemented!();
    let cvt4: *const () = (unsafe { use_facet_97(loc_c) });
    (unsafe { test_utf8_utf32_cvt_26(cvt4) });
}
pub unsafe fn test_utf8_utf16_codecvts_98() {
    let loc_c: *const () = unimplemented!();
    unimplemented!();
    let cvt: *const () = (unsafe { use_facet_99(loc_c) });
    (unsafe { test_utf8_utf16_cvt_59(cvt) });
    let mut cvt2: () = unimplemented!();
    (unsafe { test_utf8_utf16_cvt_59(&cvt2) });
    let mut cvt3: () = unimplemented!();
    (unsafe { test_utf8_utf16_cvt_60(&cvt3) });
    let mut cvt4: () = unimplemented!();
    (unsafe { test_utf8_utf16_cvt_61(&cvt4) });
    unimplemented!();
    let cvt5: *const () = (unsafe { use_facet_100(loc_c) });
    (unsafe { test_utf8_utf16_cvt_62(cvt5) });
}
pub unsafe fn test_utf8_ucs2_codecvts_101() {
    let mut cvt: () = unimplemented!();
    (unsafe { test_utf8_ucs2_cvt_71(&cvt) });
}
pub unsafe fn test_utf16_utf32_codecvts_102() {
    let mut cvt: () = unimplemented!();
    (unsafe { test_utf16_utf32_cvt_86(&cvt, utf16_endianess_utf16_big_endian) });
    let mut cvt2: () = unimplemented!();
    (unsafe { test_utf16_utf32_cvt_86(&cvt2, utf16_endianess_utf16_little_endian) });
    let mut cvt3: () = unimplemented!();
    (unsafe { test_utf16_utf32_cvt_87(&cvt3, utf16_endianess_utf16_big_endian) });
    let mut cvt4: () = unimplemented!();
    (unsafe { test_utf16_utf32_cvt_87(&cvt4, utf16_endianess_utf16_little_endian) });
}
pub unsafe fn test_utf16_ucs2_codecvts_103() {
    let mut cvt: () = unimplemented!();
    (unsafe { test_utf16_ucs2_cvt_94(&cvt, utf16_endianess_utf16_big_endian) });
    let mut cvt2: () = unimplemented!();
    (unsafe { test_utf16_ucs2_cvt_94(&cvt2, utf16_endianess_utf16_little_endian) });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_utf8_utf32_codecvts_95() });
    (unsafe { test_utf8_utf16_codecvts_98() });
    (unsafe { test_utf8_ucs2_codecvts_101() });
    (unsafe { test_utf16_utf32_codecvts_102() });
    (unsafe { test_utf16_ucs2_codecvts_103() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
