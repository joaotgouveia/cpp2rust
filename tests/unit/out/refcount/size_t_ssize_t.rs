extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn add_sizes_0(mut a: u64, mut b: u64) -> u64 {
    return (a).wrapping_add(b);
}
pub fn take_ulong_1(x: u64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn sub_signed_2(mut a: i64, mut b: i64) -> i64 {
    return (a - b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut n: u64 = ((::std::mem::size_of::<i32>() as u64).wrapping_add(4_u64) as u64);
    ({ cpp2rust_assert_fail_3((n == (::std::mem::size_of::<i32>() as u64).wrapping_add(4_u64))) });
    let ul: Value<u64> = Rc::new(RefCell::new(10_u64));
    let sz: Value<u64> = Rc::new(RefCell::new(20_u64));
    let mut mixed: u64 = (*sz.borrow()).wrapping_add((*ul.borrow()));
    ({ cpp2rust_assert_fail_3((mixed == 30_u64)) });
    ({ cpp2rust_assert_fail_3(((*sz.borrow()) > (*ul.borrow()))) });
    ({ cpp2rust_assert_fail_3(((*ul.borrow()) < (*sz.borrow()))) });
    ({ cpp2rust_assert_fail_3(!((*sz.borrow()) == (*ul.borrow()))) });
    let mut chain: u64 = (((*sz.borrow()).wrapping_add((*ul.borrow()))).wrapping_add(5_u64))
        .wrapping_add((::std::mem::size_of::<i64>() as u64));
    ({
        cpp2rust_assert_fail_3(
            (chain == (((20 + 10) + 5) as u64).wrapping_add((::std::mem::size_of::<i64>() as u64))),
        )
    });
    let mut acc: u64 = 100_u64;
    acc = { (acc).wrapping_add((::std::mem::size_of::<f64>() as u64)) };
    acc = { (acc).wrapping_mul(2_u64) };
    acc = { (acc).wrapping_sub((*ul.borrow())) };
    ({
        cpp2rust_assert_fail_3(
            (acc == ((((100_u64).wrapping_add((::std::mem::size_of::<f64>() as u64))) as u64)
                .wrapping_mul(2_u64) as u64)
                .wrapping_sub(10_u64)),
        )
    });
    (*sz.borrow_mut()) = { (*sz.borrow()).wrapping_add(1_u64) };
    ({ cpp2rust_assert_fail_3(((*sz.borrow()) == 21_u64)) });
    let mut fr: u64 = ({
        add_sizes_0(
            (::std::mem::size_of::<i32>() as u64).wrapping_add((*sz.borrow())),
            (*ul.borrow()),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            (fr == ((::std::mem::size_of::<i32>() as u64).wrapping_add(21_u64) as u64)
                .wrapping_add(10_u64)),
        )
    });
    let mut fr2: u64 = ({ take_ulong_1((*sz.borrow())) });
    ({ cpp2rust_assert_fail_3((fr2 == 21_u64)) });
    let mut lo: u64 = (({
        let ___b: Value<u64> = Rc::new(RefCell::new(
            (::std::mem::size_of::<i64>() as u64).wrapping_add((*ul.borrow())),
        ));
        min_4(sz.as_pointer(), ___b.as_pointer())
    })
    .read());
    let mut hi: u64 = (({
        let ___a: Value<u64> = Rc::new(RefCell::new(
            (::std::mem::size_of::<i32>() as u64).wrapping_add((*sz.borrow())),
        ));
        max_5(___a.as_pointer(), ul.as_pointer())
    })
    .read());
    ({
        cpp2rust_assert_fail_3((lo == (::std::mem::size_of::<i64>() as u64).wrapping_add(10_u64)))
    });
    ({
        cpp2rust_assert_fail_3((hi == (::std::mem::size_of::<i32>() as u64).wrapping_add(21_u64)))
    });
    let mut bound: u64 = (({
        let ___b: Value<u64> = Rc::new(RefCell::new(4_u64));
        min_4(sz.as_pointer(), ___b.as_pointer())
    })
    .read());
    ({ cpp2rust_assert_fail_3((bound == 4_u64)) });
    let mut data: [i32; 8] = [0_i32; 8];
    let mut count: u64 = ((::std::mem::size_of::<[i32; 8]>() as u64)
        .wrapping_div((::std::mem::size_of::<i32>() as u64)) as u64);
    let mut i: u64 = 0_u64;
    'loop_: while (i < count) {
        data[(i) as usize] = { (((i).wrapping_mul(2_u64)) as i32) };
        i.postfix_inc();
    }
    let mut total: u64 = 0_u64;
    let mut i: u64 = 0_u64;
    'loop_: while (i < count) {
        total = { (total).wrapping_add((data[(i) as usize] as u64)) };
        i.postfix_inc();
    }
    ({ cpp2rust_assert_fail_3((total == 56_u64)) });
    let mut cond: u64 = if ((*sz.borrow()) > (*ul.borrow())) {
        (*sz.borrow()).wrapping_add((::std::mem::size_of::<i32>() as u64))
    } else {
        (*ul.borrow())
    };
    ({
        cpp2rust_assert_fail_3(
            (cond == (21_u64).wrapping_add((::std::mem::size_of::<i32>() as u64))),
        )
    });
    let mut arr: [u64; 4] = [0_u64, 1_u64, 2_u64, 3_u64];
    let mut idx: u64 = (if (::std::mem::size_of::<i32>() > 2_u64) {
        2
    } else {
        0
    } as u64);
    ({ cpp2rust_assert_fail_3((arr[(idx) as usize] == 2_u64)) });
    let mut s1: i64 = 5_i64;
    let mut s2: i64 = 12_i64;
    let sd: Value<i64> = Rc::new(RefCell::new(({ sub_signed_2(s1, s2) })));
    ({ cpp2rust_assert_fail_3(((*sd.borrow()) == (-7_i32 as i64))) });
    ({ cpp2rust_assert_fail_3(((*sd.borrow()) < 0_i64)) });
    let mut l: i64 = 3_i64;
    let sm: Value<i64> = Rc::new(RefCell::new((s2 + l)));
    ({ cpp2rust_assert_fail_3(((*sm.borrow()) == 15_i64)) });
    ({ cpp2rust_assert_fail_3(((*sm.borrow()) > l)) });
    let mut smin: i64 = (({ min_6(sd.as_pointer(), sm.as_pointer()) }).read());
    let mut smax: i64 = (({ max_7(sd.as_pointer(), sm.as_pointer()) }).read());
    ({ cpp2rust_assert_fail_3((smin == (-7_i32 as i64))) });
    ({ cpp2rust_assert_fail_3((smax == 15_i64)) });
    let mut delta: i64 = (((*sz.borrow()) as i64) - ((*ul.borrow()) as i64));
    ({ cpp2rust_assert_fail_3((delta == 11_i64)) });
    let mut a64: i64 = 100_i64;
    let mut b: i64 = 30_i64;
    a64 -= b;
    ({ cpp2rust_assert_fail_3((a64 == 70_i64)) });
    a64 += b;
    ({ cpp2rust_assert_fail_3((a64 == 100_i64)) });
    let mut c: i64 = (-20_i32 as i64);
    a64 -= c;
    ({ cpp2rust_assert_fail_3((a64 == 120_i64)) });
    ({ cpp2rust_assert_fail_3(((((n).wrapping_rem(7_u64)) as i32) == 1)) });
    let mx: Value<u64> = Rc::new(RefCell::new(5_u64));
    let mut mins: [u64; 4] = [
        0_u64,
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            min_4(___a.as_pointer(), mx.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = mx.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*mx.borrow()).wrapping_sub(3_u64)));
            min_4(___a, ___b.as_pointer())
        })
        .read()),
        (*mx.borrow()),
    ];
    ({ cpp2rust_assert_fail_3((mins[(1) as usize] == 1_u64)) });
    ({ cpp2rust_assert_fail_3((mins[(2) as usize] == 2_u64)) });
    let pr: Value<std_pair_unsigned_long__int_> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        std_pair_unsigned_long__int_::new_1({ sz.as_pointer() }, { __tmp_0.as_pointer() })
    }));
    ({ cpp2rust_assert_fail_3(({ (*pr.borrow()).first } == 21_u64)) });
    let sizes: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([1_u64, 2_u64, 3_u64, 4_u64])));
    let mut sizes_end: Ptr<u64> = ({ end_8(sizes.as_pointer()) });
    ({
        cpp2rust_assert_fail_3(
            (((sizes_end).clone() - (sizes.as_pointer() as Ptr<u64>)) as i64 == 4_i64),
        )
    });
    ({ cpp2rust_assert_fail_3((((sizes_end.offset(-((1) as isize))).read()) == 4_u64)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
