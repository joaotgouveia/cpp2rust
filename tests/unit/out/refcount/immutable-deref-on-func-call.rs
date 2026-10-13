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
    pub value: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut arr: Ptr<Item> = Ptr::alloc_array(
        (0..2_usize)
            .map(|_| <Item>::default())
            .collect::<Box<[Item]>>(),
    );
    field!(elem!(arr, 0), value).write(1);
    field!(elem!(arr, 1), value).write(2);
    ({
        let _other: Ptr<Item> = (arr.offset((1) as isize));
        ItemImpl::foo(&arr.offset((0) as isize), _other)
    });
    let mut result: i32 = ({ (*elem!(arr, 0).upgrade().deref()).value } + {
        (*elem!(arr, 1).upgrade().deref()).value
    });
    arr.delete();
    (if (result == 11) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"result == 11"),
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
pub trait ItemImpl {
    fn foo(&self, other: Ptr<Item>);
}
impl ItemImpl for Ptr<Item> {
    fn foo(&self, mut other: Ptr<Item>) {
        field!(other, value).write(10);
    }
}
pub fn __cpp2rust_init_globals() {}
