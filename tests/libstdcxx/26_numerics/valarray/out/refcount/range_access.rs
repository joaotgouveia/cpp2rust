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
    &({ begin_1(va.as_pointer()) });
    &({ libcc2rs::end_refcount(va.as_pointer()) });
    let cva: Ptr<std_valarray_double_> = va.as_pointer();
    &({ begin_2((cva).clone()) });
    &({ libcc2rs::end_refcount((cva).clone()) });
    const _: () = assert!(true, "std::is_same<Iter, decltype(std::end(va))>::value");;
    const _: () = assert!(
        true,
        "std::is_same<IterTraits::iterator_category,\n\t\t\t      std::random_access_iterator_tag>::value"
    );;
    const _: () = assert!(true, "std::is_same<IterTraits::value_type, double>::value");;
    const _: () = assert!(true, "std::is_same<IterTraits::reference, double&>::value");;
    const _: () = assert!(true, "std::is_same<CIter, decltype(std::end(cva))>::value");;
    const _: () = assert!(
        true,
        "std::is_same<CIterTraits::iterator_category,\n\t\t\t      std::random_access_iterator_tag>::value"
    );;
    const _: () = assert!(true, "std::is_same<CIterTraits::value_type, double>::value");;
    const _: () = assert!(
        true,
        "std::is_same<CIterTraits::reference, const double&>::value"
    );;
    const _: () = assert!(true, "std::contiguous_iterator<Iter>");;
    const _: () = assert!(true, "std::contiguous_iterator<CIter>");;
}
pub fn test02_3() {
    let va: Value<std_valarray_double_> = Rc::new(RefCell::new(<std_valarray_double_>::default()));
    &({ begin_1(va.as_pointer()) });
    &({ libcc2rs::end_refcount(va.as_pointer()) });
    let cva: Ptr<std_valarray_double_> = va.as_pointer();
    &({ begin_2((cva).clone()) });
    &({ libcc2rs::end_refcount((cva).clone()) });
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
