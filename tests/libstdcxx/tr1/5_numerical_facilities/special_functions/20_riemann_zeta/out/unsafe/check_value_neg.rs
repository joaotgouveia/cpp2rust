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
pub struct testcase_riemann_zeta_double_ {
    pub f0: (),
    pub s: (),
    pub f: (),
}
pub static mut data001_10: std::cell::LazyCell<[testcase_riemann_zeta_double_; 55]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_riemann_zeta_double_ {
                f0: 0_f64,
                s: -10_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.003366982045101957_f64,
                s: -9.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.005812951776731901_f64,
                s: -9.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.007290873229055696_f64,
                s: -9.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.007842091065448439_f64,
                s: -9.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.007575757575757576_f64,
                s: -9_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.006647655567755189_f64,
                s: -8.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.00524000953508594_f64,
                s: -8.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0035434308017674907_f64,
                s: -8.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0017417330388368587_f64,
                s: -8.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0_f64,
                s: -8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0015440036789213937_f64,
                s: -7.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.00278521310864974_f64,
                s: -7.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0036537321227995885_f64,
                s: -7.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.004114793081705353_f64,
                s: -7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.004166666666666667_f64,
                s: -7_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.003836997503273836_f64,
                s: -6.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0031780270571783_f64,
                s: -6.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0022611282027338573_f64,
                s: -6.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0011710237049390457_f64,
                s: -6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0_f64,
                s: -6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0011576366649881868_f64,
                s: -5.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.002210678431856433_f64,
                s: -5.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0030755853460586913_f64,
                s: -5.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.003680438047793476_f64,
                s: -5.199999999999999_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.003968253968253968_f64,
                s: -5_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.003899689130199977_f64,
                s: -4.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0034551830834302698_f64,
                s: -4.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0026366345018725037_f64,
                s: -4.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.001468720930505692_f64,
                s: -4.199999999999999_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0_f64,
                s: -4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0016960463875825209_f64,
                s: -3.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0035198355903356634_f64,
                s: -3.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.005344150320651346_f64,
                s: -3.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.007011972077091057_f64,
                s: -3.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.008333333333333333_f64,
                s: -3_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.00908072948568525_f64,
                s: -2.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.008982462378839656_f64,
                s: -2.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.007713023987424338_f64,
                s: -2.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.004879212359303582_f64,
                s: -2.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0_f64,
                s: -2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.007522934776596874_f64,
                s: -1.799999999999999_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.01844898667896372_f64,
                s: -1.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.033764987694047545_f64,
                s: -1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.05478844124388051_f64,
                s: -1.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.08333333333333333_f64,
                s: -1_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.12198707766977138_f64,
                s: -0.7999999999999989_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.1745957119380135_f64,
                s: -0.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.24716546083171545_f64,
                s: -0.3999999999999986_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.34966628059831456_f64,
                s: -0.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.5_f64,
                s: 0_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.7339209248963422_f64,
                s: 0.20000000000000107_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -1.1347977838669825_f64,
                s: 0.40000000000000036_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -1.9526614482240094_f64,
                s: 0.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -4.437538415895569_f64,
                s: 0.8000000000000007_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_12: std::cell::LazyCell<[testcase_riemann_zeta_double_; 145]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_riemann_zeta_double_ {
                f0: 5.591582441177752_f64,
                s: 1.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 3.105547277977581_f64,
                s: 1.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 2.2857656656801297_f64,
                s: 1.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.882229618102822_f64,
                s: 1.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.6449340668482264_f64,
                s: 2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.4905432565068935_f64,
                s: 2.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.3833428588407355_f64,
                s: 2.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.3054778090727805_f64,
                s: 2.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.2470314223172534_f64,
                s: 2.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.2020569031595942_f64,
                s: 3_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.166773370984467_f64,
                s: 3.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.1386637757280416_f64,
                s: 3.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.1159890791233376_f64,
                s: 3.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0975105764590043_f64,
                s: 3.8000000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0823232337111381_f64,
                s: 4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0697514772338095_f64,
                s: 4.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0592817259798355_f64,
                s: 4.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0505173825665735_f64,
                s: 4.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.043148013335179_f64,
                s: 4.800000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.03692775514337_f64,
                s: 5_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0316598766779166_f64,
                s: 5.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.027185538920354_f64,
                s: 5.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.02337547922703_f64,
                s: 5.6000000000000005_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0201237683883446_f64,
                s: 5.800000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0173430619844492_f64,
                s: 6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.014960945185223_f64,
                s: 6.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.012917088712184_f64,
                s: 6.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.011161014154271_f64,
                s: 6.6000000000000005_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.009650322344712_f64,
                s: 6.800000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.008349277381923_f64,
                s: 7_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0072276664807172_f64,
                s: 7.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0062598756930512_f64,
                s: 7.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0054241359879634_f64,
                s: 7.6000000000000005_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0047019048164696_f64,
                s: 7.800000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0040773561979444_f64,
                s: 8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0035369583062013_f64,
                s: 8.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0030691220374448_f64,
                s: 8.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0026639074861505_f64,
                s: 8.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.002312777909822_f64,
                s: 8.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0020083928260821_f64,
                s: 9_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0017444334995897_f64,
                s: 9.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0015154553480514_f64,
                s: 9.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0013167628052648_f64,
                s: 9.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0011443029840295_f64,
                s: 9.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000994575127818_f64,
                s: 10_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0008645533615088_f64,
                s: 10.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000751620674465_f64,
                s: 10.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000653512414085_f64,
                s: 10.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0005682678503411_f64,
                s: 10.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0004941886041194_f64,
                s: 11_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0004298029239942_f64,
                s: 11.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0003738349551166_f64,
                s: 11.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0003251782761946_f64,
                s: 11.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000282873090999_f64,
                s: 11.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000246086553308_f64,
                s: 12_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000214095781875_f64,
                s: 12.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0001862731874054_f64,
                s: 12.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000162073788746_f64,
                s: 12.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000141024242209_f64,
                s: 12.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0001227133475785_f64,
                s: 13_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0001067838280169_f64,
                s: 13.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000929252097515_f64,
                s: 13.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000080867651872_f64,
                s: 13.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000703765974504_f64,
                s: 13.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000612481350588_f64,
                s: 14_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000533049750668_f64,
                s: 14.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000463929582293_f64,
                s: 14.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000403780253397_f64,
                s: 14.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000351435864272_f64,
                s: 14.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000030588236307_f64,
                s: 15_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000266237704787_f64,
                s: 15.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000231734615617_f64,
                s: 15.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000201705617975_f64,
                s: 15.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000017557001761_f64,
                s: 15.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000152822594086_f64,
                s: 16_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000133023770335_f64,
                s: 16.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000115791066833_f64,
                s: 16.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000100791671644_f64,
                s: 16.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000087735980012_f64,
                s: 16.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000076371976379_f64,
                s: 17_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000066480348633_f64,
                s: 17.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000057870238737_f64,
                s: 17.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000005037554661_f64,
                s: 17.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000043851715013_f64,
                s: 17.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000003817293265_f64,
                s: 18_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000033229700953_f64,
                s: 18.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000002892671715_f64,
                s: 18.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000002518103242_f64,
                s: 18.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000021920449285_f64,
                s: 18.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000019082127165_f64,
                s: 19_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000016611368951_f64,
                s: 19.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000014460565094_f64,
                s: 19.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000012588274738_f64,
                s: 19.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000010958426055_f64,
                s: 19.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000009539620338_f64,
                s: 20_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000008304526344_f64,
                s: 20.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000007229353187_f64,
                s: 20.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000006293391575_f64,
                s: 20.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000005478614529_f64,
                s: 20.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000004769329869_f64,
                s: 21_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000415187772_f64,
                s: 21.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000003614367252_f64,
                s: 21.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000003146447527_f64,
                s: 21.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000273910802_f64,
                s: 21.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000002384505027_f64,
                s: 22_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000002075810521_f64,
                s: 22.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001807080625_f64,
                s: 22.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001573141093_f64,
                s: 22.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001369487659_f64,
                s: 22.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000119219926_f64,
                s: 23_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001037862518_f64,
                s: 23.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000903506006_f64,
                s: 23.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000078654301_f64,
                s: 23.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000684721728_f64,
                s: 23.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000059608189_f64,
                s: 24_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000051891702_f64,
                s: 24.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000451741575_f64,
                s: 24.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000393262332_f64,
                s: 24.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.00000003423535_f64,
                s: 24.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000298035034_f64,
                s: 25_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000259453767_f64,
                s: 25.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000225866978_f64,
                s: 25.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000019662811_f64,
                s: 25.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000171174297_f64,
                s: 25.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000149015549_f64,
                s: 26_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000129725302_f64,
                s: 26.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000112932221_f64,
                s: 26.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000098313035_f64,
                s: 26.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000085586331_f64,
                s: 26.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000074507118_f64,
                s: 27_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000064862125_f64,
                s: 27.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000056465688_f64,
                s: 27.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000004915618_f64,
                s: 27.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000042792894_f64,
                s: 27.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000003725334_f64,
                s: 28_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000032430887_f64,
                s: 28.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000028232703_f64,
                s: 28.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000024577975_f64,
                s: 28.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000021396356_f64,
                s: 28.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000018626598_f64,
                s: 29_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000016215385_f64,
                s: 29.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000014116304_f64,
                s: 29.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000001228895_f64,
                s: 29.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000010698147_f64,
                s: 29.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000009313275_f64,
                s: 30_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub unsafe fn test_14(data: *const [testcase_riemann_zeta_double_; 55], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = ((::std::mem::size_of::<[testcase_riemann_zeta_double_; 55]>() as ())
        .wrapping_div((::std::mem::size_of::<testcase_riemann_zeta_double_>() as ())));
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
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value_neg.cc" .as_ptr() ) .into() , ( 296_() ) .into() , ( c"void test(const testcase_riemann_zeta<Ret> (&)[Num], Ret) [Ret = double, Num = 55U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_16(data: *const [testcase_riemann_zeta_double_; 145], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = ((::std::mem::size_of::<[testcase_riemann_zeta_double_; 55]>() as ())
        .wrapping_div((::std::mem::size_of::<testcase_riemann_zeta_double_>() as ())));
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
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value_neg.cc" .as_ptr() ) .into() , ( 296_() ) .into() , ( c"void test(const testcase_riemann_zeta<Ret> (&)[Num], Ret) [Ret = double, Num = 145U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
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
        test_14(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_11)),
        )
    });
    (unsafe {
        test_16(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_13)),
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
        this.pid_ = ((unsafe { getpid_17() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_18(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_19(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_20(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_21(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_22(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_21(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_22(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_21((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_22(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_23(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_21(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_22(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_24(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_25(
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
pub unsafe fn run_tests_wrapped_locale_26(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_27(
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
        if ((self.pid_) == (unsafe { getpid_17() })) {
            (unsafe { semctl_19(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_28(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_28(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_29(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30));
}
static mut tmp_30: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const data001_10);
    std::cell::LazyCell::force(&*&raw const toler001_11);
    std::cell::LazyCell::force(&*&raw const data002_12);
    std::cell::LazyCell::force(&*&raw const toler002_13);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
