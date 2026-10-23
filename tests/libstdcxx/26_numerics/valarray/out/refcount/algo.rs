extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut m: () = ({
        ( { let ___e : Value<std__Expr_std___detail__BinClos_std___minus__std__ValArray__std__ValArray__double__double___double_ > = Rc::new(RefCell::new(unimplemented!()  )); abs_0 ( ___e.as_pointer() , ) } )  . max ( )
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
