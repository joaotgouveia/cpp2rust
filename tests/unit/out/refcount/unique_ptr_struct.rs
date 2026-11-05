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
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl CppMove for Point {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn sum_0(mut p: Point) -> i32 {
    return (p.x + p.y);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<std_unique_ptr_Point__std_default_delete_Point__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<Point> = Rc::new(RefCell::new(Point { x: 3, y: 4 }));
            make_unique_1(___args.as_pointer())
        }),
    ));
    {
        field!((*p.borrow_mut()), x).with_mut(|__v| *__v = *__v + 10)
    };
    let __rhs = ({ ((*p.borrow()).read()).x } + { ((*p.borrow()).read()).y });
    field!((*p.borrow_mut()), y).write(__rhs);
    let mut s: i32 = ({ sum_0((*(*p.borrow()).upgrade().deref()).clone()) });
    ({ cpp2rust_assert_fail_2((s == 30)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
