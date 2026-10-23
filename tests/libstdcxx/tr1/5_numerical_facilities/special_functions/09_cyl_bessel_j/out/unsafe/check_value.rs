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
pub struct testcase_cyl_bessel_j_double_ {
    pub f0: (),
    pub nu: (),
    pub x: (),
    pub f: (),
}
pub static mut data007_10: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 1_f64,
                nu: 0_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.9844359292958527_f64,
                nu: 0_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.938469807240813_f64,
                nu: 0_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.8642422751666485_f64,
                nu: 0_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.7651976865579666_f64,
                nu: 0_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.6459060852712853_f64,
                nu: 0_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5118276717359181_f64,
                nu: 0_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.36903253018515075_f64,
                nu: 0_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.22389077914123562_f64,
                nu: 0_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.08274985128873402_f64,
                nu: 0_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.048383776468198_f64,
                nu: 0_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.16414142780851368_f64,
                nu: 0_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.26005195490193334_f64,
                nu: 0_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3327508021706113_f64,
                nu: 0_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.38012773998726335_f64,
                nu: 0_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.40140605493617426_f64,
                nu: 0_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3971498098638474_f64,
                nu: 0_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.36919977029989554_f64,
                nu: 0_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3205425089851215_f64,
                nu: 0_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.25512082749137405_f64,
                nu: 0_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.17759677131433835_f64,
                nu: 0_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data008_12: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 0.3333333333333333_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5533835954964771_f64,
                nu: 0.3333333333333333_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.6728308294979454_f64,
                nu: 0.3333333333333333_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.7249086319937902_f64,
                nu: 0.3333333333333333_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.7308764021694475_f64,
                nu: 0.3333333333333333_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.6995337443389446_f64,
                nu: 0.3333333333333333_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.6371326370648918_f64,
                nu: 0.3333333333333333_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5495635273078846_f64,
                nu: 0.3333333333333333_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.44293981814857586_f64,
                nu: 0.3333333333333333_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.323669889462925_f64,
                nu: 0.3333333333333333_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.19832093341860796_f64,
                nu: 0.3333333333333333_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.07338963787429746_f64,
                nu: 0.3333333333333333_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.04496382094023335_f64,
                nu: 0.3333333333333333_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.15118395956666372_f64,
                nu: 0.3333333333333333_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.24056593952693622_f64,
                nu: 0.3333333333333333_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3094609468192129_f64,
                nu: 0.3333333333333333_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3554273734545761_f64,
                nu: 0.3333333333333333_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3773185282545707_f64,
                nu: 0.3333333333333333_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3753018915935808_f64,
                nu: 0.3333333333333333_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.35080916720916927_f64,
                nu: 0.3333333333333333_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.30642046380026405_f64,
                nu: 0.3333333333333333_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data009_14: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 0.5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.3947995987413697_f64,
                nu: 0.5_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5409737899345276_f64,
                nu: 0.5_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.6280058763758862_f64,
                nu: 0.5_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.6713967071418024_f64,
                nu: 0.5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.6772425381001431_f64,
                nu: 0.5_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.6498380747537466_f64,
                nu: 0.5_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5934852544714738_f64,
                nu: 0.5_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5130161365618272_f64,
                nu: 0.5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.4138750606475999_f64,
                nu: 0.5_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.30200490606236535_f64,
                nu: 0.5_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.1836333213843152_f64,
                nu: 0.5_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.06500818287737575_f64,
                nu: 0.5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.04788572997589854_f64,
                nu: 0.5_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.14960456964952618_f64,
                nu: 0.5_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.23549801845815518_f64,
                nu: 0.5_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.30192051329163944_f64,
                nu: 0.5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.34638850218952444_f64,
                nu: 0.5_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.36767487332724025_f64,
                nu: 0.5_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.365835638023504_f64,
                nu: 0.5_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3421679847981618_f64,
                nu: 0.5_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data010_16: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 0.6666666666666666_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.2743443899886514_f64,
                nu: 0.6666666666666666_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.4233107506844833_f64,
                nu: 0.6666666666666666_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.528705515481628_f64,
                nu: 0.6666666666666666_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5979499736736281_f64,
                nu: 0.6666666666666666_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.633387268890759_f64,
                nu: 0.6666666666666666_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.6367323450287741_f64,
                nu: 0.6666666666666666_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.6102223046013191_f64,
                nu: 0.6666666666666666_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5569696769191372_f64,
                nu: 0.6666666666666666_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.4810127674910613_f64,
                nu: 0.6666666666666666_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.3872124247708432_f64,
                nu: 0.6666666666666666_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.2810572477108055_f64,
                nu: 0.6666666666666666_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.16841218049067047_f64,
                nu: 0.6666666666666666_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.055235893475364936_f64,
                nu: 0.6666666666666666_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.05271158440403193_f64,
                nu: 0.6666666666666666_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.15015178042293031_f64,
                nu: 0.6666666666666666_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.23254408502670393_f64,
                nu: 0.6666666666666666_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.29630067002972543_f64,
                nu: 0.6666666666666666_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.33894810189777724_f64,
                nu: 0.6666666666666666_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.359227069603211_f64,
                nu: 0.6666666666666666_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3571253354916887_f64,
                nu: 0.6666666666666666_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data011_18: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.12402597732272694_f64,
                nu: 1_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.2422684576748739_f64,
                nu: 1_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.3492436021748622_f64,
                nu: 1_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.44005058574493355_f64,
                nu: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5106232603198806_f64,
                nu: 1_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5579365079100995_f64,
                nu: 1_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5801561976389924_f64,
                nu: 1_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5767248077568736_f64,
                nu: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.5483783566469601_f64,
                nu: 1_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.49709410246427416_f64,
                nu: 1_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.42597230295790256_f64,
                nu: 1_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.3390589585259365_f64,
                nu: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.241119688015204_f64,
                nu: 1_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.13737752736232706_f64,
                nu: 1_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.033229349129679724_f64,
                nu: 1_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.06604332802354923_f64,
                nu: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.15555319297834286_f64,
                nu: 1_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.2310604319233707_f64,
                nu: 1_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.28918679864711044_f64,
                nu: 1_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3275791375914653_f64,
                nu: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data012_20: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.007771889285962676_f64,
                nu: 2_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.030604023458682638_f64,
                nu: 2_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.06707399729965055_f64,
                nu: 2_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.11490348493190047_f64,
                nu: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.1710911312405235_f64,
                nu: 2_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.23208767214421472_f64,
                nu: 2_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.29400312425941216_f64,
                nu: 2_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.35283402861563773_f64,
                nu: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.40469757684189717_f64,
                nu: 2_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.4460590584396172_f64,
                nu: 2_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.4739394663233516_f64,
                nu: 2_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.4860912605858912_f64,
                nu: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.4811321486415063_f64,
                nu: 2_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.45862918419430765_f64,
                nu: 2_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.4191283744720035_f64,
                nu: 2_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.3641281458520729_f64,
                nu: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.2959982677218519_f64,
                nu: 2_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.2178489836858455_f64,
                nu: 2_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.13335796490311685_f64,
                nu: 2_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.04656511627775197_f64,
                nu: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data013_22: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.5365161587472413e-07_f64,
                nu: 5_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 8.053627241357475e-06_f64,
                nu: 5_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 6.036416651057644e-05_f64,
                nu: 5_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0002497577302112345_f64,
                nu: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0007444088525474982_f64,
                nu: 5_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.001799421767360611_f64,
                nu: 5_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0037577257273157133_f64,
                nu: 5_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.007039629755871684_f64,
                nu: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.012121078633445751_f64,
                nu: 5_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.019501625134503223_f64,
                nu: 5_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.029664058320006174_f64,
                nu: 5_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.04302843487704758_f64,
                nu: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.059903888098560426_f64,
                nu: 5_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.08044198664799179_f64,
                nu: 5_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.1045955474231407_f64,
                nu: 5_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.13208665604709827_f64,
                nu: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.1623872164362368_f64,
                nu: 5_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.19471465863871368_f64,
                nu: 5_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.22804452118769436_f64,
                nu: 5_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.26114054612017007_f64,
                nu: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data014_24: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.5628321598050096e-16_f64,
                nu: 10_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.6131773608228023e-13_f64,
                nu: 10_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.4962171311759677e-11_f64,
                nu: 10_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.6306151236874524e-10_f64,
                nu: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.4187548221114514e-09_f64,
                nu: 10_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.4743269078039996e-08_f64,
                nu: 10_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 6.760850284989756e-08_f64,
                nu: 10_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.515386282716736e-07_f64,
                nu: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 7.971705158373004e-07_f64,
                nu: 10_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.224728417398384e-06_f64,
                nu: 10_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 5.598547563921043e-06_f64,
                nu: 10_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.292835164571588e-05_f64,
                nu: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.7761691354244538e-05_f64,
                nu: 10_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 5.6009495875078844e-05_f64,
                nu: 10_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.00010703761729231951_f64,
                nu: 10_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.00019504055466003446_f64,
                nu: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.00034068888474064193_f64,
                nu: 10_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.000573009776671645_f64,
                nu: 10_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0009314217258888681_f64,
                nu: 10_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0014678026473104744_f64,
                nu: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data015_26: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 20_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 3.562480551058697e-37_f64,
                nu: 20_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 3.727201961704713e-31_f64,
                nu: 20_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.2347870693633488e-27_f64,
                nu: 20_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 3.873503008524656e-25_f64,
                nu: 20_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 3.3372897667043766e-23_f64,
                nu: 20_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.2689972189332558e-21_f64,
                nu: 20_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.742771594403299e-20_f64,
                nu: 20_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 3.9189728050907524e-19_f64,
                nu: 20_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 4.080523255136516e-18_f64,
                nu: 20_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 3.3090793836587786e-17_f64,
                nu: 20_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.191540468064599e-16_f64,
                nu: 20_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.2275946737992981e-15_f64,
                nu: 20_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 5.972766393830538e-15_f64,
                nu: 20_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.576855310280759e-14_f64,
                nu: 20_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.0021112208287217e-13_f64,
                nu: 20_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 3.5595116285938516e-13_f64,
                nu: 20_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.1673622958555074e-12_f64,
                nu: 20_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 3.5665470983611762e-12_f64,
                nu: 20_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.0227564044880958e-11_f64,
                nu: 20_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.7703300521289426e-11_f64,
                nu: 20_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler015_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data016_28: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 1_f64,
                nu: 0_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.17759677131433835_f64,
                nu: 0_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.24593576445134835_f64,
                nu: 0_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.01422447282678077_f64,
                nu: 0_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.1670246643405832_f64,
                nu: 0_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09626678327595815_f64,
                nu: 0_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.08636798358104014_f64,
                nu: 0_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.12684568275631256_f64,
                nu: 0_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0073668905842374085_f64,
                nu: 0_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.11581867067325631_f64,
                nu: 0_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.055812327669251746_f64,
                nu: 0_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.07454830264823681_f64,
                nu: 0_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.09147180408906186_f64,
                nu: 0_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.01868734322767798_f64,
                nu: 0_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09490872648301354_f64,
                nu: 0_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.03464391380509701_f64,
                nu: 0_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.06974216551221003_f64,
                nu: 0_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.07094039479627327_f64,
                nu: 0_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.026630016699969526_f64,
                nu: 0_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.08181196778338413_f64,
                nu: 0_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.01998585030422317_f64,
                nu: 0_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler016_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data017_30: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 0.3333333333333333_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.30642046380026405_f64,
                nu: 0.3333333333333333_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.18614516704869571_f64,
                nu: 0.3333333333333333_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.08974000422115265_f64,
                nu: 0.3333333333333333_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.176060580012939_f64,
                nu: 0.3333333333333333_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.020097162141383233_f64,
                nu: 0.3333333333333333_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.1333405338742616_f64,
                nu: 0.3333333333333333_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.0871180093977655_f64,
                nu: 0.3333333333333333_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.06920294281885818_f64,
                nu: 0.3333333333333333_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.11387616964518317_f64,
                nu: 0.3333333333333333_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.0005722668077180804_f64,
                nu: 0.3333333333333333_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.10331600929280822_f64,
                nu: 0.3333333333333333_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.055618147270528_f64,
                nu: 0.3333333333333333_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.06471195401411395_f64,
                nu: 0.3333333333333333_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0868799264624816_f64,
                nu: 0.3333333333333333_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.012614484229891068_f64,
                nu: 0.3333333333333333_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.08819978440003454_f64,
                nu: 0.3333333333333333_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.03670361107656452_f64,
                nu: 0.3333333333333333_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.06291628682877955_f64,
                nu: 0.3333333333333333_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.06946524441680603_f64,
                nu: 0.3333333333333333_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.021271244853702364_f64,
                nu: 0.3333333333333333_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler017_31: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000017e-10_f64 });
