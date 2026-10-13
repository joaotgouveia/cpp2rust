extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Item {
    #[offset(0)]
    pub flags: u8,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let item: Value<Item> = Rc::new(RefCell::new(Item { flags: 0_u8 }));
    let mut ptr: Ptr<Item> = (item.as_pointer());
    field!((ptr), flags).write({ (((ptr).with(|__s| __s.flags) as i32) | (1 << 0)) as u8 });
    field!((ptr), flags).write({ (((ptr).with(|__s| __s.flags) as i32) | (1 << 1)) as u8 });
    (if ((ptr.with(|__s| __s.flags) as i32) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"ptr->flags == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    field!((ptr), flags)
        .write({ (((ptr).with(|__s| __s.flags) as i32) & ((!(1 << 0) as u8) as i32)) as u8 });
    (if ((ptr.with(|__s| __s.flags) as i32) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"ptr->flags == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut bits: [u8; 4] = [0_u8, 0_u8, 0_u8, 0_u8];
    (bits)[((5) / 8) as usize] =
        { (((bits)[((5) / 8) as usize] as i32) | (((1 << ((5) & 7)) as u8) as i32)) as u8 };
    (bits)[((13) / 8) as usize] =
        { (((bits)[((13) / 8) as usize] as i32) | (((1 << ((13) & 7)) as u8) as i32)) as u8 };
    (if ((bits[(0) as usize] as i32) == 32) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"bits[0] == 32"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((bits[(1) as usize] as i32) == 32) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"bits[1] == 32"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((bits[(2) as usize] as i32) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"bits[2] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    if ((ptr.with(|__s| __s.flags) as i32) != 0) {
        field!((ptr), flags)
            .write({ (((ptr).with(|__s| __s.flags) as i32) & ((!(1 << 1) as u8) as i32)) as u8 });
    }
    (if ((ptr.with(|__s| __s.flags) as i32) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"ptr->flags == 0"),
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
pub fn __cpp2rust_init_globals() {}
