extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
fn main_0() -> () {
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
    let mut table_size: () = 4_();
    ({
        let ___dest: () =
            (((table.as_pointer() as Ptr<Entry>).offset(table_size)) as Ptr<Entry>).to_any();
        let ___src: () = (((table.as_pointer() as Ptr<Entry>).offset(0_())) as Ptr<Entry>).to_any();
        let ___n: () = (table_size).wrapping_mul((4usize as ()));
        memcpy_0(___dest, ___src, ___n)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
