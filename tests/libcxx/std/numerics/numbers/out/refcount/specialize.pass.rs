extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> bool {
    let mut f0: f32 = e_v_1.with(|rc| *rc.borrow());
    let mut f1: f32 = log2e_v_2.with(|rc| *rc.borrow());
    let mut f2: f32 = log10e_v_3.with(|rc| *rc.borrow());
    let mut f3: f32 = pi_v_4.with(|rc| *rc.borrow());
    let mut f4: f32 = inv_pi_v_5.with(|rc| *rc.borrow());
    let mut f5: f32 = inv_sqrtpi_v_6.with(|rc| *rc.borrow());
    let mut f6: f32 = ln2_v_7.with(|rc| *rc.borrow());
    let mut f7: f32 = ln10_v_8.with(|rc| *rc.borrow());
    let mut f8: f32 = sqrt2_v_9.with(|rc| *rc.borrow());
    let mut f9: f32 = sqrt3_v_10.with(|rc| *rc.borrow());
    let mut f10: f32 = inv_sqrt3_v_11.with(|rc| *rc.borrow());
    let mut f11: f32 = egamma_v_12.with(|rc| *rc.borrow());
    let mut f12: f32 = phi_v_13.with(|rc| *rc.borrow());
    let mut d0: f64 = e_v_14.with(|rc| *rc.borrow());
    let mut d1: f64 = log2e_v_15.with(|rc| *rc.borrow());
    let mut d2: f64 = log10e_v_16.with(|rc| *rc.borrow());
    let mut d3: f64 = pi_v_17.with(|rc| *rc.borrow());
    let mut d4: f64 = inv_pi_v_18.with(|rc| *rc.borrow());
    let mut d5: f64 = inv_sqrtpi_v_19.with(|rc| *rc.borrow());
    let mut d6: f64 = ln2_v_20.with(|rc| *rc.borrow());
    let mut d7: f64 = ln10_v_21.with(|rc| *rc.borrow());
    let mut d8: f64 = sqrt2_v_22.with(|rc| *rc.borrow());
    let mut d9: f64 = sqrt3_v_23.with(|rc| *rc.borrow());
    let mut d10: f64 = inv_sqrt3_v_24.with(|rc| *rc.borrow());
    let mut d11: f64 = egamma_v_25.with(|rc| *rc.borrow());
    let mut d12: f64 = phi_v_26.with(|rc| *rc.borrow());
    ({ cpp2rust_assert_fail_27((d0 == e_28.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d1 == log2e_29.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d2 == log10e_30.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d3 == pi_31.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d4 == inv_pi_32.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d5 == inv_sqrtpi_33.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d6 == ln2_34.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d7 == ln10_35.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d8 == sqrt2_36.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d9 == sqrt3_37.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d10 == inv_sqrt3_38.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d11 == egamma_39.with(|rc| *rc.borrow()))) });
    ({ cpp2rust_assert_fail_27((d12 == phi_40.with(|rc| *rc.borrow()))) });
    let mut ld0: f64 = e_v_41.with(|rc| *rc.borrow());
    let mut ld1: f64 = log2e_v_42.with(|rc| *rc.borrow());
    let mut ld2: f64 = log10e_v_43.with(|rc| *rc.borrow());
    let mut ld3: f64 = pi_v_44.with(|rc| *rc.borrow());
    let mut ld4: f64 = inv_pi_v_45.with(|rc| *rc.borrow());
    let mut ld5: f64 = inv_sqrtpi_v_46.with(|rc| *rc.borrow());
    let mut ld6: f64 = ln2_v_47.with(|rc| *rc.borrow());
    let mut ld7: f64 = ln10_v_48.with(|rc| *rc.borrow());
    let mut ld8: f64 = sqrt2_v_49.with(|rc| *rc.borrow());
    let mut ld9: f64 = sqrt3_v_50.with(|rc| *rc.borrow());
    let mut ld10: f64 = inv_sqrt3_v_51.with(|rc| *rc.borrow());
    let mut ld11: f64 = egamma_v_52.with(|rc| *rc.borrow());
    let mut ld12: f64 = phi_v_53.with(|rc| *rc.borrow());
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ tests_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
