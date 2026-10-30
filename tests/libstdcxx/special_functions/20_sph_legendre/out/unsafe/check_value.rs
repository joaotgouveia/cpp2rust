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
pub struct testcase_sph_legendre_double_ {
    pub f0: (),
    pub l: (),
    pub m: (),
    pub theta: (),
    pub f: (),
}
pub static mut data001_9: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28209479177387814_f64,
                l: 0_(),
                m: 0_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_11: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0.4886025119029199_f64,
                l: 1_(),
                m: 0_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.48258700419201095_f64,
                l: 1_(),
                m: 0_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.46468860282345237_f64,
                l: 1_(),
                m: 0_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.43534802584032634_f64,
                l: 1_(),
                m: 0_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3952877356237497_f64,
                l: 1_(),
                m: 0_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3454941494713355_f64,
                l: 1_(),
                m: 0_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28719335072959395_f64,
                l: 1_(),
                m: 0_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.22182089855280449_f64,
                l: 1_(),
                m: 0_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.15098647967228981_f64,
                l: 1_(),
                m: 0_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.07643427256684635_f64,
                l: 1_(),
                m: 0_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 2.991827511286333e-17_f64,
                l: 1_(),
                m: 0_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.07643427256684629_f64,
                l: 1_(),
                m: 0_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.15098647967228976_f64,
                l: 1_(),
                m: 0_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.22182089855280446_f64,
                l: 1_(),
                m: 0_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.28719335072959384_f64,
                l: 1_(),
                m: 0_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.34549414947133544_f64,
                l: 1_(),
                m: 0_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.39528773562374975_f64,
                l: 1_(),
                m: 0_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4353480258403263_f64,
                l: 1_(),
                m: 0_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4646886028234523_f64,
                l: 1_(),
                m: 0_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.48258700419201095_f64,
                l: 1_(),
                m: 0_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4886025119029199_f64,
                l: 1_(),
                m: 0_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_13: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 1_(),
                m: 1_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.05404719244707817_f64,
                l: 1_(),
                m: 1_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.10676356364376091_f64,
                l: 1_(),
                m: 1_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.1568510615755813_f64,
                l: 1_(),
                m: 1_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.20307636581258248_f64,
                l: 1_(),
                m: 1_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.24430125595146007_f64,
                l: 1_(),
                m: 1_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2795106383794288_f64,
                l: 1_(),
                m: 1_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3078375412478712_f64,
                l: 1_(),
                m: 1_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32858446219656556_f64,
                l: 1_(),
                m: 1_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.341240543176672_f64,
                l: 1_(),
                m: 1_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.34549414947133567_f64,
                l: 1_(),
                m: 1_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.341240543176672_f64,
                l: 1_(),
                m: 1_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32858446219656556_f64,
                l: 1_(),
                m: 1_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.30783754124787127_f64,
                l: 1_(),
                m: 1_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2795106383794288_f64,
                l: 1_(),
                m: 1_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.24430125595146013_f64,
                l: 1_(),
                m: 1_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.20307636581258243_f64,
                l: 1_(),
                m: 1_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.1568510615755814_f64,
                l: 1_(),
                m: 1_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.10676356364376104_f64,
                l: 1_(),
                m: 1_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.05404719244707817_f64,
                l: 1_(),
                m: 1_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 1_(),
                m: 1_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data004_15: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0.6307831305050401_f64,
                l: 2_(),
                m: 0_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6076285876031658_f64,
                l: 2_(),
                m: 0_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.540431486883966_f64,
                l: 2_(),
                m: 0_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4357695487555659_f64,
                l: 2_(),
                m: 0_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3038878129445757_f64,
                l: 2_(),
                m: 0_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.1576957826262601_f64,
                l: 2_(),
                m: 0_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.011503752307944374_f64,
                l: 2_(),
                m: 0_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.1203779835030457_f64,
                l: 2_(),
                m: 0_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.22503992163144576_f64,
                l: 2_(),
                m: 0_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.29223702235064597_f64,
                l: 2_(),
                m: 0_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31539156525252005_f64,
                l: 2_(),
                m: 0_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.29223702235064597_f64,
                l: 2_(),
                m: 0_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.22503992163144584_f64,
                l: 2_(),
                m: 0_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.12037798350304577_f64,
                l: 2_(),
                m: 0_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.011503752307944164_f64,
                l: 2_(),
                m: 0_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.15769578262625994_f64,
                l: 2_(),
                m: 0_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3038878129445758_f64,
                l: 2_(),
                m: 0_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4357695487555656_f64,
                l: 2_(),
                m: 0_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5404314868839657_f64,
                l: 2_(),
                m: 0_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6076285876031658_f64,
                l: 2_(),
                m: 0_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6307831305050401_f64,
                l: 2_(),
                m: 0_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_17: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 1_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.11936529291378781_f64,
                l: 2_(),
                m: 1_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2270462792902742_f64,
                l: 2_(),
                m: 1_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31250239392538215_f64,
                l: 2_(),
                m: 1_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.36736859691086526_f64,
                l: 2_(),
                m: 1_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3862742020231898_f64,
                l: 2_(),
                m: 1_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3673685969108653_f64,
                l: 2_(),
                m: 1_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31250239392538226_f64,
                l: 2_(),
                m: 1_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.22704627929027438_f64,
                l: 2_(),
                m: 1_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.1193652929137874_f64,
                l: 2_(),
                m: 1_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -4.730494651008968e-17_f64,
                l: 2_(),
                m: 1_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.11936529291378731_f64,
                l: 2_(),
                m: 1_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.2270462792902743_f64,
                l: 2_(),
                m: 1_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.31250239392538226_f64,
                l: 2_(),
                m: 1_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3673685969108652_f64,
                l: 2_(),
                m: 1_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3862742020231898_f64,
                l: 2_(),
                m: 1_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.36736859691086526_f64,
                l: 2_(),
                m: 1_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3125023939253823_f64,
                l: 2_(),
                m: 1_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.2270462792902745_f64,
                l: 2_(),
                m: 1_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.11936529291378781_f64,
                l: 2_(),
                m: 1_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 1_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data006_19: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 2_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.009452802556162349_f64,
                l: 2_(),
                m: 2_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.03688590404890373_f64,
                l: 2_(),
                m: 2_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.07961396136645768_f64,
                l: 2_(),
                m: 2_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.13345445455470126_f64,
                l: 2_(),
                m: 2_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.1931371010115949_f64,
                l: 2_(),
                m: 2_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.2528197474684885_f64,
                l: 2_(),
                m: 2_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3066602406567321_f64,
                l: 2_(),
                m: 2_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.349388297974286_f64,
                l: 2_(),
                m: 2_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.37682139946702753_f64,
                l: 2_(),
                m: 2_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.38627420202318985_f64,
                l: 2_(),
                m: 2_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.37682139946702753_f64,
                l: 2_(),
                m: 2_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.34938829797428606_f64,
                l: 2_(),
                m: 2_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.30666024065673214_f64,
                l: 2_(),
                m: 2_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.25281974746848856_f64,
                l: 2_(),
                m: 2_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.19313710101159498_f64,
                l: 2_(),
                m: 2_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.13345445455470123_f64,
                l: 2_(),
                m: 2_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.07961396136645776_f64,
                l: 2_(),
                m: 2_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.03688590404890381_f64,
                l: 2_(),
                m: 2_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.009452802556162349_f64,
                l: 2_(),
                m: 2_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 2_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data007_21: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0.9356025796273888_f64,
                l: 5_(),
                m: 0_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.7701442294207985_f64,
                l: 5_(),
                m: 0_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3589218503236535_f64,
                l: 5_(),
                m: 0_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.0902149320905943_f64,
                l: 5_(),
                m: 0_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.36214460396518905_f64,
                l: 5_(),
                m: 0_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.35145955579226906_f64,
                l: 5_(),
                m: 0_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.11441703594725193_f64,
                l: 5_(),
                m: 0_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.17248966720808107_f64,
                l: 5_(),
                m: 0_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32128384287200523_f64,
                l: 5_(),
                m: 0_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.24377632246714948_f64,
                l: 5_(),
                m: 0_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.0741712853887687e-16_f64,
                l: 5_(),
                m: 0_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.24377632246714936_f64,
                l: 5_(),
                m: 0_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32128384287200534_f64,
                l: 5_(),
                m: 0_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.17248966720808118_f64,
                l: 5_(),
                m: 0_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.11441703594725151_f64,
                l: 5_(),
                m: 0_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.35145955579226895_f64,
                l: 5_(),
                m: 0_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3621446039651888_f64,
                l: 5_(),
                m: 0_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.09021493209059475_f64,
                l: 5_(),
                m: 0_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.35892185032365165_f64,
                l: 5_(),
                m: 0_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.7701442294207985_f64,
                l: 5_(),
                m: 0_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.9356025796273888_f64,
                l: 5_(),
                m: 0_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_23: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 1_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.367123737133184_f64,
                l: 5_(),
                m: 1_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.5461032901053471_f64,
                l: 5_(),
                m: 1_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4538199149363176_f64,
                l: 5_(),
                m: 1_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.15679720635769906_f64,
                l: 5_(),
                m: 1_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.169854994198386_f64,
                l: 5_(),
                m: 1_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3446800449972517_f64,
                l: 5_(),
                m: 1_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28349471119605985_f64,
                l: 5_(),
                m: 1_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.044286619339675815_f64,
                l: 5_(),
                m: 1_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2119378417719347_f64,
                l: 5_(),
                m: 1_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32028164857621527_f64,
                l: 5_(),
                m: 1_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.21193784177193487_f64,
                l: 5_(),
                m: 1_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.04428661933967559_f64,
                l: 5_(),
                m: 1_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28349471119605973_f64,
                l: 5_(),
                m: 1_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.34468004499725174_f64,
                l: 5_(),
                m: 1_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.1698549941983864_f64,
                l: 5_(),
                m: 1_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.15679720635769961_f64,
                l: 5_(),
                m: 1_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4538199149363177_f64,
                l: 5_(),
                m: 1_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.5461032901053475_f64,
                l: 5_(),
                m: 1_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.367123737133184_f64,
                l: 5_(),
                m: 1_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 1_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data009_25: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 2_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.07891944174554691_f64,
                l: 5_(),
                m: 2_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.2637379914043795_f64,
                l: 5_(),
                m: 2_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.43002359842080096_f64,
                l: 5_(),
                m: 2_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.45642486439050983_f64,
                l: 5_(),
                m: 2_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.299596049060833_f64,
                l: 5_(),
                m: 2_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.02378123984953251_f64,
                l: 5_(),
                m: 2_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.23313989334673826_f64,
                l: 5_(),
                m: 2_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.33799912776303714_f64,
                l: 5_(),
                m: 2_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.23964508489529746_f64,
                l: 5_(),
                m: 2_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -1.0377480524338157e-16_f64,
                l: 5_(),
                m: 2_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.23964508489529732_f64,
                l: 5_(),
                m: 2_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.33799912776303714_f64,
                l: 5_(),
                m: 2_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.23313989334673843_f64,
                l: 5_(),
                m: 2_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.023781239849531916_f64,
                l: 5_(),
                m: 2_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2995960490608327_f64,
                l: 5_(),
                m: 2_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.45642486439050983_f64,
                l: 5_(),
                m: 2_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4300235984208012_f64,
                l: 5_(),
                m: 2_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2637379914043798_f64,
                l: 5_(),
                m: 2_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.07891944174554691_f64,
                l: 5_(),
                m: 2_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 2_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_26: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data010_27: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 5_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -4.348143909791015e-05_f64,
                l: 5_(),
                m: 5_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.001307836708643174_f64,
                l: 5_(),
                m: 5_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.008951081819192276_f64,
                l: 5_(),
                m: 5_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.03256380377757391_f64,
                l: 5_(),
                m: 5_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.08204775710502124_f64,
                l: 5_(),
                m: 5_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.16085328164143806_f64,
                l: 5_(),
                m: 5_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2606430343664538_f64,
                l: 5_(),
                m: 5_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3611381179082057_f64,
                l: 5_(),
                m: 5_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4362559245944614_f64,
                l: 5_(),
                m: 5_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4641322034408581_f64,
                l: 5_(),
                m: 5_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.43625592459446144_f64,
                l: 5_(),
                m: 5_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.36113811790820577_f64,
                l: 5_(),
                m: 5_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2606430343664538_f64,
                l: 5_(),
                m: 5_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.16085328164143822_f64,
                l: 5_(),
                m: 5_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.08204775710502131_f64,
                l: 5_(),
                m: 5_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.032563803777573896_f64,
                l: 5_(),
                m: 5_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.0089510818191923_f64,
                l: 5_(),
                m: 5_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.0013078367086431812_f64,
                l: 5_(),
                m: 5_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -4.348143909791015e-05_f64,
                l: 5_(),
                m: 5_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 5_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_28: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data011_29: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 1.2927207364566027_f64,
                l: 10_(),
                m: 0_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5528889515052201_f64,
                l: 10_(),
                m: 0_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4487442837971133_f64,
                l: 10_(),
                m: 0_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.25532095827149687_f64,
                l: 10_(),
                m: 0_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.36625249688013994_f64,
                l: 10_(),
                m: 0_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.14880806329084206_f64,
                l: 10_(),
                m: 0_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3353335679784874_f64,
                l: 10_(),
                m: 0_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.08063996766233567_f64,
                l: 10_(),
                m: 0_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3219798645017452_f64,
                l: 10_(),
                m: 0_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.025713542103667848_f64,
                l: 10_(),
                m: 0_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31813049373736707_f64,
                l: 10_(),
                m: 0_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.02571354210366753_f64,
                l: 10_(),
                m: 0_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3219798645017454_f64,
                l: 10_(),
                m: 0_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.08063996766233542_f64,
                l: 10_(),
                m: 0_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.33533356797848757_f64,
                l: 10_(),
                m: 0_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.14880806329084156_f64,
                l: 10_(),
                m: 0_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3662524968801396_f64,
                l: 10_(),
                m: 0_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.25532095827149576_f64,
                l: 10_(),
                m: 0_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.44874428379711545_f64,
                l: 10_(),
                m: 0_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5528889515052201_f64,
                l: 10_(),
                m: 0_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.2927207364566027_f64,
                l: 10_(),
                m: 0_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_30: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data012_31: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 1_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.7437372391906405_f64,
                l: 10_(),
                m: 1_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2903511045620965_f64,
                l: 10_(),
                m: 1_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4221928207527153_f64,
                l: 10_(),
                m: 1_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.1710925689893116_f64,
                l: 10_(),
                m: 1_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3558357464854428_f64,
                l: 10_(),
                m: 1_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.10089212303544023_f64,
                l: 10_(),
                m: 1_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32997652649321085_f64,
                l: 10_(),
                m: 1_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.04741637689003294_f64,
                l: 10_(),
                m: 1_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3199935675029566_f64,
                l: 10_(),
                m: 1_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -2.0430664782290742e-16_f64,
                l: 10_(),
                m: 1_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3199935675029566_f64,
                l: 10_(),
                m: 1_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.047416376890032544_f64,
                l: 10_(),
                m: 1_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32997652649321096_f64,
                l: 10_(),
                m: 1_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.10089212303543935_f64,
                l: 10_(),
                m: 1_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3558357464854429_f64,
                l: 10_(),
                m: 1_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.1710925689893127_f64,
                l: 10_(),
                m: 1_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4221928207527157_f64,
                l: 10_(),
                m: 1_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.290351104562096_f64,
                l: 10_(),
                m: 1_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.7437372391906405_f64,
                l: 10_(),
                m: 1_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 1_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_32: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data013_33: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 2_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.34571695599980556_f64,
                l: 10_(),
                m: 2_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6248553597819815_f64,
                l: 10_(),
                m: 2_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.09821003964471625_f64,
                l: 10_(),
                m: 2_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.41494799233049695_f64,
                l: 10_(),
                m: 2_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.08169897383147273_f64,
                l: 10_(),
                m: 2_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3525313222227128_f64,
                l: 10_(),
                m: 2_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.04902629855598098_f64,
                l: 10_(),
                m: 2_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3279124687413079_f64,
                l: 10_(),
                m: 2_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.01619678243394687_f64,
                l: 10_(),
                m: 2_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32106263400438334_f64,
                l: 10_(),
                m: 2_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.016196782433946497_f64,
                l: 10_(),
                m: 2_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.327912468741308_f64,
                l: 10_(),
                m: 2_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.04902629855598073_f64,
                l: 10_(),
                m: 2_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.35253132222271266_f64,
                l: 10_(),
                m: 2_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.08169897383147212_f64,
                l: 10_(),
                m: 2_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.41494799233049684_f64,
                l: 10_(),
                m: 2_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.0982100396447152_f64,
                l: 10_(),
                m: 2_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6248553597819808_f64,
                l: 10_(),
                m: 2_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.34571695599980556_f64,
                l: 10_(),
                m: 2_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 2_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_34: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data014_35: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 5_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.0030300124052750873_f64,
                l: 10_(),
                m: 5_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.07034858524805651_f64,
                l: 10_(),
                m: 5_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3005502929070364_f64,
                l: 10_(),
                m: 5_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4998781814400914_f64,
                l: 10_(),
                m: 5_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2810877175715011_f64,
                l: 10_(),
                m: 5_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.22068081187249255_f64,
                l: 10_(),
                m: 5_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3368950221259212_f64,
                l: 10_(),
                m: 5_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.08609551552076411_f64,
                l: 10_(),
                m: 5_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3393582731851156_f64,
                l: 10_(),
                m: 5_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -1.9213014340664546e-16_f64,
                l: 10_(),
                m: 5_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3393582731851156_f64,
                l: 10_(),
                m: 5_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.08609551552076451_f64,
                l: 10_(),
                m: 5_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.33689502212592104_f64,
                l: 10_(),
                m: 5_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.22068081187249344_f64,
                l: 10_(),
                m: 5_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.28108771757150064_f64,
                l: 10_(),
                m: 5_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.49987818144009155_f64,
                l: 10_(),
                m: 5_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3005502929070367_f64,
                l: 10_(),
                m: 5_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.0703485852480568_f64,
                l: 10_(),
                m: 5_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.0030300124052750873_f64,
                l: 10_(),
                m: 5_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 5_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_36: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data015_37: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 10_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.762428273334567e-09_f64,
                l: 10_(),
                m: 10_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.3085156534549315e-06_f64,
                l: 10_(),
                m: 10_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.00020182347649472387_f64,
                l: 10_(),
                m: 10_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.0026711045506511706_f64,
                l: 10_(),
                m: 10_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.01695719662325691_f64,
                l: 10_(),
                m: 10_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.06517491600499027_f64,
                l: 10_(),
                m: 10_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.17112476903017845_f64,
                l: 10_(),
                m: 10_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32852414199733554_f64,
                l: 10_(),
                m: 10_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.47940582314838287_f64,
                l: 10_(),
                m: 10_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5426302919442215_f64,
                l: 10_(),
                m: 10_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4794058231483829_f64,
                l: 10_(),
                m: 10_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3285241419973357_f64,
                l: 10_(),
                m: 10_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.1711247690301785_f64,
                l: 10_(),
                m: 10_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.06517491600499041_f64,
                l: 10_(),
                m: 10_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.016957196623256943_f64,
                l: 10_(),
                m: 10_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.0026711045506511684_f64,
                l: 10_(),
                m: 10_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.00020182347649472493_f64,
                l: 10_(),
                m: 10_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.308515653454977e-06_f64,
                l: 10_(),
                m: 10_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.762428273334567e-09_f64,
                l: 10_(),
                m: 10_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 10_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler015_38: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data016_39: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 1.8062879984608917_f64,
                l: 20_(),
                m: 0_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.5890654929141673_f64,
                l: 20_(),
                m: 0_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.45624611402342063_f64,
                l: 20_(),
                m: 0_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.39955402700466724_f64,
                l: 20_(),
                m: 0_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.36818552901640805_f64,
                l: 20_(),
                m: 0_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.34873131330857743_f64,
                l: 20_(),
                m: 0_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3360088282918647_f64,
                l: 20_(),
                m: 0_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3275928630812293_f64,
                l: 20_(),
                m: 0_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32222458068091325_f64,
                l: 20_(),
                m: 0_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31922731037135965_f64,
                l: 20_(),
                m: 0_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.31826262039531755_f64,
                l: 20_(),
                m: 0_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3192273103713597_f64,
                l: 20_(),
                m: 0_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3222245806809134_f64,
                l: 20_(),
                m: 0_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32759286308122926_f64,
                l: 20_(),
                m: 0_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3360088282918652_f64,
                l: 20_(),
                m: 0_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3487313133085778_f64,
                l: 20_(),
                m: 0_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3681855290164077_f64,
                l: 20_(),
                m: 0_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.39955402700466824_f64,
                l: 20_(),
                m: 0_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4562461140234241_f64,
                l: 20_(),
                m: 0_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.5890654929141673_f64,
                l: 20_(),
                m: 0_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.8062879984608917_f64,
                l: 20_(),
                m: 0_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler016_40: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data017_41: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 1_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.45905213045059046_f64,
                l: 20_(),
                m: 1_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.31166370423309414_f64,
                l: 20_(),
                m: 1_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.23278757741246778_f64,
                l: 20_(),
                m: 1_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.17937240823503983_f64,
                l: 20_(),
                m: 1_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.13857299972299839_f64,
                l: 20_(),
                m: 1_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.10495324841927815_f64,
                l: 20_(),
                m: 1_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.07570777435216366_f64,
                l: 20_(),
                m: 1_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.049168697683476224_f64,
                l: 20_(),
                m: 1_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.024216050551253303_f64,
                l: 20_(),
                m: 1_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 3.993844351069431e-16_f64,
                l: 20_(),
                m: 1_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.02421605055125238_f64,
                l: 20_(),
                m: 1_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.04916869768347548_f64,
                l: 20_(),
                m: 1_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.07570777435216335_f64,
                l: 20_(),
                m: 1_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.10495324841927638_f64,
                l: 20_(),
                m: 1_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.13857299972299741_f64,
                l: 20_(),
                m: 1_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.17937240823504172_f64,
                l: 20_(),
                m: 1_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.23278757741246703_f64,
                l: 20_(),
                m: 1_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31166370423309253_f64,
                l: 20_(),
                m: 1_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.45905213045059046_f64,
                l: 20_(),
                m: 1_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 1_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler017_42: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data018_43: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 2_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.873998051415742_f64,
                l: 20_(),
                m: 2_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.5511685408089491_f64,
                l: 20_(),
                m: 2_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4452013730855757_f64,
                l: 20_(),
                m: 2_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3932163787790828_f64,
                l: 20_(),
                m: 2_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.36312025711350954_f64,
                l: 20_(),
                m: 2_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3442710300487309_f64,
                l: 20_(),
                m: 2_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3321491763838764_f64,
                l: 20_(),
                m: 2_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3245573444883909_f64,
                l: 20_(),
                m: 2_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32036529628513255_f64,
                l: 20_(),
                m: 2_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31902310563819986_f64,
                l: 20_(),
                m: 2_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3203652962851326_f64,
                l: 20_(),
                m: 2_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.324557344488391_f64,
                l: 20_(),
                m: 2_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3321491763838766_f64,
                l: 20_(),
                m: 2_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.34427103004873105_f64,
                l: 20_(),
                m: 2_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3631202571135098_f64,
                l: 20_(),
                m: 2_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3932163787790823_f64,
                l: 20_(),
                m: 2_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4452013730855765_f64,
                l: 20_(),
                m: 2_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.5511685408089504_f64,
                l: 20_(),
                m: 2_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.873998051415742_f64,
                l: 20_(),
                m: 2_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 2_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler018_44: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data019_45: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 5_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.10024848623505046_f64,
                l: 20_(),
                m: 5_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.6811536107594065_f64,
                l: 20_(),
                m: 5_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.317745325511563_f64,
                l: 20_(),
                m: 5_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.16011868165390658_f64,
                l: 20_(),
                m: 5_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.08584414330411558_f64,
                l: 20_(),
                m: 5_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.0474675408408638_f64,
                l: 20_(),
                m: 5_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.026283575189471796_f64,
                l: 20_(),
                m: 5_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.013891104052597688_f64,
                l: 20_(),
                m: 5_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.005987330823949696_f64,
                l: 20_(),
                m: 5_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 3.935528658208305e-16_f64,
                l: 20_(),
                m: 5_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.005987330823950332_f64,
                l: 20_(),
                m: 5_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.013891104052598547_f64,
                l: 20_(),
                m: 5_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.026283575189472153_f64,
                l: 20_(),
                m: 5_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.04746754084086593_f64,
                l: 20_(),
                m: 5_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.08584414330411701_f64,
                l: 20_(),
                m: 5_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.16011868165390544_f64,
                l: 20_(),
                m: 5_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3177453255115638_f64,
                l: 20_(),
                m: 5_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6811536107594048_f64,
                l: 20_(),
                m: 5_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.10024848623505046_f64,
                l: 20_(),
                m: 5_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 5_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler019_46: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data020_47: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 10_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 3.059579760370789e-05_f64,
                l: 20_(),
                m: 10_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.015924453916396863_f64,
                l: 20_(),
                m: 10_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.26588079118745744_f64,
                l: 20_(),
                m: 10_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5404508142068674_f64,
                l: 20_(),
                m: 10_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2821527939428553_f64,
                l: 20_(),
                m: 10_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.008529733758223922_f64,
                l: 20_(),
                m: 10_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.16930127953533738_f64,
                l: 20_(),
                m: 10_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.27215134048018325_f64,
                l: 20_(),
                m: 10_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32456597088029526_f64,
                l: 20_(),
                m: 10_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.34057893241353715_f64,
                l: 20_(),
                m: 10_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32456597088029493_f64,
                l: 20_(),
                m: 10_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2721513404801829_f64,
                l: 20_(),
                m: 10_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.16930127953533702_f64,
                l: 20_(),
                m: 10_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.008529733758225744_f64,
                l: 20_(),
                m: 10_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2821527939428562_f64,
                l: 20_(),
                m: 10_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5404508142068687_f64,
                l: 20_(),
                m: 10_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.2658807911874583_f64,
                l: 20_(),
                m: 10_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.015924453916397_f64,
                l: 20_(),
                m: 10_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 3.059579760370789e-05_f64,
                l: 20_(),
                m: 10_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 10_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler020_48: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data021_49: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 20_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.9264471419250786e-17_f64,
                l: 20_(),
                m: 20_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.032109168153092e-11_f64,
                l: 20_(),
                m: 20_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 8.847494418447166e-08_f64,
                l: 20_(),
                m: 20_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.549739512938779e-05_f64,
                l: 20_(),
                m: 20_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.000624575642829845_f64,
                l: 20_(),
                m: 20_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.00922651924589673_f64,
                l: 20_(),
                m: 20_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.0636066732363233_f64,
                l: 20_(),
                m: 20_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.23442909509776316_f64,
                l: 20_(),
                m: 20_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4992103048108701_f64,
                l: 20_(),
                m: 20_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6395654582577622_f64,
                l: 20_(),
                m: 20_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.49921030481087025_f64,
                l: 20_(),
                m: 20_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.23442909509776336_f64,
                l: 20_(),
                m: 20_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.06360667323632332_f64,
                l: 20_(),
                m: 20_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.009226519245896774_f64,
                l: 20_(),
                m: 20_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.0006245756428298477_f64,
                l: 20_(),
                m: 20_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.5497395129387764e-05_f64,
                l: 20_(),
                m: 20_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 8.847494418447262e-08_f64,
                l: 20_(),
                m: 20_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.032109168153178e-11_f64,
                l: 20_(),
                m: 20_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.9264471419250786e-17_f64,
                l: 20_(),
                m: 20_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 20_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler021_50: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data022_51: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 2.8350175706934717_f64,
                l: 50_(),
                m: 0_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5315753749517276_f64,
                l: 50_(),
                m: 0_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4605618347630056_f64,
                l: 50_(),
                m: 0_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2487603207967791_f64,
                l: 50_(),
                m: 0_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.36926172901532667_f64,
                l: 50_(),
                m: 0_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.14571730283563575_f64,
                l: 50_(),
                m: 0_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3363619917085075_f64,
                l: 50_(),
                m: 0_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.07913271626709151_f64,
                l: 50_(),
                m: 0_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3223292194130145_f64,
                l: 50_(),
                m: 0_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.025253991969481544_f64,
                l: 50_(),
                m: 0_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3183020872415236_f64,
                l: 50_(),
                m: 0_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.02525399196947988_f64,
                l: 50_(),
                m: 0_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3223292194130148_f64,
                l: 50_(),
                m: 0_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.07913271626709006_f64,
                l: 50_(),
                m: 0_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.33636199170850883_f64,
                l: 50_(),
                m: 0_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.14571730283563347_f64,
                l: 50_(),
                m: 0_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3692617290153252_f64,
                l: 50_(),
                m: 0_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.24876032079677354_f64,
                l: 50_(),
                m: 0_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.46056183476301255_f64,
                l: 50_(),
                m: 0_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5315753749517276_f64,
                l: 50_(),
                m: 0_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 2.8350175706934717_f64,
                l: 50_(),
                m: 0_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler022_52: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data023_53: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 1_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.6375175215522825_f64,
                l: 50_(),
                m: 1_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32616619317605133_f64,
                l: 50_(),
                m: 1_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4064993082616285_f64,
                l: 50_(),
                m: 1_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.18473991408343635_f64,
                l: 50_(),
                m: 1_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.35083930302013117_f64,
                l: 50_(),
                m: 1_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.10755382110947342_f64,
                l: 50_(),
                m: 1_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.328225683164999_f64,
                l: 50_(),
                m: 1_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.05028605660979739_f64,
                l: 50_(),
                m: 1_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31935368562159644_f64,
                l: 50_(),
                m: 1_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -9.842160268619584e-16_f64,
                l: 50_(),
                m: 1_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3193536856215965_f64,
                l: 50_(),
                m: 1_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.050286056609795446_f64,
                l: 50_(),
                m: 1_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3282256831649992_f64,
                l: 50_(),
                m: 1_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.10755382110946902_f64,
                l: 50_(),
                m: 1_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.35083930302013205_f64,
                l: 50_(),
                m: 1_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.18473991408344057_f64,
                l: 50_(),
                m: 1_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4064993082616301_f64,
                l: 50_(),
                m: 1_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32616619317604606_f64,
                l: 50_(),
                m: 1_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6375175215522825_f64,
                l: 50_(),
                m: 1_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 1_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler023_54: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data024_55: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 2_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3723026116383626_f64,
                l: 50_(),
                m: 2_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5005159968031552_f64,
                l: 50_(),
                m: 2_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.21724795180329495_f64,
                l: 50_(),
                m: 2_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.37948127307611107_f64,
                l: 50_(),
                m: 2_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.13187372121003413_f64,
                l: 50_(),
                m: 2_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3395900916240013_f64,
                l: 50_(),
                m: 2_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.07253750311248956_f64,
                l: 50_(),
                m: 2_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32310306941855266_f64,
                l: 50_(),
                m: 2_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.023259822816436636_f64,
                l: 50_(),
                m: 2_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.31842698506357275_f64,
                l: 50_(),
                m: 2_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.023259822816434638_f64,
                l: 50_(),
                m: 2_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32310306941855316_f64,
                l: 50_(),
                m: 2_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.07253750311248841_f64,
                l: 50_(),
                m: 2_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.33959009162400267_f64,
                l: 50_(),
                m: 2_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.13187372121003124_f64,
                l: 50_(),
                m: 2_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.37948127307610924_f64,
                l: 50_(),
                m: 2_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.2172479518032909_f64,
                l: 50_(),
                m: 2_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.500515996803161_f64,
                l: 50_(),
                m: 2_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3723026116383626_f64,
                l: 50_(),
                m: 2_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 2_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler024_56: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data025_57: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 5_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.577503859031916_f64,
                l: 50_(),
                m: 5_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.07736049706557029_f64,
                l: 50_(),
                m: 5_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.47707267400540226_f64,
                l: 50_(),
                m: 5_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.05537061512662681_f64,
                l: 50_(),
                m: 5_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3762945184720283_f64,
                l: 50_(),
                m: 5_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.048042277801963115_f64,
                l: 50_(),
                m: 5_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3361937936222872_f64,
                l: 50_(),
                m: 5_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.025265227185718764_f64,
                l: 50_(),
                m: 5_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32083679430964535_f64,
                l: 50_(),
                m: 5_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -9.818920101975173e-16_f64,
                l: 50_(),
                m: 5_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32083679430964546_f64,
                l: 50_(),
                m: 5_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.02526522718571679_f64,
                l: 50_(),
                m: 5_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3361937936222873_f64,
                l: 50_(),
                m: 5_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.048042277801958064_f64,
                l: 50_(),
                m: 5_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3762945184720287_f64,
                l: 50_(),
                m: 5_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.05537061512663052_f64,
                l: 50_(),
                m: 5_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.47707267400540176_f64,
                l: 50_(),
                m: 5_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.07736049706558863_f64,
                l: 50_(),
                m: 5_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.577503859031916_f64,
                l: 50_(),
                m: 5_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 5_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler025_58: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-12_f64 });
