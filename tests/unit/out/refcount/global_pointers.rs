extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Entry {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub p: Ptr<()>,
}
thread_local!(
    pub static single_entry_0: Value<Entry> = Rc::new(RefCell::new(Entry {
        name: Ptr::<()>::from_string_literal(b"alone"),
        p: Ptr::<()>::null(),
    }));
);
thread_local!(
    pub static entries_1: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            name: Ptr::<()>::from_string_literal(b"first"),
            p: Ptr::<()>::null(),
        },
        Entry {
            name: Ptr::<()>::from_string_literal(b"second"),
            p: Ptr::<()>::null(),
        },
    ])));
);
thread_local!(
    pub static arr_of_pointers_2: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (i < 2_()) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {
    let _ = single_entry_0.with(|_| ());
    let _ = entries_1.with(|_| ());
    let _ = arr_of_pointers_2.with(|_| ());
}