pub static mut data018_32: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 0.5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3421679847981618_f64,
                nu: 0.5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.1372637357550505_f64,
                nu: 0.5_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.13396768882243937_f64,
                nu: 0.5_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.16288076385502984_f64,
                nu: 0.5_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.021120283599650493_f64,
                nu: 0.5_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.14392965337039987_f64,
                nu: 0.5_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.05774775758945878_f64,
                nu: 0.5_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09400096238953365_f64,
                nu: 0.5_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.10120783324271411_f64,
                nu: 0.5_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.02960583188892464_f64,
                nu: 0.5_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.10756039213265806_f64,
                nu: 0.5_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.03139746118252044_f64,
                nu: 0.5_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.08182743077562855_f64,
                nu: 0.5_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.07380242953905455_f64,
                nu: 0.5_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.035727009681702615_f64,
                nu: 0.5_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.08866103581176546_f64,
                nu: 0.5_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.015238065106312516_f64,
                nu: 0.5_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.07518906855026943_f64,
                nu: 0.5_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.05593264348149413_f64,
                nu: 0.5_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.04040213271625213_f64,
                nu: 0.5_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler018_33: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data019_34: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 0.6666666666666666_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3571253354916887_f64,
                nu: 0.6666666666666666_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.08014960330431581_f64,
                nu: 0.6666666666666666_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.1691887517579808_f64,
                nu: 0.6666666666666666_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.13904826122116531_f64,
                nu: 0.6666666666666666_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.0607706296984976_f64,
                nu: 0.6666666666666666_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.14489851974205062_f64,
                nu: 0.6666666666666666_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.024604880159644394_f64,
                nu: 0.6666666666666666_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.1124393646491201_f64,
                nu: 0.6666666666666666_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.08177627551252531_f64,
                nu: 0.6666666666666666_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.05658990874936778_f64,
                nu: 0.6666666666666666_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.10455814523765931_f64,
                nu: 0.6666666666666666_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.005103014854860846_f64,
                nu: 0.6666666666666666_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09339822706163924_f64,
                nu: 0.6666666666666666_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.05576388361186491_f64,
                nu: 0.6666666666666666_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.056395322915757364_f64,
                nu: 0.6666666666666666_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.08313134780578309_f64,
                nu: 0.6666666666666666_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.007231539787409665_f64,
                nu: 0.6666666666666666_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.08236279852090525_f64,
                nu: 0.6666666666666666_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.03863050440344617_f64,
                nu: 0.6666666666666666_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.056778819380529734_f64,
                nu: 0.6666666666666666_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler019_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-10_f64 });
