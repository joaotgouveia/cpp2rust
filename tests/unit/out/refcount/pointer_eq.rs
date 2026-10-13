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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let mut p1: Ptr<i32> = (x.as_pointer());
    let mut p2: Ptr<i32> = (x.as_pointer());
    (if ({ (p1).clone() } == { (p2).clone() }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"p1 == p2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let y: Value<i32> = Rc::new(RefCell::new(5));
    let mut p3: Ptr<i32> = (y.as_pointer());
    (if ({ (p1).clone() } != { (p3).clone() }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"p1 != p3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let mut p: Ptr<i32> = (arr.as_pointer() as Ptr<i32>);
    (if ({ (p).clone() } == { (arr.as_pointer() as Ptr<i32>) }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"p == arr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ p.offset((1) as isize) } == { ((arr.as_pointer() as Ptr<i32>).offset(1)) }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"p + 1 == &arr[1]"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ p.offset((2) as isize) } == { ((arr.as_pointer() as Ptr<i32>).offset(2)) }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"p + 2 == &arr[2]"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let val: Value<i32> = Rc::new(RefCell::new(42));
    let mut orig: Ptr<i32> = (val.as_pointer());
    let mut as_bytes: Ptr<u8> = orig.reinterpret_cast::<u8>();
    let mut back: Ptr<i32> = as_bytes.reinterpret_cast::<i32>();
    (if ({ (orig).clone() } == { (back).clone() }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"orig == back"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut arr_bytes: Ptr<u8> = (arr.as_pointer() as Ptr<i32>).reinterpret_cast::<u8>();
    let mut arr_back: Ptr<i32> = arr_bytes.reinterpret_cast::<i32>();
    (if ({ (arr_back).clone() } == { (arr.as_pointer() as Ptr<i32>) }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"arr_back == arr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ arr_back.offset((1) as isize) } == { ((arr.as_pointer() as Ptr<i32>).offset(1)) }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"arr_back + 1 == &arr[1]"),
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
