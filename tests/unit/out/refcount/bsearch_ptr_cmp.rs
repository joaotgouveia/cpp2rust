extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn int_cmp_0(mut v1: AnyPtr, mut v2: AnyPtr) -> i32 {
    return ({ (v1.reinterpret_cast::<i32>().read()) } - { (v2.reinterpret_cast::<i32>().read()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let mut vptr1: AnyPtr = ({
        let ___key: AnyPtr = (((a1.as_pointer() as Ptr<i32>).offset(0)) as Ptr<i32>).to_any();
        let ___base: AnyPtr = ((a1.as_pointer() as Ptr<i32>) as Ptr<i32>).to_any();
        bsearch_1(
            ___key,
            ___base,
            1_u64,
            (::std::mem::size_of::<i32>() as u64),
            FnPtr::<fn(AnyPtr, AnyPtr) -> i32>::new(int_cmp_0),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            ({ (vptr1).clone() } == {
                (((a1.as_pointer() as Ptr<i32>).offset(0)) as Ptr<i32>).to_any()
            }),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
