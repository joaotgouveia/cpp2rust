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
fn main_0() -> i32 {
    let two: Value<(i32, f64)> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        let __tmp_1: Value<f64> = Rc::new(RefCell::new(2.5_f64));
        std_tuple_int__double_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    assert!(({ ((*(*two.borrow_mut()).get_mut::<0>().read()) == 1) }));
    assert!(({ ((*(*two.borrow_mut()).get_mut::<1>().read()) == 2.5_f64) }));
    *(*two.borrow_mut()).get_mut::<0>().write(7);
    assert!(({ ((*(*two.borrow_mut()).get_mut::<0>().read()) == 7) }));
    let made: Value<(i32, f64, i8)> = Rc::new(RefCell::new((1, 2.5_f64, ('c' as i8))));
    assert!(({ (((*(*made.borrow_mut()).get_mut::<2>().read()) as i32) == (('c' as i8) as i32)) }));
    assert!((*made.borrow()) == (1, 2.5_f64, ('c' as i8),));
    assert!(!((*made.borrow()) == (2, 2.5_f64, ('c' as i8),)));
    assert!(!((1, 2,) == (2, 1,)));
    let single: Value<(i64,)> = Rc::new(RefCell::new(<(i64,)>::tuple_from((3,))));
    assert!(({ ((*(*single.borrow_mut()).get_mut::<0>().read()) == 3_i64) }));
    let converted: Value<(i64, f64)> = Rc::new(RefCell::new(<(i64, f64)>::tuple_from({
        let __tmp_2: Value<i32> = Rc::new(RefCell::new(4));
        let __tmp_3: Value<f32> = Rc::new(RefCell::new(1.5_f32));
        std_tuple_int__float_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
    })));
    assert!(({ ((*(*converted.borrow_mut()).get_mut::<0>().read()) == 4_i64) }));
    assert!(({ ((*(*converted.borrow_mut()).get_mut::<1>().read()) == 1.5_f64) }));
    let empty: Value<()> = Rc::new(RefCell::new(()));
    assert!((*empty.borrow()) == ());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
