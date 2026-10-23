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
            (unsafe { __throw_runtime_error_3(c"copy_constructor::mark_call".as_ptr()) });
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
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__4));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__4)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__4))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__5)))
        {
            (unsafe { __throw_runtime_error_3(c"assignment_operator::mark_call".as_ptr()) });
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__4)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__5)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__5)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_6));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_6)).postfix_inc();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_6)) = 0_();
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut next_id__7)).postfix_dec()
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
            if operator_lt_8(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_8(
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
            operator_eq_9(
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
pub unsafe fn operator_eq_9(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        == (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
pub unsafe fn operator_lt_8(
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
    pid_: ::libc::pid_t,
}
impl __gnu_test_semaphore {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct testcase_sph_bessel_double_ {
    pub f0: (),
    pub n: (),
    pub x: (),
    pub f: (),
}
pub static mut data001_10: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.9896158370180917_f64,
                n: 0_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.958851077208406_f64,
                n: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.9088516800311123_f64,
                n: 0_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.8414709848078965_f64,
                n: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.759187695484469_f64,
                n: 0_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.6649966577360363_f64,
                n: 0_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.562277683927964_f64,
                n: 0_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.4546487134128408_f64,
                n: 0_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.345810309727965_f64,
                n: 0_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.23938885764158258_f64,
                n: 0_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.13878581529175696_f64,
                n: 0_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0470400026866224_f64,
                n: 0_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.03329081062464873_f64,
                n: 0_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.10022377933989138_f64,
                n: 0_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.15241635166462503_f64,
                n: 0_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.18920062382698205_f64,
                n: 0_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.21058573134790204_f64,
                n: 0_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.21722891503668823_f64,
                n: 0_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.2103774292579743_f64,
                n: 0_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.1917848549326277_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_12: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.08281366122978806_f64,
                n: 1_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1625370306360665_f64,
                n: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.236217081543055_f64,
                n: 1_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.30116867893975674_f64,
                n: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.3550922664713602_f64,
                n: 1_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.3961729707122223_f64,
                n: 1_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.42315642261568914_f64,
                n: 1_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.43539777497999166_f64,
                n: 1_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.4328817477558685_f64,
                n: 1_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.41621298927540656_f64,
                n: 1_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.3865775250633529_f64,
                n: 1_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.34567749976235596_f64,
                n: 1_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.29564272783258383_f64,
                n: 1_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.23892368798597285_f64,
                n: 1_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1781714681799829_f64,
                n: 1_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.11611074925915747_f64,
                n: 1_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.05541217848609196_f64,
                n: 1_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0014295812457574522_f64,
                n: 1_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.05220622782020018_f64,
                n: 1_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0950894080791708_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data003_14: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.004148097739361125_f64,
                n: 2_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.016371106607993412_f64,
                n: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.036016646141108236_f64,
                n: 2_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.06203505201137386_f64,
                n: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.09303374404679562_f64,
                n: 2_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.12734928368840817_f64,
                n: 2_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1631333262703603_f64,
                n: 2_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1984479490571466_f64,
                n: 2_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.23136535394652627_f64,
                n: 2_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.26006672948890525_f64,
                n: 2_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.2829351211409916_f64,
                n: 2_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.29863749707573356_f64,
                n: 2_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.30619179016241843_f64,
                n: 2_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.3050155118992967_f64,
                n: 2_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.2949535262086113_f64,
                n: 2_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.27628368577135015_f64,
                n: 2_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.24970021027926106_f64,
                n: 2_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.21627586087284995_f64,
                n: 2_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.17740507484521628_f64,
                n: 2_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1347312100851252_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_16: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.371981123726823e-08_f64,
                n: 5_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.9774668754574457e-06_f64,
                n: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.2339447678335765e-05_f64,
                n: 5_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.256115861125816e-05_f64,
                n: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.00027638888920123806_f64,
                n: 5_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0006696205962893247_f64,
                n: 5_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.00140217290225728_f64,
                n: 5_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0026351697702441173_f64,
                n: 5_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.004554003475056755_f64,
                n: 5_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.007357638737768938_f64,
                n: 5_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.011244740276407147_f64,
                n: 5_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01639748095599911_f64,
                n: 5_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.02296411247484551_f64,
                n: 5_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.03104153653739119_f64,
                n: 5_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.04065918944094895_f64,
                n: 5_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.05176553975736347_f64,
                n: 5_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.06421839577342563_f64,
                n: 5_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.07778003083289287_f64,
                n: 5_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.09211787059372922_f64,
                n: 5_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.10681116145650453_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_18: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.926742745370847e-17_f64,
                n: 10_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.064123963661874e-14_f64,
                n: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 4.045930747410929e-12_f64,
                n: 10_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.11655264004731e-11_f64,
                n: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.547073953019994e-10_f64,
                n: 10_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 3.99344069948363e-09_f64,
                n: 10_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.8327719460735247e-08_f64,
                n: 10_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.825300864974722e-08_f64,
                n: 10_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.1653870546846626e-07_f64,
                n: 10_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.050436229638538e-07_f64,
                n: 10_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.524648535215844e-06_f64,
                n: 10_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 3.5260038931752543e-06_f64,
                n: 10_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.583904002053146e-06_f64,
                n: 10_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.5327786999397103e-05_f64,
                n: 10_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.9348811002317664e-05_f64,
                n: 10_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.358986576863261e-05_f64,
                n: 10_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.381860241047799e-05_f64,
                n: 10_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.000158175163714558_f64,
                n: 10_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.00025777607369970674_f64,
                n: 10_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0004073442442494605_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data006_20: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.930748707339934e-38_f64,
                n: 20_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.251588081015394e-32_f64,
                n: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.4025911398834722e-28_f64,
                n: 20_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.53779572223687e-26_f64,
                n: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.495343924359341e-24_f64,
                n: 20_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.470312039088405e-22_f64,
                n: 20_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.34046271382972e-21_f64,
                n: 20_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.632641100887607e-20_f64,
                n: 20_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.9496335952781085e-19_f64,
                n: 20_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.448853275957898e-18_f64,
                n: 20_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 4.2725223040880135e-17_f64,
                n: 20_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.3942249272752627e-16_f64,
                n: 20_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.165403374149986e-15_f64,
                n: 20_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.0303402625237494e-15_f64,
                n: 20_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.9572475798118565e-14_f64,
                n: 20_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.95598806449061e-14_f64,
                n: 20_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.2825949745670935e-13_f64,
                n: 20_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.978182302179282e-13_f64,
                n: 20_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.002415738866502e-12_f64,
                n: 20_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.42772676079321e-12_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data007_22: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.1917848549326277_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.054402111088936986_f64,
                n: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.04335252267714113_f64,
                n: 0_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.045647262536381385_f64,
                n: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005294070003910922_f64,
                n: 0_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.03293438746976206_f64,
                n: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.012233790557032886_f64,
                n: 0_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01862782901198372_f64,
                n: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.018908967211869295_f64,
                n: 0_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005247497074078575_f64,
                n: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.018177366788338544_f64,
                n: 0_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005080177018370278_f64,
                n: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.012720441222924667_f64,
                n: 0_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.011055581165112701_f64,
                n: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005170421805459072_f64,
                n: 0_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01242360817404219_f64,
                n: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0020714778817480834_f64,
                n: 0_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.009933296262228422_f64,
                n: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.007192228576169695_f64,
                n: 0_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005063656411097588_f64,
                n: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data008_24: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0950894080791708_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.07846694179875155_f64,
                n: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.05353602903573083_f64,
                n: 1_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01812173996385053_f64,
                n: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.03985987527469538_f64,
                n: 1_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0062395279119115375_f64,
                n: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.02547024041527068_f64,
                n: 1_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01713914726660614_f64,
                n: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.011253622702352454_f64,
                n: 1_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01940427051132384_f64,
                n: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0007328022372780778_f64,
                n: 1_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0157888800566131_f64,
                n: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.008848835268632258_f64,
                n: 1_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.008889480313159816_f64,
                n: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.012358955887069445_f64,
                n: 1_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0012245454458125673_f64,
                n: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01155653135896816_f64,
                n: 1_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0050889656932377614_f64,
                n: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.007610329814933157_f64,
                n: 1_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.008673825286987815_f64,
                n: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-11_f64 });
pub static mut data009_26: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1347312100851252_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.07794219362856245_f64,
                n: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.032645316869994966_f64,
                n: 2_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.048365523530958965_f64,
                n: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0005108849709474761_f64,
                n: 2_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.03231043467857091_f64,
                n: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.014416954021198941_f64,
                n: 2_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.017342392966988262_f64,
                n: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01965920872535946_f64,
                n: 2_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.004083240843399146_f64,
                n: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.018137395757214285_f64,
                n: 2_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.005869621021200933_f64,
                n: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01231203344129549_f64,
                n: 2_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01143655889281955_f64,
                n: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.004676063569976294_f64,
                n: 2_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.012469528628260161_f64,
                n: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0024793554591234306_f64,
                n: 2_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.009763664072453828_f64,
                n: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.007432554780851794_f64,
                n: 2_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.004803441652487954_f64,
                n: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data010_28: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.10681116145650453_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.055534511621452155_f64,
                n: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.06596800707652195_f64,
                n: 5_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.016683908063095682_f64,
                n: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.03611779598972238_f64,
                n: 5_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.02050400873682749_f64,
                n: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01849948120681456_f64,
                n: 5_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.022448773791044995_f64,
                n: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.004855269484502013_f64,
                n: 5_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.020048300563664877_f64,
                n: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005299992445556574_f64,
                n: 5_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.014151556281331407_f64,
                n: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.011354588594416778_f64,
                n: 5_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.006498378178532357_f64,
                n: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.013089909320064257_f64,
                n: 5_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0009620045007130245_f64,
                n: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.011048668899130202_f64,
                n: 5_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0065639581708136045_f64,
                n: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.006464611936820277_f64,
                n: 5_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.009290148934907571_f64,
                n: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-11_f64 });
pub static mut data011_30: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0004073442442494605_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.06460515449256427_f64,
                n: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0018969790010883577_f64,
                n: 10_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.039686698644626366_f64,
                n: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.03625328560112858_f64,
                n: 10_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.014529646403897799_f64,
                n: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.026281264603993857_f64,
                n: 10_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.013124803182748323_f64,
                n: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01760083138372898_f64,
                n: 10_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.015039221463465955_f64,
                n: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.009525628934916739_f64,
                n: 10_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01582271939400834_f64,
                n: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0019391391708249754_f64,
                n: 10_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.014293389028395012_f64,
                n: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.004421028503169623_f64,
                n: 10_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.010516146958338813_f64,
                n: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.008673627513132573_f64,
                n: 10_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005290506635723933_f64,
                n: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.010258326955210768_f64,
                n: 10_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.00019565785971342414_f64,
                n: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_31: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000017e-10_f64 });
