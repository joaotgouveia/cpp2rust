extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(mut t: Local_1) -> i32 {
    return (t.x as i32);
}
pub fn get_2(mut t: Local_3) -> i32 {
    return t.x;
}
pub fn get_4(mut t: Local_5) -> i32 {
    return t.x;
}
pub fn get_6(mut t: Local_7) -> i32 {
    return (t.x as i32);
}
pub fn twice_8(mut t: Local_3) -> i32 {
    return (t.x * 2);
}
pub fn wrap_9(mut v: i32) -> i32 {
    let mut l: Local_5 = Local_5 { x: v };
    return ({ get_4(l) });
}
pub fn wrap_10(mut v: i64) -> i32 {
    let mut l: Local_7 = Local_7 { x: v };
    return ({ get_6(l) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Local_5 {
    #[offset(0)]
    pub x: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Local_7 {
    #[offset(0)]
    pub x: i64,
}
pub fn other_11() -> i32 {
    let mut l: Local_1 = Local_1 { x: 3_i64, y: 4_i64 };
    return (({ get_0((l).clone()) }) + (l.y as i32));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Local_1 {
    #[offset(0)]
    pub x: i64,
    #[offset(8)]
    pub y: i64,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut l: Local_3 = Local_3 { x: 7 };
    (if (({ get_2((l).clone()) }) == 7) {
        &(0);
    } else {
        ({
            __assert_fail_12(
                Ptr::<i8>::from_string_literal(b"get(l) == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ twice_8((l).clone()) }) == 14) {
        &(0);
    } else {
        ({
            __assert_fail_12(
                Ptr::<i8>::from_string_literal(b"ns::twice(l) == 14"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ other_11() }) == 7) {
        &(0);
    } else {
        ({
            __assert_fail_12(
                Ptr::<i8>::from_string_literal(b"other() == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ wrap_9(5) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_12(
                Ptr::<i8>::from_string_literal(b"wrap(5) == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ wrap_10(6_i64) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_12(
                Ptr::<i8>::from_string_literal(b"wrap(6L) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Local_3 {
    #[offset(0)]
    pub x: i32,
}
pub fn __assert_fail_12(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