pub static mut data020_36: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.3275791375914653_f64,
                nu: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.04347274616886146_f64,
                nu: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.2051040386135228_f64,
                nu: 1_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.06683312417585008_f64,
                nu: 1_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.1253502495802899_f64,
                nu: 1_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.11875106261662294_f64,
                nu: 1_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.043990942179625646_f64,
                nu: 1_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.126038318037585_f64,
                nu: 1_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.02834885437642456_f64,
                nu: 1_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.09751182812517513_f64,
                nu: 1_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.07825003830868471_f64,
                nu: 1_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.04659838375816637_f64,
                nu: 1_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09733017222612693_f64,
                nu: 1_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.009987788784838513_f64,
                nu: 1_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.08513999504482908_f64,
                nu: 1_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.056057296675712555_f64,
                nu: 1_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.04915146033489113_f64,
                nu: 1_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.07992564670886809_f64,
                nu: 1_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.0023925612997269283_f64,
                nu: 1_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.07714535201411213_f64,
                nu: 1_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler020_37: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-10_f64 });
pub static mut data021_38: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.04656511627775197_f64,
                nu: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.2546303136851207_f64,
                nu: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.041571677975250486_f64,
                nu: 2_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.1603413519229982_f64,
                nu: 2_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.10629480324238134_f64,
                nu: 2_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0784512460732653_f64,
                nu: 2_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.1293594508808626_f64,
                nu: 2_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.0010649746823579794_f64,
                nu: 2_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.11455872158985966_f64,
                nu: 2_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.05971280079425886_f64,
                nu: 2_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.07170284670973924_f64,
                nu: 2_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09302508354766742_f64,
                nu: 2_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.015692568697643128_f64,
                nu: 2_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.09462336108916103_f64,
                nu: 2_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.03691431367295918_f64,
                nu: 2_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.06834073309531717_f64,
                nu: 2_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.07209689974532954_f64,
                nu: 2_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.024853891217550248_f64,
                nu: 2_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.08186233749495733_f64,
                nu: 2_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.021528757344505364_f64,
                nu: 2_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler021_39: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000017e-10_f64 });
