extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
pub fn sum_0(mut p: Point) -> () {
    return (p.x + p.y);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let p: Value<std_unique_ptr_Point__std_default_delete_Point__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<Point> = Rc::new(RefCell::new(Point { x: 3_(), y: 4_() }));
            make_unique_1(___args.as_pointer())
        }),
    ));
    {
        field!((*p.borrow_mut()), x).with_mut(|__v| *__v = *__v + 10_())
    };
    let __rhs = ({ ((*p.borrow()).read()).x } + { ((*p.borrow()).read()).y });
    field!((*p.borrow_mut()), y).write(__rhs);
    let mut s: () = ({ sum_0((*(*p.borrow()).upgrade().deref()).clone()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
