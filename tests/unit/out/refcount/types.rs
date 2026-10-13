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
#[byte_size(16)]
pub struct imaxdiv_t {
    #[offset(0)]
    pub quot: i64,
    #[offset(8)]
    pub rem: i64,
}
impl Clone for imaxdiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for imaxdiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
const _: () = assert!((8usize == 8usize), "sizeof(std::size_t) == sizeof(void *)");
const _: () = assert!((8usize == 8_usize), "sizeof(int *) == 8");
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut xu8: u8 = 8_u8;
    let mut xu16: u16 = 16_u16;
    let mut xu32: u32 = 32_u32;
    let mut xu64: u64 = 64_u64;
    let mut xsz1: usize = 64_usize;
    let mut xsz2: usize = 64_usize;
    let mut xi1: i8 = (-8_i32 as i8);
    let mut xi2: i16 = 16_i16;
    let mut xi3: i32 = 32;
    let mut xi4: i64 = 64_i64;
    let mut b: bool = (xu64 == 64_u64);
    let mut xld: f64 = 1.5E+0;
    let mut xwc: i32 = 65_i32;
    let mut xc8: u8 = 66_u8;
    let mut xc16: u16 = 67_u16;
    let mut xc32: u32 = 68_u32;
    let xnp: Value<AnyPtr> = Rc::new(RefCell::new(Default::default()));
    (if ((((((((((((xu8 as i32) + (xu16 as i32)) as u32).wrapping_add(xu32)) as u64)
        .wrapping_add(xu64))
    .wrapping_add((xsz1 as u64)))
    .wrapping_add((xsz2 as u64)))
    .wrapping_add((xi1 as u64)))
    .wrapping_add((xi2 as u64)))
    .wrapping_add((xi3 as u64)))
    .wrapping_add((xi4 as u64))
        == 352_u64)
    {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(
                    b"xu8 + xu16 + xu32 + xu64 + xsz1 + xsz2 + xi1 + xi2 + xi3 + xi4 == 352",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((xld * 2_f64) == 3_f64) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"xld * 2 == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut pi: f64 = 3.14159265358979E+0;
    (if (pi < 3.1415926540000001E+0) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"pi < 3.141592654"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut dmax: f64 = -1.7976931348623157E+308;
    (if (dmax < -1.0E+308) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"dmax < -1e308"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut big: f64 = f64::INFINITY;
    (if (big > 1.7976931348623157E+308) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"big > DBL_MAX"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut fbig: f32 = f64::INFINITY;
    (if (fbig > 3.40282347E+38) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"fbig > FLT_MAX"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((((xwc + (xc8 as i32)) + (xc16 as i32)) as u32).wrapping_add(xc32) == 266_u32) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"xwc + xc8 + xc16 + xc32 == 266"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (AnyPtr::default()).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"xnp == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ip: Ptr<i32> = Ptr::<i32>::null();
    (if (8usize == 8usize) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"sizeof(ip) == sizeof(void *)"),
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
