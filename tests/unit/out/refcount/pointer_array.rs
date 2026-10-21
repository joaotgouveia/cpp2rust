extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct StackArray {
    #[offset(0)]
    #[byte_size(24)]
    pub arr: Box<[Ptr<()>]>,
}
impl Default for StackArray {
    fn default() -> Self {
        {
            StackArray {
                arr: (0..3)
                    .map(|_| Ptr::<()>::null())
                    .collect::<Box<[Ptr<()>]>>(),
            }
        }
    }
}
pub fn IncrementAll_0(s: Ptr<StackArray>) {
    let mut i: () = unimplemented!();
    'loop_: while (i < 3_()) {
        {
            (array_elem!(s, arr, i).read()).with_mut(|__v| *__v = *__v + 1_())
        };
        i.prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let s: Value<StackArray> = Rc::new(RefCell::new(StackArray {
        arr: Box::new([(x.as_pointer()), (x.as_pointer()), (x.as_pointer())]),
    }));
    ({ IncrementAll_0(s.as_pointer()) });
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