pub static mut data022_40: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.26114054612017007_f64,
                nu: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.2340615281867937_f64,
                nu: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.13045613456502966_f64,
                nu: 5_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.15116976798239498_f64,
                nu: 5_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.06600799539842293_f64,
                nu: 5_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.1432402955120771_f64,
                nu: 5_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.0015053072953907251_f64,
                nu: 5_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.12257346597711777_f64,
                nu: 5_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.05798449920095411_f64,
                nu: 5_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.08140024769656962_f64,
                nu: 5_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.09256989578643277_f64,
                nu: 5_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.027454744228344204_f64,
                nu: 5_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09911052770153903_f64,
                nu: 5_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.026058129823895364_f64,
                nu: 5_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.0785239770137514_f64,
                nu: 5_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.06586234914003158_f64,
                nu: 5_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.03866907228468098_f64,
                nu: 5_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.08275931952841516_f64,
                nu: 5_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.007942337270247287_f64,
                nu: 5_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.0741957369645139_f64,
                nu: 5_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler022_41: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-11_f64 });
pub static mut data023_42: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0014678026473104744_f64,
                nu: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.20748610663335865_f64,
                nu: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.09007181104765909_f64,
                nu: 10_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.18648255802394512_f64,
                nu: 10_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.07517984394852331_f64,
                nu: 10_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.12987689399858882_f64,
                nu: 10_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.06354639134396287_f64,
                nu: 10_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.11938336278226094_f64,
                nu: 10_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.02697140247501083_f64,
                nu: 10_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.1138478491494694_f64,
                nu: 10_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.01577379030374608_f64,
                nu: 10_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09717714332807106_f64,
                nu: 10_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.05461738995111213_f64,
                nu: 10_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.06587033856195201_f64,
                nu: 10_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.08041786789189444_f64,
                nu: 10_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.024043850978184747_f64,
                nu: 10_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.08682483270006787_f64,
                nu: 10_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0195547488563123_f64,
                nu: 10_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.07234159866944376_f64,
                nu: 10_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.054732176935472096_f64,
                nu: 10_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler023_43: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data024_44: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 20_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.7703300521289426e-11_f64,
                nu: 20_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.1513369247813403e-05_f64,
                nu: 20_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0073602340792234934_f64,
                nu: 20_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.16474777377532665_f64,
                nu: 20_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.05199404922830331_f64,
                nu: 20_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.004831019993404092_f64,
                nu: 20_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.10927417397178038_f64,
                nu: 20_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.1277939335508489_f64,
                nu: 20_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.004763343790031362_f64,
                nu: 20_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.11670435275957974_f64,
                nu: 20_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.02538920457456664_f64,
                nu: 20_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.10266020557876326_f64,
                nu: 20_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.023138582263434154_f64,
                nu: 20_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.09605857348995236_f64,
                nu: 20_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.006896104722152227_f64,
                nu: 20_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09056540548991836_f64,
                nu: 20_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.015985497599497172_f64,
                nu: 20_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.08034534404442253_f64,
                nu: 20_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.04025307570161405_f64,
                nu: 20_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.06221745849833867_f64,
                nu: 20_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler024_45: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-10_f64 });
