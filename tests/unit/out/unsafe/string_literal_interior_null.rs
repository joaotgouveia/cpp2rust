extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_bytes_0(mut buf: *const (), mut len: ()) -> () {
    let mut sum: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (len)) {
        sum += (*buf.offset((i) as isize));
        i.postfix_inc();
    }
    return sum;
}
pub static mut g_packet_1: std::cell::LazyCell<*const ()> = std::cell::LazyCell::new(|| unsafe {
    (&[
        (1 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ])
        .as_ptr()
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: () = (unsafe {
        sum_bytes_0(
            (&[
                (1 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ])
                .as_ptr(),
            2_(),
        )
    });
    let mut b: () = (unsafe {
        sum_bytes_0(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut g_packet_1)),
            2_(),
        )
    });
    unimplemented!();
    unimplemented!();
    let mut high: () = (unsafe {
        sum_bytes_0(
            (&[
                (-62 as libc::c_char),
                (0 as libc::c_char),
                (65 as libc::c_char),
                (0 as libc::c_char),
            ])
                .as_ptr(),
            3_(),
        )
    });
    unimplemented!();
    let mut c: () = ((*c"\r\n.\r\n".as_ptr().offset((unimplemented!()) as isize))
        + (*c"\r\n.\r\n".as_ptr().offset((3_()) as isize)));
    unimplemented!();
    let mut idx: () = 1_();
    let mut d: () = (*c"abcd".as_ptr().offset((idx) as isize));
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const g_packet_1);
}
