extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Point {
    pub x: (),
    pub y: (),
}
pub unsafe fn set_0(mut ref_: std_reference_wrapper_int_, mut val: ()) {
    (*(unsafe { ref_.get() })) = val;
}
pub unsafe fn read_1(mut ref_: std_reference_wrapper_int_) -> () {
    let r: *mut () = (unsafe { ref_() });
    return (*r);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut i1: () = 10_();
    let mut ref_1: std_reference_wrapper_int_ = std_reference_wrapper_int_::new({ &mut i1 });
    (*(unsafe { ref_1.get() })) = 20_();
    let i2: *mut () = (unsafe { ref_1() });
    (*i2) += 5_();
    write!(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_2)),
        "{:}\n",
        i1,
    );
    let mut i3: () = 1_();
    let mut i4: () = 2_();
    let mut ref_3: std_reference_wrapper_int_ = std_reference_wrapper_int_::new({ &mut i3 });
    let mut ref_4: std_reference_wrapper_int_ = std_reference_wrapper_int_::new({ &mut i4 });
    (*(unsafe { ref_3.get() })) = (*(unsafe { ref_4.get() }));
    write!(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_2)),
        "{:} {:}\n",
        i3,
        i4,
    );
    (unsafe { set_0(ref_1, 99_()) });
    write!(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_2)),
        "{:} {:}\n",
        i1,
        (unsafe { read_1(ref_1,) }),
    );
    let mut point: Point = Point { x: 3_(), y: 4_() };
    let mut point_ref: std_reference_wrapper_Point_ =
        std_reference_wrapper_Point_::new({ &mut point });
    (*(unsafe { point_ref.get() })).x = 30_();
    (*(unsafe { point_ref() })).y = 40_();
    write!(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_2)),
        "{:} {:}\n",
        point.x,
        point.y,
    );
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