pub static mut data025_46: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 50_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.2942476159525415e-45_f64,
                nu: 50_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.7845136078715964e-30_f64,
                nu: 50_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 6.106051949533873e-22_f64,
                nu: 50_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 4.451039284700687e-16_f64,
                nu: 50_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 9.75615942802298e-12_f64,
                nu: 50_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.058165663156417e-08_f64,
                nu: 50_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 7.606995169927296e-06_f64,
                nu: 50_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.0006818524353176831_f64,
                nu: 50_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.017284343240791214_f64,
                nu: 50_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.12140902189761507_f64,
                nu: 50_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.13594720957176012_f64,
                nu: 50_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.1379827314853521_f64,
                nu: 50_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.1211621774661941_f64,
                nu: 50_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.11394866738787145_f64,
                nu: 50_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09407679958157335_f64,
                nu: 50_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.03945776459025135_f64,
                nu: 50_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.04041206073413638_f64,
                nu: 50_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09080209983803227_f64,
                nu: 50_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.055979156267280165_f64,
                nu: 50_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.03869833972852544_f64,
                nu: 50_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler025_47: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-11_f64 });
pub static mut data026_48: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0_f64,
                nu: 100_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 6.267789395541876e-119_f64,
                nu: 100_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 6.597316064155382e-89_f64,
                nu: 100_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.9660095611249536e-71_f64,
                nu: 100_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 3.9617550943362524e-59_f64,
                nu: 100_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.1064482655301687e-49_f64,
                nu: 100_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 4.5788015281752354e-42_f64,
                nu: 100_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 9.921020671473543e-36_f64,
                nu: 100_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.3866062996026735e-30_f64,
                nu: 100_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.0329791804565538e-25_f64,
                nu: 100_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 1.1159273690838498e-21_f64,
                nu: 100_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 3.789975345190094e-18_f64,
                nu: 100_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 4.783274407878256e-15_f64,
                nu: 100_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 2.537556457949043e-12_f64,
                nu: 100_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 6.198245214164126e-10_f64,
                nu: 100_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 7.447900590590446e-08_f64,
                nu: 100_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 4.606553064823494e-06_f64,
                nu: 100_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.00015043869999501787_f64,
                nu: 100_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.002602130581996363_f64,
                nu: 100_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.02315076800942803_f64,
                nu: 100_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.09636667329586157_f64,
                nu: 100_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler026_49: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-11_f64 });
