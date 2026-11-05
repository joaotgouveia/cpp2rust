extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_int_0(mut p: AnyPtr) -> i32 {
    return (p.reinterpret_cast::<i32>().read());
}
pub fn first_char_1(mut p: AnyPtr) -> i8 {
    return (p.reinterpret_cast::<i8>().read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let rows: Value<Box<[Box<[i32]>]>> = Rc::new(RefCell::new(Box::new([
        Box::new([1, 2, 3]),
        Box::new([4, 5, 6]),
    ])));
    let mut p: Ptr<Box<[i32]>> = (rows.as_pointer() as Ptr<Box<[i32]>>);
    ({
        cpp2rust_assert_fail_2(
            (({ first_int_0((rows.as_pointer() as Ptr<Box<[i32]>>).decay().to_any()) }) == 1),
        )
    });
    ({ cpp2rust_assert_fail_2((({ first_int_0((p).decay().to_any()) }) == 1)) });
    ({
        cpp2rust_assert_fail_2(
            (({
                let ___s1: AnyPtr = (rows.as_pointer() as Ptr<Box<[i32]>>).decay().to_any();
                let ___s2: AnyPtr = ((((rows.as_pointer() as Ptr<Box<[i32]>>)
                    .offset(0)
                    .as_pointer()) as Ptr<i32>) as Ptr<i32>)
                    .to_any();
                let ___n: u64 = (::std::mem::size_of::<[i32; 3]>() as u64);
                memcmp_3(___s1, ___s2, ___n)
            }) == 0),
        )
    });
    p.prefix_inc();
    ({ cpp2rust_assert_fail_2((({ first_int_0((p).decay().to_any()) }) == 4)) });
    let words: Value<Box<[Box<[i8]>]>> = Rc::new(RefCell::new(Box::new([
        i8::array_from_literal(b"cat\0"),
        i8::array_from_literal(b"ant\0"),
        i8::array_from_literal(b"bee\0"),
    ])));
    ({
        cpp2rust_assert_fail_2(
            ((({ first_char_1((words.as_pointer() as Ptr<Box<[i8]>>).decay().to_any()) }) as i32)
                == (('c' as i8) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                memcmp_3(
                    (words.as_pointer() as Ptr<Box<[i8]>>).decay().to_any(),
                    Ptr::<i8>::from_string_literal(b"cat").to_any(),
                    4_u64,
                )
            }) == 0),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
