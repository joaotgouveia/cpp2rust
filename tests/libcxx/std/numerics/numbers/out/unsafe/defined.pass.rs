extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn tests_0() -> () {
    let mut dd0: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut e_1)) as *const ());
    let mut dd1: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut log2e_2)) as *const ());
    let mut dd2: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut log10e_3)) as *const ());
    let mut dd3: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut pi_4)) as *const ());
    let mut dd4: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_pi_5)) as *const ());
    let mut dd5: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrtpi_6)) as *const ());
    let mut dd6: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut ln2_7)) as *const ());
    let mut dd7: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut ln10_8)) as *const ());
    let mut dd8: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt2_9)) as *const ());
    let mut dd9: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt3_10)) as *const ());
    let mut dd10: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrt3_11)) as *const ());
    let mut dd11: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut egamma_12)) as *const ());
    let mut dd12: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut phi_13)) as *const ());
    let mut f0: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut e_v_14)) as *const ());
    let mut f1: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut log2e_v_15)) as *const ());
    let mut f2: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut log10e_v_16)) as *const ());
    let mut f3: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut pi_v_17)) as *const ());
    let mut f4: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_pi_v_18)) as *const ());
    let mut f5: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrtpi_v_19)) as *const ());
    let mut f6: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut ln2_v_20)) as *const ());
    let mut f7: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut ln10_v_21)) as *const ());
    let mut f8: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt2_v_22)) as *const ());
    let mut f9: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt3_v_23)) as *const ());
    let mut f10: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrt3_v_24)) as *const ());
    let mut f11: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut egamma_v_25)) as *const ());
    let mut f12: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut phi_v_26)) as *const ());
    let mut d0: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut e_v_27)) as *const ());
    let mut d1: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut log2e_v_28)) as *const ());
    let mut d2: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut log10e_v_29)) as *const ());
    let mut d3: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut pi_v_30)) as *const ());
    let mut d4: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_pi_v_31)) as *const ());
    let mut d5: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrtpi_v_32)) as *const ());
    let mut d6: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut ln2_v_33)) as *const ());
    let mut d7: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut ln10_v_34)) as *const ());
    let mut d8: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt2_v_35)) as *const ());
    let mut d9: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt3_v_36)) as *const ());
    let mut d10: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrt3_v_37)) as *const ());
    let mut d11: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut egamma_v_38)) as *const ());
    let mut d12: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut phi_v_39)) as *const ());
    let mut ld0: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut e_v_40)) as *const ());
    let mut ld1: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut log2e_v_41)) as *const ());
    let mut ld2: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut log10e_v_42)) as *const ());
    let mut ld3: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut pi_v_43)) as *const ());
    let mut ld4: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_pi_v_44)) as *const ());
    let mut ld5: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrtpi_v_45)) as *const ());
    let mut ld6: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut ln2_v_46)) as *const ());
    let mut ld7: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut ln10_v_47)) as *const ());
    let mut ld8: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt2_v_48)) as *const ());
    let mut ld9: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut sqrt3_v_49)) as *const ());
    let mut ld10: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut inv_sqrt3_v_50)) as *const ());
    let mut ld11: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut egamma_v_51)) as *const ());
    let mut ld12: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut phi_v_52)) as *const ());
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
