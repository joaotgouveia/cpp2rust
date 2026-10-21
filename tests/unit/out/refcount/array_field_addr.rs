extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct S {
    #[offset(0)]
    pub before: (),
    #[offset(4)]
    #[byte_size(4)]
    pub mask: Box<[()]>,
    #[offset(8)]
    pub after: (),
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                before: unimplemented!(),
                mask: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                after: unimplemented!(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut s: Ptr<S> = unimplemented!().reinterpret_cast::<S>();
    unimplemented!();
    field!(s, before).write(1_());
    ({
        let ___s: () = ((array_field_ptr!(s, mask) as Ptr<()>) as Ptr<()>).to_any();
        let ___n: () = ::std::mem::size_of::<[(); 4]>();
        memset_0(___s, 5_(), ___n)
    });
    field!(s, after).write(2_());
    ((array_field_ptr!(s, mask)) as Ptr<()>).write(7_());
    let out: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    ({
        let ___src: () = ((array_field_ptr!(s, mask)) as Ptr<()>).to_any();
        let ___n: () = ::std::mem::size_of::<[(); 4]>();
        memcpy_1(
            ((out.as_pointer() as Ptr<()>) as Ptr<()>).to_any(),
            ___src,
            ___n,
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
