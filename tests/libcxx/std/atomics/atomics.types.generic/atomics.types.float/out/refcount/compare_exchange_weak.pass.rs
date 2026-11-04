extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_seq_cst_0(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_1(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_2(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_3(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_4(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_5(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_6(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_7(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_8(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_9(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_10(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_11(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_12(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_13(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_14(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_15(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_16(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_17(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_18(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_19(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_20(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_21(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_22(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_23(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_24(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_25(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_26(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_27(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_28(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_29(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_30(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_31(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_32(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_33(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_34(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_35(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_36(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_37(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_38(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_39(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_40(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_41(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_42(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_43(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_44(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_45(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_46(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_47(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn testBasic_48() {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.2_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_22(
                expected.as_pointer(),
                desired,
                None,
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_float_).load_13(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
    }
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.5_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_22(
                expected.as_pointer(),
                desired,
                None,
            )
        });
        assert!(!(r));
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_float_).load_13(None) })
                    == (1.2_f64 as f32))
            })
        );
        {
            assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
        }
    }
}
pub fn testBasic_49(mut memory_order: std_memory_order) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.2_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_22(
                expected.as_pointer(),
                desired,
                Some(memory_order),
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_float_).load_13(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
    }
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.5_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_22(
                expected.as_pointer(),
                desired,
                Some(memory_order),
            )
        });
        assert!(!(r));
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_float_).load_13(None) })
                    == (1.2_f64 as f32))
            })
        );
        {
            assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
        }
    }
}
pub fn testBasic_50(mut memory_order_0: std_memory_order, mut memory_order_1: std_memory_order) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.2_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_18(
                expected.as_pointer(),
                desired,
                memory_order_0,
                memory_order_1,
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_float_).load_13(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
    }
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.5_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_18(
                expected.as_pointer(),
                desired,
                memory_order_0,
                memory_order_1,
            )
        });
        assert!(!(r));
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_float_).load_13(None) })
                    == (1.2_f64 as f32))
            })
        );
        {
            assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
        }
    }
}
pub fn testBasic_51() {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.2_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_23(
                expected.as_pointer(),
                desired,
                None,
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_float_).load_12(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
    }
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.5_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_23(
                expected.as_pointer(),
                desired,
                None,
            )
        });
        assert!(!(r));
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_float_).load_12(None) })
                    == (1.2_f64 as f32))
            })
        );
        {
            assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
        }
    }
}
pub fn testBasic_52(mut memory_order: std_memory_order) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.2_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_23(
                expected.as_pointer(),
                desired,
                Some(memory_order),
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_float_).load_12(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
    }
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.5_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_23(
                expected.as_pointer(),
                desired,
                Some(memory_order),
            )
        });
        assert!(!(r));
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_float_).load_12(None) })
                    == (1.2_f64 as f32))
            })
        );
        {
            assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
        }
    }
}
pub fn testBasic_53(mut memory_order_0: std_memory_order, mut memory_order_1: std_memory_order) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.2_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_19(
                expected.as_pointer(),
                desired,
                memory_order_0,
                memory_order_1,
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_float_).load_12(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
    }
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (1.2_f64 as f32) })));
        let expected: Value<f32> = Rc::new(RefCell::new((1.5_f64 as f32)));
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_float_).compare_exchange_weak_19(
                expected.as_pointer(),
                desired,
                memory_order_0,
                memory_order_1,
            )
        });
        assert!(!(r));
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_float_).load_12(None) })
                    == (1.2_f64 as f32))
            })
        );
        {
            assert!(({ ((*expected.borrow()) == (1.2_f64 as f32)) }));
        }
    }
}
pub fn testBasic_54() {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.2_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_22(
                expected.as_pointer(),
                desired,
                None,
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_double_).load_13(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == 1.2_f64) }));
    }
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.5_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_22(
                expected.as_pointer(),
                desired,
                None,
            )
        });
        assert!(!(r));
        assert!(
            ({ (({ ((*a.borrow()) as std___atomic_float_double_).load_13(None,) }) == 1.2_f64) })
        );
        {
            assert!(({ ((*expected.borrow()) == 1.2_f64) }));
        }
    }
}
pub fn testBasic_55(mut memory_order: std_memory_order) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.2_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_22(
                expected.as_pointer(),
                desired,
                Some(memory_order),
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_double_).load_13(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == 1.2_f64) }));
    }
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.5_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_22(
                expected.as_pointer(),
                desired,
                Some(memory_order),
            )
        });
        assert!(!(r));
        assert!(
            ({ (({ ((*a.borrow()) as std___atomic_float_double_).load_13(None,) }) == 1.2_f64) })
        );
        {
            assert!(({ ((*expected.borrow()) == 1.2_f64) }));
        }
    }
}
pub fn testBasic_56(mut memory_order_0: std_memory_order, mut memory_order_1: std_memory_order) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.2_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_18(
                expected.as_pointer(),
                desired,
                memory_order_0,
                memory_order_1,
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_double_).load_13(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == 1.2_f64) }));
    }
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.5_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_18(
                expected.as_pointer(),
                desired,
                memory_order_0,
                memory_order_1,
            )
        });
        assert!(!(r));
        assert!(
            ({ (({ ((*a.borrow()) as std___atomic_float_double_).load_13(None,) }) == 1.2_f64) })
        );
        {
            assert!(({ ((*expected.borrow()) == 1.2_f64) }));
        }
    }
}
pub fn testBasic_57() {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.2_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_23(
                expected.as_pointer(),
                desired,
                None,
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_double_).load_12(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == 1.2_f64) }));
    }
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.5_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_23(
                expected.as_pointer(),
                desired,
                None,
            )
        });
        assert!(!(r));
        assert!(
            ({ (({ ((*a.borrow()) as std___atomic_float_double_).load_12(None,) }) == 1.2_f64) })
        );
        {
            assert!(({ ((*expected.borrow()) == 1.2_f64) }));
        }
    }
}
pub fn testBasic_58(mut memory_order: std_memory_order) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.2_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_23(
                expected.as_pointer(),
                desired,
                Some(memory_order),
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_double_).load_12(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == 1.2_f64) }));
    }
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.5_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_23(
                expected.as_pointer(),
                desired,
                Some(memory_order),
            )
        });
        assert!(!(r));
        assert!(
            ({ (({ ((*a.borrow()) as std___atomic_float_double_).load_12(None,) }) == 1.2_f64) })
        );
        {
            assert!(({ ((*expected.borrow()) == 1.2_f64) }));
        }
    }
}
pub fn testBasic_59(mut memory_order_0: std_memory_order, mut memory_order_1: std_memory_order) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.2_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_19(
                expected.as_pointer(),
                desired,
                memory_order_0,
                memory_order_1,
            )
        });
        if r {
            assert!(
                ({
                    (({ ((*a.borrow()) as std___atomic_float_double_).load_12(None) }) == desired)
                })
            );
        }
        assert!(({ ((*expected.borrow()) == 1.2_f64) }));
    }
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 1.2_f64 })));
        let expected: Value<f64> = Rc::new(RefCell::new(1.5_f64));
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = ({
            ((*a.borrow()) as std___atomic_float_double_).compare_exchange_weak_19(
                expected.as_pointer(),
                desired,
                memory_order_0,
                memory_order_1,
            )
        });
        assert!(!(r));
        assert!(
            ({ (({ ((*a.borrow()) as std___atomic_float_double_).load_12(None,) }) == 1.2_f64) })
        );
        {
            assert!(({ ((*expected.borrow()) == 1.2_f64) }));
        }
    }
}
pub fn test_impl_60() {
    ({ testBasic_48() });
    ({ testBasic_49(std_memory_order_relaxed) });
    ({
        testBasic_50(
            std_memory_order_relaxed,
            memory_order_relaxed_61.with(|rc| *rc.borrow()),
        )
    });
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_18(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_release,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .load_13(Some(std_memory_order_acquire))
                    });
                }
            },
        )));
        ({ test_acquire_release_12((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_22(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_release),
                            )
                    }) {}
                },
            )),
        );
        ({
            test_acquire_release_13(
                (*store_one_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut _a1: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .store_11(new_val, Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_18(
                                ___expected,
                                ___desired,
                                std_memory_order_acquire,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            },
        )));
        ({ test_acquire_release_14((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>) -> f32>::new(|x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_22(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_acquire),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_15(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_18(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_acq_rel,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_18(
                                ___expected,
                                ___desired,
                                std_memory_order_acq_rel,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            },
        )));
        ({ test_acquire_release_16((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_22(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_acq_rel),
                            )
                    }) {}
                },
            )),
        );
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>) -> f32>::new(|x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_22(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_acq_rel),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_17(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_18(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_seq_cst,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_18(
                                ___expected,
                                ___desired,
                                std_memory_order_seq_cst,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            },
        )));
        ({ test_seq_cst_0((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_22(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_seq_cst),
                            )
                    }) {}
                },
            )),
        );
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>) -> f32>::new(|x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_22(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_seq_cst),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_seq_cst_1(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut _a1: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .store_11(new_val, Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let result: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f32> = Rc::new(RefCell::new((-9999.99_f64 as f32)));
                    let mut r: bool = ({
                        let ___expected: Ptr<f32> = unexpected.as_pointer();
                        let ___desired: f32 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_18(
                                ___expected,
                                ___desired,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                                memory_order_acquire_62.with(|rc| *rc.borrow()),
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            },
        )));
        ({ test_acquire_release_18((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>) -> f32>::new(|x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let result: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f32> = Rc::new(RefCell::new((-9999.99_f64 as f32)));
                    let mut r: bool = ({
                        let ___expected: Ptr<f32> = unexpected.as_pointer();
                        let ___desired: f32 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_22(
                                ___expected,
                                ___desired,
                                Some(memory_order_acquire_62.with(|rc| *rc.borrow())),
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_19(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
        let load_one_arg_acq_rel: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_float_>) -> f32>::new(
                |x: Ptr<std_atomic_float_>| -> f32 {
                    {
                        let result: Value<f32> = Rc::new(RefCell::new(
                            ({
                                ((*x.upgrade().deref()) as std___atomic_float_float_)
                                    .load_13(Some(std_memory_order_relaxed))
                            }),
                        ));
                        let unexpected: Value<f32> = Rc::new(RefCell::new((-9999.99_f64 as f32)));
                        let mut r: bool = ({
                            let ___expected: Ptr<f32> = unexpected.as_pointer();
                            let ___desired: f32 = (*unexpected.borrow());
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .compare_exchange_weak_22(
                                    ___expected,
                                    ___desired,
                                    Some(memory_order_acq_rel_63.with(|rc| *rc.borrow())),
                                )
                        });
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )),
        );
        ({
            test_acquire_release_20(
                (*store.borrow()).copy_from(),
                (*load_one_arg_acq_rel.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut _a1: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .store_11(new_val, Some(std_memory_order_seq_cst))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let result: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f32> = Rc::new(RefCell::new((-9999.99_f64 as f32)));
                    let mut r: bool = ({
                        let ___expected: Ptr<f32> = unexpected.as_pointer();
                        let ___desired: f32 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_18(
                                ___expected,
                                ___desired,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                                std_memory_order_seq_cst,
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            },
        )));
        ({ test_seq_cst_2((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_impl_64() {
    ({ testBasic_51() });
    ({ testBasic_52(std_memory_order_relaxed) });
    ({
        testBasic_53(
            std_memory_order_relaxed,
            memory_order_relaxed_61.with(|rc| *rc.borrow()),
        )
    });
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_19(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_release,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .load_12(Some(std_memory_order_acquire))
                    });
                }
            },
        )));
        ({ test_acquire_release_21((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_23(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_release),
                            )
                    }) {}
                },
            )),
        );
        ({
            test_acquire_release_22(
                (*store_one_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut _a1: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .store_10(new_val, Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_19(
                                ___expected,
                                ___desired,
                                std_memory_order_acquire,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            },
        )));
        ({ test_acquire_release_23((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>) -> f32>::new(|x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_23(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_acquire),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_24(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_19(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_acq_rel,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_19(
                                ___expected,
                                ___desired,
                                std_memory_order_acq_rel,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            },
        )));
        ({ test_acquire_release_25((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_23(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_acq_rel),
                            )
                    }) {}
                },
            )),
        );
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>) -> f32>::new(|x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_23(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_acq_rel),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_26(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_19(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_seq_cst,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_19(
                                ___expected,
                                ___desired,
                                std_memory_order_seq_cst,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            },
        )));
        ({ test_seq_cst_3((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, old_val: f32, mut new_val: f32| {
                    let old_val: Value<f32> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_23(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_seq_cst),
                            )
                    }) {}
                },
            )),
        );
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>) -> f32>::new(|x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let val: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f32> = val.as_pointer();
                        let ___desired: f32 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_23(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_seq_cst),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_seq_cst_4(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut _a1: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .store_10(new_val, Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let result: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f32> = Rc::new(RefCell::new((-9999.99_f64 as f32)));
                    let mut r: bool = ({
                        let ___expected: Ptr<f32> = unexpected.as_pointer();
                        let ___desired: f32 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_19(
                                ___expected,
                                ___desired,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                                memory_order_acquire_62.with(|rc| *rc.borrow()),
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            },
        )));
        ({ test_acquire_release_27((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>) -> f32>::new(|x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let result: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f32> = Rc::new(RefCell::new((-9999.99_f64 as f32)));
                    let mut r: bool = ({
                        let ___expected: Ptr<f32> = unexpected.as_pointer();
                        let ___desired: f32 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_23(
                                ___expected,
                                ___desired,
                                Some(memory_order_acquire_62.with(|rc| *rc.borrow())),
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_28(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
        let load_one_arg_acq_rel: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_float_>) -> f32>::new(
                |x: Ptr<std_atomic_float_>| -> f32 {
                    {
                        let result: Value<f32> = Rc::new(RefCell::new(
                            ({
                                ((*x.upgrade().deref()) as std___atomic_float_float_)
                                    .load_12(Some(std_memory_order_relaxed))
                            }),
                        ));
                        let unexpected: Value<f32> = Rc::new(RefCell::new((-9999.99_f64 as f32)));
                        let mut r: bool = ({
                            let ___expected: Ptr<f32> = unexpected.as_pointer();
                            let ___desired: f32 = (*unexpected.borrow());
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .compare_exchange_weak_23(
                                    ___expected,
                                    ___desired,
                                    Some(memory_order_acq_rel_63.with(|rc| *rc.borrow())),
                                )
                        });
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )),
        );
        ({
            test_acquire_release_29(
                (*store.borrow()).copy_from(),
                (*load_one_arg_acq_rel.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut _a1: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .store_10(new_val, Some(std_memory_order_seq_cst))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    let result: Value<f32> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_float_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f32> = Rc::new(RefCell::new((-9999.99_f64 as f32)));
                    let mut r: bool = ({
                        let ___expected: Ptr<f32> = unexpected.as_pointer();
                        let ___desired: f32 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .compare_exchange_weak_19(
                                ___expected,
                                ___desired,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                                std_memory_order_seq_cst,
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            },
        )));
        ({ test_seq_cst_5((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_impl_65() {
    ({ testBasic_54() });
    ({ testBasic_55(std_memory_order_relaxed) });
    ({
        testBasic_56(
            std_memory_order_relaxed,
            memory_order_relaxed_61.with(|rc| *rc.borrow()),
        )
    });
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_18(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_release,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .load_13(Some(std_memory_order_acquire))
                    });
                }
            }),
        ));
        ({ test_acquire_release_30((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_22(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_release),
                            )
                    }) {}
                },
            )),
        );
        ({
            test_acquire_release_31(
                (*store_one_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut _a1: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .store_11(new_val, Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_18(
                                ___expected,
                                ___desired,
                                std_memory_order_acquire,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({ test_acquire_release_32((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_22(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_acquire),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_33(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_18(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_acq_rel,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_18(
                                ___expected,
                                ___desired,
                                std_memory_order_acq_rel,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({ test_acquire_release_34((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_22(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_acq_rel),
                            )
                    }) {}
                },
            )),
        );
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_22(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_acq_rel),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_35(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_18(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_seq_cst,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_18(
                                ___expected,
                                ___desired,
                                std_memory_order_seq_cst,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({ test_seq_cst_6((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_22(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_seq_cst),
                            )
                    }) {}
                },
            )),
        );
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_22(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_seq_cst),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_seq_cst_7(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut _a1: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .store_11(new_val, Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let result: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f64> = Rc::new(RefCell::new(-9999.99_f64));
                    let mut r: bool = ({
                        let ___expected: Ptr<f64> = unexpected.as_pointer();
                        let ___desired: f64 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_18(
                                ___expected,
                                ___desired,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                                memory_order_acquire_62.with(|rc| *rc.borrow()),
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({ test_acquire_release_36((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let result: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f64> = Rc::new(RefCell::new(-9999.99_f64));
                    let mut r: bool = ({
                        let ___expected: Ptr<f64> = unexpected.as_pointer();
                        let ___desired: f64 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_22(
                                ___expected,
                                ___desired,
                                Some(memory_order_acquire_62.with(|rc| *rc.borrow())),
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_37(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
        let load_one_arg_acq_rel: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(
                |x: Ptr<std_atomic_double_>| -> f64 {
                    {
                        let result: Value<f64> = Rc::new(RefCell::new(
                            ({
                                ((*x.upgrade().deref()) as std___atomic_float_double_)
                                    .load_13(Some(std_memory_order_relaxed))
                            }),
                        ));
                        let unexpected: Value<f64> = Rc::new(RefCell::new(-9999.99_f64));
                        let mut r: bool = ({
                            let ___expected: Ptr<f64> = unexpected.as_pointer();
                            let ___desired: f64 = (*unexpected.borrow());
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .compare_exchange_weak_22(
                                    ___expected,
                                    ___desired,
                                    Some(memory_order_acq_rel_63.with(|rc| *rc.borrow())),
                                )
                        });
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )),
        );
        ({
            test_acquire_release_38(
                (*store.borrow()).copy_from(),
                (*load_one_arg_acq_rel.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut _a1: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .store_11(new_val, Some(std_memory_order_seq_cst))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let result: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_13(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f64> = Rc::new(RefCell::new(-9999.99_f64));
                    let mut r: bool = ({
                        let ___expected: Ptr<f64> = unexpected.as_pointer();
                        let ___desired: f64 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_18(
                                ___expected,
                                ___desired,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                                std_memory_order_seq_cst,
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({ test_seq_cst_8((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_impl_66() {
    ({ testBasic_57() });
    ({ testBasic_58(std_memory_order_relaxed) });
    ({
        testBasic_59(
            std_memory_order_relaxed,
            memory_order_relaxed_61.with(|rc| *rc.borrow()),
        )
    });
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_19(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_release,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .load_12(Some(std_memory_order_acquire))
                    });
                }
            }),
        ));
        ({ test_acquire_release_39((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_23(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_release),
                            )
                    }) {}
                },
            )),
        );
        ({
            test_acquire_release_40(
                (*store_one_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut _a1: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .store_10(new_val, Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_19(
                                ___expected,
                                ___desired,
                                std_memory_order_acquire,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({ test_acquire_release_41((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_23(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_acquire),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_42(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_19(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_acq_rel,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_19(
                                ___expected,
                                ___desired,
                                std_memory_order_acq_rel,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({ test_acquire_release_43((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_23(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_acq_rel),
                            )
                    }) {}
                },
            )),
        );
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_23(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_acq_rel),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_44(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_19(
                                old_val.as_pointer(),
                                new_val,
                                std_memory_order_seq_cst,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_19(
                                ___expected,
                                ___desired,
                                std_memory_order_seq_cst,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({ test_seq_cst_9((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, old_val: f64, mut new_val: f64| {
                    let old_val: Value<f64> = Rc::new(RefCell::new(old_val));
                    'loop_: while !({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_23(
                                old_val.as_pointer(),
                                new_val,
                                Some(std_memory_order_seq_cst),
                            )
                    }) {}
                },
            )),
        );
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let val: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    'loop_: while !({
                        let ___expected: Ptr<f64> = val.as_pointer();
                        let ___desired: f64 = (*val.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_23(
                                ___expected,
                                ___desired,
                                Some(std_memory_order_seq_cst),
                            )
                    }) {}
                    return (*val.borrow());
                }
            }),
        ));
        ({
            test_seq_cst_10(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut _a1: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .store_10(new_val, Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let result: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f64> = Rc::new(RefCell::new(-9999.99_f64));
                    let mut r: bool = ({
                        let ___expected: Ptr<f64> = unexpected.as_pointer();
                        let ___desired: f64 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_19(
                                ___expected,
                                ___desired,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                                memory_order_acquire_62.with(|rc| *rc.borrow()),
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({ test_acquire_release_45((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let result: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f64> = Rc::new(RefCell::new(-9999.99_f64));
                    let mut r: bool = ({
                        let ___expected: Ptr<f64> = unexpected.as_pointer();
                        let ___desired: f64 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_23(
                                ___expected,
                                ___desired,
                                Some(memory_order_acquire_62.with(|rc| *rc.borrow())),
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_46(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
        let load_one_arg_acq_rel: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(
                |x: Ptr<std_atomic_double_>| -> f64 {
                    {
                        let result: Value<f64> = Rc::new(RefCell::new(
                            ({
                                ((*x.upgrade().deref()) as std___atomic_float_double_)
                                    .load_12(Some(std_memory_order_relaxed))
                            }),
                        ));
                        let unexpected: Value<f64> = Rc::new(RefCell::new(-9999.99_f64));
                        let mut r: bool = ({
                            let ___expected: Ptr<f64> = unexpected.as_pointer();
                            let ___desired: f64 = (*unexpected.borrow());
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .compare_exchange_weak_23(
                                    ___expected,
                                    ___desired,
                                    Some(memory_order_acq_rel_63.with(|rc| *rc.borrow())),
                                )
                        });
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )),
        );
        ({
            test_acquire_release_47(
                (*store.borrow()).copy_from(),
                (*load_one_arg_acq_rel.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut _a1: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .store_10(new_val, Some(std_memory_order_seq_cst))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    let result: Value<f64> = Rc::new(RefCell::new(
                        ({
                            ((*x.upgrade().deref()) as std___atomic_float_double_)
                                .load_12(Some(std_memory_order_relaxed))
                        }),
                    ));
                    let unexpected: Value<f64> = Rc::new(RefCell::new(-9999.99_f64));
                    let mut r: bool = ({
                        let ___expected: Ptr<f64> = unexpected.as_pointer();
                        let ___desired: f64 = (*unexpected.borrow());
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .compare_exchange_weak_19(
                                ___expected,
                                ___desired,
                                memory_order_relaxed_61.with(|rc| *rc.borrow()),
                                std_memory_order_seq_cst,
                            )
                    });
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({ test_seq_cst_11((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_67() {
    ({ test_impl_60() });
    {
        ({ test_impl_64() });
    }
}
pub fn test_68() {
    ({ test_impl_65() });
    {
        ({ test_impl_66() });
    }
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
    ({ test_67() });
    ({ test_68() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
