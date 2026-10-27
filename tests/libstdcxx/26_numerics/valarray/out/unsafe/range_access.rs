extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut va: () = unimplemented!();
    &(unimplemented!());
    &(unimplemented!());
    let cva: *const () = &va;
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
}
pub unsafe fn test02_1() {
    let mut va: () = unimplemented!();
    &(unimplemented!());
    &(unimplemented!());
    let cva: *const () = &va;
    &(unimplemented!());
    &(unimplemented!());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_0() });
    (unsafe { test02_1() });
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
