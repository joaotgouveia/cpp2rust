extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(mut x: ()) -> () {
    return x;
}
pub fn swap_by_ptr_1(mut a: Ptr<()>, mut b: Ptr<()>) {
    let mut tmp: () = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn swap_by_ref_2(a: Ptr<()>, b: Ptr<()>) {
    let mut tmp: () = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut x: () = 0_();
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", x,);
    let mut a: () = 1_();
    let mut b: () = ({ identity_0(a) });
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", b,);
    let c: Value<()> = Rc::new(RefCell::new(2_()));
    let mut p: Ptr<()> = (c.as_pointer());
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", (p.read()),);
    let d: Value<()> = Rc::new(RefCell::new(3_()));
    let e: Value<()> = Rc::new(RefCell::new(4_()));
    ({ swap_by_ptr_1((d.as_pointer()), (e.as_pointer())) });
    let f: Value<()> = Rc::new(RefCell::new(4_()));
    let g: Value<()> = Rc::new(RefCell::new(5_()));
    ({ swap_by_ref_2(f.as_pointer(), g.as_pointer()) });
    let mut h: Ptr<()> = Ptr::alloc(6_());
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", (h.read()),);
    h.delete();
    let mut i: Ptr<()> = Ptr::alloc_array(Box::new([7_(), 8_(), unimplemented!()]));
    write!(
        cout_3.with(|v| v.as_pointer()),
        "{:} {:}\n",
        (elem!(i, 0_()).read()),
        (elem!(i, 1_()).read()),
    );
    i.delete();
    ({ swap_by_ptr_1(Ptr::alloc(7_()), Ptr::alloc(8_())) });
    ({
        swap_by_ptr_1(
            Ptr::alloc(7_()).offset((0_()) as isize),
            Ptr::alloc(8_()).offset((0_()) as isize),
        )
    });
    ({ swap_by_ref_2(Ptr::alloc(9_()), Ptr::alloc(10_())) });
    ({
        swap_by_ref_2(
            (Ptr::alloc(9_())).offset((0_()) as isize),
            (Ptr::alloc(10_())).offset((0_()) as isize),
        )
    });
    let j: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        std_unique_ptr_int__std_default_delete_int__::new_1({ Ptr::alloc(11_()) }),
    ));
    let mut k: Ptr<()> = ({ (*j.borrow()).get() });
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", (k.read()),);
    let l: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<()> = Rc::new(RefCell::new(11_()));
            make_unique_4(___args.as_pointer())
        }),
    ));
    let mut m: Ptr<()> = ({ (*l.borrow()).get() });
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", (m.read()),);
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
