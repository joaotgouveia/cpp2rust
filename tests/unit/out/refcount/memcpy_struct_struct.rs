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
#[byte_size(4)]
pub struct Entry {
    #[offset(0)]
    pub bits: u8,
    #[offset(2)]
    pub value: u16,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let table: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            bits: 1_u8,
            value: 4369_u16,
        },
        Entry {
            bits: 2_u8,
            value: 8738_u16,
        },
        Entry {
            bits: 3_u8,
            value: 13107_u16,
        },
        Entry {
            bits: 4_u8,
            value: 17476_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
    ])));
    let mut table_size: usize = 4_usize;
    ({
        let ___dest: AnyPtr =
            (((table.as_pointer() as Ptr<Entry>).offset(table_size)) as Ptr<Entry>).to_any();
        let ___src: AnyPtr =
            (((table.as_pointer() as Ptr<Entry>).offset(0)) as Ptr<Entry>).to_any();
        let ___n: usize = ((table_size as u64).wrapping_mul((4usize as u64)) as usize);
        memcpy_0(___dest, ___src, ___n)
    });
    (if (({ (*table.borrow())[(4) as usize].bits } as i32) == 1)
        && (({ (*table.borrow())[(4) as usize].value } as i32) == 4369)
    {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"table[4].bits == 1 && table[4].value == 0x1111"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*table.borrow())[(5) as usize].bits } as i32) == 2)
        && (({ (*table.borrow())[(5) as usize].value } as i32) == 8738)
    {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"table[5].bits == 2 && table[5].value == 0x2222"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*table.borrow())[(6) as usize].bits } as i32) == 3)
        && (({ (*table.borrow())[(6) as usize].value } as i32) == 13107)
    {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"table[6].bits == 3 && table[6].value == 0x3333"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*table.borrow())[(7) as usize].bits } as i32) == 4)
        && (({ (*table.borrow())[(7) as usize].value } as i32) == 17476)
    {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"table[7].bits == 4 && table[7].value == 0x4444"),
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
