extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X {
    pub v: (),
}
impl std::cmp::Ord for X {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if ((operator_lt_0(self.clone(), other.clone())) != 0) {
                std::cmp::Ordering::Less
            } else if ((operator_lt_0(other.clone(), self.clone())) != 0) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for X {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ((operator_eq_1(self.clone(), other.clone())) != 0) }
    }
}
impl std::cmp::Eq for X {}
pub unsafe fn operator_eq_1(mut a: X, mut b: X) -> () {
    return if ((a.v) == (b.v)) {
        2_()
    } else {
        unimplemented!()
    };
}
pub unsafe fn operator_ne_2(mut a: X, mut b: X) -> () {
    return if ((a.v) != (b.v)) {
        3_()
    } else {
        unimplemented!()
    };
}
pub unsafe fn operator_lt_0(mut a: X, mut b: X) -> () {
    return if ((a.v) < (b.v)) {
        4_()
    } else {
        unimplemented!()
    };
}
pub unsafe fn operator_gt_3(mut a: X, mut b: X) -> () {
    return if ((a.v) > (b.v)) {
        5_()
    } else {
        unimplemented!()
    };
}
pub unsafe fn operator_le_4(mut a: X, mut b: X) -> () {
    return if ((a.v) <= (b.v)) {
        6_()
    } else {
        unimplemented!()
    };
}
pub unsafe fn operator_ge_5(mut a: X, mut b: X) -> () {
    return if ((a.v) >= (b.v)) {
        7_()
    } else {
        unimplemented!()
    };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Result {
    pub r: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Custom {
    pub v: (),
}
impl Custom {
    pub unsafe fn operator_cmp(&self, o: *const Custom) -> Result {
        return Result {
            r: ((self.v) - ((*o).v)),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Mixed {
    pub v: (),
}
impl Mixed {
    pub unsafe fn operator_cmp(&self, o: *const Mixed) -> Result {
        return Result {
            r: ((self.v) - ((*o).v)),
        };
    }
    pub unsafe fn operator_lt(&self, o: *const Mixed) -> () {
        return if ((self.v) < ((*o).v)) {
            8_()
        } else {
            unimplemented!()
        };
    }
    pub unsafe fn operator_eq(&self, o: *const Mixed) -> () {
        return if ((self.v) == ((*o).v)) {
            9_()
        } else {
            unimplemented!()
        };
    }
}
impl std::cmp::Ord for Mixed {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if ((Mixed::operator_lt(self, other as *const Mixed)) != 0) {
                std::cmp::Ordering::Less
            } else if ((Mixed::operator_lt(other, self as *const Mixed)) != 0) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Mixed {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Mixed {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ((Mixed::operator_eq(self, other as *const Mixed)) != 0) }
    }
}
impl std::cmp::Eq for Mixed {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyBool {
    pub value: (),
}
impl MyBool {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.value;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Boolish {
    pub v: (),
}
impl std::cmp::Ord for Boolish {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if {
                let __cmp = &operator_lt_6(self.clone(), other.clone());
                MyBool::to_(__cmp)
            } {
                std::cmp::Ordering::Less
            } else if {
                let __cmp = &operator_lt_6(other.clone(), self.clone());
                MyBool::to_(__cmp)
            } {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Boolish {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Boolish {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            {
                let __cmp = &operator_eq_7(self.clone(), other.clone());
                MyBool::to_(__cmp)
            }
        }
    }
}
impl std::cmp::Eq for Boolish {}
pub unsafe fn operator_eq_7(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ ((a.v) == (b.v)) });
}
pub unsafe fn operator_lt_6(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ ((a.v) < (b.v)) });
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct RefBool {
    pub value: (),
}
impl RefBool {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.value;
    }
}
pub static mut ref_true_8: std::cell::LazyCell<RefBool> =
    std::cell::LazyCell::new(|| unsafe { RefBool::new({ true }) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RefInt {
    pub v: (),
}
impl RefInt {
    pub unsafe fn operator_eq(&self, _a0: *const RefInt) -> *mut RefBool {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut ref_true_8));
    }
}
impl std::cmp::PartialEq for RefInt {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            {
                let __cmp = RefInt::operator_eq(self, other as *const RefInt);
                RefBool::to_(&*__cmp)
            }
        }
    }
}
impl std::cmp::Eq for RefInt {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: X = X { v: 1_() };
    let mut b: X = X { v: 2_() };
    let mut c: X = X { v: 1_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut xs: [X; 3] = [X { v: 3_() }, X { v: 1_() }, X { v: 2_() }];
    unimplemented!();
    unimplemented!();
    let mut p: Custom = Custom { v: 5_() };
    let mut q: Custom = Custom { v: 2_() };
    let mut r: Result = (unsafe { Custom::operator_cmp(&p, &q) });
    unimplemented!();
    let mut m: Mixed = Mixed { v: 4_() };
    let mut n: Mixed = Mixed { v: 4_() };
    unimplemented!();
    unimplemented!();
    let mut ms: [Mixed; 3] = [Mixed { v: 6_() }, Mixed { v: 4_() }, Mixed { v: 5_() }];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b1: Boolish = Boolish { v: 1_() };
    let mut b2: Boolish = Boolish { v: 2_() };
    unimplemented!();
    unimplemented!();
    let mut lt: MyBool = (unsafe {
        let _a: Boolish = b1;
        operator_lt_6(_a, b2)
    });
    unimplemented!();
    let mut bs: [Boolish; 3] = [
        Boolish { v: 3_() },
        Boolish { v: 1_() },
        Boolish { v: 2_() },
    ];
    unimplemented!();
    unimplemented!();
    let mut ri1: RefInt = RefInt { v: 1_() };
    let mut ri2: RefInt = RefInt { v: 2_() };
    unimplemented!();
    let mut ris: [RefInt; 2] = [RefInt { v: 1_() }, RefInt { v: 2_() }];
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ref_true_8);
}
