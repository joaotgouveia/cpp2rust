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
    let mut buf: Ptr<i32> =
        ({ malloc_0(((3_u64).wrapping_mul((::std::mem::size_of::<i32>() as u64)) as u64)) })
            .reinterpret_cast::<i32>();
    elem!(buf, 0).write(1);
    elem!(buf, 1).write(2);
    elem!(buf, 2).write(3);
    ({
        let ___a: Ptr<i32> = buf.offset((0) as isize);
        let ___b: Ptr<i32> = buf.offset((2) as isize);
        swap_1(___a, ___b)
    });
    ({
        cpp2rust_assert_fail_2(
            (((elem!(buf, 0).read()) == 3) && ((elem!(buf, 1).read()) == 2))
                && ((elem!(buf, 2).read()) == 1),
        )
    });
    ({
        let ___a: Ptr<i32> = buf.offset((1) as isize);
        let ___b: Ptr<i32> = buf.offset((1) as isize);
        swap_1(___a, ___b)
    });
    ({ cpp2rust_assert_fail_2(((elem!(buf, 1).read()) == 2)) });
    let local: Value<i32> = Rc::new(RefCell::new(4));
    ({
        let ___a: Ptr<i32> = buf.offset((1) as isize);
        let ___b: Ptr<i32> = local.as_pointer();
        swap_1(___a, ___b)
    });
    ({ cpp2rust_assert_fail_2(((elem!(buf, 1).read()) == 4) && ((*local.borrow()) == 2)) });
    let mut other: Ptr<i32> =
        ({ malloc_0((::std::mem::size_of::<i32>() as u64)) }).reinterpret_cast::<i32>();
    other.write(5);
    ({
        let ___a: Ptr<i32> = (buf).clone();
        let ___b: Ptr<i32> = (other).clone();
        swap_1(___a, ___b)
    });
    ({ cpp2rust_assert_fail_2(((elem!(buf, 0).read()) == 5) && ((other.read()) == 3)) });
    let mut bytes: Ptr<u8> = buf.reinterpret_cast::<u8>();
    elem!(bytes, 0).write(170_u8);
    elem!(bytes, 4).write(187_u8);
    ({
        let ___a: Ptr<u8> = bytes.offset((0) as isize);
        let ___b: Ptr<u8> = bytes.offset((4) as isize);
        swap_3(___a, ___b)
    });
    ({
        cpp2rust_assert_fail_2(
            (((elem!(bytes, 0).read()) as i32) == 187)
                && (((elem!(bytes, 4).read()) as i32) == 170),
        )
    });
    let words: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([16909060_u32, 84281096_u32])));
    let mut halves: Ptr<u16> = (words.as_pointer() as Ptr<u32>).reinterpret_cast::<u16>();
    ({
        let ___a: Ptr<u16> = halves.offset((0) as isize);
        let ___b: Ptr<u16> = halves.offset((3) as isize);
        swap_4(___a, ___b)
    });
    ({
        cpp2rust_assert_fail_2(
            ((*words.borrow())[(0) as usize] == 16909574_u32)
                && ((*words.borrow())[(1) as usize] == 50595592_u32),
        )
    });
    ({ free_5((other).to_any()) });
    ({ free_5((buf).to_any()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
