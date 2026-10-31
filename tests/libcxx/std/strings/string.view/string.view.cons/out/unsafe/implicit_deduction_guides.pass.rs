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
pub struct test_macros_detail_is_same_std_basic_string_view_char__std_char_traits_char____std_basic_string_view_char__std_char_traits_char___
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_view_wchar_t__constexpr_char_traits_wchar_t____std_basic_string_view_wchar_t__constexpr_char_traits_wchar_t___
{}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_view_wchar_t__std_char_traits_wchar_t____std_basic_string_view_wchar_t__std_char_traits_wchar_t___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct constexpr_char_traits_wchar_t_ {}
impl constexpr_char_traits_wchar_t_ {
    pub unsafe fn eq(mut c1: i32, mut c2: i32) -> bool {
        return ((c1) == (c2));
    }
    pub unsafe fn lt(mut c1: i32, mut c2: i32) -> bool {
        return ((c1) < (c2));
    }
    pub unsafe fn compare(mut s1: *const i32, mut s2: *const i32, mut n: usize) -> i32 {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: i32 = (*s1);
                let _c2: i32 = (*s2);
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return -1_i32;
            }
            if (unsafe {
                let _c1: i32 = (*s2);
                let _c2: i32 = (*s1);
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return 1;
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0;
    }
    pub unsafe fn length(mut s: *const i32) -> usize {
        let mut len: usize = 0_usize;
        'loop_: while !(unsafe {
            let _c1: i32 = (*s);
            let _c2: i32 = 0_i32;
            constexpr_char_traits_wchar_t_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
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
    {}
    {
        let sin: () = unimplemented!();
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        let win: () = unimplemented!();
        let mut w: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        let mut w: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        let mut w: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
