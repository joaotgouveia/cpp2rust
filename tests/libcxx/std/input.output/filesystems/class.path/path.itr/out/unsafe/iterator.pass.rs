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
pub struct test_macros_detail_is_same_std_filesystem_path_iterator__std_filesystem_path_iterator_ {
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_filesystem_path__std_filesystem_path_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_filesystem_path_iterator_ref__std_filesystem_path_iterator_ref_
{}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_filesystem_path_ref__const_std_filesystem_path_ref_
{}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_filesystem_path_ptr__const_std_filesystem_path_ptr_
{}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ref__const_std_basic_string_char__std_char_traits_char___std_allocator_char___ref_
{}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub unsafe fn checkCollectionsEqual_7(
    mut start1: (),
    end1: (),
    mut start2: *const Vec<libc::c_char>,
    end2: *const Vec<libc::c_char>,
) -> bool {
    'loop_: while (start1 != end1) && ((start2) != (end2)) {
        if !(unimplemented!()) {
            return false;
        }
        start1.iter_inc();
        start2.prefix_inc();
    }
    return ((start1 == end1) && ((start2) == (end2)));
}
pub unsafe fn checkCollectionsEqualBackwards_8(
    start1: (),
    mut end1: (),
    start2: *const Vec<libc::c_char>,
    mut end2: *const Vec<libc::c_char>,
) -> bool {
    'loop_: while (start1 != end1) && ((start2) != (end2)) {
        end1.iter_dec();
        end2.prefix_dec();
        if !(unimplemented!()) {
            return false;
        }
    }
    return ((start1 == end1) && ((start2) == (end2)));
}
pub unsafe fn checkIteratorConcepts_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::bidirectional_iterator<It>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let mut it: () = unimplemented!();
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
        let p: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == unimplemented!());
    }
}
pub unsafe fn checkBeginEndBasic_10() {
    {
        let p: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == unimplemented!());
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"foo"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        let mut default_constructed: () = unimplemented!();
        default_constructed = unimplemented!().clone();
        assert!(default_constructed == unimplemented!());
        assert!(default_constructed != unimplemented!());
        default_constructed = unimplemented!().clone();
        assert!(default_constructed == unimplemented!());
        assert!(default_constructed != unimplemented!());
    }
    {
        let mut p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"//root_name//first_dir////second_dir"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        let expect: [Vec<libc::c_char>; 4] = [
            {
                let mut __v: Vec<libc::c_char> = c"/"
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as libc::c_char)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<libc::c_char> = c"root_name"
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as libc::c_char)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<libc::c_char> = c"first_dir"
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as libc::c_char)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<libc::c_char> = c"second_dir"
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as libc::c_char)
                    .collect();
                __v.push(0);
                __v
            },
        ];
        assert!(
            (unsafe {
                let _start1: () = unimplemented!();
                let _end1: () = unimplemented!();
                let _start2: *const Vec<libc::c_char> = unimplemented!();
                let _end2: *const Vec<libc::c_char> = expect.as_ptr().add(expect.len());
                checkCollectionsEqual_7(_start1, _end1, _start2, _end2)
            })
        );
        assert!(
            (unsafe {
                let _start1: () = unimplemented!();
                let _end1: () = unimplemented!();
                let _start2: *const Vec<libc::c_char> = unimplemented!();
                let _end2: *const Vec<libc::c_char> = expect.as_ptr().add(expect.len());
                checkCollectionsEqualBackwards_8(_start1, _end1, _start2, _end2)
            })
        );
    }
    {
        let mut p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"////foo/bar/baz///"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        let expect: [Vec<libc::c_char>; 5] = [
            {
                let mut __v: Vec<libc::c_char> = c"/"
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as libc::c_char)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<libc::c_char> = c"foo"
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as libc::c_char)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<libc::c_char> = c"bar"
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as libc::c_char)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<libc::c_char> = c"baz"
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as libc::c_char)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<libc::c_char> = c""
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as libc::c_char)
                    .collect();
                __v.push(0);
                __v
            },
        ];
        assert!(
            (unsafe {
                let _start1: () = unimplemented!();
                let _end1: () = unimplemented!();
                let _start2: *const Vec<libc::c_char> = unimplemented!();
                let _end2: *const Vec<libc::c_char> = expect.as_ptr().add(expect.len());
                checkCollectionsEqual_7(_start1, _end1, _start2, _end2)
            })
        );
        assert!(
            (unsafe {
                let _start1: () = unimplemented!();
                let _end1: () = unimplemented!();
                let _start2: *const Vec<libc::c_char> = unimplemented!();
                let _end2: *const Vec<libc::c_char> = expect.as_ptr().add(expect.len());
                checkCollectionsEqualBackwards_8(_start1, _end1, _start2, _end2)
            })
        );
    }
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
    (unsafe { checkIteratorConcepts_9() });
    (unsafe { checkBeginEndBasic_10() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
