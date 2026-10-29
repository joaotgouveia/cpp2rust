extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_close_0() {
    let mut fds: [(); 2] = [unimplemented!(); 2];
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 1] = [unimplemented!(); 1];
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lseek_1() {
    let mut path: *const () = (c"cpp2rust_lseek_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    let mut fd: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 8] = [
        0_(),
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
}
pub unsafe fn test_read_2() {
    let mut path: *const () = (c"cpp2rust_read_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    let mut fd: () = unimplemented!();
    let mut buf: [(); 16] = [
        0_(),
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
}
pub unsafe fn test_unlink_3() {
    let mut path: *const () = (c"cpp2rust_unlink_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_pipe_4() {
    let mut fds: [(); 2] = [unimplemented!(); 2];
    unimplemented!();
    let mut msg: *const () = (c"world".as_ptr().cast_mut()).cast_const();
    unimplemented!();
    let mut buf: [(); 8] = [
        0_(),
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
    unimplemented!();
}
pub unsafe fn test_ftruncate_5() {
    let mut path: *const () = (c"cpp2rust_ftruncate_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fd: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    fd = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_open_6() {
    let mut fd: () = unimplemented!();
    unimplemented!();
    if ((((fd) >= (0_())) as ()) != 0) {
        unimplemented!();
    }
    fd = unimplemented!();
    unimplemented!();
    if ((((fd) >= (0_())) as ()) != 0) {
        unimplemented!();
    }
}
pub unsafe fn test_fcntl_7() {
    unimplemented!();
    let mut duped: () = unimplemented!();
    unimplemented!();
    if ((((duped) >= (0_())) as ()) != 0) {
        unimplemented!();
    }
}
pub unsafe fn test_ioctl_8() {
    let mut arg: () = 0_();
    unimplemented!();
}
pub unsafe fn test_isatty_9() {
    unimplemented!();
}
pub unsafe fn test_geteuid_10() {
    unimplemented!();
}
pub unsafe fn test_gethostname_11() {
    let mut name: [(); 256] = [unimplemented!(); 256];
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_close_0() });
    (unsafe { test_lseek_1() });
    (unsafe { test_read_2() });
    (unsafe { test_unlink_3() });
    (unsafe { test_pipe_4() });
    (unsafe { test_ftruncate_5() });
    (unsafe { test_open_6() });
    (unsafe { test_fcntl_7() });
    (unsafe { test_ioctl_8() });
    (unsafe { test_isatty_9() });
    (unsafe { test_geteuid_10() });
    (unsafe { test_gethostname_11() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
