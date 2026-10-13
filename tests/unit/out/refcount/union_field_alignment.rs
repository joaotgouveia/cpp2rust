extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct max_align_t {
    #[offset(0)]
    pub __clang_max_align_nonce1: i64,
    #[offset(16)]
    #[byte_size(16)]
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
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
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn bytes(this: Ptr<Self>) -> Ptr<u8> {
        this.reinterpret_cast()
    }
    pub fn aligner(this: Ptr<Self>) -> Ptr<AnyPtr> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct node {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<node>,
    #[offset(8)]
    #[byte_size(8)]
    pub x: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<node> = <Value<node>>::default();
    (*n.borrow_mut()).next = Ptr::<node>::null();
    elem!(
        (anon_0::bytes(field_ptr!(n.as_pointer(), x)).reinterpret_cast::<u8>() as Ptr::<u8>),
        0
    )
    .write(171_u8);
    {
        &(4usize);
        {
            if (((((elem!(
                (anon_0::bytes(field_ptr!(n.as_pointer(), x)).reinterpret_cast::<u8>()
                    as Ptr::<u8>),
                0
            )
            .read()) as i32)
                == 171) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"n.x.bytes[0] == 0xAB"),
                        Ptr::<i8>::from_string_literal(b"union_field_alignment.c"),
                        19_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
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
pub fn __cpp2rust_init_globals() {}
