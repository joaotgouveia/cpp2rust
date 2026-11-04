extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn add_const_0() -> i32 {
    return 0_i32;
}
pub fn add_cv_1() -> i32 {
    return 0_i32;
}
pub fn add_lvalue_reference_2() -> Ptr<i32> {
    return i_3.with(|v| v.as_pointer());
}
thread_local!(
    static i_3: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn add_pointer_4() -> Ptr<i32> {
    return Ptr::<i32>::null();
}
pub fn add_rvalue_reference_5() -> Ptr<i32> {
    return i_6.with(|v| v.as_pointer());
}
thread_local!(
    static i_6: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn add_volatile_7() -> i32 {
    return i_8.with(|rc| *rc.borrow());
}
thread_local!(
    static i_8: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn decay_9() -> i32 {
    return 0_i32;
}
pub fn enable_if_10() -> i32 {
    return 0_i32;
}
pub fn make_signed_11() -> i32 {
    return 0_i32;
}
pub fn make_unsigned_12() -> u32 {
    return 0_u32;
}
pub fn remove_all_extents_13() -> i32 {
    return 0_i32;
}
pub fn remove_const_14() -> i32 {
    return 0_i32;
}
pub fn remove_cv_15() -> i32 {
    return 0_i32;
}
pub fn remove_cvref_16() -> i32 {
    return 0_i32;
}
pub fn remove_extent_17() -> i32 {
    return 0_i32;
}
pub fn remove_pointer_18() -> i32 {
    return 0_i32;
}
pub fn remove_reference_19() -> i32 {
    return 0_i32;
}
pub fn remove_volatile_20() -> i32 {
    return 0_i32;
}
pub fn type_identity_21() -> i32 {
    return 0_i32;
}
pub fn underlying_type_22() -> i32 {
    return 0_i32;
}
pub type E = i32;
pub fn void_t_23() {}
pub fn instantiate_24() {
    ({ add_const_0() });
    ({ add_cv_1() });
    ({ add_lvalue_reference_2() });
    ({ add_pointer_4() });
    ({ add_rvalue_reference_5() });
    ({ add_volatile_7() });
    ({ decay_9() });
    ({ enable_if_10() });
    ({ make_signed_11() });
    ({ make_unsigned_12() });
    ({ remove_all_extents_13() });
    ({ remove_const_14() });
    ({ remove_cv_15() });
    ({ remove_cvref_16() });
    ({ remove_extent_17() });
    ({ remove_pointer_18() });
    ({ remove_reference_19() });
    ({ remove_volatile_20() });
    ({ type_identity_21() });
    ({ underlying_type_22() });
    ({ void_t_23() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    return 0;
}
pub fn __cpp2rust_init_globals() {}
