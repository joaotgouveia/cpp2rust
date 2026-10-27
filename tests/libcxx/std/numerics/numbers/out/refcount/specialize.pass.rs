extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> () {
    let mut f0: () = e_v_1.with(|rc| *rc.borrow());
    let mut f1: () = log2e_v_2.with(|rc| *rc.borrow());
    let mut f2: () = log10e_v_3.with(|rc| *rc.borrow());
    let mut f3: () = pi_v_4.with(|rc| *rc.borrow());
    let mut f4: () = inv_pi_v_5.with(|rc| *rc.borrow());
    let mut f5: () = inv_sqrtpi_v_6.with(|rc| *rc.borrow());
    let mut f6: () = ln2_v_7.with(|rc| *rc.borrow());
    let mut f7: () = ln10_v_8.with(|rc| *rc.borrow());
    let mut f8: () = sqrt2_v_9.with(|rc| *rc.borrow());
    let mut f9: () = sqrt3_v_10.with(|rc| *rc.borrow());
    let mut f10: () = inv_sqrt3_v_11.with(|rc| *rc.borrow());
    let mut f11: () = egamma_v_12.with(|rc| *rc.borrow());
    let mut f12: () = phi_v_13.with(|rc| *rc.borrow());
    let mut d0: () = e_v_14.with(|rc| *rc.borrow());
    let mut d1: () = log2e_v_15.with(|rc| *rc.borrow());
    let mut d2: () = log10e_v_16.with(|rc| *rc.borrow());
    let mut d3: () = pi_v_17.with(|rc| *rc.borrow());
    let mut d4: () = inv_pi_v_18.with(|rc| *rc.borrow());
    let mut d5: () = inv_sqrtpi_v_19.with(|rc| *rc.borrow());
    let mut d6: () = ln2_v_20.with(|rc| *rc.borrow());
    let mut d7: () = ln10_v_21.with(|rc| *rc.borrow());
    let mut d8: () = sqrt2_v_22.with(|rc| *rc.borrow());
    let mut d9: () = sqrt3_v_23.with(|rc| *rc.borrow());
    let mut d10: () = inv_sqrt3_v_24.with(|rc| *rc.borrow());
    let mut d11: () = egamma_v_25.with(|rc| *rc.borrow());
    let mut d12: () = phi_v_26.with(|rc| *rc.borrow());
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
    let mut ld0: () = e_v_27.with(|rc| *rc.borrow());
    let mut ld1: () = log2e_v_28.with(|rc| *rc.borrow());
    let mut ld2: () = log10e_v_29.with(|rc| *rc.borrow());
    let mut ld3: () = pi_v_30.with(|rc| *rc.borrow());
    let mut ld4: () = inv_pi_v_31.with(|rc| *rc.borrow());
    let mut ld5: () = inv_sqrtpi_v_32.with(|rc| *rc.borrow());
    let mut ld6: () = ln2_v_33.with(|rc| *rc.borrow());
    let mut ld7: () = ln10_v_34.with(|rc| *rc.borrow());
    let mut ld8: () = sqrt2_v_35.with(|rc| *rc.borrow());
    let mut ld9: () = sqrt3_v_36.with(|rc| *rc.borrow());
    let mut ld10: () = inv_sqrt3_v_37.with(|rc| *rc.borrow());
    let mut ld11: () = egamma_v_38.with(|rc| *rc.borrow());
    let mut ld12: () = phi_v_39.with(|rc| *rc.borrow());
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
