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
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(1)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(10);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(2)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(20);
    {
        let __it = ({
            let mut __it = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin());
            __it.iter_inc()
        })
        .clone();
        Map::erase(m.as_pointer(), &__it)
    };
    assert!(((*m.borrow()).len() == 1_usize));
    assert!(
        ((*(*m
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
            .iter_deref()
            .upgrade()
            .deref())
        .0
        .borrow())
            == 1)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
