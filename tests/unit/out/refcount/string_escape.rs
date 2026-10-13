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
    let mut special: Ptr<i8> = Ptr::<i8>::from_string_literal(
        b"\x07\x08\t\n\x0b\x0c\r !\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~\xff",
    );
    let mut i: i32 = 0;
    'loop_: while (i < (((41usize as usize).wrapping_div((1usize as usize))) as i32)) {
        (if ({ ((elem!(special, i).read()) as i32) } == {
            (({
                let __idx = (i) as usize;
                expected_0.with(|rc| rc.borrow()[__idx])
            }) as i32)
        }) {
            &(0);
        } else {
            ({
                __assert_fail_1(
                    Ptr::<i8>::from_string_literal(b"special[i] == expected[i]"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.postfix_inc();
    }
    return 0;
}
thread_local!(
    static expected_0: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        7_i8,
        8_i8,
        9_i8,
        10_i8,
        11_i8,
        12_i8,
        13_i8,
        32_i8,
        33_i8,
        34_i8,
        35_i8,
        36_i8,
        37_i8,
        38_i8,
        39_i8,
        40_i8,
        41_i8,
        42_i8,
        43_i8,
        44_i8,
        45_i8,
        46_i8,
        47_i8,
        58_i8,
        59_i8,
        60_i8,
        61_i8,
        62_i8,
        63_i8,
        64_i8,
        91_i8,
        92_i8,
        93_i8,
        94_i8,
        95_i8,
        96_i8,
        123_i8,
        124_i8,
        125_i8,
        126_i8,
        (b'\xff' as i8),
    ])));
);
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