pub static mut data012_32: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.42772676079321e-12_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.308371961319467e-06_f64,
                n: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0015467058510412498_f64,
                n: 20_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.03832485163980516_f64,
                n: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.028500071484154645_f64,
                n: 20_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.014711593353429081_f64,
                n: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01079765307026423_f64,
                n: 20_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.026535391837540293_f64,
                n: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.011582959134716391_f64,
                n: 20_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01578502989826929_f64,
                n: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01388551918586274_f64,
                n: 20_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.011112458964023273_f64,
                n: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.011938384963927568_f64,
                n: 20_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.010117695207156904_f64,
                n: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.008987121410238323_f64,
                n: 20_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.010400578884991936_f64,
                n: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.00553590206563267_f64,
                n: 20_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.010639343320787522_f64,
                n: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.001805166145597953_f64,
                n: 20_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.010107671283873054_f64,
                n: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_33: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data013_34: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.857479350440152e-46_f64,
                n: 50_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.230696023218648e-31_f64,
                n: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.68047166400808e-23_f64,
                n: 50_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.650080791872522e-17_f64,
                n: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.2540416973758975e-12_f64,
                n: 50_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.6901637185735326e-09_f64,
                n: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.0167148174422245e-06_f64,
                n: 50_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.394917403817907e-05_f64,
                n: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0024888927213794557_f64,
                n: 50_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.018829107369282647_f64,
                n: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.02637319843814549_f64,
                n: 50_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.021230978268739_f64,
                n: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.016539881802291313_f64,
                n: 50_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.015985416061436664_f64,
                n: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01546254898440559_f64,
                n: 50_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.010638570118081819_f64,
                n: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.00046961239784540793_f64,
                n: 50_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.009606588218992025_f64,
                n: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.010613873910261154_f64,
                n: 50_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0005797140882277493_f64,
                n: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000017e-10_f64 });
