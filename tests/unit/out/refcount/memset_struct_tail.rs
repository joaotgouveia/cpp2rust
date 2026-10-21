extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    pub keep: (),
    #[offset(4)]
    pub a: (),
    #[offset(8)]
    pub b: (),
    #[offset(16)]
    #[byte_size(5)]
    pub c: Box<[()]>,
    #[offset(24)]
    pub last: (),
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                keep: unimplemented!(),
                a: unimplemented!(),
                b: unimplemented!(),
                c: (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                last: unimplemented!(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut p: Ptr<S> = unimplemented!().reinterpret_cast::<S>();
    unimplemented!();
    ({
        let ___s: () = (p).to_any();
        let ___n: () = 32usize;
        memset_0(___s, 255_(), ___n)
    });
    field!(p, keep).write(7_());
    ({
        memset_0(
            ((field_ptr!(p, a)) as Ptr<()>).to_any(),
            0_(),
            (32usize as ()).wrapping_sub((4_usize as ())),
        )
    });
    unimplemented!();
    unimplemented!();
    field!(p, a).write(1_());
    field!(p, b).write(2_());
    array_elem!(p, c, 0_()).write(('x' as ()));
    field!(p, last).write(3_());
    ({
        memset_0(
            ((field_ptr!(p, b)) as Ptr<()>).to_any(),
            0_(),
            (24_usize as ()).wrapping_sub((8_usize as ())),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
