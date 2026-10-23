extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> () {
    let mut dd0: Ptr<()> = (e_1.with(|v| v.as_pointer()));
    let mut dd1: Ptr<()> = (log2e_2.with(|v| v.as_pointer()));
    let mut dd2: Ptr<()> = (log10e_3.with(|v| v.as_pointer()));
    let mut dd3: Ptr<()> = (pi_4.with(|v| v.as_pointer()));
    let mut dd4: Ptr<()> = (inv_pi_5.with(|v| v.as_pointer()));
    let mut dd5: Ptr<()> = (inv_sqrtpi_6.with(|v| v.as_pointer()));
    let mut dd6: Ptr<()> = (ln2_7.with(|v| v.as_pointer()));
    let mut dd7: Ptr<()> = (ln10_8.with(|v| v.as_pointer()));
    let mut dd8: Ptr<()> = (sqrt2_9.with(|v| v.as_pointer()));
    let mut dd9: Ptr<()> = (sqrt3_10.with(|v| v.as_pointer()));
    let mut dd10: Ptr<()> = (inv_sqrt3_11.with(|v| v.as_pointer()));
    let mut dd11: Ptr<()> = (egamma_12.with(|v| v.as_pointer()));
    let mut dd12: Ptr<()> = (phi_13.with(|v| v.as_pointer()));
    let mut f0: Ptr<()> = (e_v_14.with(|v| v.as_pointer()));
    let mut f1: Ptr<()> = (log2e_v_15.with(|v| v.as_pointer()));
    let mut f2: Ptr<()> = (log10e_v_16.with(|v| v.as_pointer()));
    let mut f3: Ptr<()> = (pi_v_17.with(|v| v.as_pointer()));
    let mut f4: Ptr<()> = (inv_pi_v_18.with(|v| v.as_pointer()));
    let mut f5: Ptr<()> = (inv_sqrtpi_v_19.with(|v| v.as_pointer()));
    let mut f6: Ptr<()> = (ln2_v_20.with(|v| v.as_pointer()));
    let mut f7: Ptr<()> = (ln10_v_21.with(|v| v.as_pointer()));
    let mut f8: Ptr<()> = (sqrt2_v_22.with(|v| v.as_pointer()));
    let mut f9: Ptr<()> = (sqrt3_v_23.with(|v| v.as_pointer()));
    let mut f10: Ptr<()> = (inv_sqrt3_v_24.with(|v| v.as_pointer()));
    let mut f11: Ptr<()> = (egamma_v_25.with(|v| v.as_pointer()));
    let mut f12: Ptr<()> = (phi_v_26.with(|v| v.as_pointer()));
    let mut d0: Ptr<()> = (e_v_27.with(|v| v.as_pointer()));
    let mut d1: Ptr<()> = (log2e_v_28.with(|v| v.as_pointer()));
    let mut d2: Ptr<()> = (log10e_v_29.with(|v| v.as_pointer()));
    let mut d3: Ptr<()> = (pi_v_30.with(|v| v.as_pointer()));
    let mut d4: Ptr<()> = (inv_pi_v_31.with(|v| v.as_pointer()));
    let mut d5: Ptr<()> = (inv_sqrtpi_v_32.with(|v| v.as_pointer()));
    let mut d6: Ptr<()> = (ln2_v_33.with(|v| v.as_pointer()));
    let mut d7: Ptr<()> = (ln10_v_34.with(|v| v.as_pointer()));
    let mut d8: Ptr<()> = (sqrt2_v_35.with(|v| v.as_pointer()));
    let mut d9: Ptr<()> = (sqrt3_v_36.with(|v| v.as_pointer()));
    let mut d10: Ptr<()> = (inv_sqrt3_v_37.with(|v| v.as_pointer()));
    let mut d11: Ptr<()> = (egamma_v_38.with(|v| v.as_pointer()));
    let mut d12: Ptr<()> = (phi_v_39.with(|v| v.as_pointer()));
    let mut ld0: Ptr<()> = (e_v_40.with(|v| v.as_pointer()));
    let mut ld1: Ptr<()> = (log2e_v_41.with(|v| v.as_pointer()));
    let mut ld2: Ptr<()> = (log10e_v_42.with(|v| v.as_pointer()));
    let mut ld3: Ptr<()> = (pi_v_43.with(|v| v.as_pointer()));
    let mut ld4: Ptr<()> = (inv_pi_v_44.with(|v| v.as_pointer()));
    let mut ld5: Ptr<()> = (inv_sqrtpi_v_45.with(|v| v.as_pointer()));
    let mut ld6: Ptr<()> = (ln2_v_46.with(|v| v.as_pointer()));
    let mut ld7: Ptr<()> = (ln10_v_47.with(|v| v.as_pointer()));
    let mut ld8: Ptr<()> = (sqrt2_v_48.with(|v| v.as_pointer()));
    let mut ld9: Ptr<()> = (sqrt3_v_49.with(|v| v.as_pointer()));
    let mut ld10: Ptr<()> = (inv_sqrt3_v_50.with(|v| v.as_pointer()));
    let mut ld11: Ptr<()> = (egamma_v_51.with(|v| v.as_pointer()));
    let mut ld12: Ptr<()> = (phi_v_52.with(|v| v.as_pointer()));
    return true;
}
const _: () = assert!(true, "tests()");
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
    ({ tests_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
