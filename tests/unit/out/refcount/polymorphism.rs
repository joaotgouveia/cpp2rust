extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait Animal {
    fn bark(&self) -> bool;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Dog {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Cat {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let dog: Value<Dog> = Rc::new(RefCell::new(<Dog>::default()));
    let mut animal: PtrDyn<dyn Animal> = (dog.as_pointer()).to_dyn::<dyn Animal>(|w| w);
    let mut eat1: bool = ({ (*animal.upgrade().deref()).bark() });
    let cat: Value<Cat> = Rc::new(RefCell::new(<Cat>::default()));
    animal = (cat.as_pointer()).to_dyn::<dyn Animal>(|w| w);
    let mut eat2: bool = ({ (*animal.upgrade().deref()).bark() });
    (if (eat1) && (!(eat2)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"eat1 && !eat2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_0(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
impl Animal for Cat {
    fn bark(&self) -> bool {
        return false;
    }
}
impl Animal for Dog {
    fn bark(&self) -> bool {
        return true;
    }
}
pub trait CatImpl {
    fn meow(&self) -> bool;
}
impl CatImpl for Ptr<Cat> {
    fn meow(&self) -> bool {
        return true;
    }
}
pub fn __cpp2rust_init_globals() {}
