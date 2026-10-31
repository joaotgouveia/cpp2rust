extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cmpxchg_weak_loop_0(
    atomic: *mut (),
    expected: *mut libc::c_char,
    mut desired: libc::c_char,
) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_1(
    atomic: *mut (),
    expected: *mut libc::c_char,
    mut desired: libc::c_char,
) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_2(atomic: *mut (), expected: *mut i8, mut desired: i8) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_3(atomic: *mut (), expected: *mut i8, mut desired: i8) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_4(atomic: *mut (), expected: *mut u8, mut desired: u8) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_5(atomic: *mut (), expected: *mut u8, mut desired: u8) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_6(atomic: *mut (), expected: *mut i16, mut desired: i16) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_7(atomic: *mut (), expected: *mut i16, mut desired: i16) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_8(atomic: *mut (), expected: *mut u16, mut desired: u16) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_9(atomic: *mut (), expected: *mut u16, mut desired: u16) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_10(atomic: *mut (), expected: *mut i32, mut desired: i32) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_11(atomic: *mut (), expected: *mut i32, mut desired: i32) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_12(atomic: *mut (), expected: *mut u32, mut desired: u32) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_13(atomic: *mut (), expected: *mut u32, mut desired: u32) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_14(atomic: *mut (), expected: *mut i64, mut desired: i64) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_15(atomic: *mut (), expected: *mut i64, mut desired: i64) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_16(atomic: *mut (), expected: *mut u64, mut desired: u64) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_17(atomic: *mut (), expected: *mut u64, mut desired: u64) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_18(atomic: *mut (), expected: *mut i64, mut desired: i64) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_19(atomic: *mut (), expected: *mut i64, mut desired: i64) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_20(atomic: *mut (), expected: *mut u64, mut desired: u64) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_21(atomic: *mut (), expected: *mut u64, mut desired: u64) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_22(atomic: *mut (), expected: *mut u8, mut desired: u8) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_23(atomic: *mut (), expected: *mut u8, mut desired: u8) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_24(atomic: *mut (), expected: *mut u16, mut desired: u16) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_25(atomic: *mut (), expected: *mut u16, mut desired: u16) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_26(atomic: *mut (), expected: *mut u32, mut desired: u32) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_27(atomic: *mut (), expected: *mut u32, mut desired: u32) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_28(atomic: *mut (), expected: *mut i32, mut desired: i32) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_29(atomic: *mut (), expected: *mut i32, mut desired: i32) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn do_test_30() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == ((0 as libc::c_char) as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == ((0 as libc::c_char) as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((3 as libc::c_char) as i32)));
    let mut x: libc::c_char = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_0(&mut obj, &mut x, (2 as libc::c_char),) }) as i32)
            == (true as i32))
    );
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((x as i32) == ((3 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((x as i32) == ((2 as libc::c_char) as i32)));
    x = (2 as libc::c_char);
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((x as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((x as i32) == ((1 as libc::c_char) as i32)));
    assert!((((unimplemented!()) as i32) == ((0 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((0 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((0 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((0 as libc::c_char) as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == ((5 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((5 as libc::c_char) as i32)));
    assert!((((unimplemented!()) as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!((((unimplemented!()) as i32) == ((7 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((7 as libc::c_char) as i32)));
    assert!((((unimplemented!()) as i32) == ((7 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((7 as libc::c_char) as i32)));
    assert!((((unimplemented!()) as i32) == ((8 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((8 as libc::c_char) as i32)));
    {
        let mut storage: [libc::c_char; 1] = [(23 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_31() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == ((0 as libc::c_char) as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == ((0 as libc::c_char) as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((3 as libc::c_char) as i32)));
    let mut x: libc::c_char = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_1(&mut obj, &mut x, (2 as libc::c_char),) }) as i32)
            == (true as i32))
    );
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((x as i32) == ((3 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((x as i32) == ((2 as libc::c_char) as i32)));
    x = (2 as libc::c_char);
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((x as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((x as i32) == ((1 as libc::c_char) as i32)));
    assert!((((unimplemented!()) as i32) == ((0 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((0 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((0 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((1 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((0 as libc::c_char) as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == ((5 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((5 as libc::c_char) as i32)));
    assert!((((unimplemented!()) as i32) == ((2 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((2 as libc::c_char) as i32)));
    assert!((((unimplemented!()) as i32) == ((7 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((7 as libc::c_char) as i32)));
    assert!((((unimplemented!()) as i32) == ((7 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((7 as libc::c_char) as i32)));
    assert!((((unimplemented!()) as i32) == ((8 as libc::c_char) as i32)));
    assert!(((unimplemented!() as i32) == ((8 as libc::c_char) as i32)));
    {
        let mut storage: [libc::c_char; 1] = [(23 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_32() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i8 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i8 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (3_i8 as i32)));
    let mut x: i8 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_2(&mut obj, &mut x, 2_i8,) }) as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((x as i32) == (3_i8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((x as i32) == (2_i8 as i32)));
    x = 2_i8;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((x as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((x as i32) == (1_i8 as i32)));
    assert!((((unimplemented!()) as i32) == (0_i8 as i32)));
    assert!(((unimplemented!() as i32) == (0_i8 as i32)));
    assert!(((unimplemented!() as i32) == (0_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (0_i8 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_i8 as i32)));
    assert!(((unimplemented!() as i32) == (5_i8 as i32)));
    assert!((((unimplemented!()) as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_i8 as i32)));
    assert!(((unimplemented!() as i32) == (7_i8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_i8 as i32)));
    assert!(((unimplemented!() as i32) == (7_i8 as i32)));
    assert!((((unimplemented!()) as i32) == (8_i8 as i32)));
    assert!(((unimplemented!() as i32) == (8_i8 as i32)));
    {
        let mut storage: [libc::c_char; 1] = [(23 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_33() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i8 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i8 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (3_i8 as i32)));
    let mut x: i8 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_3(&mut obj, &mut x, 2_i8,) }) as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((x as i32) == (3_i8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((x as i32) == (2_i8 as i32)));
    x = 2_i8;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((x as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((x as i32) == (1_i8 as i32)));
    assert!((((unimplemented!()) as i32) == (0_i8 as i32)));
    assert!(((unimplemented!() as i32) == (0_i8 as i32)));
    assert!(((unimplemented!() as i32) == (0_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (1_i8 as i32)));
    assert!(((unimplemented!() as i32) == (0_i8 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_i8 as i32)));
    assert!(((unimplemented!() as i32) == (5_i8 as i32)));
    assert!((((unimplemented!()) as i32) == (2_i8 as i32)));
    assert!(((unimplemented!() as i32) == (2_i8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_i8 as i32)));
    assert!(((unimplemented!() as i32) == (7_i8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_i8 as i32)));
    assert!(((unimplemented!() as i32) == (7_i8 as i32)));
    assert!((((unimplemented!()) as i32) == (8_i8 as i32)));
    assert!(((unimplemented!() as i32) == (8_i8 as i32)));
    {
        let mut storage: [libc::c_char; 1] = [(23 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_34() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (3_u8 as i32)));
    let mut x: u8 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_4(&mut obj, &mut x, 2_u8,) }) as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((x as i32) == (3_u8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((x as i32) == (2_u8 as i32)));
    x = 2_u8;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((x as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((x as i32) == (1_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_u8 as i32)));
    assert!(((unimplemented!() as i32) == (5_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u8 as i32)));
    assert!(((unimplemented!() as i32) == (7_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u8 as i32)));
    assert!(((unimplemented!() as i32) == (7_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (8_u8 as i32)));
    assert!(((unimplemented!() as i32) == (8_u8 as i32)));
    {
        let mut storage: [libc::c_char; 1] = [(23 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_35() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (3_u8 as i32)));
    let mut x: u8 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_5(&mut obj, &mut x, 2_u8,) }) as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((x as i32) == (3_u8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((x as i32) == (2_u8 as i32)));
    x = 2_u8;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((x as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((x as i32) == (1_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_u8 as i32)));
    assert!(((unimplemented!() as i32) == (5_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u8 as i32)));
    assert!(((unimplemented!() as i32) == (7_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u8 as i32)));
    assert!(((unimplemented!() as i32) == (7_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (8_u8 as i32)));
    assert!(((unimplemented!() as i32) == (8_u8 as i32)));
    {
        let mut storage: [libc::c_char; 1] = [(23 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_36() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i16 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i16 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (3_i16 as i32)));
    let mut x: i16 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_6(&mut obj, &mut x, 2_i16,) }) as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((x as i32) == (3_i16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((x as i32) == (2_i16 as i32)));
    x = 2_i16;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((x as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((x as i32) == (1_i16 as i32)));
    assert!((((unimplemented!()) as i32) == (0_i16 as i32)));
    assert!(((unimplemented!() as i32) == (0_i16 as i32)));
    assert!(((unimplemented!() as i32) == (0_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (0_i16 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_i16 as i32)));
    assert!(((unimplemented!() as i32) == (5_i16 as i32)));
    assert!((((unimplemented!()) as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_i16 as i32)));
    assert!(((unimplemented!() as i32) == (7_i16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_i16 as i32)));
    assert!(((unimplemented!() as i32) == (7_i16 as i32)));
    assert!((((unimplemented!()) as i32) == (8_i16 as i32)));
    assert!(((unimplemented!() as i32) == (8_i16 as i32)));
    {
        let mut storage: [libc::c_char; 2] = [(23 as libc::c_char), (0 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_37() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i16 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i16 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (3_i16 as i32)));
    let mut x: i16 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_7(&mut obj, &mut x, 2_i16,) }) as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((x as i32) == (3_i16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((x as i32) == (2_i16 as i32)));
    x = 2_i16;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((x as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((x as i32) == (1_i16 as i32)));
    assert!((((unimplemented!()) as i32) == (0_i16 as i32)));
    assert!(((unimplemented!() as i32) == (0_i16 as i32)));
    assert!(((unimplemented!() as i32) == (0_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (1_i16 as i32)));
    assert!(((unimplemented!() as i32) == (0_i16 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_i16 as i32)));
    assert!(((unimplemented!() as i32) == (5_i16 as i32)));
    assert!((((unimplemented!()) as i32) == (2_i16 as i32)));
    assert!(((unimplemented!() as i32) == (2_i16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_i16 as i32)));
    assert!(((unimplemented!() as i32) == (7_i16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_i16 as i32)));
    assert!(((unimplemented!() as i32) == (7_i16 as i32)));
    assert!((((unimplemented!()) as i32) == (8_i16 as i32)));
    assert!(((unimplemented!() as i32) == (8_i16 as i32)));
    {
        let mut storage: [libc::c_char; 2] = [(23 as libc::c_char), (0 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_38() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (3_u16 as i32)));
    let mut x: u16 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_8(&mut obj, &mut x, 2_u16,) }) as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((x as i32) == (3_u16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((x as i32) == (2_u16 as i32)));
    x = 2_u16;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((x as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((x as i32) == (1_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_u16 as i32)));
    assert!(((unimplemented!() as i32) == (5_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u16 as i32)));
    assert!(((unimplemented!() as i32) == (7_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u16 as i32)));
    assert!(((unimplemented!() as i32) == (7_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (8_u16 as i32)));
    assert!(((unimplemented!() as i32) == (8_u16 as i32)));
    {
        let mut storage: [libc::c_char; 2] = [(23 as libc::c_char), (0 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_39() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (3_u16 as i32)));
    let mut x: u16 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_9(&mut obj, &mut x, 2_u16,) }) as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((x as i32) == (3_u16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((x as i32) == (2_u16 as i32)));
    x = 2_u16;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((x as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((x as i32) == (1_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_u16 as i32)));
    assert!(((unimplemented!() as i32) == (5_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u16 as i32)));
    assert!(((unimplemented!() as i32) == (7_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u16 as i32)));
    assert!(((unimplemented!() as i32) == (7_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (8_u16 as i32)));
    assert!(((unimplemented!() as i32) == (8_u16 as i32)));
    {
        let mut storage: [libc::c_char; 2] = [(23 as libc::c_char), (0 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_40() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0)));
    unimplemented!();
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (3)));
    let mut x: i32 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_10(&mut obj, &mut x, 2,) }) as i32) == (true as i32)));
    assert!(((unimplemented!()) == (2)));
    assert!(((x) == (3)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2)));
    assert!(((x) == (2)));
    x = 2;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1)));
    assert!(((x) == (2)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1)));
    assert!(((x) == (1)));
    assert!(((unimplemented!()) == (0)));
    assert!(((unimplemented!()) == (0)));
    assert!(((unimplemented!()) == (0)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (0)));
    unimplemented!();
    assert!(((unimplemented!()) == (5)));
    assert!(((unimplemented!()) == (5)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (7)));
    assert!(((unimplemented!()) == (7)));
    assert!(((unimplemented!()) == (7)));
    assert!(((unimplemented!()) == (7)));
    assert!(((unimplemented!()) == (8)));
    assert!(((unimplemented!()) == (8)));
    {
        let mut storage: [libc::c_char; 4] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0)));
    }
}
pub unsafe fn do_test_41() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0)));
    unimplemented!();
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (3)));
    let mut x: i32 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_11(&mut obj, &mut x, 2,) }) as i32) == (true as i32)));
    assert!(((unimplemented!()) == (2)));
    assert!(((x) == (3)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2)));
    assert!(((x) == (2)));
    x = 2;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1)));
    assert!(((x) == (2)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1)));
    assert!(((x) == (1)));
    assert!(((unimplemented!()) == (0)));
    assert!(((unimplemented!()) == (0)));
    assert!(((unimplemented!()) == (0)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (1)));
    assert!(((unimplemented!()) == (0)));
    unimplemented!();
    assert!(((unimplemented!()) == (5)));
    assert!(((unimplemented!()) == (5)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (2)));
    assert!(((unimplemented!()) == (7)));
    assert!(((unimplemented!()) == (7)));
    assert!(((unimplemented!()) == (7)));
    assert!(((unimplemented!()) == (7)));
    assert!(((unimplemented!()) == (8)));
    assert!(((unimplemented!()) == (8)));
    {
        let mut storage: [libc::c_char; 4] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0)));
    }
}
pub unsafe fn do_test_42() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0_u32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0_u32)));
    unimplemented!();
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (3_u32)));
    let mut x: u32 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_12(&mut obj, &mut x, 2_u32,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((x) == (3_u32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((x) == (2_u32)));
    x = 2_u32;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((x) == (2_u32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((x) == (1_u32)));
    assert!(((unimplemented!()) == (0_u32)));
    assert!(((unimplemented!()) == (0_u32)));
    assert!(((unimplemented!()) == (0_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (0_u32)));
    unimplemented!();
    assert!(((unimplemented!()) == (5_u32)));
    assert!(((unimplemented!()) == (5_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (7_u32)));
    assert!(((unimplemented!()) == (7_u32)));
    assert!(((unimplemented!()) == (7_u32)));
    assert!(((unimplemented!()) == (7_u32)));
    assert!(((unimplemented!()) == (8_u32)));
    assert!(((unimplemented!()) == (8_u32)));
    {
        let mut storage: [libc::c_char; 4] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0_u32)));
    }
}
pub unsafe fn do_test_43() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0_u32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0_u32)));
    unimplemented!();
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (3_u32)));
    let mut x: u32 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_13(&mut obj, &mut x, 2_u32,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((x) == (3_u32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((x) == (2_u32)));
    x = 2_u32;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((x) == (2_u32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((x) == (1_u32)));
    assert!(((unimplemented!()) == (0_u32)));
    assert!(((unimplemented!()) == (0_u32)));
    assert!(((unimplemented!()) == (0_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (1_u32)));
    assert!(((unimplemented!()) == (0_u32)));
    unimplemented!();
    assert!(((unimplemented!()) == (5_u32)));
    assert!(((unimplemented!()) == (5_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (2_u32)));
    assert!(((unimplemented!()) == (7_u32)));
    assert!(((unimplemented!()) == (7_u32)));
    assert!(((unimplemented!()) == (7_u32)));
    assert!(((unimplemented!()) == (7_u32)));
    assert!(((unimplemented!()) == (8_u32)));
    assert!(((unimplemented!()) == (8_u32)));
    {
        let mut storage: [libc::c_char; 4] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0_u32)));
    }
}
pub unsafe fn do_test_44() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0_i64)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0_i64)));
    unimplemented!();
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (3_i64)));
    let mut x: i64 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_14(&mut obj, &mut x, 2_i64,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((x) == (3_i64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((x) == (2_i64)));
    x = 2_i64;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((x) == (2_i64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((x) == (1_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    unimplemented!();
    assert!(((unimplemented!()) == (5_i64)));
    assert!(((unimplemented!()) == (5_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (8_i64)));
    assert!(((unimplemented!()) == (8_i64)));
    {
        let mut storage: [libc::c_char; 8] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0_i64)));
    }
}
pub unsafe fn do_test_45() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0_i64)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0_i64)));
    unimplemented!();
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (3_i64)));
    let mut x: i64 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_15(&mut obj, &mut x, 2_i64,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((x) == (3_i64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((x) == (2_i64)));
    x = 2_i64;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((x) == (2_i64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((x) == (1_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    unimplemented!();
    assert!(((unimplemented!()) == (5_i64)));
    assert!(((unimplemented!()) == (5_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (8_i64)));
    assert!(((unimplemented!()) == (8_i64)));
    {
        let mut storage: [libc::c_char; 8] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0_i64)));
    }
}
pub unsafe fn do_test_46() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0_u64)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0_u64)));
    unimplemented!();
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (3_u64)));
    let mut x: u64 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_16(&mut obj, &mut x, 2_u64,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((x) == (3_u64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((x) == (2_u64)));
    x = 2_u64;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((x) == (2_u64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((x) == (1_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    unimplemented!();
    assert!(((unimplemented!()) == (5_u64)));
    assert!(((unimplemented!()) == (5_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (8_u64)));
    assert!(((unimplemented!()) == (8_u64)));
    {
        let mut storage: [libc::c_char; 8] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0_u64)));
    }
}
pub unsafe fn do_test_47() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0_u64)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0_u64)));
    unimplemented!();
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (3_u64)));
    let mut x: u64 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_17(&mut obj, &mut x, 2_u64,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((x) == (3_u64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((x) == (2_u64)));
    x = 2_u64;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((x) == (2_u64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((x) == (1_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    unimplemented!();
    assert!(((unimplemented!()) == (5_u64)));
    assert!(((unimplemented!()) == (5_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (8_u64)));
    assert!(((unimplemented!()) == (8_u64)));
    {
        let mut storage: [libc::c_char; 8] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0_u64)));
    }
}
pub unsafe fn do_test_48() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0_i64)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0_i64)));
    unimplemented!();
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (3_i64)));
    let mut x: i64 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_18(&mut obj, &mut x, 2_i64,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((x) == (3_i64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((x) == (2_i64)));
    x = 2_i64;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((x) == (2_i64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((x) == (1_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    unimplemented!();
    assert!(((unimplemented!()) == (5_i64)));
    assert!(((unimplemented!()) == (5_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (8_i64)));
    assert!(((unimplemented!()) == (8_i64)));
    {
        let mut storage: [libc::c_char; 8] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0_i64)));
    }
}
pub unsafe fn do_test_49() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0_i64)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0_i64)));
    unimplemented!();
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (3_i64)));
    let mut x: i64 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_19(&mut obj, &mut x, 2_i64,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((x) == (3_i64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((x) == (2_i64)));
    x = 2_i64;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((x) == (2_i64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((x) == (1_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (1_i64)));
    assert!(((unimplemented!()) == (0_i64)));
    unimplemented!();
    assert!(((unimplemented!()) == (5_i64)));
    assert!(((unimplemented!()) == (5_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (2_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (7_i64)));
    assert!(((unimplemented!()) == (8_i64)));
    assert!(((unimplemented!()) == (8_i64)));
    {
        let mut storage: [libc::c_char; 8] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0_i64)));
    }
}
pub unsafe fn do_test_50() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0_u64)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0_u64)));
    unimplemented!();
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (3_u64)));
    let mut x: u64 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_20(&mut obj, &mut x, 2_u64,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((x) == (3_u64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((x) == (2_u64)));
    x = 2_u64;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((x) == (2_u64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((x) == (1_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    unimplemented!();
    assert!(((unimplemented!()) == (5_u64)));
    assert!(((unimplemented!()) == (5_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (8_u64)));
    assert!(((unimplemented!()) == (8_u64)));
    {
        let mut storage: [libc::c_char; 8] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0_u64)));
    }
}
pub unsafe fn do_test_51() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!()) == (0_u64)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!()) == (0_u64)));
    unimplemented!();
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (3_u64)));
    let mut x: u64 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_21(&mut obj, &mut x, 2_u64,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((x) == (3_u64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((x) == (2_u64)));
    x = 2_u64;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((x) == (2_u64)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((x) == (1_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (1_u64)));
    assert!(((unimplemented!()) == (0_u64)));
    unimplemented!();
    assert!(((unimplemented!()) == (5_u64)));
    assert!(((unimplemented!()) == (5_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (7_u64)));
    assert!(((unimplemented!()) == (8_u64)));
    assert!(((unimplemented!()) == (8_u64)));
    {
        let mut storage: [libc::c_char; 8] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!()) == (0_u64)));
    }
}
pub unsafe fn do_test_52() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (3_u8 as i32)));
    let mut x: u8 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_22(&mut obj, &mut x, 2_u8,) }) as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((x as i32) == (3_u8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((x as i32) == (2_u8 as i32)));
    x = 2_u8;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((x as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((x as i32) == (1_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_u8 as i32)));
    assert!(((unimplemented!() as i32) == (5_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u8 as i32)));
    assert!(((unimplemented!() as i32) == (7_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u8 as i32)));
    assert!(((unimplemented!() as i32) == (7_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (8_u8 as i32)));
    assert!(((unimplemented!() as i32) == (8_u8 as i32)));
    {
        let mut storage: [libc::c_char; 1] = [(23 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_53() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (3_u8 as i32)));
    let mut x: u8 = unimplemented!();
    assert!((((unsafe { cmpxchg_weak_loop_23(&mut obj, &mut x, 2_u8,) }) as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((x as i32) == (3_u8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((x as i32) == (2_u8 as i32)));
    x = 2_u8;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((x as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((x as i32) == (1_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (1_u8 as i32)));
    assert!(((unimplemented!() as i32) == (0_u8 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_u8 as i32)));
    assert!(((unimplemented!() as i32) == (5_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (2_u8 as i32)));
    assert!(((unimplemented!() as i32) == (2_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u8 as i32)));
    assert!(((unimplemented!() as i32) == (7_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u8 as i32)));
    assert!(((unimplemented!() as i32) == (7_u8 as i32)));
    assert!((((unimplemented!()) as i32) == (8_u8 as i32)));
    assert!(((unimplemented!() as i32) == (8_u8 as i32)));
    {
        let mut storage: [libc::c_char; 1] = [(23 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_54() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (3_u16 as i32)));
    let mut x: u16 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_24(&mut obj, &mut x, 2_u16,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((x as i32) == (3_u16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((x as i32) == (2_u16 as i32)));
    x = 2_u16;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((x as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((x as i32) == (1_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_u16 as i32)));
    assert!(((unimplemented!() as i32) == (5_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u16 as i32)));
    assert!(((unimplemented!() as i32) == (7_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u16 as i32)));
    assert!(((unimplemented!() as i32) == (7_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (8_u16 as i32)));
    assert!(((unimplemented!() as i32) == (8_u16 as i32)));
    {
        let mut storage: [libc::c_char; 2] = [(23 as libc::c_char), (0 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_55() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (3_u16 as i32)));
    let mut x: u16 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_25(&mut obj, &mut x, 2_u16,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((x as i32) == (3_u16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((x as i32) == (2_u16 as i32)));
    x = 2_u16;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((x as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((x as i32) == (1_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (1_u16 as i32)));
    assert!(((unimplemented!() as i32) == (0_u16 as i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_u16 as i32)));
    assert!(((unimplemented!() as i32) == (5_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (2_u16 as i32)));
    assert!(((unimplemented!() as i32) == (2_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u16 as i32)));
    assert!(((unimplemented!() as i32) == (7_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (7_u16 as i32)));
    assert!(((unimplemented!() as i32) == (7_u16 as i32)));
    assert!((((unimplemented!()) as i32) == (8_u16 as i32)));
    assert!(((unimplemented!() as i32) == (8_u16 as i32)));
    {
        let mut storage: [libc::c_char; 2] = [(23 as libc::c_char), (0 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_56() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as u32) == (0_u32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as u32) == (0_u32)));
    unimplemented!();
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((unimplemented!() as u32) == (3_u32)));
    let mut x: u32 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_26(&mut obj, &mut x, 2_u32,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((x) == (3_u32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((x) == (2_u32)));
    x = 2_u32;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((x) == (2_u32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((x) == (1_u32)));
    assert!((((unimplemented!()) as u32) == (0_u32)));
    assert!(((unimplemented!() as u32) == (0_u32)));
    assert!(((unimplemented!() as u32) == (0_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (0_u32)));
    unimplemented!();
    assert!((((unimplemented!()) as u32) == (5_u32)));
    assert!(((unimplemented!() as u32) == (5_u32)));
    assert!((((unimplemented!()) as u32) == (2_u32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!((((unimplemented!()) as u32) == (7_u32)));
    assert!(((unimplemented!() as u32) == (7_u32)));
    assert!((((unimplemented!()) as u32) == (7_u32)));
    assert!(((unimplemented!() as u32) == (7_u32)));
    assert!((((unimplemented!()) as u32) == (8_u32)));
    assert!(((unimplemented!() as u32) == (8_u32)));
    {
        let mut storage: [libc::c_char; 4] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as u32) == (0_u32)));
    }
}
pub unsafe fn do_test_57() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as u32) == (0_u32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as u32) == (0_u32)));
    unimplemented!();
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((unimplemented!() as u32) == (3_u32)));
    let mut x: u32 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_27(&mut obj, &mut x, 2_u32,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((x) == (3_u32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((x) == (2_u32)));
    x = 2_u32;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((x) == (2_u32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((x) == (1_u32)));
    assert!((((unimplemented!()) as u32) == (0_u32)));
    assert!(((unimplemented!() as u32) == (0_u32)));
    assert!(((unimplemented!() as u32) == (0_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (1_u32)));
    assert!(((unimplemented!() as u32) == (0_u32)));
    unimplemented!();
    assert!((((unimplemented!()) as u32) == (5_u32)));
    assert!(((unimplemented!() as u32) == (5_u32)));
    assert!((((unimplemented!()) as u32) == (2_u32)));
    assert!(((unimplemented!() as u32) == (2_u32)));
    assert!((((unimplemented!()) as u32) == (7_u32)));
    assert!(((unimplemented!() as u32) == (7_u32)));
    assert!((((unimplemented!()) as u32) == (7_u32)));
    assert!(((unimplemented!() as u32) == (7_u32)));
    assert!((((unimplemented!()) as u32) == (8_u32)));
    assert!(((unimplemented!() as u32) == (8_u32)));
    {
        let mut storage: [libc::c_char; 4] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as u32) == (0_u32)));
    }
}
pub unsafe fn do_test_58() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((unimplemented!() as i32) == (3_i32)));
    let mut x: i32 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_28(&mut obj, &mut x, 2_i32,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((x) == (3_i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((x) == (2_i32)));
    x = 2_i32;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((x) == (2_i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((x) == (1_i32)));
    assert!((((unimplemented!()) as i32) == (0_i32)));
    assert!(((unimplemented!() as i32) == (0_i32)));
    assert!(((unimplemented!() as i32) == (0_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (0_i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_i32)));
    assert!(((unimplemented!() as i32) == (5_i32)));
    assert!((((unimplemented!()) as i32) == (2_i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!((((unimplemented!()) as i32) == (7_i32)));
    assert!(((unimplemented!() as i32) == (7_i32)));
    assert!((((unimplemented!()) as i32) == (7_i32)));
    assert!(((unimplemented!() as i32) == (7_i32)));
    assert!((((unimplemented!()) as i32) == (8_i32)));
    assert!(((unimplemented!() as i32) == (8_i32)));
    {
        let mut storage: [libc::c_char; 4] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn do_test_59() {
    let mut obj: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i32)));
    {
        let mut lockfree: bool = unimplemented!();
        &(lockfree);
        {
            assert!(lockfree);
        }
    }
    unimplemented!();
    assert!(((unimplemented!() as i32) == (0_i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((unimplemented!() as i32) == (3_i32)));
    let mut x: i32 = unimplemented!();
    assert!(
        (((unsafe { cmpxchg_weak_loop_29(&mut obj, &mut x, 2_i32,) }) as i32) == (true as i32))
    );
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((x) == (3_i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((x) == (2_i32)));
    x = 2_i32;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((x) == (2_i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((x) == (1_i32)));
    assert!((((unimplemented!()) as i32) == (0_i32)));
    assert!(((unimplemented!() as i32) == (0_i32)));
    assert!(((unimplemented!() as i32) == (0_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (1_i32)));
    assert!(((unimplemented!() as i32) == (0_i32)));
    unimplemented!();
    assert!((((unimplemented!()) as i32) == (5_i32)));
    assert!(((unimplemented!() as i32) == (5_i32)));
    assert!((((unimplemented!()) as i32) == (2_i32)));
    assert!(((unimplemented!() as i32) == (2_i32)));
    assert!((((unimplemented!()) as i32) == (7_i32)));
    assert!(((unimplemented!() as i32) == (7_i32)));
    assert!((((unimplemented!()) as i32) == (7_i32)));
    assert!(((unimplemented!() as i32) == (7_i32)));
    assert!((((unimplemented!()) as i32) == (8_i32)));
    assert!(((unimplemented!() as i32) == (8_i32)));
    {
        let mut storage: [libc::c_char; 4] = [
            (23 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (0)));
    }
}
pub unsafe fn test_60() {
    (unsafe { do_test_30() });
    (unsafe { do_test_31() });
}
pub unsafe fn test_61() {
    (unsafe { do_test_32() });
    (unsafe { do_test_33() });
}
pub unsafe fn test_62() {
    (unsafe { do_test_34() });
    (unsafe { do_test_35() });
}
pub unsafe fn test_63() {
    (unsafe { do_test_36() });
    (unsafe { do_test_37() });
}
pub unsafe fn test_64() {
    (unsafe { do_test_38() });
    (unsafe { do_test_39() });
}
pub unsafe fn test_65() {
    (unsafe { do_test_40() });
    (unsafe { do_test_41() });
}
pub unsafe fn test_66() {
    (unsafe { do_test_42() });
    (unsafe { do_test_43() });
}
pub unsafe fn test_67() {
    (unsafe { do_test_44() });
    (unsafe { do_test_45() });
}
pub unsafe fn test_68() {
    (unsafe { do_test_46() });
    (unsafe { do_test_47() });
}
pub unsafe fn test_69() {
    (unsafe { do_test_48() });
    (unsafe { do_test_49() });
}
pub unsafe fn test_70() {
    (unsafe { do_test_50() });
    (unsafe { do_test_51() });
}
pub unsafe fn test_71() {
    (unsafe { do_test_52() });
    (unsafe { do_test_53() });
}
pub unsafe fn test_72() {
    (unsafe { do_test_54() });
    (unsafe { do_test_55() });
}
pub unsafe fn test_73() {
    (unsafe { do_test_56() });
    (unsafe { do_test_57() });
}
pub unsafe fn test_74() {
    (unsafe { do_test_58() });
    (unsafe { do_test_59() });
}
pub unsafe fn test_75() {
    (unsafe { do_test_31() });
    (unsafe { do_test_31() });
}
pub unsafe fn test_76() {
    (unsafe { do_test_33() });
    (unsafe { do_test_33() });
}
pub unsafe fn test_77() {
    (unsafe { do_test_35() });
    (unsafe { do_test_35() });
}
pub unsafe fn test_78() {
    (unsafe { do_test_37() });
    (unsafe { do_test_37() });
}
pub unsafe fn test_79() {
    (unsafe { do_test_39() });
    (unsafe { do_test_39() });
}
pub unsafe fn test_80() {
    (unsafe { do_test_41() });
    (unsafe { do_test_41() });
}
pub unsafe fn test_81() {
    (unsafe { do_test_43() });
    (unsafe { do_test_43() });
}
pub unsafe fn test_82() {
    (unsafe { do_test_45() });
    (unsafe { do_test_45() });
}
pub unsafe fn test_83() {
    (unsafe { do_test_47() });
    (unsafe { do_test_47() });
}
pub unsafe fn test_84() {
    (unsafe { do_test_49() });
    (unsafe { do_test_49() });
}
pub unsafe fn test_85() {
    (unsafe { do_test_51() });
    (unsafe { do_test_51() });
}
pub unsafe fn test_86() {
    (unsafe { do_test_55() });
    (unsafe { do_test_55() });
}
pub unsafe fn test_87() {
    (unsafe { do_test_57() });
    (unsafe { do_test_57() });
}
pub unsafe fn test_88() {
    (unsafe { do_test_59() });
    (unsafe { do_test_59() });
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_60() });
    (unsafe { test_61() });
    (unsafe { test_62() });
    (unsafe { test_63() });
    (unsafe { test_64() });
    (unsafe { test_65() });
    (unsafe { test_66() });
    (unsafe { test_67() });
    (unsafe { test_68() });
    (unsafe { test_69() });
    (unsafe { test_70() });
    (unsafe { test_71() });
    (unsafe { test_72() });
    (unsafe { test_73() });
    (unsafe { test_74() });
    (unsafe { test_61() });
    (unsafe { test_62() });
    (unsafe { test_63() });
    (unsafe { test_64() });
    (unsafe { test_65() });
    (unsafe { test_66() });
    (unsafe { test_67() });
    (unsafe { test_68() });
    (unsafe { test_75() });
    (unsafe { test_76() });
    (unsafe { test_77() });
    (unsafe { test_78() });
    (unsafe { test_79() });
    (unsafe { test_80() });
    (unsafe { test_81() });
    (unsafe { test_82() });
    (unsafe { test_83() });
    (unsafe { test_84() });
    (unsafe { test_85() });
    (unsafe { test_86() });
    (unsafe { test_87() });
    (unsafe { test_88() });
    (unsafe { test_76() });
    (unsafe { test_77() });
    (unsafe { test_78() });
    (unsafe { test_79() });
    (unsafe { test_80() });
    (unsafe { test_81() });
    (unsafe { test_82() });
    (unsafe { test_83() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
