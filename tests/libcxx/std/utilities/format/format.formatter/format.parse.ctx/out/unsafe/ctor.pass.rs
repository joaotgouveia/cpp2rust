extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut fmt: *const libc::c_char) {
    const _: () = assert!(
        true,
        "!std::is_convertible_v<std::basic_string_view<CharT>,\n                             std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_copy_constructible_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_copy_assignable_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_move_constructible_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_move_assignable_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let mut context: () = unimplemented!();
        assert!(((unimplemented!()) == (&(*fmt.offset((0) as isize)) as *const libc::c_char)));
        assert!(((unimplemented!()) == (&(*fmt.offset((3) as isize)) as *const libc::c_char)));
    }
    {
        let mut view: () = unimplemented!();
        let mut context: () = unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (unimplemented!())));
    }
}
pub unsafe fn test_1(mut fmt: *const i32) {
    const _: () = assert!(
        true,
        "!std::is_convertible_v<std::basic_string_view<CharT>,\n                             std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_copy_constructible_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_copy_assignable_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_move_constructible_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_move_assignable_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let mut context: () = unimplemented!();
        assert!(((unimplemented!()) == (&(*fmt.offset((0) as isize)) as *const i32)));
        assert!(((unimplemented!()) == (&(*fmt.offset((3) as isize)) as *const i32)));
    }
    {
        let mut view: () = unimplemented!();
        let mut context: () = unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (unimplemented!())));
    }
}
pub unsafe fn test_2(mut fmt: *const u8) {
    const _: () = assert!(
        true,
        "!std::is_convertible_v<std::basic_string_view<CharT>,\n                             std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_copy_constructible_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_copy_assignable_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_move_constructible_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_move_assignable_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let mut context: () = unimplemented!();
        assert!(((unimplemented!()) == (&(*fmt.offset((0) as isize)) as *const u8)));
        assert!(((unimplemented!()) == (&(*fmt.offset((3) as isize)) as *const u8)));
    }
    {
        let mut view: () = unimplemented!();
        let mut context: () = unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (unimplemented!())));
    }
}
pub unsafe fn test_3(mut fmt: *const u16) {
    const _: () = assert!(
        true,
        "!std::is_convertible_v<std::basic_string_view<CharT>,\n                             std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_copy_constructible_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_copy_assignable_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_move_constructible_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_move_assignable_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let mut context: () = unimplemented!();
        assert!(((unimplemented!()) == (&(*fmt.offset((0) as isize)) as *const u16)));
        assert!(((unimplemented!()) == (&(*fmt.offset((3) as isize)) as *const u16)));
    }
    {
        let mut view: () = unimplemented!();
        let mut context: () = unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (unimplemented!())));
    }
}
pub unsafe fn test_4(mut fmt: *const u32) {
    const _: () = assert!(
        true,
        "!std::is_convertible_v<std::basic_string_view<CharT>,\n                             std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_copy_constructible_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_copy_assignable_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_move_constructible_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(
        true,
        "!std::is_move_assignable_v<std::basic_format_parse_context<CharT> >"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let mut context: () = unimplemented!();
        assert!(((unimplemented!()) == (&(*fmt.offset((0) as isize)) as *const u32)));
        assert!(((unimplemented!()) == (&(*fmt.offset((3) as isize)) as *const u32)));
    }
    {
        let mut view: () = unimplemented!();
        let mut context: () = unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (unimplemented!())));
    }
}
pub unsafe fn test_5() -> bool {
    (unsafe { test_0(c"abc".as_ptr()) });
    (unsafe { test_1([97 as i32, 98 as i32, 99 as i32, 0 as i32].as_ptr()) });
    (unsafe { test_2([97 as u8, 98 as u8, 99 as u8, 0 as u8].as_ptr()) });
    (unsafe { test_3([97 as u16, 98 as u16, 99 as u16, 0 as u16].as_ptr()) });
    (unsafe { test_4([97 as u32, 98 as u32, 99 as u32, 0 as u32].as_ptr()) });
    return true;
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
    (unsafe { test_5() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
