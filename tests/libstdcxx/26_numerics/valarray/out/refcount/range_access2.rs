extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let va: Value<std_valarray_double_> = Rc::new(RefCell::new(std_valarray_double_::new_1({
        Vec::from([1_f64, 2_f64, 3_f64])
    })));
    &({ cbegin_1(va.as_pointer()) });
    &({ cend_2(va.as_pointer()) });
    let cva: Ptr<std_valarray_double_> = va.as_pointer();
    &({ cbegin_1((cva).clone()) });
    &({ cend_2((cva).clone()) });
}
pub fn test02_3() {
    let va: Value<std_valarray_double_> = Rc::new(RefCell::new(<std_valarray_double_>::default()));
    &({ cbegin_1(va.as_pointer()) });
    &({ cend_2(va.as_pointer()) });
    let cva: Ptr<std_valarray_double_> = va.as_pointer();
    &({ cbegin_1((cva).clone()) });
    &({ cend_2((cva).clone()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    ({ test02_3() });
    0
}
pub fn __cpp2rust_init_globals() {}
