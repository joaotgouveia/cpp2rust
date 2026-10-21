extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct F {
    #[offset(0)]
    pub n: (),
    #[offset(4)]
    #[byte_size(0)]
    pub tail: Box<[()]>,
}
impl Default for F {
    fn default() -> Self {
        {
            F {
                n: unimplemented!(),
                tail: Box::<[()]>::default(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    let mut f: Ptr<F> = unimplemented!().reinterpret_cast::<F>();
    unimplemented!();
    field!(f, n).write(4_());
    ({
        memcpy_0(
            ((array_field_ptr!(f, tail) as Ptr<()>) as Ptr<()>).to_any(),
            Ptr::<()>::from_string_literal(b"xyz").to_any(),
            4_(),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut g: Ptr<F> = unimplemented!().reinterpret_cast::<F>();
    unimplemented!();
    field!(g, n).write(8_());
    ({
        memcpy_0(
            ((array_field_ptr!(g, tail) as Ptr<()>) as Ptr<()>).to_any(),
            Ptr::<()>::from_string_literal(b"abcdefg").to_any(),
            8_(),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
