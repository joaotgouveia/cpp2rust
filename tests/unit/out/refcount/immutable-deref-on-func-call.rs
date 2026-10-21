extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Item {
    #[offset(0)]
    pub value: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut arr: Ptr<Item> = Ptr::alloc_array(
        (0..2_())
            .map(|_| <Item>::default())
            .collect::<Box<[Item]>>(),
    );
    field!(elem!(arr, unimplemented!()), value).write(1_());
    field!(elem!(arr, 1_()), value).write(2_());
    ({
        let _other: Ptr<Item> = (arr.offset((1_()) as isize));
        ItemImpl::foo(&arr.offset((unimplemented!()) as isize), _other)
    });
    let mut result: () = (elem!(arr, unimplemented!()).with(|__s| __s.value)
        + elem!(arr, 1_()).with(|__s| __s.value));
    arr.delete();
    unimplemented!();
    return unimplemented!();
}
pub trait ItemImpl {
    fn foo(&self, other: Ptr<Item>);
}
impl ItemImpl for Ptr<Item> {
    fn foo(&self, mut other: Ptr<Item>) {
        field!(other, value).write(10_());
    }
}
pub fn __cpp2rust_init_globals() {}
