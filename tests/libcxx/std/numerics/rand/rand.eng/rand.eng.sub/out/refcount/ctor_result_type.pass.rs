extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_implicit_0() -> () {
    return unimplemented!();
}
pub fn make_implicit_1() -> () {
    return unimplemented!();
}
pub fn test_convertible_imp_2(mut _a0: ()) -> () {
    return true;
}
pub fn test_convertible_imp_3(mut _a0: ()) -> () {
    return true;
}
pub fn test_convertible_imp_4(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_5(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_6() -> () {
    return ({ test_convertible_imp_2(0_()) });
}
pub fn test_convertible_7() -> () {
    return ({ test_convertible_imp_3(0_()) });
}
pub fn test_convertible_8() -> () {
    return ({ test_convertible_imp_4(0_()) });
}
pub fn test_convertible_9() -> () {
    return ({ test_convertible_imp_5(0_()) });
}
pub fn to_string_10(e: Ptr<()>) -> () {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_11(e: Ptr<()>) -> () {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn test1_12() {
    let mut a : Ptr< ()  >  = Ptr::<() >::from_string_literal(b"15136306 8587749 2346244 16479026 15515802 9510553 16090340 14501685 13839944 10789678 11581259 9590790 5840316 5953700 13398366 8134459 16629731 6851902 15583892 1317475 4231148 9092691 5707268 2355175 0" )  ;
    let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test2_13() {
    let mut a : Ptr< ()  >  = Ptr::<() >::from_string_literal(b"10880375256626 126660097854724 33643165434010 78293780235492 179418984296008 96783156950859 238199764491708 34339434557790 155299155394531 29014415493780 209265474179052 263777435457028 0" )  ;
    let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_implicit_ctor_14() {
    unimplemented!();
}
pub fn test_implicit_ctor_15() {
    unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test1_12() });
    ({ test2_13() });
    const _: () = assert!(true, "test_convertible<std::ranlux24_base>()");;
    const _: () = assert!(true, "test_convertible<std::ranlux48_base>()");;
    ({ test_implicit_ctor_14() });
    ({ test_implicit_ctor_15() });
    const _: () = assert!(
        true,
        "!test_convertible<std::ranlux24_base, std::uint_fast32_t>()"
    );;
    const _: () = assert!(
        true,
        "!test_convertible<std::ranlux48_base, std::uint_fast64_t>()"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
