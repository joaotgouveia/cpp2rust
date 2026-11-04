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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 4;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 7;
        (*v.borrow_mut()).push(__a1)
    };
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        ({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(7));
            match ({ CppRange::begin(&v.decay()) }, {
                CppRange::end(&v.decay())
            }) {
                (mut __tmp_0, mut __tmp_1) => {
                    let mut __it = (__tmp_0).clone();
                    while __it != __tmp_1
                        && !CppEq::equal(&__it.iter_deref(), &__tmp_2.as_pointer())
                    {
                        __it.iter_inc();
                    }
                    __it
                }
            }
        }),
    ));
    assert!((*it.borrow()) != CppRange::end(&v.decay()));
    assert!(({ (((*it.borrow()).iter_deref().read()) == 7) }));
    match ({ CppRange::begin(&v.decay()) },) {
        (mut __tmp_3,) => {
            let __n = CppRange::end(&v.decay()).iter_distance(&__tmp_3);
            let __random = std::hash::RandomState::new();
            for __i in (1..__n).rev() {
                let __j =
                    (std::hash::BuildHasher::hash_one(&__random, __i) % (__i + 1) as u64) as isize;
                cpp_swap!(__tmp_3.iter_at(__i), __tmp_3.iter_at(__j));
            }
        }
    };
    match ({ CppRange::begin(&v.decay()) },) {
        (mut __tmp_4,) => {
            let __n = CppRange::end(&v.decay()).iter_distance(&__tmp_4);
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_4.iter_at(__x), &__tmp_4.iter_at(__y)));
            reorder(__order, |__i| __tmp_4.iter_at(__i));
        }
    };
    assert!(
        ((v.decay().offset(0_usize as isize).read()) == 4)
            && ((v.decay().offset(1_usize as isize).read()) == 7)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
