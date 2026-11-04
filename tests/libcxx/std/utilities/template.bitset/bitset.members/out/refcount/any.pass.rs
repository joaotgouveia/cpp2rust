extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_any_0() {
    let v: Value<std_bitset_0_> = Rc::new(RefCell::new(<std_bitset_0_>::default()));
    ({ (*v.borrow()).reset_22() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (false as i32))) });
    ({ (*v.borrow()).set_20() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == ((0_u64 != 0_u64) as i32))) });
    {}
}
pub fn test_any_2() {
    let v: Value<std_bitset_1_> = Rc::new(RefCell::new(<std_bitset_1_>::default()));
    ({ (*v.borrow()).reset_22() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (false as i32))) });
    ({ (*v.borrow()).set_20() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == ((1_u64 != 0_u64) as i32))) });
    {}
}
pub fn test_any_3() {
    let v: Value<std_bitset_31_> = Rc::new(RefCell::new(<std_bitset_31_>::default()));
    ({ (*v.borrow()).reset_22() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (false as i32))) });
    ({ (*v.borrow()).set_20() });
    ({
        cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == ((31_u64 != 0_u64) as i32)))
    });
    {
        (*v.borrow_mut())[(31_u64 as u64).wrapping_div(2_u64)] = false;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
        ({ (*v.borrow()).reset_22() });
        (*v.borrow_mut())[(31_u64 as u64).wrapping_div(2_u64)] = true;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
    }
}
pub fn test_any_4() {
    let v: Value<std_bitset_32_> = Rc::new(RefCell::new(<std_bitset_32_>::default()));
    ({ (*v.borrow()).reset_22() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (false as i32))) });
    ({ (*v.borrow()).set_20() });
    ({
        cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == ((32_u64 != 0_u64) as i32)))
    });
    {
        (*v.borrow_mut())[(32_u64 as u64).wrapping_div(2_u64)] = false;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
        ({ (*v.borrow()).reset_22() });
        (*v.borrow_mut())[(32_u64 as u64).wrapping_div(2_u64)] = true;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
    }
}
pub fn test_any_5() {
    let v: Value<std_bitset_33_> = Rc::new(RefCell::new(<std_bitset_33_>::default()));
    ({ (*v.borrow()).reset_22() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (false as i32))) });
    ({ (*v.borrow()).set_20() });
    ({
        cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == ((33_u64 != 0_u64) as i32)))
    });
    {
        (*v.borrow_mut())[(33_u64 as u64).wrapping_div(2_u64)] = false;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
        ({ (*v.borrow()).reset_22() });
        (*v.borrow_mut())[(33_u64 as u64).wrapping_div(2_u64)] = true;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
    }
}
pub fn test_any_6() {
    let v: Value<std_bitset_63_> = Rc::new(RefCell::new(<std_bitset_63_>::default()));
    ({ (*v.borrow()).reset_22() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (false as i32))) });
    ({ (*v.borrow()).set_20() });
    ({
        cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == ((63_u64 != 0_u64) as i32)))
    });
    {
        (*v.borrow_mut())[(63_u64 as u64).wrapping_div(2_u64)] = false;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
        ({ (*v.borrow()).reset_22() });
        (*v.borrow_mut())[(63_u64 as u64).wrapping_div(2_u64)] = true;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
    }
}
pub fn test_any_7() {
    let v: Value<std_bitset_64_> = Rc::new(RefCell::new(<std_bitset_64_>::default()));
    ({ (*v.borrow()).reset_22() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (false as i32))) });
    ({ (*v.borrow()).set_20() });
    ({
        cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == ((64_u64 != 0_u64) as i32)))
    });
    {
        (*v.borrow_mut())[(64_u64 as u64).wrapping_div(2_u64)] = false;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
        ({ (*v.borrow()).reset_22() });
        (*v.borrow_mut())[(64_u64 as u64).wrapping_div(2_u64)] = true;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
    }
}
pub fn test_any_8() {
    let v: Value<std_bitset_65_> = Rc::new(RefCell::new(<std_bitset_65_>::default()));
    ({ (*v.borrow()).reset_22() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (false as i32))) });
    ({ (*v.borrow()).set_20() });
    ({
        cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == ((65_u64 != 0_u64) as i32)))
    });
    {
        (*v.borrow_mut())[(65_u64 as u64).wrapping_div(2_u64)] = false;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
        ({ (*v.borrow()).reset_22() });
        (*v.borrow_mut())[(65_u64 as u64).wrapping_div(2_u64)] = true;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
    }
}
pub fn test_any_9() {
    let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(<std_bitset_1000_>::default()));
    ({ (*v.borrow()).reset_22() });
    ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (false as i32))) });
    ({ (*v.borrow()).set_20() });
    ({
        cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == ((1000_u64 != 0_u64) as i32)))
    });
    {
        (*v.borrow_mut())[(1000_u64 as u64).wrapping_div(2_u64)] = false;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
        ({ (*v.borrow()).reset_22() });
        (*v.borrow_mut())[(1000_u64 as u64).wrapping_div(2_u64)] = true;
        ({ cpp2rust_assert_fail_1(((({ (*v.borrow()).any() }) as i32) == (true as i32))) });
    }
}
pub fn test_10() -> bool {
    ({ test_any_0() });
    ({ test_any_2() });
    ({ test_any_3() });
    ({ test_any_4() });
    ({ test_any_5() });
    ({ test_any_6() });
    ({ test_any_7() });
    ({ test_any_8() });
    ({ test_any_9() });
    return true;
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
    ({ test_10() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
