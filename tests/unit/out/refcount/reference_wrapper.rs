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
pub fn set_0(ref_: std_reference_wrapper_int_, mut val: i32) {
    let ref_: Value<std_reference_wrapper_int_> = Rc::new(RefCell::new(ref_));
    ({ (*ref_.borrow()).get() }).write(val);
}
pub fn read_1(ref_: std_reference_wrapper_int_) -> i32 {
    let ref_: Value<std_reference_wrapper_int_> = Rc::new(RefCell::new(ref_));
    let r: Ptr<i32> = ({ (*ref_.borrow())() });
    return (r.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i1: Value<i32> = Rc::new(RefCell::new(10));
    let ref_1: Value<std_reference_wrapper_int_> =
        Rc::new(RefCell::new(std_reference_wrapper_int_::new({
            i1.as_pointer()
        })));
    ({ (*ref_1.borrow()).get() }).write(20);
    let i2: Ptr<i32> = ({ (*ref_1.borrow())() });
    {
        i2.with_mut(|__v| *__v = *__v + 5)
    };
    write!(cout_2.with(|v| v.as_pointer()), "{:}\n", (*i1.borrow()),);
    let i3: Value<i32> = Rc::new(RefCell::new(1));
    let i4: Value<i32> = Rc::new(RefCell::new(2));
    let ref_3: Value<std_reference_wrapper_int_> =
        Rc::new(RefCell::new(std_reference_wrapper_int_::new({
            i3.as_pointer()
        })));
    let ref_4: Value<std_reference_wrapper_int_> =
        Rc::new(RefCell::new(std_reference_wrapper_int_::new({
            i4.as_pointer()
        })));
    let __rhs = (({ (*ref_4.borrow()).get() }).read());
    ({ (*ref_3.borrow()).get() }).write(__rhs);
    write!(
        cout_2.with(|v| v.as_pointer()),
        "{:} {:}\n",
        (*i3.borrow()),
        (*i4.borrow()),
    );
    ({ set_0((*ref_1.borrow()).clone(), 99) });
    write!(
        cout_2.with(|v| v.as_pointer()),
        "{:} {:}\n",
        (*i1.borrow()),
        ({ read_1((*ref_1.borrow()).clone(),) }),
    );
    let point: Value<Point> = Rc::new(RefCell::new(Point { x: 3, y: 4 }));
    let point_ref: Value<std_reference_wrapper_Point_> =
        Rc::new(RefCell::new(std_reference_wrapper_Point_::new({
            point.as_pointer()
        })));
    field!(({ (*point_ref.borrow()).get() }), x).write(30);
    field!(({ (*point_ref.borrow())() }), y).write(40);
    write!(
        cout_2.with(|v| v.as_pointer()),
        "{:} {:}\n",
        { (*point.borrow()).x },
        { (*point.borrow()).y },
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
