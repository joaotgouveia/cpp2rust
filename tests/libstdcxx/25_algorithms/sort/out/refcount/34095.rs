extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let v: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(
        (0..(20000_usize) as usize)
            .map(|_| Default::default())
            .collect::<Vec<_>>(),
    ));
    {
        let __first = v.decay().begin();
        let __n = v.decay().end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__first.iter_at(__x), &__first.iter_at(__y)));
        reorder(__order, |__i| __first.iter_at(__i));
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
