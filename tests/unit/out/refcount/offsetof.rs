extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Layout {
    #[offset(0)]
    pub a: u8,
    #[offset(4)]
    pub b: u32,
    #[offset(8)]
    pub c: u16,
}
impl CppMove for Layout {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(66)]
pub struct Frame {
    #[offset(0)]
    pub tag: u16,
    #[offset(2)]
    #[byte_size(64)]
    pub body: Box<[i8]>,
}
impl CppMove for Frame {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Frame {
    fn default() -> Self {
        {
            Frame {
                tag: 0_u16,
                body: (0..64).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ cpp2rust_assert_fail_0((0_usize == 0_u64)) });
    ({ cpp2rust_assert_fail_0((4_usize == 4_u64)) });
    ({ cpp2rust_assert_fail_0((8_usize == 8_u64)) });
    let v: Value<Layout> = Rc::new(RefCell::new(Layout {
        a: 0_u8,
        b: 0_u32,
        c: 0_u16,
    }));
    (*v.borrow_mut()).b = 3735928559_u32;
    let mut base: Ptr<u8> = (v.as_pointer()).reinterpret_cast::<u8>();
    let mut bp: Ptr<u32> = (base.offset((4_usize) as isize)).reinterpret_cast::<u32>();
    ({ cpp2rust_assert_fail_0(((bp.read()) == 3735928559_u32)) });
    (base.offset((4_usize) as isize))
        .reinterpret_cast::<u32>()
        .write(305419896_u32);
    ({ cpp2rust_assert_fail_0(({ (*v.borrow()).b } == 305419896_u32)) });
    let mut text: Ptr<i8> = Ptr::<i8>::from_string_literal(b"example-body");
    let mut len: u64 = ({ strlen_1((text).clone()) }).wrapping_add(1_u64);
    let mut total: u64 = (2_usize as u64).wrapping_add(len);
    ({ cpp2rust_assert_fail_0((total == (2_u64).wrapping_add(len))) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
