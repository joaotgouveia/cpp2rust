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
pub type std_byte = u8;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __byte_operand_0 {}
impl Clone for __byte_operand_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __byte_operand_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let b1: Value<std_byte> = Rc::new(RefCell::new(1_std_byte));
    let mut ushift1: u32 = 3_u32;
    let mut shl1: std_byte = ({
        let ___b: std_byte = (*b1.borrow());
        operator_shl_1(___b, ushift1)
    });
    (if (shl1 == ((8) as std_byte)) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"shl1 == std::byte(0x08)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ushift2: u32 = 2_u32;
    let mut shr1: std_byte = ({
        let ___b: std_byte = shl1;
        operator_shr_3(___b, ushift2)
    });
    (if (shr1 == ((2) as std_byte)) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"shr1 == std::byte(0x02)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ushift3: u32 = 5_u32;
    ({
        let ___b: Ptr<std_byte> = b1.as_pointer();
        operator_shl_assign_4(___b, ushift3)
    });
    (if ((*b1.borrow()) == ((32) as std_byte)) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"b1 == std::byte(0x20)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ushift4: u32 = 3_u32;
    ({
        let ___b: Ptr<std_byte> = b1.as_pointer();
        operator_shr_assign_5(___b, ushift4)
    });
    (if ((*b1.borrow()) == ((4) as std_byte)) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"b1 == std::byte(0x04)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn operator_shl_1(mut __b: std_byte, mut __shift: u32) -> std_byte {
    unimplemented!()
}
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn operator_shr_3(mut __b: std_byte, mut __shift: u32) -> std_byte {
    unimplemented!()
}
pub fn operator_shl_assign_4(__b: Ptr<std_byte>, mut __shift: u32) -> Ptr<std_byte> {
    unimplemented!()
}
pub fn operator_shr_assign_5(__b: Ptr<std_byte>, mut __shift: u32) -> Ptr<std_byte> {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
