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
pub type Width_enum = u32;
pub const Width_enum_W_64: Width_enum = 0;
pub const Width_enum_W_32: Width_enum = 1;
pub const Width_enum_W_16: Width_enum = 2;
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn text(this: Ptr<Self>) -> Ptr<Ptr<i8>> {
        this.reinterpret_cast()
    }
    pub fn handle(this: Ptr<Self>) -> Ptr<AnyPtr> {
        this.reinterpret_cast()
    }
    pub fn signed_n(this: Ptr<Self>) -> Ptr<i64> {
        this.reinterpret_cast()
    }
    pub fn f(this: Ptr<Self>) -> Ptr<f64> {
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
pub struct Sink {
    #[offset(0)]
    pub width: Width_enum,
    #[offset(8)]
    #[byte_size(8)]
    pub out: anon_0,
}
pub fn write_count_1(mut s: Ptr<Sink>, mut count: i64) {
    'switch: {
        match { (s.with(|__s| __s.width) as u32) } {
            __v if __v == ((Width_enum_W_64 as i32) as u32) => {
                (anon_0::handle(field_ptr!(s, out)).read())
                    .reinterpret_cast::<i64>()
                    .write(count);
                break 'switch;
            }
            __v if __v == ((Width_enum_W_32 as i32) as u32) => {
                (anon_0::handle(field_ptr!(s, out)).read())
                    .reinterpret_cast::<i32>()
                    .write((count as i32));
                break 'switch;
            }
            __v if __v == ((Width_enum_W_16 as i32) as u32) => {
                (anon_0::handle(field_ptr!(s, out)).read())
                    .reinterpret_cast::<i16>()
                    .write((count as i16));
                break 'switch;
            }
            _ => {}
        }
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf64: Value<i64> = Rc::new(RefCell::new(0_i64));
    let buf32: Value<i32> = Rc::new(RefCell::new(0));
    let buf16: Value<i16> = Rc::new(RefCell::new(0_i16));
    let s: Value<Sink> = <Value<Sink>>::default();
    (*s.borrow_mut()).width = Width_enum_W_64;
    anon_0::handle(field_ptr!(s.as_pointer(), out))
        .write(((buf64.as_pointer()) as Ptr<i64>).to_any());
    ({ write_count_1((s.as_pointer()), 1234605616436508552_i64) });
    {
        &(4usize);
        {
            if ((((*buf64.borrow()) == 1234605616436508552_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"buf64 == 0x1122334455667788LL"),
                        Ptr::<i8>::from_string_literal(b"union_void_ptr_sized_deref.c"),
                        46_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    (*s.borrow_mut()).width = Width_enum_W_32;
    anon_0::handle(field_ptr!(s.as_pointer(), out))
        .write(((buf32.as_pointer()) as Ptr<i32>).to_any());
    ({ write_count_1((s.as_pointer()), 305419896_i64) });
    {
        &(4usize);
        {
            if ((((*buf32.borrow()) == 305419896) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"buf32 == 0x12345678"),
                        Ptr::<i8>::from_string_literal(b"union_void_ptr_sized_deref.c"),
                        51_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    (*s.borrow_mut()).width = Width_enum_W_16;
    anon_0::handle(field_ptr!(s.as_pointer(), out))
        .write(((buf16.as_pointer()) as Ptr<i16>).to_any());
    ({ write_count_1((s.as_pointer()), 4660_i64) });
    {
        &(4usize);
        {
            if (((((*buf16.borrow()) as i32) == 4660) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"buf16 == 0x1234"),
                        Ptr::<i8>::from_string_literal(b"union_void_ptr_sized_deref.c"),
                        56_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
