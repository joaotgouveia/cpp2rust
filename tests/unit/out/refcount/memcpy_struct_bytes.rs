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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<point> = Rc::new(RefCell::new(point { x: 3, y: 7 }));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new((0..8).map(|_| 0_u8).collect::<Box<[u8]>>()));
    ({
        let ___dest: AnyPtr = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any();
        let ___n: usize = 8usize;
        memcpy_0(___dest, ((src.as_pointer()) as Ptr<point>).to_any(), ___n)
    });
    let dst: Value<point> = <Value<point>>::default();
    ({
        let ___dest: AnyPtr = ((dst.as_pointer()) as Ptr<point>).to_any();
        let ___n: usize = 8usize;
        memcpy_0(
            ___dest,
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
            ___n,
        )
    });
    {
        &(4usize);
        {
            if ((({ (*dst.borrow()).x } == 3) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"dst.x == 3"),
                        Ptr::<i8>::from_string_literal(b"memcpy_struct_bytes.c"),
                        19_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (*dst.borrow()).y } == 7) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"dst.y == 7"),
                        Ptr::<i8>::from_string_literal(b"memcpy_struct_bytes.c"),
                        20_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn memcpy_0(mut __dest: AnyPtr, mut __src: AnyPtr, mut __n: usize) -> AnyPtr {
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
