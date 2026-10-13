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
pub fn out_param_0(out: Option<Ptr<usize>>) {
    let mut out: Ptr<usize> = out.unwrap_or_else(|| Ptr::<usize>::null());
    if !(out).is_null() {
        out.write(4_usize);
    }
}
pub fn parse_1(mut v: i32, idx: Option<Ptr<usize>>) -> i32 {
    let mut idx: Ptr<usize> = idx.unwrap_or_else(|| Ptr::<usize>::null());
    if !(idx).is_null() {
        idx.write(3_usize);
    }
    return v;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v2: Value<usize> = Rc::new(RefCell::new(0_usize));
    ({ out_param_0(Some((v2.as_pointer()))) });
    (if ((*v2.borrow()) == 4_usize) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"v2 == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let pidx: Value<usize> = Rc::new(RefCell::new(0_usize));
    (if (({ parse_1(1, Some((pidx.as_pointer()))) }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"ns::parse(1, &pidx) == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*pidx.borrow()) == 3_usize) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"pidx == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let sv: Value<usize> = Rc::new(RefCell::new(7_usize));
    let sp: Value<Ptr<usize>> = Rc::new(RefCell::new((sv.as_pointer())));
    let mut spp: Ptr<Ptr<usize>> = (sp.as_pointer());
    (if (((spp.read()).read()) == 7_usize) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"**spp == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
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
