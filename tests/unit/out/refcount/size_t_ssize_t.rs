extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn add_sizes_0(mut a: (), mut b: ()) -> () {
    return (a).wrapping_add(b);
}
pub fn take_ulong_1(mut x: ()) -> () {
    return x;
}
pub fn sub_signed_2(mut a: (), mut b: ()) -> () {
    return (a - b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut n: () = (::std::mem::size_of::<()>() as ()).wrapping_add(4_());
    unimplemented!();
    let ul: Value<()> = Rc::new(RefCell::new(10_()));
    let sz: Value<()> = Rc::new(RefCell::new(20_()));
    let mut mixed: () = (*sz.borrow()).wrapping_add((*ul.borrow()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut chain: () = (((*sz.borrow()).wrapping_add((*ul.borrow()))).wrapping_add(5_()))
        .wrapping_add((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    let mut acc: () = 100_();
    acc = { (acc).wrapping_add((::std::mem::size_of::<()>() as ())) };
    acc = { (acc).wrapping_mul(2_()) };
    acc = { (acc).wrapping_sub((*ul.borrow())) };
    unimplemented!();
    (*sz.borrow_mut()) = { (*sz.borrow()).wrapping_add(1_()) };
    unimplemented!();
    let mut fr: () = ({
        add_sizes_0(
            (::std::mem::size_of::<()>() as ()).wrapping_add((*sz.borrow())),
            (*ul.borrow()),
        )
    });
    unimplemented!();
    let mut fr2: () = ({ take_ulong_1((*sz.borrow())) });
    unimplemented!();
    let mut lo: () = (({
        let ___b: Value<()> = Rc::new(RefCell::new(
            (::std::mem::size_of::<()>() as ()).wrapping_add((*ul.borrow())),
        ));
        min_3(sz.as_pointer(), ___b.as_pointer())
    })
    .read());
    let mut hi: () = (({
        let ___a: Value<()> = Rc::new(RefCell::new(
            (::std::mem::size_of::<()>() as ()).wrapping_add((*sz.borrow())),
        ));
        max_4(___a.as_pointer(), ul.as_pointer())
    })
    .read());
    unimplemented!();
    unimplemented!();
    let mut bound: () = (({
        let ___b: Value<()> = Rc::new(RefCell::new(4_()));
        min_3(sz.as_pointer(), ___b.as_pointer())
    })
    .read());
    unimplemented!();
    let mut data: [(); 8] = [unimplemented!(); 8];
    let mut count: () =
        (::std::mem::size_of::<[(); 8]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut i: () = 0_();
    'loop_: while (i < count) {
        data[(i) as usize] = { ((i).wrapping_mul(2_())) };
        i.postfix_inc();
    }
    let mut total: () = 0_();
    let mut i: () = 0_();
    'loop_: while (i < count) {
        total = { (total).wrapping_add(data[(i) as usize]) };
        i.postfix_inc();
    }
    unimplemented!();
    let mut cond: () = if ((*sz.borrow()) > (*ul.borrow())) {
        (*sz.borrow()).wrapping_add((::std::mem::size_of::<()>() as ()))
    } else {
        (*ul.borrow())
    };
    unimplemented!();
    let mut arr: [(); 4] = [0_(), 1_(), 2_(), 3_()];
    let mut idx: () = if (::std::mem::size_of::<()>() > 2_()) {
        2_()
    } else {
        0_()
    };
    unimplemented!();
    let mut s1: () = 5_();
    let mut s2: () = 12_();
    let sd: Value<()> = Rc::new(RefCell::new(({ sub_signed_2(s1, s2) })));
    unimplemented!();
    unimplemented!();
    let mut l: () = 3_();
    let sm: Value<()> = Rc::new(RefCell::new((s2 + l)));
    unimplemented!();
    unimplemented!();
    let mut smin: () = (({ min_5(sd.as_pointer(), sm.as_pointer()) }).read());
    let mut smax: () = (({ max_6(sd.as_pointer(), sm.as_pointer()) }).read());
    unimplemented!();
    unimplemented!();
    let mut delta: () = ((*sz.borrow()) - (*ul.borrow()));
    unimplemented!();
    let mut a64: () = 100_();
    let mut b: () = 30_();
    a64 -= b;
    unimplemented!();
    a64 += b;
    unimplemented!();
    let mut c: () = -20_();
    a64 -= c;
    unimplemented!();
    unimplemented!();
    let mx: Value<()> = Rc::new(RefCell::new(5_()));
    let mut mins: [(); 4] = [
        0_(),
        (({
            let ___a: Value<()> = Rc::new(RefCell::new(1_()));
            min_3(___a.as_pointer(), mx.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<()> = mx.as_pointer();
            let ___b: Value<()> = Rc::new(RefCell::new((*mx.borrow()).wrapping_sub(3_())));
            min_3(___a, ___b.as_pointer())
        })
        .read()),
        (*mx.borrow()),
    ];
    unimplemented!();
    unimplemented!();
    let pr: Value<std_pair_unsigned_long__int_> = Rc::new(RefCell::new({
        let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
        std_pair_unsigned_long__int_::new({ sz.as_pointer() }, { __tmp_0.as_pointer() })
    }));
    unimplemented!();
    let sizes: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
    let mut sizes_end: Ptr<()> = ({ end_7(sizes.as_pointer()) });
    unimplemented!();
    unimplemented!();
    let set: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!().write(42_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