pub static mut data026_59: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 10_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.15606941844801256_f64,
                l: 50_(),
                m: 10_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.5374886883681417_f64,
                l: 50_(),
                m: 10_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4930491902518397_f64,
                l: 50_(),
                m: 10_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2626758275042792_f64,
                l: 50_(),
                m: 10_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.22058983666314153_f64,
                l: 50_(),
                m: 10_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3293672516067184_f64,
                l: 50_(),
                m: 10_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.09205331155944796_f64,
                l: 50_(),
                m: 10_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3254291349593552_f64,
                l: 50_(),
                m: 10_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.02567322378910335_f64,
                l: 50_(),
                m: 10_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32150019350255743_f64,
                l: 50_(),
                m: 10_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.02567322378910526_f64,
                l: 50_(),
                m: 10_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3254291349593551_f64,
                l: 50_(),
                m: 10_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.09205331155944903_f64,
                l: 50_(),
                m: 10_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32936725160671687_f64,
                l: 50_(),
                m: 10_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.2205898366631438_f64,
                l: 50_(),
                m: 10_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.26267582750428364_f64,
                l: 50_(),
                m: 10_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.49304919025184135_f64,
                l: 50_(),
                m: 10_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.537488688368145_f64,
                l: 50_(),
                m: 10_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.15606941844801256_f64,
                l: 50_(),
                m: 10_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 10_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler026_60: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data027_61: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 20_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 3.040959871283589e-07_f64,
                l: 50_(),
                m: 20_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.030940518122881844_f64,
                l: 50_(),
                m: 20_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.641345887216598_f64,
                l: 50_(),
                m: 20_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.29895244392136594_f64,
                l: 50_(),
                m: 20_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.2530932478187387_f64,
                l: 50_(),
                m: 20_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3436863471493157_f64,
                l: 50_(),
                m: 20_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.33996764360663945_f64,
                l: 50_(),
                m: 20_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.12866267745104024_f64,
                l: 50_(),
                m: 20_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.18201114398922874_f64,
                l: 50_(),
                m: 20_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3321668343151086_f64,
                l: 50_(),
                m: 20_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.1820111439892301_f64,
                l: 50_(),
                m: 20_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.12866267745103857_f64,
                l: 50_(),
                m: 20_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.33996764360663895_f64,
                l: 50_(),
                m: 20_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3436863471493181_f64,
                l: 50_(),
                m: 20_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.25309324781874126_f64,
                l: 50_(),
                m: 20_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.29895244392136405_f64,
                l: 50_(),
                m: 20_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6413458872165987_f64,
                l: 50_(),
                m: 20_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.030940518122882274_f64,
                l: 50_(),
                m: 20_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 3.040959871283589e-07_f64,
                l: 50_(),
                m: 20_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 20_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler027_62: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data028_63: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 50_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.1649039898161316e-41_f64,
                l: 50_(),
                m: 50_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 2.5240684647722935e-26_f64,
                l: 50_(),
                m: 50_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 5.6927376423967334e-18_f64,
                l: 50_(),
                m: 50_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 2.3116239814797222e-12_f64,
                l: 50_(),
                m: 50_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 2.3835981241325056e-08_f64,
                l: 50_(),
                m: 50_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.9992410287270217e-05_f64,
                l: 50_(),
                m: 50_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.002494750567082983_f64,
                l: 50_(),
                m: 50_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.06505777464797123_f64,
                l: 50_(),
                m: 50_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.43050607056732243_f64,
                l: 50_(),
                m: 50_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.7998028117153198_f64,
                l: 50_(),
                m: 50_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4305060705673229_f64,
                l: 50_(),
                m: 50_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.0650577746479714_f64,
                l: 50_(),
                m: 50_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.0024947505670829878_f64,
                l: 50_(),
                m: 50_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.999241028727043e-05_f64,
                l: 50_(),
                m: 50_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 2.383598124132531e-08_f64,
                l: 50_(),
                m: 50_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 2.3116239814797057e-12_f64,
                l: 50_(),
                m: 50_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 5.692737642396895e-18_f64,
                l: 50_(),
                m: 50_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 2.5240684647724192e-26_f64,
                l: 50_(),
                m: 50_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.1649039898161316e-41_f64,
                l: 50_(),
                m: 50_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 50_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler028_64: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data029_65: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 3.9993839251484076_f64,
                l: 100_(),
                m: 0_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.6077016028593946_f64,
                l: 100_(),
                m: 0_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4619302788395444_f64,
                l: 100_(),
                m: 0_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.40218718869815234_f64,
                l: 100_(),
                m: 0_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.369602014069111_f64,
                l: 100_(),
                m: 0_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3495372654737839_f64,
                l: 100_(),
                m: 0_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3364695935249773_f64,
                l: 100_(),
                m: 0_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32784733067663224_f64,
                l: 100_(),
                m: 0_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3223562447404797_f64,
                l: 100_(),
                m: 0_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3192933070660135_f64,
                l: 100_(),
                m: 0_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.31830791662110325_f64,
                l: 100_(),
                m: 0_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31929330706601333_f64,
                l: 100_(),
                m: 0_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32235624474048036_f64,
                l: 100_(),
                m: 0_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3278473306766329_f64,
                l: 100_(),
                m: 0_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3364695935249801_f64,
                l: 100_(),
                m: 0_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3495372654737859_f64,
                l: 100_(),
                m: 0_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.36960201406910725_f64,
                l: 100_(),
                m: 0_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4021871886981572_f64,
                l: 100_(),
                m: 0_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4619302788395592_f64,
                l: 100_(),
                m: 0_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.6077016028593946_f64,
                l: 100_(),
                m: 0_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 3.9993839251484076_f64,
                l: 100_(),
                m: 0_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler029_66: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data030_67: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 1_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.5085194901371416_f64,
                l: 100_(),
                m: 1_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3312964140222309_f64,
                l: 100_(),
                m: 1_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.24390405750942512_f64,
                l: 100_(),
                m: 1_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.18659755088413388_f64,
                l: 100_(),
                m: 1_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.14355908970517178_f64,
                l: 100_(),
                m: 1_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.10844906813251505_f64,
                l: 100_(),
                m: 1_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.07810008869085767_f64,
                l: 100_(),
                m: 1_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.05067000299830272_f64,
                l: 100_(),
                m: 1_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.0249412517471389_f64,
                l: 100_(),
                m: 1_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.9587949830851608e-15_f64,
                l: 100_(),
                m: 1_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.024941251747135025_f64,
                l: 100_(),
                m: 1_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.050670002998298824_f64,
                l: 100_(),
                m: 1_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.07810008869085577_f64,
                l: 100_(),
                m: 1_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.10844906813250622_f64,
                l: 100_(),
                m: 1_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.14355908970516626_f64,
                l: 100_(),
                m: 1_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.18659755088414104_f64,
                l: 100_(),
                m: 1_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.2439040575094168_f64,
                l: 100_(),
                m: 1_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3312964140222175_f64,
                l: 100_(),
                m: 1_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5085194901371416_f64,
                l: 100_(),
                m: 1_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 1_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler030_68: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data031_69: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 2_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6716627429719685_f64,
                l: 100_(),
                m: 2_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.48226933687994433_f64,
                l: 100_(),
                m: 2_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.41175421895715525_f64,
                l: 100_(),
                m: 2_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.37475021787822776_f64,
                l: 100_(),
                m: 2_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.35242909383605286_f64,
                l: 100_(),
                m: 2_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.33807110409159863_f64,
                l: 100_(),
                m: 2_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32867180390710077_f64,
                l: 100_(),
                m: 2_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.322715837902785_f64,
                l: 100_(),
                m: 2_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.31940354677687466_f64,
                l: 100_(),
                m: 2_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31833943693772526_f64,
                l: 100_(),
                m: 2_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.31940354677687455_f64,
                l: 100_(),
                m: 2_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3227158379027855_f64,
                l: 100_(),
                m: 2_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.32867180390710116_f64,
                l: 100_(),
                m: 2_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.33807110409160157_f64,
                l: 100_(),
                m: 2_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.35242909383605503_f64,
                l: 100_(),
                m: 2_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3747502178782244_f64,
                l: 100_(),
                m: 2_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4117542189571605_f64,
                l: 100_(),
                m: 2_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.4822693368799536_f64,
                l: 100_(),
                m: 2_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.6716627429719685_f64,
                l: 100_(),
                m: 2_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 2_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler031_70: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data032_71: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 5_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.06256436110595458_f64,
                l: 100_(),
                m: 5_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.14179554455882706_f64,
                l: 100_(),
                m: 5_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.1435686694290596_f64,
                l: 100_(),
                m: 5_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.12355483388447824_f64,
                l: 100_(),
                m: 5_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.10090029999681642_f64,
                l: 100_(),
                m: 5_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.07890513446023524_f64,
                l: 100_(),
                m: 5_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.05804018239818507_f64,
                l: 100_(),
                m: 5_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.038142759389482424_f64,
                l: 100_(),
                m: 5_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.018906264170660478_f64,
                l: 100_(),
                m: 5_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.9576303042914504e-15_f64,
                l: 100_(),
                m: 5_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.01890626417065702_f64,
                l: 100_(),
                m: 5_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.038142759389478365_f64,
                l: 100_(),
                m: 5_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.058040182398183086_f64,
                l: 100_(),
                m: 5_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.0789051344602257_f64,
                l: 100_(),
                m: 5_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.10090029999681013_f64,
                l: 100_(),
                m: 5_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.1235548338844855_f64,
                l: 100_(),
                m: 5_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.14356866942904906_f64,
                l: 100_(),
                m: 5_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.14179554455879767_f64,
                l: 100_(),
                m: 5_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.06256436110595458_f64,
                l: 100_(),
                m: 5_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 5_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler032_72: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data033_73: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 10_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.7536654518799799_f64,
                l: 100_(),
                m: 10_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.35914570017278186_f64,
                l: 100_(),
                m: 10_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.43480692911578295_f64,
                l: 100_(),
                m: 10_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.408621110803155_f64,
                l: 100_(),
                m: 10_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3783268869290941_f64,
                l: 100_(),
                m: 10_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.35484056194773494_f64,
                l: 100_(),
                m: 10_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3382198117119634_f64,
                l: 100_(),
                m: 10_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32729120767830594_f64,
                l: 100_(),
                m: 10_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3211033693709144_f64,
                l: 100_(),
                m: 10_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31910064020036194_f64,
                l: 100_(),
                m: 10_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3211033693709146_f64,
                l: 100_(),
                m: 10_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.32729120767830605_f64,
                l: 100_(),
                m: 10_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.33821981171196364_f64,
                l: 100_(),
                m: 10_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3548405619477346_f64,
                l: 100_(),
                m: 10_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3783268869290937_f64,
                l: 100_(),
                m: 10_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.40862111080315755_f64,
                l: 100_(),
                m: 10_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.4348069291157775_f64,
                l: 100_(),
                m: 10_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.359145700172768_f64,
                l: 100_(),
                m: 10_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.7536654518799799_f64,
                l: 100_(),
                m: 10_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 10_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler033_74: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data034_75: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 20_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.05356966084155708_f64,
                l: 100_(),
                m: 20_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5715492687473134_f64,
                l: 100_(),
                m: 20_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.47536909969585633_f64,
                l: 100_(),
                m: 20_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.28882554564109575_f64,
                l: 100_(),
                m: 20_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.020116179014043743_f64,
                l: 100_(),
                m: 20_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.14752195931707024_f64,
                l: 100_(),
                m: 20_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.24069428588868366_f64,
                l: 100_(),
                m: 20_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.29031796025014306_f64,
                l: 100_(),
                m: 20_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.3143725685114346_f64,
                l: 100_(),
                m: 20_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3215395485114179_f64,
                l: 100_(),
                m: 20_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.31437256851143375_f64,
                l: 100_(),
                m: 20_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.2903179602501414_f64,
                l: 100_(),
                m: 20_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.2406942858886824_f64,
                l: 100_(),
                m: 20_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.14752195931706186_f64,
                l: 100_(),
                m: 20_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.02011617901404956_f64,
                l: 100_(),
                m: 20_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.28882554564109075_f64,
                l: 100_(),
                m: 20_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.47536909969585545_f64,
                l: 100_(),
                m: 20_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.5715492687473235_f64,
                l: 100_(),
                m: 20_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.05356966084155708_f64,
                l: 100_(),
                m: 20_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 20_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler034_76: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data035_77: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 50_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 3.304791039259782e-21_f64,
                l: 100_(),
                m: 50_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.0592655372554564e-07_f64,
                l: 100_(),
                m: 50_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.08041874422395277_f64,
                l: 100_(),
                m: 50_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.5645060058039297_f64,
                l: 100_(),
                m: 50_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3333873984474211_f64,
                l: 100_(),
                m: 50_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.3974171481651468_f64,
                l: 100_(),
                m: 50_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.35223993750972243_f64,
                l: 100_(),
                m: 50_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.17885891940721577_f64,
                l: 100_(),
                m: 50_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.15341660126461967_f64,
                l: 100_(),
                m: 50_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.341759243035031_f64,
                l: 100_(),
                m: 50_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.1534166012646227_f64,
                l: 100_(),
                m: 50_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.17885891940721302_f64,
                l: 100_(),
                m: 50_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.35223993750972227_f64,
                l: 100_(),
                m: 50_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.39741714816514595_f64,
                l: 100_(),
                m: 50_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.33338739844741666_f64,
                l: 100_(),
                m: 50_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: -0.564506005803931_f64,
                l: 100_(),
                m: 50_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.08041874422395391_f64,
                l: 100_(),
                m: 50_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 1.0592655372554981e-07_f64,
                l: 100_(),
                m: 50_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 3.304791039259782e-21_f64,
                l: 100_(),
                m: 50_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 50_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler035_78: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data036_79: std::cell::LazyCell<[testcase_sph_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 100_(),
                theta: 0_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 2.57441366088739e-81_f64,
                l: 100_(),
                m: 100_(),
                theta: 0.15707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 9.455197486894709e-52_f64,
                l: 100_(),
                m: 100_(),
                theta: 0.3141592653589793_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.8096190703397596e-35_f64,
                l: 100_(),
                m: 100_(),
                theta: 0.47123889803846897_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 7.930539363634402e-24_f64,
                l: 100_(),
                m: 100_(),
                theta: 0.6283185307179586_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 8.432074061094486e-16_f64,
                l: 100_(),
                m: 100_(),
                theta: 0.7853981633974483_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 5.931966014602667e-10_f64,
                l: 100_(),
                m: 100_(),
                theta: 0.9424777960769379_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 9.236822594679724e-06_f64,
                l: 100_(),
                m: 100_(),
                theta: 1.0995574287564276_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.0062815489742044095_f64,
                l: 100_(),
                m: 100_(),
                theta: 1.2566370614359172_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.27505966018176986_f64,
                l: 100_(),
                m: 100_(),
                theta: 1.413716694115407_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.9493671399876462_f64,
                l: 100_(),
                m: 100_(),
                theta: 1.5707963267948966_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.27505966018177036_f64,
                l: 100_(),
                m: 100_(),
                theta: 1.7278759594743862_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0.006281548974204443_f64,
                l: 100_(),
                m: 100_(),
                theta: 1.8849555921538759_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 9.236822594679758e-06_f64,
                l: 100_(),
                m: 100_(),
                theta: 2.0420352248333655_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 5.931966014602795e-10_f64,
                l: 100_(),
                m: 100_(),
                theta: 2.199114857512855_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 8.432074061094665e-16_f64,
                l: 100_(),
                m: 100_(),
                theta: 2.356194490192345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 7.930539363634289e-24_f64,
                l: 100_(),
                m: 100_(),
                theta: 2.5132741228718345_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 4.809619070340033e-35_f64,
                l: 100_(),
                m: 100_(),
                theta: 2.670353755551324_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 9.45519748689565e-52_f64,
                l: 100_(),
                m: 100_(),
                theta: 2.827433388230814_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 2.57441366088739e-81_f64,
                l: 100_(),
                m: 100_(),
                theta: 2.9845130209103035_f64,
                f: 0_f64,
            },
            testcase_sph_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 100_(),
                theta: 3.141592653589793_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler036_80: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub unsafe fn test_81(data: *const [testcase_sph_legendre_double_; 21], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 21_();
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
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_9)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_10)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_11)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_12)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_13)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_14)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_15)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_16)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_17)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_18)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_19)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_20)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_21)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_22)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_23)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_24)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_25)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_26)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_27)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_28)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_29)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_30)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_31)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_32)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_33)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_34)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_35)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_36)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_37)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_38)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_39)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_40)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data017_41)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler017_42)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data018_43)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler018_44)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data019_45)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler019_46)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data020_47)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler020_48)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data021_49)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler021_50)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data022_51)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler022_52)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data023_53)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler023_54)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data024_55)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler024_56)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data025_57)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler025_58)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data026_59)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler026_60)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data027_61)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler027_62)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data028_63)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler028_64)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data029_65)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler029_66)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data030_67)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler030_68)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data031_69)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler031_70)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data032_71)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler032_72)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data033_73)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler033_74)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data034_75)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler034_76)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data035_77)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler035_78)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data036_79)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler036_80)),
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
pub unsafe fn set_memory_limits_82(mut size: Option<()>) {
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
pub unsafe fn set_file_limit_83(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_84(mut mangled: *const (), mut wanted: *const ()) {
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
pub unsafe fn run_tests_wrapped_locale_85(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_86(
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
pub unsafe fn test_tm_87(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).clone();
}
static mut tmp_88: std::cell::LazyCell<()> =
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
    std::cell::LazyCell::force(&*&raw const data011_29);
    std::cell::LazyCell::force(&*&raw const toler011_30);
    std::cell::LazyCell::force(&*&raw const data012_31);
    std::cell::LazyCell::force(&*&raw const toler012_32);
    std::cell::LazyCell::force(&*&raw const data013_33);
    std::cell::LazyCell::force(&*&raw const toler013_34);
    std::cell::LazyCell::force(&*&raw const data014_35);
    std::cell::LazyCell::force(&*&raw const toler014_36);
    std::cell::LazyCell::force(&*&raw const data015_37);
    std::cell::LazyCell::force(&*&raw const toler015_38);
    std::cell::LazyCell::force(&*&raw const data016_39);
    std::cell::LazyCell::force(&*&raw const toler016_40);
    std::cell::LazyCell::force(&*&raw const data017_41);
    std::cell::LazyCell::force(&*&raw const toler017_42);
    std::cell::LazyCell::force(&*&raw const data018_43);
    std::cell::LazyCell::force(&*&raw const toler018_44);
    std::cell::LazyCell::force(&*&raw const data019_45);
    std::cell::LazyCell::force(&*&raw const toler019_46);
    std::cell::LazyCell::force(&*&raw const data020_47);
    std::cell::LazyCell::force(&*&raw const toler020_48);
    std::cell::LazyCell::force(&*&raw const data021_49);
    std::cell::LazyCell::force(&*&raw const toler021_50);
    std::cell::LazyCell::force(&*&raw const data022_51);
    std::cell::LazyCell::force(&*&raw const toler022_52);
    std::cell::LazyCell::force(&*&raw const data023_53);
    std::cell::LazyCell::force(&*&raw const toler023_54);
    std::cell::LazyCell::force(&*&raw const data024_55);
    std::cell::LazyCell::force(&*&raw const toler024_56);
    std::cell::LazyCell::force(&*&raw const data025_57);
    std::cell::LazyCell::force(&*&raw const toler025_58);
    std::cell::LazyCell::force(&*&raw const data026_59);
    std::cell::LazyCell::force(&*&raw const toler026_60);
    std::cell::LazyCell::force(&*&raw const data027_61);
    std::cell::LazyCell::force(&*&raw const toler027_62);
    std::cell::LazyCell::force(&*&raw const data028_63);
    std::cell::LazyCell::force(&*&raw const toler028_64);
    std::cell::LazyCell::force(&*&raw const data029_65);
    std::cell::LazyCell::force(&*&raw const toler029_66);
    std::cell::LazyCell::force(&*&raw const data030_67);
    std::cell::LazyCell::force(&*&raw const toler030_68);
    std::cell::LazyCell::force(&*&raw const data031_69);
    std::cell::LazyCell::force(&*&raw const toler031_70);
    std::cell::LazyCell::force(&*&raw const data032_71);
    std::cell::LazyCell::force(&*&raw const toler032_72);
    std::cell::LazyCell::force(&*&raw const data033_73);
    std::cell::LazyCell::force(&*&raw const toler033_74);
    std::cell::LazyCell::force(&*&raw const data034_75);
    std::cell::LazyCell::force(&*&raw const toler034_76);
    std::cell::LazyCell::force(&*&raw const data035_77);
    std::cell::LazyCell::force(&*&raw const toler035_78);
    std::cell::LazyCell::force(&*&raw const data036_79);
    std::cell::LazyCell::force(&*&raw const toler036_80);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
