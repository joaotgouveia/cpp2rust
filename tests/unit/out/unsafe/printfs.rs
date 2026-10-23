extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(mut v: ()) -> () {
    return unimplemented!();
}
pub unsafe fn fn2_1(v: *const ()) -> *const () {
    return v;
}
pub unsafe fn log_to_2(
    mut out: *mut ::libc::FILE,
    mut fmt: *const (),
    mut args_2: *const (),
    mut args_3: (),
) {
    (unsafe {
        libc::fprintf(
            out as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (args_2),
            (args_3),
        )
    });
}
pub unsafe fn log_to_3(mut out: *mut ::libc::FILE, mut fmt: *const (), mut args: ()) {
    (unsafe { libc::fprintf(out as *mut ::libc::FILE, fmt as *const libc::c_char, (args)) });
}
pub unsafe fn log_fmt_4(mut fmt: *const (), mut v: ()) {
    (unsafe { libc::printf(fmt as *const libc::c_char, (v)) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe {
        libc::fprintf(
            libcc2rs::stdout_unsafe() as *mut ::libc::FILE,
            c"%s\n".as_ptr() as *const libc::c_char,
            (c"fprintf stdout".as_ptr()),
        )
    });
    (unsafe {
        libc::fprintf(
            libcc2rs::stdout_unsafe() as *mut ::libc::FILE,
            c"%d %u %ld\n".as_ptr() as *const libc::c_char,
            (1_()),
            (2_()),
            (3_()),
        )
    });
    (unsafe {
        libc::fprintf(
            libcc2rs::stdout_unsafe() as *mut ::libc::FILE,
            c"hello world".as_ptr() as *const libc::c_char,
        )
    });
    let mut in_: *mut ::libc::FILE = libcc2rs::stdin_unsafe();
    unimplemented!();
    (unsafe {
        libc::printf(
            c"%s\n".as_ptr() as *const libc::c_char,
            (c"printf".as_ptr()),
        )
    });
    (unsafe { libc::printf(c"hello world".as_ptr() as *const libc::c_char) });
    let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"a string".as_ptr() },
        None,
    );
    (unsafe { libc::printf(c"%s\n".as_ptr() as *const libc::c_char, (unimplemented!())) });
    (unsafe { libc::printf(c"%s\n".as_ptr() as *const libc::c_char, (unimplemented!())) });
    (unsafe { libc::printf(c"%s\n".as_ptr() as *const libc::c_char, (unimplemented!())) });
    (unsafe {
        log_to_2(
            libcc2rs::stdout_unsafe(),
            c"%s %d\n".as_ptr(),
            c"runtime".as_ptr(),
            4_(),
        )
    });
    (unsafe { log_to_3(libcc2rs::stderr_unsafe(), c"%d\n".as_ptr(), 5_()) });
    (unsafe { log_fmt_4(c"%d\n".as_ptr(), 6_()) });
    (unsafe {
        libc::printf(
            c"{} %ju %jd\n".as_ptr() as *const libc::c_char,
            (5_u64),
            (-6_() as i64),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
