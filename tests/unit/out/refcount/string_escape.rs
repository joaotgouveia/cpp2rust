extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut special: Ptr<()> = Ptr::<()>::from_string_literal(
        b"\x07\x08\t\n\x0b\x0c\r !\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~\xff",
    );
    let mut i: () = unimplemented!();
    'loop_: while (i
        < ((::std::mem::size_of::<[(); 41]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()))))
    {
        unimplemented!();
        i.postfix_inc();
    }
    return unimplemented!();
}
thread_local!(
    static expected_0: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        7_(),
        8_(),
        9_(),
        10_(),
        11_(),
        12_(),
        13_(),
        32_(),
        33_(),
        34_(),
        35_(),
        36_(),
        37_(),
        38_(),
        39_(),
        40_(),
        41_(),
        42_(),
        43_(),
        44_(),
        45_(),
        46_(),
        47_(),
        58_(),
        59_(),
        60_(),
        61_(),
        62_(),
        63_(),
        64_(),
        91_(),
        92_(),
        93_(),
        94_(),
        95_(),
        96_(),
        123_(),
        124_(),
        125_(),
        126_(),
        (b'\xff' as ()),
    ])));
);
pub fn __cpp2rust_init_globals() {}
