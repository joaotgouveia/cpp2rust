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
    let two: Value<<(i32, f64) as ValueTuple>::Values> = Rc::new(RefCell::new(
        <(i32, f64)>::into_values(<(i32, f64)>::tuple_from((1, 2.5_f64))),
    ));
    assert!(({ (((*two.borrow()).get_value::<0>().as_pointer().read()) == 1) }));
    assert!(({ (((*two.borrow()).get_value::<1>().as_pointer().read()) == 2.5_f64) }));
    (*two.borrow()).get_value::<0>().as_pointer().write(7);
    assert!(({ (((*two.borrow()).get_value::<0>().as_pointer().read()) == 7) }));
    let made: Value<<(i32, f64, i8) as ValueTuple>::Values> = Rc::new(RefCell::new(
        <(i32, f64, i8)>::into_values((1, 2.5_f64, ('c' as i8))),
    ));
    assert!(
        ({
            ((((*made.borrow()).get_value::<2>().as_pointer().read()) as i32)
                == (('c' as i8) as i32))
        })
    );
    assert!(
        <(i32, f64, i8,)>::from_values(&(*made.borrow()))
            == <(i32, f64, i8,)>::from_values(&<(i32, f64, i8,)>::into_values((
                1,
                2.5_f64,
                ('c' as i8),
            )))
    );
    assert!(
        !(<(i32, f64, i8,)>::from_values(&(*made.borrow()))
            == <(i32, f64, i8,)>::from_values(&<(i32, f64, i8,)>::into_values((
                2,
                2.5_f64,
                ('c' as i8),
            ))))
    );
    assert!(
        !(<(i32, i32,)>::from_values(&<(i32, i32,)>::into_values((1, 2,)))
            == <(i32, i32,)>::from_values(&<(i32, i32,)>::into_values((2, 1,))))
    );
    let single: Value<<(i64,) as ValueTuple>::Values> = Rc::new(RefCell::new(
        <(i64,)>::into_values(<(i64,)>::tuple_from((3,))),
    ));
    assert!(({ (((*single.borrow()).get_value::<0>().as_pointer().read()) == 3_i64) }));
    let converted: Value<<(i64, f64) as ValueTuple>::Values> = Rc::new(RefCell::new(
        <(i64, f64)>::into_values(<(i64, f64)>::tuple_from(<(i32, f32)>::from_values(
            &<(i32, f32)>::into_values(<(i32, f32)>::tuple_from((4, 1.5_f32))),
        ))),
    ));
    assert!(({ (((*converted.borrow()).get_value::<0>().as_pointer().read()) == 4_i64) }));
    assert!(({ (((*converted.borrow()).get_value::<1>().as_pointer().read()) == 1.5_f64) }));
    let empty: Value<<() as ValueTuple>::Values> = Rc::new(RefCell::new(<()>::default_values()));
    assert!(<()>::from_values(&(*empty.borrow())) == <()>::from_values(&<()>::into_values(())));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
