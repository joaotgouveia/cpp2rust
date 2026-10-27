extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn tests_0() -> () {
    let mut f0: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut e_v_1));
    let mut f1: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut log2e_v_2));
    let mut f2: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut log10e_v_3));
    let mut f3: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut pi_v_4));
    let mut f4: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_pi_v_5));
    let mut f5: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrtpi_v_6));
    let mut f6: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut ln2_v_7));
    let mut f7: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut ln10_v_8));
    let mut f8: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt2_v_9));
    let mut f9: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt3_v_10));
    let mut f10: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrt3_v_11));
    let mut f11: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut egamma_v_12));
    let mut f12: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut phi_v_13));
    let mut d0: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut e_v_14));
    let mut d1: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut log2e_v_15));
    let mut d2: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut log10e_v_16));
    let mut d3: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut pi_v_17));
    let mut d4: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_pi_v_18));
    let mut d5: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrtpi_v_19));
    let mut d6: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut ln2_v_20));
    let mut d7: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut ln10_v_21));
    let mut d8: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt2_v_22));
    let mut d9: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt3_v_23));
    let mut d10: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrt3_v_24));
    let mut d11: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut egamma_v_25));
    let mut d12: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut phi_v_26));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ld0: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut e_v_27));
    let mut ld1: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut log2e_v_28));
    let mut ld2: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut log10e_v_29));
    let mut ld3: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut pi_v_30));
    let mut ld4: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_pi_v_31));
    let mut ld5: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrtpi_v_32));
    let mut ld6: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut ln2_v_33));
    let mut ld7: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut ln10_v_34));
    let mut ld8: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt2_v_35));
    let mut ld9: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt3_v_36));
    let mut ld10: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrt3_v_37));
    let mut ld11: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut egamma_v_38));
    let mut ld12: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut phi_v_39));
    return true;
}
const _: () = assert!(true, "tests()");
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
    (unsafe { tests_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
