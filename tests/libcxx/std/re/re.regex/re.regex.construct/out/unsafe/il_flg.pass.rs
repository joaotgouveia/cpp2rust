extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut il: (), mut f: (), mut mc: ()) {
    let mut r: () = unimplemented!();
    unimplemented!();
    unimplemented!();
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
    let mut s1: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"\\(a\\)".as_ptr() },
        None,
    );
    let mut s2: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"\\(a[bc]\\)".as_ptr() },
        None,
    );
    let mut s3: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"\\(a\\([bc]\\)\\)".as_ptr() },
        None,
    );
    let mut s4: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"(a([bc]))".as_ptr() },
        None,
    );
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_basic,
            1_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_basic,
            1_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_basic,
            2_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_basic,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_extended,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_extended,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_extended,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_extended,
            2_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_ECMAScript,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_ECMAScript,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_ECMAScript,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_ECMAScript,
            2_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_awk,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_awk,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_awk,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_awk,
            2_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_grep,
            1_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_grep,
            1_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_grep,
            2_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_grep,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_egrep,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_egrep,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_egrep,
            0_(),
        )
    });
    (unsafe {
        test_0(
            unimplemented!(),
            std_regex_constants_syntax_option_type_egrep,
            2_(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
