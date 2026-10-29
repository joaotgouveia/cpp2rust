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
pub fn set_0(ref_: std_reference_wrapper_int_, mut val: ()) {
    let ref_: Value<std_reference_wrapper_int_> = Rc::new(RefCell::new(ref_));
    ({ (*ref_.borrow()).get() }).write(val);
}
pub fn read_1(ref_: std_reference_wrapper_int_) -> () {
    let ref_: Value<std_reference_wrapper_int_> = Rc::new(RefCell::new(ref_));
    let r: Ptr<()> = ({ (*ref_.borrow())() });
    return (r.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let i1: Value<()> = Rc::new(RefCell::new(10_()));
    let ref_1: Value<std_reference_wrapper_int_> =
        Rc::new(RefCell::new(std_reference_wrapper_int_::new({
            i1.as_pointer()
        })));
    ({ (*ref_1.borrow()).get() }).write(20_());
    let i2: Ptr<()> = ({ (*ref_1.borrow())() });
    {
        i2.with_mut(|__v| *__v = *__v + 5_())
    };
    write!(cout_2.with(|v| v.as_pointer()), "{:}\n", (*i1.borrow()),);
    let i3: Value<()> = Rc::new(RefCell::new(1_()));
    let i4: Value<()> = Rc::new(RefCell::new(2_()));
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
    ({ set_0((*ref_1.borrow()).clone(), 99_()) });
    write!(
        cout_2.with(|v| v.as_pointer()),
        "{:} {:}\n",
        (*i1.borrow()),
        ({ read_1((*ref_1.borrow()).clone(),) }),
    );
    let point: Value<Point> = Rc::new(RefCell::new(Point { x: 3_(), y: 4_() }));
    let point_ref: Value<std_reference_wrapper_Point_> =
        Rc::new(RefCell::new(std_reference_wrapper_Point_::new({
            point.as_pointer()
        })));
    field!(({ (*point_ref.borrow()).get() }), x).write(30_());
    field!(({ (*point_ref.borrow())() }), y).write(40_());
    write!(
        cout_2.with(|v| v.as_pointer()),
        "{:} {:}\n",
        { (*point.borrow()).x },
        { (*point.borrow()).y },
    );
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
