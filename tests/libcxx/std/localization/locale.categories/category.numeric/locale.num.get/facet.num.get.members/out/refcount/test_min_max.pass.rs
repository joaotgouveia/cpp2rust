extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_stream_failed_0(val: Ptr<Vec<i8>>) -> bool {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: i16 = 0_i16;
    return unimplemented!();
}
pub fn check_stream_failed_1(val: Ptr<Vec<i8>>) -> bool {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: u16 = 0_u16;
    return unimplemented!();
}
pub fn check_stream_failed_2(val: Ptr<Vec<i8>>) -> bool {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: i32 = 0_i32;
    return unimplemented!();
}
pub fn check_stream_failed_3(val: Ptr<Vec<i8>>) -> bool {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: u32 = 0_u32;
    return unimplemented!();
}
pub fn check_stream_failed_4(val: Ptr<Vec<i8>>) -> bool {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: i64 = 0_i64;
    return unimplemented!();
}
pub fn check_stream_failed_5(val: Ptr<Vec<i8>>) -> bool {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: u64 = 0_u64;
    return unimplemented!();
}
pub fn check_stream_failed_6(val: Ptr<Vec<i8>>) -> bool {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: i64 = 0_i64;
    return unimplemented!();
}
pub fn check_stream_failed_7(val: Ptr<Vec<i8>>) -> bool {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: u64 = 0_u64;
    return unimplemented!();
}
pub fn check_limits_8() {
    let mut is_unsigned: bool = unimplemented!();
    let mut minv: i16 = i16::MIN;
    let mut maxv: i16 = i16::MAX;
    let miniss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let maxiss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mins: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let minoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut new_minv: i16 = 0_i16;
    let mut new_maxv: i16 = 0_i16;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((new_minv as i32) == (minv as i32)));
    assert!(((new_maxv as i32) == (maxv as i32)));
    maxs.decay()
        .offset(({ ((*maxs.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(({ check_stream_failed_0(maxs.as_pointer(),) }));
    {
        mins.decay()
            .offset(({ ((*mins.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
            .with_mut(|__v| __v.postfix_inc());
        assert!(({ check_stream_failed_0(mins.as_pointer(),) }));
    }
}
pub fn check_limits_9() {
    let mut is_unsigned: bool = unimplemented!();
    let mut minv: u16 = u16::MIN;
    let mut maxv: u16 = u16::MAX;
    let miniss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let maxiss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mins: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let minoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut new_minv: u16 = 0_u16;
    let mut new_maxv: u16 = 0_u16;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((new_minv as i32) == (minv as i32)));
    assert!(((new_maxv as i32) == (maxv as i32)));
    maxs.decay()
        .offset(({ ((*maxs.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(({ check_stream_failed_1(maxs.as_pointer(),) }));
    {}
}
pub fn check_limits_10() {
    let mut is_unsigned: bool = unimplemented!();
    let mut minv: i32 = i32::MIN;
    let mut maxv: i32 = i32::MAX;
    let miniss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let maxiss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mins: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let minoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut new_minv: i32 = 0_i32;
    let mut new_maxv: i32 = 0_i32;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!((new_minv == minv));
    assert!((new_maxv == maxv));
    maxs.decay()
        .offset(({ ((*maxs.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(({ check_stream_failed_2(maxs.as_pointer(),) }));
    {
        mins.decay()
            .offset(({ ((*mins.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
            .with_mut(|__v| __v.postfix_inc());
        assert!(({ check_stream_failed_2(mins.as_pointer(),) }));
    }
}
pub fn check_limits_11() {
    let mut is_unsigned: bool = unimplemented!();
    let mut minv: u32 = u32::MIN;
    let mut maxv: u32 = u32::MAX;
    let miniss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let maxiss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mins: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let minoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut new_minv: u32 = 0_u32;
    let mut new_maxv: u32 = 0_u32;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!((new_minv == minv));
    assert!((new_maxv == maxv));
    maxs.decay()
        .offset(({ ((*maxs.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(({ check_stream_failed_3(maxs.as_pointer(),) }));
    {}
}
pub fn check_limits_12() {
    let mut is_unsigned: bool = unimplemented!();
    let mut minv: i64 = i64::MIN;
    let mut maxv: i64 = i64::MAX;
    let miniss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let maxiss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mins: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let minoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut new_minv: i64 = 0_i64;
    let mut new_maxv: i64 = 0_i64;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!((new_minv == minv));
    assert!((new_maxv == maxv));
    maxs.decay()
        .offset(({ ((*maxs.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(({ check_stream_failed_4(maxs.as_pointer(),) }));
    {
        mins.decay()
            .offset(({ ((*mins.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
            .with_mut(|__v| __v.postfix_inc());
        assert!(({ check_stream_failed_4(mins.as_pointer(),) }));
    }
}
pub fn check_limits_13() {
    let mut is_unsigned: bool = unimplemented!();
    let mut minv: u64 = u64::MIN;
    let mut maxv: u64 = u64::MAX;
    let miniss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let maxiss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mins: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let minoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut new_minv: u64 = 0_u64;
    let mut new_maxv: u64 = 0_u64;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!((new_minv == minv));
    assert!((new_maxv == maxv));
    maxs.decay()
        .offset(({ ((*maxs.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(({ check_stream_failed_5(maxs.as_pointer(),) }));
    {}
}
pub fn check_limits_14() {
    let mut is_unsigned: bool = unimplemented!();
    let mut minv: i64 = i64::MIN;
    let mut maxv: i64 = i64::MAX;
    let miniss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let maxiss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mins: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let minoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut new_minv: i64 = 0_i64;
    let mut new_maxv: i64 = 0_i64;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!((new_minv == minv));
    assert!((new_maxv == maxv));
    maxs.decay()
        .offset(({ ((*maxs.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(({ check_stream_failed_6(maxs.as_pointer(),) }));
    {
        mins.decay()
            .offset(({ ((*mins.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
            .with_mut(|__v| __v.postfix_inc());
        assert!(({ check_stream_failed_6(mins.as_pointer(),) }));
    }
}
pub fn check_limits_15() {
    let mut is_unsigned: bool = unimplemented!();
    let mut minv: u64 = unimplemented!();
    let mut maxv: u64 = unimplemented!();
    let miniss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let maxiss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mins: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let maxoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let minoss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut new_minv: u64 = 0_u64;
    let mut new_maxv: u64 = 0_u64;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!((new_minv == minv));
    assert!((new_maxv == maxv));
    maxs.decay()
        .offset(({ ((*maxs.borrow()).len() - 1).wrapping_sub(1_usize) }) as isize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(({ check_stream_failed_7(maxs.as_pointer(),) }));
    {}
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
    ({ check_limits_8() });
    ({ check_limits_9() });
    ({ check_limits_10() });
    ({ check_limits_11() });
    ({ check_limits_12() });
    ({ check_limits_13() });
    ({ check_limits_14() });
    ({ check_limits_15() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