pub static mut data014_36: std::cell::LazyCell<[testcase_sph_bessel_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.535650303388993e-120_f64,
                n: 100_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.832040182005877e-90_f64,
                n: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.7406387750766626e-72_f64,
                n: 100_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 3.515271112531701e-60_f64,
                n: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.845545935381596e-51_f64,
                n: 100_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 4.088859674430158e-43_f64,
                n: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 8.897585491113394e-37_f64,
                n: 100_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.151349254773383e-31_f64,
                n: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.36735869945391e-27_f64,
                n: 100_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.0190122629310471e-22_f64,
                n: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 3.488780497769039e-19_f64,
                n: 100_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 4.444288342555559e-16_f64,
                n: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.3832619568710723e-13_f64,
                n: 100_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.894838417560799e-11_f64,
                n: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.188444635702228e-09_f64,
                n: 100_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 4.5247964400095e-07_f64,
                n: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.5096093228779032e-05_f64,
                n: 100_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.00026825172647807513_f64,
                n: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0024744308520581117_f64,
                n: 100_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01088047701143835_f64,
                n: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_37: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-12_f64 });
pub unsafe fn test_38(data: *const [testcase_sph_bessel_double_; 21], mut toler: ()) {
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
            (unsafe {
                __builtin_fprintf_39 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 527_() ) .into() , ( c"void test(const testcase_sph_bessel<Ret> (&)[Num], Ret) [Ret = double, Num = 21U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
            });
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
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_11)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_13)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_14)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_15)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_16)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_17)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_18)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_19)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_20)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_21)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_22)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_23)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_24)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_25)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_26)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_27)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_28)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_29)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_30)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_31)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_32)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_33)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_34)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_35)),
        )
    });
    (unsafe {
        test_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_36)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_37)),
        )
    });
    return 0_();
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_40() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_41(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_42(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_43(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_44(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_45(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_44(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_45(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_44((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_45(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_46(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_44(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_45(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_47(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_48(
            mangled,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            (&mut status as *mut ()),
        )
    });
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
    let mut w: () =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1({ wanted }, None);
    if unimplemented!() {
        (unsafe { __throw_runtime_error_3(s) });
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_49(mut name: *const (), l: *const __gnu_test_func_callback) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut res: *const () = (unimplemented!()).cast_const();
    if !(res).is_null() {
        let mut preLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { res },
                None,
            );
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (unimplemented!()).cast_const() },
                None,
            );
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                (unsafe {
                    __builtin_fprintf_39 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"LC_ALL for ".as_ptr() },
            None,
        );
        unimplemented!();
        (unsafe { __throw_runtime_error_3(unimplemented!()) });
    }
}
pub unsafe fn run_tests_wrapped_env_50(
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
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { env },
            None,
        );
        unimplemented!();
        unimplemented!();
        (unsafe { __throw_runtime_error_3(unimplemented!()) });
    }
}
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut _M_count_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut next_id__7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union __gnu_test_semun {
    pub val: (),
    pub buf: *mut semid_ds,
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
        if ((self.pid_) == (unsafe { getpid_40() })) {
            (unsafe { semctl_42(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_51(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_51(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_52(
    mut sec: (),
    mut min: (),
    mut hour: (),
    mut mday: (),
    mut mon: (),
    mut year: (),
    mut wday: (),
    mut yday: (),
    mut isdst: (),
) -> ::libc::tm {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53));
}
static mut tmp_53: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const data001_10);
    std::cell::LazyCell::force(&*&raw const toler001_11);
    std::cell::LazyCell::force(&*&raw const data002_12);
    std::cell::LazyCell::force(&*&raw const toler002_13);
    std::cell::LazyCell::force(&*&raw const data003_14);
    std::cell::LazyCell::force(&*&raw const toler003_15);
    std::cell::LazyCell::force(&*&raw const data004_16);
    std::cell::LazyCell::force(&*&raw const toler004_17);
    std::cell::LazyCell::force(&*&raw const data005_18);
    std::cell::LazyCell::force(&*&raw const toler005_19);
    std::cell::LazyCell::force(&*&raw const data006_20);
    std::cell::LazyCell::force(&*&raw const toler006_21);
    std::cell::LazyCell::force(&*&raw const data007_22);
    std::cell::LazyCell::force(&*&raw const toler007_23);
    std::cell::LazyCell::force(&*&raw const data008_24);
    std::cell::LazyCell::force(&*&raw const toler008_25);
    std::cell::LazyCell::force(&*&raw const data009_26);
    std::cell::LazyCell::force(&*&raw const toler009_27);
    std::cell::LazyCell::force(&*&raw const data010_28);
    std::cell::LazyCell::force(&*&raw const toler010_29);
    std::cell::LazyCell::force(&*&raw const data011_30);
    std::cell::LazyCell::force(&*&raw const toler011_31);
    std::cell::LazyCell::force(&*&raw const data012_32);
    std::cell::LazyCell::force(&*&raw const toler012_33);
    std::cell::LazyCell::force(&*&raw const data013_34);
    std::cell::LazyCell::force(&*&raw const toler013_35);
    std::cell::LazyCell::force(&*&raw const data014_36);
    std::cell::LazyCell::force(&*&raw const toler014_37);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
