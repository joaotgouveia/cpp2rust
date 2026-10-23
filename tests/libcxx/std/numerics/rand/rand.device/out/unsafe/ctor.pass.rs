extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_convertible_imp_0(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn test_convertible_1() -> () {
    return (unsafe { test_convertible_imp_0(0_()) });
}
pub unsafe fn check_random_device_valid_2(token: *const ()) {
    let mut r: () = unimplemented!();
}
pub unsafe fn check_random_device_invalid_3(token: *const ()) {
    (&(*token));
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
    {
        let mut r: () = unimplemented!();
        &(r);
    }
    {
        (unsafe {
            let mut _token: () =
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"/dev/urandom".as_ptr() },
                    None,
                );
            check_random_device_valid_2(&mut _token)
        });
        (unsafe {
            let mut _token: () =
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"/dev/random".as_ptr() },
                    None,
                );
            check_random_device_valid_2(&mut _token)
        });
        (unsafe {
            let mut _token: () =
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"/dev/null".as_ptr() },
                    None,
                );
            check_random_device_valid_2(&mut _token)
        });
        (unsafe {
            let mut _token: () =
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"/dev/nonexistent".as_ptr() },
                    None,
                );
            check_random_device_invalid_3(&mut _token)
        });
        (unsafe {
            let mut _token: () =
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"wrong file".as_ptr() },
                    None,
                );
            check_random_device_invalid_3(&mut _token)
        });
    }
    {
        let mut ec: () = unimplemented!();
        ec = unimplemented!();
        unimplemented!();
        ec = unimplemented!();
        unimplemented!();
        ec = unimplemented!();
        unimplemented!();
        let mut r: () = unimplemented!();
    }
    const _: () = assert!(true, "test_convertible<std::random_device>()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
