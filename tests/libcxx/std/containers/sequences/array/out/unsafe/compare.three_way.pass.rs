extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn testComparisonsComplete_0(
    t1: *const Vec<i32>,
    t2: *const Vec<i32>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
    } else if isLess {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
    } else if isGreater {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
    } else {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_1(
    t1: *const Vec<i32>,
    t2: *const Vec<i32>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_2(
    t1: *const Vec<StrongOrder>,
    t2: *const Vec<StrongOrder>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
    } else if isLess {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
    } else if isGreater {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
    } else {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_3(
    t1: *const Vec<StrongOrder>,
    t2: *const Vec<StrongOrder>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_4(
    t1: *const Vec<WeakOrder>,
    t2: *const Vec<WeakOrder>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
    } else if isLess {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
    } else if isGreater {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
    } else {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_5(
    t1: *const Vec<WeakOrder>,
    t2: *const Vec<WeakOrder>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_6(
    t1: *const Vec<PartialOrder>,
    t2: *const Vec<PartialOrder>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
    } else if isLess {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
    } else if isGreater {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
    } else {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_7(
    t1: *const Vec<PartialOrder>,
    t2: *const Vec<PartialOrder>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_8(
    t1: *const Vec<LessAndEqComp>,
    t2: *const Vec<LessAndEqComp>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
    } else if isLess {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
    } else if isGreater {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
    } else {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_9(
    t1: *const Vec<LessAndEqComp>,
    t2: *const Vec<LessAndEqComp>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testOrder_10(
    t1: *const Vec<i32>,
    t2: *const Vec<i32>,
    mut order: std::cmp::Ordering,
) -> bool {
    let mut equal: bool = order == std::cmp::Ordering::Equal;
    let mut less: bool = order == std::cmp::Ordering::Less;
    let mut greater: bool = order == std::cmp::Ordering::Greater;
    return (unimplemented!() == order)
        && (unsafe {
            let _t1: *const Vec<i32> = t1;
            let _t2: *const Vec<i32> = t2;
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_0(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_11(
    t1: *const Vec<i32>,
    t2: *const Vec<i32>,
    mut order: std::cmp::Ordering,
) -> bool {
    let mut equal: bool = order == std::cmp::Ordering::Equal;
    let mut less: bool = order == std::cmp::Ordering::Less;
    let mut greater: bool = order == std::cmp::Ordering::Greater;
    return (unimplemented!() == order)
        && (unsafe {
            let _t1: *const Vec<i32> = t1;
            let _t2: *const Vec<i32> = t2;
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_1(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_12(
    t1: *const Vec<StrongOrder>,
    t2: *const Vec<StrongOrder>,
    mut order: std::cmp::Ordering,
) -> bool {
    let mut equal: bool = order == std::cmp::Ordering::Equal;
    let mut less: bool = order == std::cmp::Ordering::Less;
    let mut greater: bool = order == std::cmp::Ordering::Greater;
    return (unimplemented!() == order)
        && (unsafe {
            let _t1: *const Vec<StrongOrder> = t1;
            let _t2: *const Vec<StrongOrder> = t2;
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_2(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_13(
    t1: *const Vec<StrongOrder>,
    t2: *const Vec<StrongOrder>,
    mut order: std::cmp::Ordering,
) -> bool {
    let mut equal: bool = order == std::cmp::Ordering::Equal;
    let mut less: bool = order == std::cmp::Ordering::Less;
    let mut greater: bool = order == std::cmp::Ordering::Greater;
    return (unimplemented!() == order)
        && (unsafe {
            let _t1: *const Vec<StrongOrder> = t1;
            let _t2: *const Vec<StrongOrder> = t2;
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_3(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_14(
    t1: *const Vec<WeakOrder>,
    t2: *const Vec<WeakOrder>,
    mut order: (),
) -> bool {
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const Vec<WeakOrder> = t1;
            let _t2: *const Vec<WeakOrder> = t2;
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_15(
    t1: *const Vec<WeakOrder>,
    t2: *const Vec<WeakOrder>,
    mut order: (),
) -> bool {
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const Vec<WeakOrder> = t1;
            let _t2: *const Vec<WeakOrder> = t2;
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_5(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_16(
    t1: *const Vec<PartialOrder>,
    t2: *const Vec<PartialOrder>,
    mut order: (),
) -> bool {
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const Vec<PartialOrder> = t1;
            let _t2: *const Vec<PartialOrder> = t2;
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_6(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_17(
    t1: *const Vec<PartialOrder>,
    t2: *const Vec<PartialOrder>,
    mut order: (),
) -> bool {
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const Vec<PartialOrder> = t1;
            let _t2: *const Vec<PartialOrder> = t2;
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_7(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_18(
    t1: *const Vec<LessAndEqComp>,
    t2: *const Vec<LessAndEqComp>,
    mut order: (),
) -> bool {
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const Vec<LessAndEqComp> = t1;
            let _t2: *const Vec<LessAndEqComp> = t2;
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_8(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_19(
    t1: *const Vec<LessAndEqComp>,
    t2: *const Vec<LessAndEqComp>,
    mut order: (),
) -> bool {
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const Vec<LessAndEqComp> = t1;
            let _t2: *const Vec<LessAndEqComp> = t2;
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_9(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn operator_lt_20(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> bool {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_21(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> bool {
    return (((*lhs).value) == ((*rhs).value));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct LessAndEqComp {
    pub value: i32,
}
impl LessAndEqComp {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_20(self as *const LessAndEqComp, other as *const LessAndEqComp) {
                std::cmp::Ordering::Less
            } else if operator_lt_20(other as *const LessAndEqComp, self as *const LessAndEqComp) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LessAndEqComp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LessAndEqComp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_21(self as *const LessAndEqComp, other as *const LessAndEqComp) }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub unsafe fn operator_cmp_22(mut _a0: StrongOrder, mut _a1: StrongOrder) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&(_a0.value), &(_a1.value));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct StrongOrder {
    pub value: i32,
}
impl StrongOrder {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_22(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_22(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub unsafe fn operator_cmp_23(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let mut cmp: () = unimplemented!();
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct WeakOrder {
    pub value: i32,
}
impl WeakOrder {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_23(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_23(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub unsafe fn operator_cmp_24(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if ((lhs.value) == (i32::MIN)) || ((rhs.value) == (i32::MIN)) {
        return unimplemented!();
    }
    if ((lhs.value) == (i32::MAX)) || ((rhs.value) == (i32::MAX)) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_25(mut lhs: PartialOrder, mut rhs: PartialOrder) -> bool {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct PartialOrder {
    pub value: i32,
}
impl PartialOrder {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_24(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_25(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for PartialOrder {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonComparable {}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub unsafe fn operator_eq_26(
    _a0: *const EqualityComparable,
    _a1: *const EqualityComparable,
) -> bool {
    return (((*_a0).value_) == ((*_a1).value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct EqualityComparable {
    value_: i32,
}
impl EqualityComparable {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_26(
                self as *const EqualityComparable,
                other as *const EqualityComparable,
            )
        }
    }
}
impl std::cmp::Eq for EqualityComparable {}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub unsafe fn operator_eq_27(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> bool {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_28(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).value_), &((*_a1).value_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct ThreeWayComparable {
    value_: i32,
}
impl ThreeWayComparable {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_28(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::PartialOrd for ThreeWayComparable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ThreeWayComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_27(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::Eq for ThreeWayComparable {}
const _: () = assert!(true, "std::equality_comparable<ThreeWayComparable>");
const _: () = assert!(true, "std::three_way_comparable<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorSpaceship<ThreeWayComparable>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E1 {
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E2 {
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(bool, to_bool_1)]
pub struct ImplicitBool {
    pub val: bool,
}
impl ImplicitBool {
    pub unsafe fn to_bool_1(&self) -> bool {
        return self.val;
    }
}
pub unsafe fn operator_eq_29(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: ((lhs.x) == (rhs.y)),
    };
}
pub static mut N_30: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe { 1_usize });
const _: () = assert!(true, "std::three_way_comparable<std::array<int, N>>");
const _: () = assert!(
    !(false),
    "!std::three_way_comparable<std::array<NonComparable, N>>"
);
pub unsafe fn test_sequence_container_array_spaceship_with_type_31() {
    {
        let mut l1: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        let mut l2: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        assert!((unsafe { testOrder_10(&l1, &l2, std::cmp::Ordering::Equal,) }));
    }
    {
        let mut l1: Vec<i32> = Vec::from([1, 1]);
        let mut l2: Vec<i32> = Vec::from([1, 1]);
        assert!((unsafe { testOrder_11(&l1, &l2, std::cmp::Ordering::Equal,) }));
    }
    {
        let mut l1: Vec<i32> = Vec::from([1, 1]);
        let mut l2: Vec<i32> = Vec::from([1, 2]);
        assert!((unsafe { testOrder_11(&l1, &l2, std::cmp::Ordering::Less,) }));
    }
    {
        let mut l1: Vec<i32> = Vec::from([1, 3]);
        let mut l2: Vec<i32> = Vec::from([1, 2]);
        assert!((unsafe { testOrder_11(&l1, &l2, std::cmp::Ordering::Greater,) }));
    }
    {}
}
pub unsafe fn test_sequence_container_array_spaceship_with_type_32() {
    {
        let mut l1: Vec<StrongOrder> =
            std::array::from_fn::<_, 0, _>(|_| <StrongOrder>::default()).to_vec();
        let mut l2: Vec<StrongOrder> =
            std::array::from_fn::<_, 0, _>(|_| <StrongOrder>::default()).to_vec();
        assert!((unsafe { testOrder_12(&l1, &l2, std::cmp::Ordering::Equal,) }));
    }
    {
        let mut l1: Vec<StrongOrder> =
            Vec::from([StrongOrder::new({ 1 }), StrongOrder::new({ 1 })]);
        let mut l2: Vec<StrongOrder> =
            Vec::from([StrongOrder::new({ 1 }), StrongOrder::new({ 1 })]);
        assert!((unsafe { testOrder_13(&l1, &l2, std::cmp::Ordering::Equal,) }));
    }
    {
        let mut l1: Vec<StrongOrder> =
            Vec::from([StrongOrder::new({ 1 }), StrongOrder::new({ 1 })]);
        let mut l2: Vec<StrongOrder> =
            Vec::from([StrongOrder::new({ 1 }), StrongOrder::new({ 2 })]);
        assert!((unsafe { testOrder_13(&l1, &l2, std::cmp::Ordering::Less,) }));
    }
    {
        let mut l1: Vec<StrongOrder> =
            Vec::from([StrongOrder::new({ 1 }), StrongOrder::new({ 3 })]);
        let mut l2: Vec<StrongOrder> =
            Vec::from([StrongOrder::new({ 1 }), StrongOrder::new({ 2 })]);
        assert!((unsafe { testOrder_13(&l1, &l2, std::cmp::Ordering::Greater,) }));
    }
    {}
}
pub unsafe fn test_sequence_container_array_spaceship_with_type_33() {
    {
        let mut l1: Vec<WeakOrder> =
            std::array::from_fn::<_, 0, _>(|_| <WeakOrder>::default()).to_vec();
        let mut l2: Vec<WeakOrder> =
            std::array::from_fn::<_, 0, _>(|_| <WeakOrder>::default()).to_vec();
        assert!((unsafe { testOrder_14(&l1, &l2, unimplemented!(),) }));
    }
    {
        let mut l1: Vec<WeakOrder> = Vec::from([WeakOrder::new({ 1 }), WeakOrder::new({ 1 })]);
        let mut l2: Vec<WeakOrder> = Vec::from([WeakOrder::new({ 1 }), WeakOrder::new({ 1 })]);
        assert!((unsafe { testOrder_15(&l1, &l2, unimplemented!(),) }));
    }
    {
        let mut l1: Vec<WeakOrder> = Vec::from([WeakOrder::new({ 1 }), WeakOrder::new({ 1 })]);
        let mut l2: Vec<WeakOrder> = Vec::from([WeakOrder::new({ 1 }), WeakOrder::new({ 2 })]);
        assert!((unsafe { testOrder_15(&l1, &l2, unimplemented!(),) }));
    }
    {
        let mut l1: Vec<WeakOrder> = Vec::from([WeakOrder::new({ 1 }), WeakOrder::new({ 3 })]);
        let mut l2: Vec<WeakOrder> = Vec::from([WeakOrder::new({ 1 }), WeakOrder::new({ 2 })]);
        assert!((unsafe { testOrder_15(&l1, &l2, unimplemented!(),) }));
    }
    {}
}
pub unsafe fn test_sequence_container_array_spaceship_with_type_34() {
    {
        let mut l1: Vec<PartialOrder> =
            std::array::from_fn::<_, 0, _>(|_| <PartialOrder>::default()).to_vec();
        let mut l2: Vec<PartialOrder> =
            std::array::from_fn::<_, 0, _>(|_| <PartialOrder>::default()).to_vec();
        assert!((unsafe { testOrder_16(&l1, &l2, unimplemented!(),) }));
    }
    {
        let mut l1: Vec<PartialOrder> =
            Vec::from([PartialOrder::new({ 1 }), PartialOrder::new({ 1 })]);
        let mut l2: Vec<PartialOrder> =
            Vec::from([PartialOrder::new({ 1 }), PartialOrder::new({ 1 })]);
        assert!((unsafe { testOrder_17(&l1, &l2, unimplemented!(),) }));
    }
    {
        let mut l1: Vec<PartialOrder> =
            Vec::from([PartialOrder::new({ 1 }), PartialOrder::new({ 1 })]);
        let mut l2: Vec<PartialOrder> =
            Vec::from([PartialOrder::new({ 1 }), PartialOrder::new({ 2 })]);
        assert!((unsafe { testOrder_17(&l1, &l2, unimplemented!(),) }));
    }
    {
        let mut l1: Vec<PartialOrder> =
            Vec::from([PartialOrder::new({ 1 }), PartialOrder::new({ 3 })]);
        let mut l2: Vec<PartialOrder> =
            Vec::from([PartialOrder::new({ 1 }), PartialOrder::new({ 2 })]);
        assert!((unsafe { testOrder_17(&l1, &l2, unimplemented!(),) }));
    }
    {
        let mut l1: Vec<PartialOrder> =
            Vec::from([PartialOrder::new({ 1 }), PartialOrder::new({ i32::MIN })]);
        let mut l2: Vec<PartialOrder> =
            Vec::from([PartialOrder::new({ 1 }), PartialOrder::new({ 2 })]);
        assert!((unsafe { testOrder_17(&l1, &l2, unimplemented!(),) }));
    }
}
pub unsafe fn test_sequence_container_array_spaceship_with_type_35() {
    {
        let mut l1: Vec<LessAndEqComp> =
            std::array::from_fn::<_, 0, _>(|_| <LessAndEqComp>::default()).to_vec();
        let mut l2: Vec<LessAndEqComp> =
            std::array::from_fn::<_, 0, _>(|_| <LessAndEqComp>::default()).to_vec();
        assert!((unsafe { testOrder_18(&l1, &l2, unimplemented!(),) }));
    }
    {
        let mut l1: Vec<LessAndEqComp> =
            Vec::from([LessAndEqComp::new({ 1 }), LessAndEqComp::new({ 1 })]);
        let mut l2: Vec<LessAndEqComp> =
            Vec::from([LessAndEqComp::new({ 1 }), LessAndEqComp::new({ 1 })]);
        assert!((unsafe { testOrder_19(&l1, &l2, unimplemented!(),) }));
    }
    {
        let mut l1: Vec<LessAndEqComp> =
            Vec::from([LessAndEqComp::new({ 1 }), LessAndEqComp::new({ 1 })]);
        let mut l2: Vec<LessAndEqComp> =
            Vec::from([LessAndEqComp::new({ 1 }), LessAndEqComp::new({ 2 })]);
        assert!((unsafe { testOrder_19(&l1, &l2, unimplemented!(),) }));
    }
    {
        let mut l1: Vec<LessAndEqComp> =
            Vec::from([LessAndEqComp::new({ 1 }), LessAndEqComp::new({ 3 })]);
        let mut l2: Vec<LessAndEqComp> =
            Vec::from([LessAndEqComp::new({ 1 }), LessAndEqComp::new({ 2 })]);
        assert!((unsafe { testOrder_19(&l1, &l2, unimplemented!(),) }));
    }
    {}
}
pub unsafe fn test_sequence_container_array_spaceship_36() -> bool {
    (unsafe { test_sequence_container_array_spaceship_with_type_31() });
    (unsafe { test_sequence_container_array_spaceship_with_type_32() });
    (unsafe { test_sequence_container_array_spaceship_with_type_33() });
    (unsafe { test_sequence_container_array_spaceship_with_type_34() });
    (unsafe { test_sequence_container_array_spaceship_with_type_35() });
    return true;
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
    assert!((unsafe { test_sequence_container_array_spaceship_36() }));
    const _: () = assert!(true, "test_sequence_container_array_spaceship()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const N_30);
}
