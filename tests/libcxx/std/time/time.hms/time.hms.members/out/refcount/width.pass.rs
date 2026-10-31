extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_width_0() -> bool {
    return (unimplemented!() == 0_u32);
}
pub fn check_width_1() -> bool {
    return (unimplemented!() == 0_u32);
}
pub fn check_width_2() -> bool {
    return (unimplemented!() == 0_u32);
}
pub fn check_width_3() -> bool {
    return (unimplemented!() == 3_u32);
}
pub fn check_width_4() -> bool {
    return (unimplemented!() == 6_u32);
}
pub fn check_width_5() -> bool {
    return (unimplemented!() == 9_u32);
}
pub fn check_width_6() -> bool {
    return (unimplemented!() == 1_u32);
}
pub fn check_width_7() -> bool {
    return (unimplemented!() == 6_u32);
}
pub fn check_width_8() -> bool {
    return (unimplemented!() == 2_u32);
}
pub fn check_width_9() -> bool {
    return (unimplemented!() == 1_u32);
}
pub fn check_width_10() -> bool {
    return (unimplemented!() == 6_u32);
}
pub fn check_width_11() -> bool {
    return (unimplemented!() == 6_u32);
}
pub fn check_width_12() -> bool {
    return (unimplemented!() == 3_u32);
}
pub fn check_width_13() -> bool {
    return (unimplemented!() == 6_u32);
}
pub fn check_width_14() -> bool {
    return (unimplemented!() == 1_u32);
}
pub fn check_width_15() -> bool {
    return (unimplemented!() == 4_u32);
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    const _: () = assert!(
        true,
        "check_width<std::chrono::hours,                               0>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::minutes,                             0>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::seconds,                             0>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::milliseconds,                        3>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::microseconds,                        6>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::nanoseconds,                         9>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   2>>, 1>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   3>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   4>>, 2>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   5>>, 1>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   6>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   7>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   8>>, 3>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   9>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,  10>>, 1>()"
    );;
    const _: () = assert!(
        true,
        "check_width<microfortnights,                                  4>()"
    );;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
