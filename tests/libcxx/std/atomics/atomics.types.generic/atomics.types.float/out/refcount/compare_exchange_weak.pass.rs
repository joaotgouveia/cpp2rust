extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_seq_cst_0(store_op: FnPtr<fn(Ptr<()>, f32, f32)>, load_op: FnPtr<fn(Ptr<()>) -> f32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_1(store_op: FnPtr<fn(Ptr<()>, f32, f32)>, load_op: FnPtr<fn(Ptr<()>) -> f32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_2(store_op: FnPtr<fn(Ptr<()>, f32, f32)>, load_op: FnPtr<fn(Ptr<()>) -> f32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_3(store_op: FnPtr<fn(Ptr<()>, f32, f32)>, load_op: FnPtr<fn(Ptr<()>) -> f32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_4(store_op: FnPtr<fn(Ptr<()>, f32, f32)>, load_op: FnPtr<fn(Ptr<()>) -> f32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_5(store_op: FnPtr<fn(Ptr<()>, f32, f32)>, load_op: FnPtr<fn(Ptr<()>) -> f32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_6(store_op: FnPtr<fn(Ptr<()>, f64, f64)>, load_op: FnPtr<fn(Ptr<()>) -> f64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_7(store_op: FnPtr<fn(Ptr<()>, f64, f64)>, load_op: FnPtr<fn(Ptr<()>) -> f64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_8(store_op: FnPtr<fn(Ptr<()>, f64, f64)>, load_op: FnPtr<fn(Ptr<()>) -> f64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_9(store_op: FnPtr<fn(Ptr<()>, f64, f64)>, load_op: FnPtr<fn(Ptr<()>) -> f64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_10(store_op: FnPtr<fn(Ptr<()>, f64, f64)>, load_op: FnPtr<fn(Ptr<()>) -> f64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_11(store_op: FnPtr<fn(Ptr<()>, f64, f64)>, load_op: FnPtr<fn(Ptr<()>) -> f64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_12(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_13(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_14(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_15(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_16(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_17(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_18(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_19(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_20(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_21(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_22(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_23(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_24(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_25(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_26(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_27(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_28(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_29(
    store_op: FnPtr<fn(Ptr<()>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<()>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_30(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_31(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_32(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_33(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_34(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_35(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_36(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_37(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_38(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_39(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_40(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_41(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_42(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_43(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_44(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_45(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_46(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_47(
    store_op: FnPtr<fn(Ptr<()>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<()>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn testBasic_48() {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.2_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == (1.2_f64 as f32)));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.5_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == (1.2_f64 as f32)));
        {
            assert!((expected == (1.2_f64 as f32)));
        }
    }
}
pub fn testBasic_49(mut memory_order: ()) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.2_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == (1.2_f64 as f32)));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.5_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == (1.2_f64 as f32)));
        {
            assert!((expected == (1.2_f64 as f32)));
        }
    }
}
pub fn testBasic_50(mut memory_order_0: (), mut memory_order_1: ()) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.2_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == (1.2_f64 as f32)));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.5_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == (1.2_f64 as f32)));
        {
            assert!((expected == (1.2_f64 as f32)));
        }
    }
}
pub fn testBasic_51() {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.2_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == (1.2_f64 as f32)));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.5_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == (1.2_f64 as f32)));
        {
            assert!((expected == (1.2_f64 as f32)));
        }
    }
}
pub fn testBasic_52(mut memory_order: ()) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.2_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == (1.2_f64 as f32)));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.5_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == (1.2_f64 as f32)));
        {
            assert!((expected == (1.2_f64 as f32)));
        }
    }
}
pub fn testBasic_53(mut memory_order_0: (), mut memory_order_1: ()) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.2_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == (1.2_f64 as f32)));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f32 = (1.5_f64 as f32);
        let mut desired: f32 = (2.3_f64 as f32);
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == (1.2_f64 as f32)));
        {
            assert!((expected == (1.2_f64 as f32)));
        }
    }
}
pub fn testBasic_54() {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.2_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == 1.2_f64));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.5_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == 1.2_f64));
        {
            assert!((expected == 1.2_f64));
        }
    }
}
pub fn testBasic_55(mut memory_order: ()) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.2_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == 1.2_f64));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.5_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == 1.2_f64));
        {
            assert!((expected == 1.2_f64));
        }
    }
}
pub fn testBasic_56(mut memory_order_0: (), mut memory_order_1: ()) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.2_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == 1.2_f64));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.5_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == 1.2_f64));
        {
            assert!((expected == 1.2_f64));
        }
    }
}
pub fn testBasic_57() {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.2_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == 1.2_f64));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.5_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == 1.2_f64));
        {
            assert!((expected == 1.2_f64));
        }
    }
}
pub fn testBasic_58(mut memory_order: ()) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.2_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == 1.2_f64));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.5_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == 1.2_f64));
        {
            assert!((expected == 1.2_f64));
        }
    }
}
pub fn testBasic_59(mut memory_order_0: (), mut memory_order_1: ()) {
    const _: () = assert!(
        true,
        "HasNoexceptCompareExchangeWeak<T, MaybeVolatile, MemoryOrder...>"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.2_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        if r {
            assert!((unimplemented!() == desired));
        }
        assert!((expected == 1.2_f64));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut expected: f64 = 1.5_f64;
        let mut desired: f64 = 2.3_f64;
        let mut r: bool = unimplemented!();
        assert!(!(r));
        assert!((unimplemented!() == 1.2_f64));
        {
            assert!((expected == 1.2_f64));
        }
    }
}
pub fn test_impl_60() {
    ({ testBasic_48() });
    ({ testBasic_49(unimplemented!()) });
    ({ testBasic_50(unimplemented!(), unimplemented!()) });
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        return unimplemented!();
                    }
                },
            )));
        ({ test_acquire_release_12((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        ({
            test_acquire_release_13(
                (*store_one_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut _a1: f32, mut new_val: f32| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_acquire_release_14((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_15(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_acquire_release_16((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_17(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_seq_cst_0((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_seq_cst_1(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut _a1: f32, mut new_val: f32| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let result: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f32 = (-9999.99_f64 as f32);
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({ test_acquire_release_18((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let result: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f32 = (-9999.99_f64 as f32);
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_19(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
        let load_one_arg_acq_rel: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> f32>::new(|x: Ptr<()>| -> f32 {
                {
                    let result: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                    let mut unexpected: f32 = (-9999.99_f64 as f32);
                    let mut r: bool = unimplemented!();
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_20(
                (*store.borrow()).copy_from(),
                (*load_one_arg_acq_rel.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut _a1: f32, mut new_val: f32| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let result: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f32 = (-9999.99_f64 as f32);
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({ test_seq_cst_2((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_impl_61() {
    ({ testBasic_51() });
    ({ testBasic_52(unimplemented!()) });
    ({ testBasic_53(unimplemented!(), unimplemented!()) });
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        return unimplemented!();
                    }
                },
            )));
        ({ test_acquire_release_21((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        ({
            test_acquire_release_22(
                (*store_one_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut _a1: f32, mut new_val: f32| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_acquire_release_23((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_24(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_acquire_release_25((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_26(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_seq_cst_3((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let val: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_seq_cst_4(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut _a1: f32, mut new_val: f32| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let result: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f32 = (-9999.99_f64 as f32);
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({ test_acquire_release_27((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let result: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f32 = (-9999.99_f64 as f32);
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_28(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
        let load_one_arg_acq_rel: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> f32>::new(|x: Ptr<()>| -> f32 {
                {
                    let result: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                    let mut unexpected: f32 = (-9999.99_f64 as f32);
                    let mut r: bool = unimplemented!();
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_29(
                (*store.borrow()).copy_from(),
                (*load_one_arg_acq_rel.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                |x: Ptr<()>, mut _a1: f32, mut new_val: f32| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                |x: Ptr<()>| -> f32 {
                    {
                        let result: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f32 = (-9999.99_f64 as f32);
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({ test_seq_cst_5((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_impl_62() {
    ({ testBasic_54() });
    ({ testBasic_55(unimplemented!()) });
    ({ testBasic_56(unimplemented!(), unimplemented!()) });
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        return unimplemented!();
                    }
                },
            )));
        ({ test_acquire_release_30((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        ({
            test_acquire_release_31(
                (*store_one_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut _a1: f64, mut new_val: f64| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_acquire_release_32((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_33(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_acquire_release_34((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_35(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_seq_cst_6((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_seq_cst_7(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut _a1: f64, mut new_val: f64| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let result: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f64 = -9999.99_f64;
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({ test_acquire_release_36((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let result: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f64 = -9999.99_f64;
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_37(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
        let load_one_arg_acq_rel: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> f64>::new(|x: Ptr<()>| -> f64 {
                {
                    let result: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                    let mut unexpected: f64 = -9999.99_f64;
                    let mut r: bool = unimplemented!();
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_38(
                (*store.borrow()).copy_from(),
                (*load_one_arg_acq_rel.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut _a1: f64, mut new_val: f64| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let result: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f64 = -9999.99_f64;
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({ test_seq_cst_8((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_impl_63() {
    ({ testBasic_57() });
    ({ testBasic_58(unimplemented!()) });
    ({ testBasic_59(unimplemented!(), unimplemented!()) });
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        return unimplemented!();
                    }
                },
            )));
        ({ test_acquire_release_39((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        ({
            test_acquire_release_40(
                (*store_one_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut _a1: f64, mut new_val: f64| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_acquire_release_41((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_42(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_acquire_release_43((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_44(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({ test_seq_cst_9((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let store_one_arg: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                    'loop_: while !(unimplemented!()) {}
                },
            )));
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let val: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        'loop_: while !(unimplemented!()) {}
                        return (*val.borrow());
                    }
                },
            )));
        ({
            test_seq_cst_10(
                (*store_one_arg.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut _a1: f64, mut new_val: f64| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let result: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f64 = -9999.99_f64;
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({ test_acquire_release_45((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
        let load_one_arg: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let result: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f64 = -9999.99_f64;
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({
            test_acquire_release_46(
                (*store.borrow()).copy_from(),
                (*load_one_arg.borrow()).copy_from(),
            )
        });
        let load_one_arg_acq_rel: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> f64>::new(|x: Ptr<()>| -> f64 {
                {
                    let result: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                    let mut unexpected: f64 = -9999.99_f64;
                    let mut r: bool = unimplemented!();
                    assert!(!(r));
                    return (*result.borrow());
                }
            }),
        ));
        ({
            test_acquire_release_47(
                (*store.borrow()).copy_from(),
                (*load_one_arg_acq_rel.borrow()).copy_from(),
            )
        });
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                |x: Ptr<()>, mut _a1: f64, mut new_val: f64| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                |x: Ptr<()>| -> f64 {
                    {
                        let result: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
                        let mut unexpected: f64 = -9999.99_f64;
                        let mut r: bool = unimplemented!();
                        assert!(!(r));
                        return (*result.borrow());
                    }
                },
            )));
        ({ test_seq_cst_11((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_64() {
    ({ test_impl_60() });
    {
        ({ test_impl_61() });
    }
}
pub fn test_65() {
    ({ test_impl_62() });
    {
        ({ test_impl_63() });
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
    ({ test_64() });
    ({ test_65() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
