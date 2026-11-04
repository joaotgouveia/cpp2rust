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
    let mut N: i32 = 3;
    let mut arr: Ptr<i32> =
        Ptr::alloc_array((0..(N as u64)).map(|_| 0_i32).collect::<Box<[i32]>>());
    ({
        let ___s: AnyPtr = (arr).to_any();
        memset_0(
            ___s,
            1,
            ((::std::mem::size_of::<i32>() as u64).wrapping_mul((N as u64)) as u64),
        )
    });
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        sum += { (elem!(arr, i).read()) };
        i.prefix_inc();
    }
    arr.delete();
    ({ cpp2rust_assert_fail_1((sum == 50529027)) });
    let bytes: Value<Box<[u8]>> =
        Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let mut fill: u8 = 7_u8;
    let mut n: i32 = 2;
    ({
        memset_0(
            (((bytes.as_pointer() as Ptr<u8>).offset(0)) as Ptr<u8>).to_any(),
            ((fill as i32) + 1),
            (n as u64),
        )
    });
    ({
        let ___s: AnyPtr = (((bytes.as_pointer() as Ptr<u8>).offset(n)) as Ptr<u8>).to_any();
        let ___n: u64 = ((4 - n) as u64);
        memset_0(___s, (fill as i32), ___n)
    });
    ({
        cpp2rust_assert_fail_1(
            (((((*bytes.borrow())[(0) as usize] as i32) == 8)
                && (((*bytes.borrow())[(1) as usize] as i32) == 8))
                && (((*bytes.borrow())[(2) as usize] as i32) == 7))
                && (((*bytes.borrow())[(3) as usize] as i32) == 7),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
