extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn where_0(_a0: Ptr<()>) {}
pub fn test1_1() {
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
    ({ where_0(word_size_2.with(|v| v.as_pointer())) });
    ({ where_0(state_size_3.with(|v| v.as_pointer())) });
    ({ where_0(shift_size_4.with(|v| v.as_pointer())) });
    ({ where_0(mask_bits_5.with(|v| v.as_pointer())) });
    ({ where_0(xor_mask_6.with(|v| v.as_pointer())) });
    ({ where_0(tempering_u_7.with(|v| v.as_pointer())) });
    ({ where_0(tempering_d_8.with(|v| v.as_pointer())) });
    ({ where_0(tempering_s_9.with(|v| v.as_pointer())) });
    ({ where_0(tempering_b_10.with(|v| v.as_pointer())) });
    ({ where_0(tempering_t_11.with(|v| v.as_pointer())) });
    ({ where_0(tempering_c_12.with(|v| v.as_pointer())) });
    ({ where_0(tempering_l_13.with(|v| v.as_pointer())) });
    ({ where_0(initialization_multiplier_14.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_15.with(|v| v.as_pointer())) });
}
pub fn test2_16() {
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
    ({ where_0(word_size_17.with(|v| v.as_pointer())) });
    ({ where_0(state_size_18.with(|v| v.as_pointer())) });
    ({ where_0(shift_size_19.with(|v| v.as_pointer())) });
    ({ where_0(mask_bits_20.with(|v| v.as_pointer())) });
    ({ where_0(xor_mask_21.with(|v| v.as_pointer())) });
    ({ where_0(tempering_u_22.with(|v| v.as_pointer())) });
    ({ where_0(tempering_d_23.with(|v| v.as_pointer())) });
    ({ where_0(tempering_s_24.with(|v| v.as_pointer())) });
    ({ where_0(tempering_b_25.with(|v| v.as_pointer())) });
    ({ where_0(tempering_t_26.with(|v| v.as_pointer())) });
    ({ where_0(tempering_c_27.with(|v| v.as_pointer())) });
    ({ where_0(tempering_l_28.with(|v| v.as_pointer())) });
    ({ where_0(initialization_multiplier_29.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_30.with(|v| v.as_pointer())) });
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test1_1() });
    ({ test2_16() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
