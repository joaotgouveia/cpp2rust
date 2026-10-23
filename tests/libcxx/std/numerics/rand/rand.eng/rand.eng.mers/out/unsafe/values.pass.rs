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
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut word_size_2))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut state_size_3))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut shift_size_4))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut mask_bits_5))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut xor_mask_6))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_u_7))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_d_8))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_s_9))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_b_10))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_t_11))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_c_12))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_l_13))) });
    (unsafe {
        where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut initialization_multiplier_14)))
    });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_15))) });
}
pub unsafe fn test2_16() {
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
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut word_size_17))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut state_size_18))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut shift_size_19))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut mask_bits_20))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut xor_mask_21))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_u_22))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_d_23))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_s_24))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_b_25))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_t_26))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_c_27))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut tempering_l_28))) });
    (unsafe {
        where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut initialization_multiplier_29)))
    });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_30))) });
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
    (unsafe { test2_16() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
