extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct label {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub probe: FnPtr<fn() -> ()>,
    #[offset(16)]
    pub mask: (),
}
pub fn probe_two_0() -> () {
    return 1_();
}
thread_local!(
    pub static table_1: Value<Box<[label]>> = Rc::new(RefCell::new(Box::new([
        label {
            name: Ptr::<()>::from_string_literal(b"first"),
            probe: FnPtr::<fn() -> ()>::null(),
            mask: (1_() << 4_()),
        },
        label {
            name: Ptr::<()>::from_string_literal(b"second"),
            probe: (FnPtr::<fn() -> ()>::new(probe_two_0)),
            mask: (1_() << 5_()),
        },
    ])));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut tail: Ptr<()> = (Ptr::<()>::from_string_literal(b"ab.cd").offset(2_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut have: () = 0_();
    let mut p: () = if (have != 0) {
        ({
            (*table_1.with(Value::clone).borrow())[(0_()) as usize]
                .name
                .clone()
        })
        .to_any()
    } else {
        Ptr::<()>::from_string_literal(b"").to_any()
    };
    unimplemented!();
    have = 1_();
    p = if (have != 0) {
        ({
            (*table_1.with(Value::clone).borrow())[(0_()) as usize]
                .name
                .clone()
        })
        .to_any()
    } else {
        Ptr::<()>::from_string_literal(b"").to_any()
    };
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = table_1.with(|_| ());
}
