extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_std_pair_0(x: ()) -> std_pair_int__bool_ {
    let x: Value<()> = Rc::new(RefCell::new(x));
    return {
        let __tmp_0: Value<()> = Rc::new(RefCell::new(((*x.borrow()) > 0_())));
        std_pair_int__bool_::new({ x.as_pointer() }, { __tmp_0.as_pointer() })
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let __decomp_1: Value<std_pair_int__bool_> = Rc::new(RefCell::new(({ make_std_pair_0(5_()) })));
    let value: Ptr<()> = ({ get_2(__decomp_1.as_pointer()) });
    let positive: Ptr<()> = ({ get_3(__decomp_1.as_pointer()) });
    unimplemented!();
    unimplemented!();
    let p: Value<std_pair_int__int_> = Rc::new(RefCell::new({
        let __tmp_1: Value<()> = Rc::new(RefCell::new(1_()));
        let __tmp_2: Value<()> = Rc::new(RefCell::new(2_()));
        std_pair_int__int_::new({ __tmp_1.as_pointer() }, { __tmp_2.as_pointer() })
    }));
    let __decomp_4: Value<std_pair_int__int_> = Rc::new(RefCell::new((*p.borrow()).clone()));
    let a: Ptr<()> = ({ get_5(__decomp_4.as_pointer()) });
    let b: Ptr<()> = ({ get_6(__decomp_4.as_pointer()) });
    a.write(10_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_7: Ptr<std_pair_int__int_> = p.as_pointer();
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
