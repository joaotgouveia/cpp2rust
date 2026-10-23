extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn where_0(_a0: *const ()) {}
pub unsafe fn test1_1() {
    const _: () = assert!(true, "(E::word_size == 32)");;
    const _: () = assert!(true, "(E::state_size == 624)");;
    const _: () = assert!(true, "(E::shift_size == 397)");;
    const _: () = assert!(true, "(E::mask_bits == 31)");;
    const _: () = assert!(true, "(E::xor_mask == 0x9908b0df)");;
    const _: () = assert!(true, "(E::tempering_u == 11)");;
    const _: () = assert!(true, "(E::tempering_d == 0xffffffff)");;
    const _: () = assert!(true, "(E::tempering_s == 7)");;
    const _: () = assert!(true, "(E::tempering_b == 0x9d2c5680)");;
    const _: () = assert!(true, "(E::tempering_t == 15)");;
    const _: () = assert!(true, "(E::tempering_c == 0xefc60000)");;
    const _: () = assert!(true, "(E::tempering_l == 18)");;
    const _: () = assert!(true, "(E::initialization_multiplier == 1812433253)");;
    const _: () = assert!(true, "(E::min() == 0)");;
    const _: () = assert!(true, "(E::max() == 0xFFFFFFFF)");;
    const _: () = assert!(true, "(E::default_seed == 5489u)");;
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
}
pub unsafe fn test2_2() {
    const _: () = assert!(true, "(E::word_size == 64)");;
    const _: () = assert!(true, "(E::state_size == 312)");;
    const _: () = assert!(true, "(E::shift_size == 156)");;
    const _: () = assert!(true, "(E::mask_bits == 31)");;
    const _: () = assert!(true, "(E::xor_mask == 0xb5026f5aa96619e9ull)");;
    const _: () = assert!(true, "(E::tempering_u == 29)");;
    const _: () = assert!(true, "(E::tempering_d == 0x5555555555555555ull)");;
    const _: () = assert!(true, "(E::tempering_s == 17)");;
    const _: () = assert!(true, "(E::tempering_b == 0x71d67fffeda60000ull)");;
    const _: () = assert!(true, "(E::tempering_t == 37)");;
    const _: () = assert!(true, "(E::tempering_c == 0xfff7eee000000000ull)");;
    const _: () = assert!(true, "(E::tempering_l == 43)");;
    const _: () = assert!(
        true,
        "(E::initialization_multiplier == 6364136223846793005ull)"
    );;
    const _: () = assert!(true, "(E::min() == 0)");;
    const _: () = assert!(true, "(E::max() == 0xFFFFFFFFFFFFFFFFull)");;
    const _: () = assert!(true, "(E::default_seed == 5489u)");;
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
    (unsafe { where_0(&unimplemented!()) });
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
    (unsafe { test1_1() });
    (unsafe { test2_2() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
