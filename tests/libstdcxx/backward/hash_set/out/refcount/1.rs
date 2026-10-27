extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let werte: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 25_(), 9_(), 16_(), -36_()])));
    let mut anzahl: () =
        (::std::mem::size_of::<[(); 5]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let intTable: Value<
        __gnu_cxx_hash_set_int____gnu_cxx_hash_int___std_equal_to_int___std_allocator_int__,
    > = Rc::new(RefCell::new(
        __gnu_cxx_hash_set_int____gnu_cxx_hash_int___std_equal_to_int___std_allocator_int__::new_1(
            { (werte.as_pointer() as Ptr<()>) },
            { (werte.as_pointer() as Ptr<()>).offset((anzahl) as isize) },
        ),
    ));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
