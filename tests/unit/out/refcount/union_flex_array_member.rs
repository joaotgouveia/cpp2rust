extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn bytes(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn aligner(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct node {
    #[offset(0)]
    pub len: (),
    #[offset(8)]
    pub pos: (),
    #[offset(16)]
    #[byte_size(8)]
    pub x: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut tail_size: () = 32_();
    let mut n: Ptr<node> = unimplemented!().reinterpret_cast::<node>();
    field!(n, len).write(tail_size);
    let mut i: () = unimplemented!();
    'loop_: while (((i < tail_size) as ()) != 0) {
        elem!(
            (anon_0::bytes(field_ptr!(n, x)).reinterpret_cast::<()>() as Ptr::<()>),
            i
        )
        .write({ ((i & 255_()) as ()) });
        i.postfix_inc();
    }
    let mut i: () = unimplemented!();
    'loop_: while (((i < tail_size) as ()) != 0) {
        unimplemented!();
        i.postfix_inc();
    }
    let mut p: Ptr<()> = ((anon_0::bytes(field_ptr!(n, x)).reinterpret_cast::<()>() as Ptr<()>)
        .offset((10_()) as isize));
    unimplemented!();
    p.write(170_());
    unimplemented!();
    field!(n, pos).write(20_());
    let mut q: Ptr<()> = ((anon_0::bytes(field_ptr!(n, x)).reinterpret_cast::<()>() as Ptr<()>)
        .offset((n.with(|__s| __s.pos)) as isize));
    unimplemented!();
    q.write(187_());
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
