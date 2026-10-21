extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Item {
    #[offset(0)]
    pub flags: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let item: Value<Item> = Rc::new(RefCell::new(Item {
        flags: unimplemented!(),
    }));
    let mut ptr: Ptr<Item> = (item.as_pointer());
    {
        field!((ptr), flags).with_mut(|__v| *__v = *__v | (1_() << unimplemented!()))
    };
    {
        field!((ptr), flags).with_mut(|__v| *__v = *__v | (1_() << 1_()))
    };
    unimplemented!();
    {
        field!((ptr), flags).with_mut(|__v| *__v = *__v & !(1_() << unimplemented!()))
    };
    unimplemented!();
    let mut bits: [(); 4] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    (bits)[((5_()) / 8_()) as usize] |= (1_() << ((5_()) & 7_()));
    (bits)[((13_()) / 8_()) as usize] |= (1_() << ((13_()) & 7_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    if (ptr.with(|__s| __s.flags) != unimplemented!()) {
        {
            field!((ptr), flags).with_mut(|__v| *__v = *__v & !(1_() << 1_()))
        };
    }
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
