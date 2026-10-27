extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_all_0() -> () {
    let mut in_: () = unimplemented!();
    let mut in2: () = unimplemented!();
    let mut output: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut output2: () = unimplemented!();
    let mut out2: () = unimplemented!();
    let mut eq: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut sum: FnPtr<fn((), A_1) -> ()> =
        FnPtr::<fn((), A_1) -> ()>::new(|mut lhs: (), mut rhs: A_1| -> () {
            unsafe {
                return ((lhs) + (rhs.x));
            }
        });
    let mut proj1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) * (-1_()));
        }
    });
    let mut proj2: FnPtr<fn(A_1) -> ()> = FnPtr::<fn(A_1) -> ()>::new(|mut a: A_1| -> () {
        unsafe {
            return ((a.x) * (-1_()));
        }
    });
    (unsafe {
        test_2(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut ends_with_3)),
            &mut in_,
            &mut in2,
            &mut eq,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_4(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut equal_5)),
            &mut in_,
            &mut in2,
            &mut eq,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_6(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut lexicographical_compare_7)),
            &mut in_,
            &mut in2,
            &mut eq,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_8(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut is_permutation_9)),
            &mut in_,
            &mut in2,
            &mut eq,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_10(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut includes_11)),
            &mut in_,
            &mut in2,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_12(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut find_first_of_13)),
            &mut in_,
            &mut in2,
            &mut eq,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_14(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15)),
            &mut in_,
            &mut in2,
            &mut eq,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_16(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut search_17)),
            &mut in_,
            &mut in2,
            &mut eq,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_18(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut find_end_19)),
            &mut in_,
            &mut in2,
            &mut eq,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_20(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut transform_21)),
            &mut in_,
            &mut in2,
            &mut out,
            &mut sum,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_22(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut transform_21)),
            &mut in_,
            &mut in2,
            &mut out2,
            &mut sum,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_23(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_24)),
            &mut in_,
            &mut in2,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_25(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut merge_26)),
            &mut in_,
            &mut in2,
            &mut out,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_27(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut merge_26)),
            &mut in_,
            &mut in2,
            &mut out2,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_28(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_intersection_29)),
            &mut in_,
            &mut in2,
            &mut out,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_30(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_intersection_29)),
            &mut in_,
            &mut in2,
            &mut out2,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_31(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_difference_32)),
            &mut in_,
            &mut in2,
            &mut out,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_33(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_difference_32)),
            &mut in_,
            &mut in2,
            &mut out2,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_34(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_symmetric_difference_35)),
            &mut in_,
            &mut in2,
            &mut out,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_36(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_symmetric_difference_35)),
            &mut in_,
            &mut in2,
            &mut out2,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_37(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_union_38)),
            &mut in_,
            &mut in2,
            &mut out,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_39(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_union_38)),
            &mut in_,
            &mut in2,
            &mut out2,
            &mut less,
            &mut proj1,
            &mut proj2,
        )
    });
    (unsafe {
        test_40(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut starts_with_41)),
            &mut in_,
            &mut in2,
            &mut eq,
            &mut proj1,
            &mut proj2,
        )
    });
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct A_1 {
    pub x: (),
}
impl A_1 {
    pub unsafe fn new_1(mut value: ()) -> Self {
        let mut this = Self { x: value };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.x;
    }
    pub unsafe fn operator_cmp(&self, _a0: *const A_1) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const A_1)).x), &((*_a0).x));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const A_1) -> () {
        return (((*(self as *const A_1)).x) == ((*_a0).x));
    }
}
impl std::cmp::Ord for A_1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { A_1::operator_cmp(self, other as *const A_1) }
    }
}
impl std::cmp::PartialOrd for A_1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for A_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { A_1::operator_cmp(self, other as *const A_1) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for A_1 {}
impl Default for A_1 {
    fn default() -> Self {
        unsafe { A_1 { x: 0_() } }
    }
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_all_0() });
    const _: () = assert!(true, "test_all()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
