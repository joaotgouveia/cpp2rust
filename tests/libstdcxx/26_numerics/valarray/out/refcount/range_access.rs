extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let va: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    &(unimplemented!());
    &(unimplemented!());
    let cva: Ptr<()> = va.as_pointer();
    &(unimplemented!());
    &(unimplemented!());
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
pub fn test02_1() {
    let va: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    &(unimplemented!());
    &(unimplemented!());
    let cva: Ptr<()> = va.as_pointer();
    &(unimplemented!());
    &(unimplemented!());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_0() });
    ({ test02_1() });
    0
}
pub fn __cpp2rust_init_globals() {}
