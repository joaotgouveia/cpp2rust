extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_std_pair_0(mut x: ()) -> () {
    return unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let __decomp_1: Value<()> = Rc::new(RefCell::new(({ make_std_pair_0(5_()) })));
    let value: Ptr<()> = ({ get_2(__decomp_1.as_pointer()) });
    let positive: Ptr<()> = ({ get_3(__decomp_1.as_pointer()) });
    unimplemented!();
    unimplemented!();
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let __decomp_4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let a: Ptr<()> = ({ get_5(__decomp_4.as_pointer()) });
    let b: Ptr<()> = ({ get_6(__decomp_4.as_pointer()) });
    a.write(10_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_7: Ptr<()> = p.as_pointer();
    let x: Ptr<()> = ({ get_8((__decomp_7).clone()) });
    let y: Ptr<()> = ({ get_9((__decomp_7).clone()) });
    x.write(3_());
    {
        y.with_mut(|__v| *__v = *__v + 4_())
    };
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