pub static mut data027_50: std::cell::LazyCell<[testcase_cyl_bessel_j_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_j_double_ {
                f0: 0.011676135007789573_f64,
                nu: 100_f64,
                x: 1000_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.011699854778025796_f64,
                nu: 100_f64,
                x: 1100_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.022801483405083697_f64,
                nu: 100_f64,
                x: 1200_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.016973500787373915_f64,
                nu: 100_f64,
                x: 1300_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.0014154528803481308_f64,
                nu: 100_f64,
                x: 1400_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.013333726584495232_f64,
                nu: 100_f64,
                x: 1500_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.01980256202014856_f64,
                nu: 100_f64,
                x: 1600_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.016129771279838816_f64,
                nu: 100_f64,
                x: 1700_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.005375336928153603_f64,
                nu: 100_f64,
                x: 1800_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.0069238868725645785_f64,
                nu: 100_f64,
                x: 1900_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.015487871720069789_f64,
                nu: 100_f64,
                x: 2000_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.01727518671767107_f64,
                nu: 100_f64,
                x: 2100_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.01223303052517315_f64,
                nu: 100_f64,
                x: 2200_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.00285185086722419_f64,
                nu: 100_f64,
                x: 2300_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.007078437227028933_f64,
                nu: 100_f64,
                x: 2400_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.013955367586928166_f64,
                nu: 100_f64,
                x: 2500_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.015574059842493392_f64,
                nu: 100_f64,
                x: 2600_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.011718043044647556_f64,
                nu: 100_f64,
                x: 2700_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: 0.004032095323128561_f64,
                nu: 100_f64,
                x: 2800_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.004689511178305398_f64,
                nu: 100_f64,
                x: 2900_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_j_double_ {
                f0: -0.011507715400035966_f64,
                nu: 100_f64,
                x: 3000_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler027_51: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-10_f64 });
pub unsafe fn test_52(data: *const [testcase_cyl_bessel_j_double_; 21], mut toler: ()) {
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
                __builtin_fprintf_53 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 758_() ) .into() , ( c"void test(const testcase_cyl_bessel_j<Ret> (&)[Num], Ret) [Ret = double, Num = 21U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
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
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_11)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_13)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_14)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_15)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_16)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_17)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_18)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_19)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_20)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_21)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_22)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_23)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_24)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_25)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_26)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_27)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_28)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_29)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data017_30)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler017_31)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data018_32)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler018_33)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data019_34)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler019_35)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data020_36)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler020_37)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data021_38)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler021_39)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data022_40)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler022_41)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data023_42)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler023_43)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data024_44)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler024_45)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data025_46)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler025_47)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data026_48)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler026_49)),
        )
    });
    (unsafe {
        test_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data027_50)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler027_51)),
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
        this.pid_ = ((unsafe { getpid_54() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_55(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_56(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_57(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_58(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_59(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_58(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_59(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_58((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_59(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_60(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_58(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_59(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_61(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_62(
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
pub unsafe fn run_tests_wrapped_locale_63(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_53 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_64(
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
        if ((self.pid_) == (unsafe { getpid_54() })) {
            (unsafe { semctl_56(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_65(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_65(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_66(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_67)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_67)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_67)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_67)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_67)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_67)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_67)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_67)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_67)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_67));
}
static mut tmp_67: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const data007_10);
    std::cell::LazyCell::force(&*&raw const toler007_11);
    std::cell::LazyCell::force(&*&raw const data008_12);
    std::cell::LazyCell::force(&*&raw const toler008_13);
    std::cell::LazyCell::force(&*&raw const data009_14);
    std::cell::LazyCell::force(&*&raw const toler009_15);
    std::cell::LazyCell::force(&*&raw const data010_16);
    std::cell::LazyCell::force(&*&raw const toler010_17);
    std::cell::LazyCell::force(&*&raw const data011_18);
    std::cell::LazyCell::force(&*&raw const toler011_19);
    std::cell::LazyCell::force(&*&raw const data012_20);
    std::cell::LazyCell::force(&*&raw const toler012_21);
    std::cell::LazyCell::force(&*&raw const data013_22);
    std::cell::LazyCell::force(&*&raw const toler013_23);
    std::cell::LazyCell::force(&*&raw const data014_24);
    std::cell::LazyCell::force(&*&raw const toler014_25);
    std::cell::LazyCell::force(&*&raw const data015_26);
    std::cell::LazyCell::force(&*&raw const toler015_27);
    std::cell::LazyCell::force(&*&raw const data016_28);
    std::cell::LazyCell::force(&*&raw const toler016_29);
    std::cell::LazyCell::force(&*&raw const data017_30);
    std::cell::LazyCell::force(&*&raw const toler017_31);
    std::cell::LazyCell::force(&*&raw const data018_32);
    std::cell::LazyCell::force(&*&raw const toler018_33);
    std::cell::LazyCell::force(&*&raw const data019_34);
    std::cell::LazyCell::force(&*&raw const toler019_35);
    std::cell::LazyCell::force(&*&raw const data020_36);
    std::cell::LazyCell::force(&*&raw const toler020_37);
    std::cell::LazyCell::force(&*&raw const data021_38);
    std::cell::LazyCell::force(&*&raw const toler021_39);
    std::cell::LazyCell::force(&*&raw const data022_40);
    std::cell::LazyCell::force(&*&raw const toler022_41);
    std::cell::LazyCell::force(&*&raw const data023_42);
    std::cell::LazyCell::force(&*&raw const toler023_43);
    std::cell::LazyCell::force(&*&raw const data024_44);
    std::cell::LazyCell::force(&*&raw const toler024_45);
    std::cell::LazyCell::force(&*&raw const data025_46);
    std::cell::LazyCell::force(&*&raw const toler025_47);
    std::cell::LazyCell::force(&*&raw const data026_48);
    std::cell::LazyCell::force(&*&raw const toler026_49);
    std::cell::LazyCell::force(&*&raw const data027_50);
    std::cell::LazyCell::force(&*&raw const toler027_51);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
