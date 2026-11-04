extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn swap_by_ptr_1(mut a: Ptr<i32>, mut b: Ptr<i32>) {
    let mut tmp: i32 = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn swap_by_ref_2(a: Ptr<i32>, b: Ptr<i32>) {
    let mut tmp: i32 = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: i32 = 0;
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", x,);
    let mut a: i32 = 1;
    let mut b: i32 = ({ identity_0(a) });
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", b,);
    let c: Value<i32> = Rc::new(RefCell::new(2));
    let mut p: Ptr<i32> = (c.as_pointer());
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", (p.read()),);
    let d: Value<i32> = Rc::new(RefCell::new(3));
    let e: Value<i32> = Rc::new(RefCell::new(4));
    ({ swap_by_ptr_1((d.as_pointer()), (e.as_pointer())) });
    let f: Value<i32> = Rc::new(RefCell::new(4));
    let g: Value<i32> = Rc::new(RefCell::new(5));
    ({ swap_by_ref_2(f.as_pointer(), g.as_pointer()) });
    let mut h: Ptr<i32> = Ptr::alloc(6);
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", (h.read()),);
    h.delete();
    let mut i: Ptr<i32> = Ptr::alloc_array(Box::new([7, 8, 0_i32]));
    write!(
        cout_3.with(|v| v.as_pointer()),
        "{:} {:}\n",
        (elem!(i, 0).read()),
        (elem!(i, 1).read()),
    );
    i.delete();
    ({ swap_by_ptr_1(Ptr::alloc(7), Ptr::alloc(8)) });
    ({
        swap_by_ptr_1(
            Ptr::alloc(7).offset((0) as isize),
            Ptr::alloc(8).offset((0) as isize),
        )
    });
    ({ swap_by_ref_2(Ptr::alloc(9), Ptr::alloc(10)) });
    ({
        swap_by_ref_2(
            (Ptr::alloc(9)).offset((0) as isize),
            (Ptr::alloc(10)).offset((0) as isize),
        )
    });
    let j: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        std_unique_ptr_int__std_default_delete_int__::new_1({ Ptr::alloc(11) }),
    ));
    let mut k: Ptr<i32> = ({ (*j.borrow()).get() });
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", (k.read()),);
    let l: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(11));
            make_unique_4(___args.as_pointer())
        }),
    ));
    let mut m: Ptr<i32> = ({ (*l.borrow()).get() });
    write!(cout_3.with(|v| v.as_pointer()), "{:}\n", (m.read()),);
    ({ cpp2rust_assert_fail_5(((*c.borrow()) == 2)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
