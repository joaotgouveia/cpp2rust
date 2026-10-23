extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    let mut a: [(); 7] = [3_(), 5_(), 2_(), 0_(), 6_(), 8_(), 1_()];
    let n: () =
        (::std::mem::size_of::<[(); 7]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut v: () = std_vector_int__std_allocator_int__::new_1({ a.as_mut_ptr() }, {
        a.as_mut_ptr().offset(((n) / (2_())) as isize)
    });
    let mut q: () = {
        let mut __tmp_0: () = unimplemented!();
        std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int__::new_1(
            { a.as_mut_ptr().offset(((n) / (2_())) as isize) },
            { a.as_mut_ptr().offset((n) as isize) },
            { &mut __tmp_0 },
            { &v },
        )
    };
    unimplemented!();
    unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
