extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg)]
pub struct __gnu_test_func_callback {
    _M_size: (),
    _M_tests: [Option<unsafe fn()>; 15],
}
impl __gnu_test_func_callback {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            _M_size: 0_(),
            _M_tests: [None; 15],
        };
        this
    }
    pub unsafe fn size(&self) -> () {
        return self._M_size;
    }
    pub unsafe fn tests(&self) -> *const Option<unsafe fn()> {
        return self._M_tests.as_ptr();
    }
    pub unsafe fn push_back(&mut self, mut test: Option<unsafe fn()>) {
        self._M_tests[(self._M_size) as usize] = test;
        self._M_size.prefix_inc();
    }
}
impl Default for __gnu_test_func_callback {
    fn default() -> Self {
        unsafe { __gnu_test_func_callback::new() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_object_counter {}
impl __gnu_test_object_counter {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const __gnu_test_object_counter) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_dec();
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_test_object_counter,
    ) -> *mut __gnu_test_object_counter {
        return &mut (*(self as *mut __gnu_test_object_counter));
    }
}
impl Clone for __gnu_test_object_counter {
    fn clone(&self) -> Self {
        unsafe { __gnu_test_object_counter::copy_from(self as *const __gnu_test_object_counter) }
    }
}
impl Default for __gnu_test_object_counter {
    fn default() -> Self {
        unsafe { __gnu_test_object_counter::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_copy_constructor {}
impl __gnu_test_copy_constructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__1))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)))
        {
            unimplemented!();
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__3))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)))
        {
            unimplemented!();
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5)).postfix_inc();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5)) = 0_();
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_copy_tracker {
    id_: (),
    throw_on_copy_: (),
}
impl __gnu_test_copy_tracker {
    pub unsafe fn new(mut id: Option<()>, mut throw_on_copy: Option<()>) -> Self {
        let mut id: () = id.unwrap_or_else(|| unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut next_id__6)).postfix_dec()
        });
        let mut throw_on_copy: () = throw_on_copy.unwrap_or_else(|| unsafe { false });
        let mut this = Self {
            id_: id,
            throw_on_copy_: throw_on_copy,
        };
        this
    }
    pub unsafe fn copy_from(rhs: *const __gnu_test_copy_tracker) -> Self {
        let mut this = Self {
            id_: (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }),
            throw_on_copy_: (*rhs).throw_on_copy_,
        };
        if this.throw_on_copy_ {
            (unsafe {
                __gnu_test_copy_constructor::throw_on(
                    (unsafe { __gnu_test_copy_constructor::count() }).wrapping_add(1_()),
                )
            });
        }
        (unsafe { __gnu_test_copy_constructor::mark_call() });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        rhs: *const __gnu_test_copy_tracker,
    ) -> *mut __gnu_test_copy_tracker {
        self.id_ = (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) });
        if (*rhs).throw_on_copy_ {
            (unsafe {
                __gnu_test_assignment_operator::throw_on(
                    (unsafe { __gnu_test_assignment_operator::count() }).wrapping_add(1_()),
                )
            });
        }
        (unsafe { __gnu_test_assignment_operator::mark_call() });
        return &mut (*(self as *mut __gnu_test_copy_tracker));
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { __gnu_test_destructor::mark_call() });
    }
    pub unsafe fn id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn reset() {
        (unsafe { __gnu_test_copy_constructor::reset() });
        (unsafe { __gnu_test_assignment_operator::reset() });
        (unsafe { __gnu_test_destructor::reset() });
    }
}
impl std::cmp::Ord for __gnu_test_copy_tracker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_7(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_7(
                other as *const __gnu_test_copy_tracker,
                self as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for __gnu_test_copy_tracker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for __gnu_test_copy_tracker {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_8(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_copy_tracker {}
impl Clone for __gnu_test_copy_tracker {
    fn clone(&self) -> Self {
        unsafe { __gnu_test_copy_tracker::copy_from(self as *const __gnu_test_copy_tracker) }
    }
}
impl Default for __gnu_test_copy_tracker {
    fn default() -> Self {
        unsafe { __gnu_test_copy_tracker::new(None, None) }
    }
}
pub unsafe fn operator_eq_8(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        == (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
pub unsafe fn operator_lt_7(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        < (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_semaphore {
    sem_set_: (),
    pid_: (),
}
impl __gnu_test_semaphore {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct testcase_beta_double_ {
    pub f0: (),
    pub x: (),
    pub y: (),
    pub f: (),
}
pub static mut data001_9: std::cell::LazyCell<[testcase_beta_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_beta_double_ {
                f0: 1.0825088224469029e-06_f64,
                x: 10_f64,
                y: 10_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 4.992508740634678e-09_f64,
                x: 10_f64,
                y: 20_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.5729567312509485e-10_f64,
                x: 10_f64,
                y: 30_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.2168673582561288e-11_f64,
                x: 10_f64,
                y: 40_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.5916380099863291e-12_f64,
                x: 10_f64,
                y: 50_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.9408957938463963e-13_f64,
                x: 10_f64,
                y: 60_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 6.941163798069168e-14_f64,
                x: 10_f64,
                y: 70_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.966561297250265e-14_f64,
                x: 10_f64,
                y: 80_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 6.41878248281544e-15_f64,
                x: 10_f64,
                y: 90_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.3455339739604842e-15_f64,
                x: 10_f64,
                y: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data002_11: std::cell::LazyCell<[testcase_beta_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_beta_double_ {
                f0: 4.992508740634678e-09_f64,
                x: 20_f64,
                y: 10_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 7.254444551924844e-13_f64,
                x: 20_f64,
                y: 20_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.7681885473062028e-15_f64,
                x: 20_f64,
                y: 30_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.7891885039182335e-17_f64,
                x: 20_f64,
                y: 40_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 4.3240677875623635e-19_f64,
                x: 20_f64,
                y: 50_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.8857342309689053e-20_f64,
                x: 20_f64,
                y: 60_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.2609804003539998e-21_f64,
                x: 20_f64,
                y: 70_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.1660809542079041e-22_f64,
                x: 20_f64,
                y: 80_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.390794427972907e-23_f64,
                x: 20_f64,
                y: 90_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.0365059099917614e-24_f64,
                x: 20_f64,
                y: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_13: std::cell::LazyCell<[testcase_beta_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_beta_double_ {
                f0: 1.5729567312509485e-10_f64,
                x: 30_f64,
                y: 10_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.7681885473062028e-15_f64,
                x: 30_f64,
                y: 20_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 5.637077964048245e-19_f64,
                x: 30_f64,
                y: 30_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.0539424603796547e-21_f64,
                x: 30_f64,
                y: 40_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 6.011819777727384e-24_f64,
                x: 30_f64,
                y: 50_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 7.427952855326015e-26_f64,
                x: 30_f64,
                y: 60_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.6212207780604767e-27_f64,
                x: 30_f64,
                y: 70_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 5.478372971531762e-29_f64,
                x: 30_f64,
                y: 80_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.6183005659681346e-30_f64,
                x: 30_f64,
                y: 90_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.658794822212223e-31_f64,
                x: 30_f64,
                y: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_15: std::cell::LazyCell<[testcase_beta_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_beta_double_ {
                f0: 1.2168673582561288e-11_f64,
                x: 40_f64,
                y: 10_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.7891885039182335e-17_f64,
                x: 40_f64,
                y: 20_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.0539424603796547e-21_f64,
                x: 40_f64,
                y: 30_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 4.650850914009066e-25_f64,
                x: 40_f64,
                y: 40_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 7.516171211855773e-28_f64,
                x: 40_f64,
                y: 50_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.031133197988607e-30_f64,
                x: 40_f64,
                y: 60_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.4175035070466313e-32_f64,
                x: 40_f64,
                y: 70_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.273483914275837e-34_f64,
                x: 40_f64,
                y: 80_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 6.769062960131558e-36_f64,
                x: 40_f64,
                y: 90_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.9797337118810115e-37_f64,
                x: 40_f64,
                y: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_17: std::cell::LazyCell<[testcase_beta_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_beta_double_ {
                f0: 1.5916380099863291e-12_f64,
                x: 50_f64,
                y: 10_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 4.3240677875623635e-19_f64,
                x: 50_f64,
                y: 20_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 6.011819777727384e-24_f64,
                x: 50_f64,
                y: 30_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 7.516171211855773e-28_f64,
                x: 50_f64,
                y: 40_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.964661208567414e-31_f64,
                x: 50_f64,
                y: 50_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 5.84256439064184e-34_f64,
                x: 50_f64,
                y: 60_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.8672362180783552e-36_f64,
                x: 50_f64,
                y: 70_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.0939382296458963e-38_f64,
                x: 50_f64,
                y: 80_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.0442781609879874e-40_f64,
                x: 50_f64,
                y: 90_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.490412111095437e-42_f64,
                x: 50_f64,
                y: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data006_19: std::cell::LazyCell<[testcase_beta_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_beta_double_ {
                f0: 2.9408957938463963e-13_f64,
                x: 60_f64,
                y: 10_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.8857342309689053e-20_f64,
                x: 60_f64,
                y: 20_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 7.427952855326015e-26_f64,
                x: 60_f64,
                y: 30_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.031133197988607e-30_f64,
                x: 60_f64,
                y: 40_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 5.84256439064184e-34_f64,
                x: 60_f64,
                y: 50_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.450123146978223e-37_f64,
                x: 60_f64,
                y: 60_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 4.77068553860866e-40_f64,
                x: 60_f64,
                y: 70_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.2902663809721126e-42_f64,
                x: 60_f64,
                y: 80_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 6.010557105857051e-45_f64,
                x: 60_f64,
                y: 90_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 4.392289889834721e-47_f64,
                x: 60_f64,
                y: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data007_21: std::cell::LazyCell<[testcase_beta_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_beta_double_ {
                f0: 6.941163798069168e-14_f64,
                x: 70_f64,
                y: 10_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.2609804003539998e-21_f64,
                x: 70_f64,
                y: 20_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.6212207780604767e-27_f64,
                x: 70_f64,
                y: 30_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.4175035070466313e-32_f64,
                x: 70_f64,
                y: 40_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.8672362180783552e-36_f64,
                x: 70_f64,
                y: 50_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 4.77068553860866e-40_f64,
                x: 70_f64,
                y: 60_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.045313714348291e-43_f64,
                x: 70_f64,
                y: 70_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 4.019227408201378e-46_f64,
                x: 70_f64,
                y: 80_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 9.58658700635018e-49_f64,
                x: 70_f64,
                y: 90_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.74091273058198e-51_f64,
                x: 70_f64,
                y: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_23: std::cell::LazyCell<[testcase_beta_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_beta_double_ {
                f0: 1.966561297250265e-14_f64,
                x: 80_f64,
                y: 10_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.1660809542079041e-22_f64,
                x: 80_f64,
                y: 20_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 5.478372971531762e-29_f64,
                x: 80_f64,
                y: 30_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.273483914275837e-34_f64,
                x: 80_f64,
                y: 40_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.0939382296458963e-38_f64,
                x: 80_f64,
                y: 50_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.2902663809721126e-42_f64,
                x: 80_f64,
                y: 60_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 4.019227408201378e-46_f64,
                x: 80_f64,
                y: 70_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.716059082866941e-49_f64,
                x: 80_f64,
                y: 80_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.459377390212537e-52_f64,
                x: 80_f64,
                y: 90_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 7.480703996850347e-55_f64,
                x: 80_f64,
                y: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data009_25: std::cell::LazyCell<[testcase_beta_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_beta_double_ {
                f0: 6.41878248281544e-15_f64,
                x: 90_f64,
                y: 10_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.390794427972907e-23_f64,
                x: 90_f64,
                y: 20_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.6183005659681346e-30_f64,
                x: 90_f64,
                y: 30_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 6.769062960131558e-36_f64,
                x: 90_f64,
                y: 40_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.0442781609879874e-40_f64,
                x: 90_f64,
                y: 50_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 6.010557105857051e-45_f64,
                x: 90_f64,
                y: 60_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 9.58658700635018e-49_f64,
                x: 90_f64,
                y: 70_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.459377390212537e-52_f64,
                x: 90_f64,
                y: 80_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.4416737907558036e-55_f64,
                x: 90_f64,
                y: 90_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.023853191656425e-58_f64,
                x: 90_f64,
                y: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_26: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data010_27: std::cell::LazyCell<[testcase_beta_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_beta_double_ {
                f0: 2.3455339739604842e-15_f64,
                x: 100_f64,
                y: 10_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.0365059099917614e-24_f64,
                x: 100_f64,
                y: 20_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.658794822212223e-31_f64,
                x: 100_f64,
                y: 30_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.9797337118810115e-37_f64,
                x: 100_f64,
                y: 40_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 1.490412111095437e-42_f64,
                x: 100_f64,
                y: 50_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 4.392289889834721e-47_f64,
                x: 100_f64,
                y: 60_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.74091273058198e-51_f64,
                x: 100_f64,
                y: 70_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 7.480703996850347e-55_f64,
                x: 100_f64,
                y: 80_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 3.023853191656425e-58_f64,
                x: 100_f64,
                y: 90_f64,
                f: 0_f64,
            },
            testcase_beta_double_ {
                f0: 2.208760693199185e-61_f64,
                x: 100_f64,
                y: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_28: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub unsafe fn test_29(data: *const [testcase_beta_double_; 10], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 10_();
    let mut i: () = 0_();
    'loop_: while ((i) < (num_datum)) {
        let f: () = unimplemented!();
        let f0: () = (*data)[(i) as usize].f0;
        let diff: () = ((f) - (f0));
        if ((unimplemented!()) > (max_abs_diff)) {
            max_abs_diff = unimplemented!();
        }
        if ((unimplemented!()) > ((10_()) * (eps))) && ((unimplemented!()) > ((10_()) * (eps))) {
            let frac: () = ((diff) / (f0));
            if ((unimplemented!()) > (max_abs_frac)) {
                max_abs_frac = unimplemented!();
            }
        }
        i.prefix_inc();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((max_abs_frac) < (toler)) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe {
        test_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_9)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_10)),
        )
    });
    (unsafe {
        test_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_11)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_12)),
        )
    });
    (unsafe {
        test_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_13)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_14)),
        )
    });
    (unsafe {
        test_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_15)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_16)),
        )
    });
    (unsafe {
        test_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_17)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_18)),
        )
    });
    (unsafe {
        test_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_19)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_20)),
        )
    });
    (unsafe {
        test_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_21)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_22)),
        )
    });
    (unsafe {
        test_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_23)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_24)),
        )
    });
    (unsafe {
        test_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_25)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_26)),
        )
    });
    (unsafe {
        test_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_27)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_28)),
        )
    });
    return 0_();
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: unimplemented!(),
        };
        this.pid_ = unimplemented!();
        this.sem_set_ = unimplemented!();
        if ((this.sem_set_) == (-1_())) {
            unimplemented!();
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
        this
    }
}
pub unsafe fn set_memory_limits_30(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) * (1048576_()));
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn set_file_limit_31(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_32(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = unimplemented!();
    if !(demangled).is_null() {
        s = (demangled).cast_const();
    } else {
        switch!(match status {
            __v if __v == 0_() => {
                s = c"error code = 0: success".as_ptr();
                break;
            }
            __v if __v == -1_() => {
                s = c"error code = -1: memory allocation failure".as_ptr();
                break;
            }
            __v if __v == -2_() => {
                s = c"error code = -2: invalid mangled name".as_ptr();
                break;
            }
            __v if __v == -3_() => {
                s = c"error code = -3: invalid arguments".as_ptr();
                break;
            }
            _ => {
                s = c"error code unknown - who knows what happened".as_ptr();
            }
        });
    }
    let mut w: () = unimplemented!();
    if !(unimplemented!()) {
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_33(mut name: *const (), l: *const __gnu_test_func_callback) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut res: *const () = (unimplemented!()).cast_const();
    if !(res).is_null() {
        let mut preLC_ALL: () = unimplemented!();
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn run_tests_wrapped_env_34(
    mut name: *const (),
    mut env: *const (),
    l: *const __gnu_test_func_callback,
) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut oldENV: *const () = (unimplemented!()).cast_const();
    if !(unimplemented!() != 0) {
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        unimplemented!();
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut _M_count_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut next_id__6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union __gnu_test_semun {
    pub val: (),
    pub buf: *mut (),
    pub array: *mut (),
}
impl Default for __gnu_test_semun {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl __gnu_test_semaphore {}
impl __gnu_test_semaphore {
    pub unsafe fn destructor(&mut self) {
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((self.pid_) == (unimplemented!())) {
            unimplemented!();
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [(); 1] = [unimplemented!()];
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [(); 1] = [unimplemented!()];
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
    }
}
pub unsafe fn test_tm_35(
    mut sec: (),
    mut min: (),
    mut hour: (),
    mut mday: (),
    mut mon: (),
    mut year: (),
    mut wday: (),
    mut yday: (),
    mut isdst: (),
) -> () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_36)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_36)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_36)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_36)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_36)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_36)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_36)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_36)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_36)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_36)).clone();
}
static mut tmp_36: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const data001_9);
    std::cell::LazyCell::force(&*&raw const toler001_10);
    std::cell::LazyCell::force(&*&raw const data002_11);
    std::cell::LazyCell::force(&*&raw const toler002_12);
    std::cell::LazyCell::force(&*&raw const data003_13);
    std::cell::LazyCell::force(&*&raw const toler003_14);
    std::cell::LazyCell::force(&*&raw const data004_15);
    std::cell::LazyCell::force(&*&raw const toler004_16);
    std::cell::LazyCell::force(&*&raw const data005_17);
    std::cell::LazyCell::force(&*&raw const toler005_18);
    std::cell::LazyCell::force(&*&raw const data006_19);
    std::cell::LazyCell::force(&*&raw const toler006_20);
    std::cell::LazyCell::force(&*&raw const data007_21);
    std::cell::LazyCell::force(&*&raw const toler007_22);
    std::cell::LazyCell::force(&*&raw const data008_23);
    std::cell::LazyCell::force(&*&raw const toler008_24);
    std::cell::LazyCell::force(&*&raw const data009_25);
    std::cell::LazyCell::force(&*&raw const toler009_26);
    std::cell::LazyCell::force(&*&raw const data010_27);
    std::cell::LazyCell::force(&*&raw const toler010_28);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
