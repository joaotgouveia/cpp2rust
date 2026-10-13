extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[i8]>> =
        Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8, 4_i8, 5_i8, 6_i8])));
    ({
        let ___dest: AnyPtr =
            ((buf.as_pointer() as Ptr<i8>).offset((2) as isize) as Ptr<i8>).to_any();
        let ___src: AnyPtr = ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
        memmove_0(___dest, ___src, 4_usize)
    });
    (if (((*buf.borrow())[(0) as usize] as i32) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"buf[0] == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((*buf.borrow())[(1) as usize] as i32) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"buf[1] == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((*buf.borrow())[(2) as usize] as i32) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"buf[2] == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((*buf.borrow())[(3) as usize] as i32) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"buf[3] == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((*buf.borrow())[(4) as usize] as i32) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"buf[4] == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((*buf.borrow())[(5) as usize] as i32) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"buf[5] == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn memmove_0(mut __dest: AnyPtr, mut __src: AnyPtr, mut __n: usize) -> AnyPtr {
    unimplemented!()
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
