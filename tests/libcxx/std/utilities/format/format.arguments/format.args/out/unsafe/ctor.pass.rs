extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut i: () = 1_();
    let mut c: () = ('c' as ());
    let mut p: () = Default::default();
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v<std::basic_format_args<Context>>"
    );;
    {
        let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___int_ =
            (unsafe { make_format_args_1(&mut i) });
        const _: () = assert!(true, "");;
        let mut format_args: () =
            std_basic_format_args_std_basic_format_context_char_ptr__char__::new_1({ &store });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___int__char_ =
            (unsafe { make_format_args_2(&mut i, &mut c) });
        const _: () = assert!(true, "");;
        let mut format_args: () =
            std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({ &store });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___int__char__std_nullptr_t_ = ( unsafe { make_format_args_3 ( & mut i , & mut c , & mut p , ) } )  ;
        const _: () = assert!(true, "");;
        let mut format_args: () =
            std_basic_format_args_std_basic_format_context_char_ptr__char__::new_3({ &store });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_4() {
    let mut i: () = 1_();
    let mut c: () = ('c' as ());
    let mut p: () = Default::default();
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v<std::basic_format_args<Context>>"
    );;
    {
        let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___int_ =
            (unsafe { make_format_args_5(&mut i) });
        const _: () = assert!(true, "");;
        let mut format_args: () =
            std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_1({
                &store
            });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___int__char_ = ( unsafe { make_format_args_6 ( & mut i , & mut c , ) } )  ;
        const _: () = assert!(true, "");;
        let mut format_args: () =
            std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_2({
                &store
            });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___int__char__std_nullptr_t_ = ( unsafe { make_format_args_7 ( & mut i , & mut c , & mut p , ) } )  ;
        const _: () = assert!(true, "");;
        let mut format_args: () =
            std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({
                &store
            });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_8() {
    (unsafe { test_0() });
    (unsafe { test_4() });
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
    (unsafe { test_8() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
