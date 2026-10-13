extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Probe {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Wrapper_0 {
    #[offset(0)]
    #[byte_size(1)]
    pub base_: Probe,
    #[offset(4)]
    pub tag: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut a: Wrapper_0 = <Wrapper_0>::default();
    a.tag = 3;
    let mut b: Wrapper_0 = (a).clone();
    (if (b.tag == 3) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"b.tag == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait ProbeImpl {
    fn operator_inc(&self) -> Ptr<Probe> {
        unimplemented!()
    }
}
impl ProbeImpl for Ptr<Probe> {}
pub fn __cpp2rust_init_globals() {}
