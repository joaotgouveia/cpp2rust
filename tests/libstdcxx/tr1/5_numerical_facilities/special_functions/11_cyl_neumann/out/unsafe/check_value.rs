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
pub struct testcase_cyl_neumann_double_ {
    pub f0: (),
    pub nu: (),
    pub x: (),
    pub f: (),
}
pub static mut data007_10: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -0.9315730249300588_f64,
                nu: 0_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.44451873350670656_f64,
                nu: 0_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.13717276938577236_f64,
                nu: 0_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08825696421567694_f64,
                nu: 0_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.2582168515945407_f64,
                nu: 0_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.38244892379775886_f64,
                nu: 0_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4654926286469061_f64,
                nu: 0_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.5103756726497449_f64,
                nu: 0_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.5200647624572786_f64,
                nu: 0_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.49807035961523194_f64,
                nu: 0_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4486587215691322_f64,
                nu: 0_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.37685001001279045_f64,
                nu: 0_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.2882869026730871_f64,
                nu: 0_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.18902194392082688_f64,
                nu: 0_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08525675697736264_f64,
                nu: 0_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.016940739325064763_f64,
                nu: 0_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.1119188511616077_f64,
                nu: 0_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.19470500862950454_f64,
                nu: 0_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.2612325032349755_f64,
                nu: 0_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.3085176252490336_f64,
                nu: 0_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data008_12: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -1.3461842332051073_f64,
                nu: 0.3333333333333333_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.8406278260433777_f64,
                nu: 0.3333333333333333_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.5248828148409709_f64,
                nu: 0.3333333333333333_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.27880164127599216_f64,
                nu: 0.3333333333333333_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.07432134972783645_f64,
                nu: 0.3333333333333333_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.09661008776662798_f64,
                nu: 0.3333333333333333_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.23582564494922068_f64,
                nu: 0.3333333333333333_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.34319996626034494_f64,
                nu: 0.3333333333333333_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4183566845234932_f64,
                nu: 0.3333333333333333_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4614594741912916_f64,
                nu: 0.3333333333333333_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4735892613578602_f64,
                nu: 0.3333333333333333_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4568930345723064_f64,
                nu: 0.3333333333333333_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.41458485697347386_f64,
                nu: 0.3333333333333333_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.35084133277859947_f64,
                nu: 0.3333333333333333_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.2706191452782089_f64,
                nu: 0.3333333333333333_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.17941676634394862_f64,
                nu: 0.3333333333333333_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08300043419152604_f64,
                nu: 0.3333333333333333_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.012886361627105348_f64,
                nu: 0.3333333333333333_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.10281143123935124_f64,
                nu: 0.3333333333333333_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.1819232112934385_f64,
                nu: 0.3333333333333333_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data009_14: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -1.5461605241060765_f64,
                nu: 0.5_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.9902458802434045_f64,
                nu: 0.5_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.6741179291445444_f64,
                nu: 0.5_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.4310988680183757_f64,
                nu: 0.5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.2250296924446648_f64,
                nu: 0.5_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.046083165893097265_f64,
                nu: 0.5_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.10750804524368722_f64,
                nu: 0.5_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.2347857104062485_f64,
                nu: 0.5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.33414002338271825_f64,
                nu: 0.5_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.40427830223905686_f64,
                nu: 0.5_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4447211511949051_f64,
                nu: 0.5_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.45604882079463316_f64,
                nu: 0.5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4399885950192437_f64,
                nu: 0.5_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.3993868253630491_f64,
                nu: 0.5_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.3380916383669334_f64,
                nu: 0.5_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.26076607667717877_f64,
                nu: 0.5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.17264962544644955_f64,
                nu: 0.5_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.07928586286297855_f64,
                nu: 0.5_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.013765943019498003_f64,
                nu: 0.5_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.10121770918510846_f64,
                nu: 0.5_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data010_16: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -1.8021638417426857_f64,
                nu: 0.6666666666666666_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.1316060101031435_f64,
                nu: 0.6666666666666666_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.8025115635845075_f64,
                nu: 0.6666666666666666_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.5627032149746333_f64,
                nu: 0.6666666666666666_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.3600745364343221_f64,
                nu: 0.6666666666666666_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.1801793746961502_f64,
                nu: 0.6666666666666666_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.019885608758103752_f64,
                nu: 0.6666666666666666_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.11989345361903521_f64,
                nu: 0.6666666666666666_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.2369088983635804_f64,
                nu: 0.6666666666666666_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.32882045742954535_f64,
                nu: 0.6666666666666666_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.39385133784531856_f64,
                nu: 0.6666666666666666_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4311510169093564_f64,
                nu: 0.6666666666666666_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4409812735144584_f64,
                nu: 0.6666666666666666_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.42477631413456485_f64,
                nu: 0.6666666666666666_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.38510384155620386_f64,
                nu: 0.6666666666666666_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.32554526794354366_f64,
                nu: 0.6666666666666666_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.25051080073878446_f64,
                nu: 0.6666666666666666_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.16500507211842136_f64,
                nu: 0.6666666666666666_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.07435964972886136_f64,
                nu: 0.6666666666666666_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.016050662643389627_f64,
                nu: 0.6666666666666666_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data011_18: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -2.7041052293152825_f64,
                nu: 1_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.4714723926702433_f64,
                nu: 1_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.0375945507692856_f64,
                nu: 1_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.7812128213002889_f64,
                nu: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.5843640366150082_f64,
                nu: 1_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.4123086269739112_f64,
                nu: 1_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.25397298594624573_f64,
                nu: 1_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.10703243154093699_f64,
                nu: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.027192057738017056_f64,
                nu: 1_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.145918137966786_f64,
                nu: 1_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.24601900149738354_f64,
                nu: 1_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.32467442479180003_f64,
                nu: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.3797777737170838_f64,
                nu: 1_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.4101884178875117_f64,
                nu: 1_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.41586877934522715_f64,
                nu: 1_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.3979257105570999_f64,
                nu: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.35856889308385076_f64,
                nu: 1_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.3009973230696545_f64,
                nu: 1_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.22922559673872217_f64,
                nu: 1_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.147863143391227_f64,
                nu: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data012_20: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -20.7012688095922_f64,
                nu: 2_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -5.441370837174267_f64,
                nu: 2_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.629746032665656_f64,
                nu: 2_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.6506826068162548_f64,
                nu: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.1931993101785539_f64,
                nu: 2_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.9321937597629737_f64,
                nu: 2_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.7557474697283297_f64,
                nu: 2_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.6174081041906819_f64,
                nu: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.4958940444679301_f64,
                nu: 2_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.38133584924180314_f64,
                nu: 2_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.26973581138921693_f64,
                nu: 2_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.16040039348492377_f64,
                nu: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.05457750346257395_f64,
                nu: 2_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.04537143772917979_f64,
                nu: 2_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.13653992534009185_f64,
                nu: 2_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.21590359460361472_f64,
                nu: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.28065715378930217_f64,
                nu: 2_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.32848159666046206_f64,
                nu: 2_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.357748543967069_f64,
                nu: 2_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.3676628826055244_f64,
                nu: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data013_22: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -251309.4815185237_f64,
                nu: 5_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -7946.301478807475_f64,
                nu: 5_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1067.246895228976_f64,
                nu: 5_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -260.4058666258123_f64,
                nu: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -88.4742524418804_f64,
                nu: 5_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -37.190308395498064_f64,
                nu: 5_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -18.165774988201832_f64,
                nu: 5_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -9.935989128481967_f64,
                nu: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -5.944634384807642_f64,
                nu: 5_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.830176000740752_f64,
                nu: 5_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.6287042009459087_f64,
                nu: 5_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.9059459538286738_f64,
                nu: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.4498157389142654_f64,
                nu: 5_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.1494603169763686_f64,
                nu: 5_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.9434310515143167_f64,
                nu: 5_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.7958514211141998_f64,
                nu: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.6847928817390702_f64,
                nu: 5_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.5963193651358756_f64,
                nu: 5_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.5213083833174759_f64,
                nu: 5_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.45369482249110193_f64,
                nu: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data014_24: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -124241617095379.48_f64,
                nu: 10_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -121963623349.56966_f64,
                nu: 10_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2133501638.9057348_f64,
                nu: 10_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -121618014.2786892_f64,
                nu: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -13265210.158452792_f64,
                nu: 10_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2183993.026086405_f64,
                nu: 10_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -478274.8238654117_f64,
                nu: 10_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -129184.54220803917_f64,
                nu: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -40993.25479438169_f64,
                nu: 10_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -14782.84771602107_f64,
                nu: 10_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -5916.533099877626_f64,
                nu: 10_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2582.6071294842995_f64,
                nu: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1213.3423564023892_f64,
                nu: 10_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -607.2743783412576_f64,
                nu: 10_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -321.174610597522_f64,
                nu: 10_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -178.33055590796428_f64,
                nu: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -103.4049658757009_f64,
                nu: 10_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -62.345024619781434_f64,
                nu: 10_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -38.94451043029694_f64,
                nu: 10_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -25.129110095610095_f64,
                nu: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data015_26: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -4.467881506415258e+34_f64,
                nu: 20_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -4.271430121565909e+28_f64,
                nu: 20_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.2898357375834223e+25_f64,
                nu: 20_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -4.1139703148355065e+22_f64,
                nu: 20_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -477835333721485803520_f64,
                nu: 20_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -12577301772964241408_f64,
                nu: 20_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -582510411766496256_f64,
                nu: 20_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -40816513889983640_f64,
                nu: 20_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3925339868516418.5_f64,
                nu: 20_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -484776559582090.25_f64,
                nu: 20_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -73320655044814.47_f64,
                nu: 20_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -13113540041757.45_f64,
                nu: 20_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2700669268882.714_f64,
                nu: 20_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -627339518240.2124_f64,
                nu: 20_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -161695236802.71753_f64,
                nu: 20_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -45637199262.2201_f64,
                nu: 20_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -13953299213.925377_f64,
                nu: 20_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -4580215756.569102_f64,
                nu: 20_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1602110715.5159132_f64,
                nu: 20_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -593396529.691432_f64,
                nu: 20_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler015_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data016_28: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -2.764348747115597e+107_f64,
                nu: 50_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.4575848224461092e+92_f64,
                nu: 50_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.860450846768383e+83_f64,
                nu: 50_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.191142812605341e+77_f64,
                nu: 50_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.1362926828833165e+72_f64,
                nu: 50_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.4584216846550566e+68_f64,
                nu: 50_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.5607714080312795e+65_f64,
                nu: 50_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.9761505765184128e+62_f64,
                nu: 50_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -5.502364049923119e+59_f64,
                nu: 50_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.853038454582685e+57_f64,
                nu: 50_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.446716932268481e+55_f64,
                nu: 50_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.179389146100518e+53_f64,
                nu: 50_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -5.857390123156866e+51_f64,
                nu: 50_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.4528262197760965e+50_f64,
                nu: 50_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -4.6566569870478635e+48_f64,
                nu: 50_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.8661134361400254e+47_f64,
                nu: 50_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -9.10058836122554e+45_f64,
                nu: 50_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -5.281377754238614e+44_f64,
                nu: 50_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.579547772211647e+43_f64,
                nu: 50_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.788837017583893e+42_f64,
                nu: 50_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler016_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data017_30: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -6.052308058585676e+245_f64,
                nu: 100_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -4.776690378041267e+215_f64,
                nu: 100_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.1758283017660654e+198_f64,
                nu: 100_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.7752878101091316e+185_f64,
                nu: 100_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -7.701329072999519e+175_f64,
                nu: 100_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -9.31526247942888e+167_f64,
                nu: 100_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.8854163374247264e+161_f64,
                nu: 100_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.0008260488566283e+155_f64,
                nu: 100_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.3075650873777408e+150_f64,
                nu: 100_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -6.147625856136938e+145_f64,
                nu: 100_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -4.475881623482959e+141_f64,
                nu: 100_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -7.474796102354786e+137_f64,
                nu: 100_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.5067022766901547e+134_f64,
                nu: 100_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.5222488313432757e+131_f64,
                nu: 100_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.5422392812241399e+128_f64,
                nu: 100_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.4400857387551062e+125_f64,
                nu: 100_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -5.711815339242229e+122_f64,
                nu: 100_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.8915420905193392e+120_f64,
                nu: 100_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -8.535794510477016e+117_f64,
                nu: 100_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -5.08486391601962e+115_f64,
                nu: 100_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler017_31: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data018_32: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -0.3085176252490336_f64,
                nu: 0_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.05567116728359946_f64,
                nu: 0_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.20546429603891822_f64,
                nu: 0_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.06264059680938405_f64,
                nu: 0_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.12724943226800617_f64,
                nu: 0_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.11729573168666413_f64,
                nu: 0_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.04579798719515569_f64,
                nu: 0_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.12593641705826092_f64,
                nu: 0_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.027060469763313333_f64,
                nu: 0_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.09806499547007712_f64,
                nu: 0_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.0775691787304126_f64,
                nu: 0_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.047358952209449426_f64,
                nu: 0_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.09718355774018192_f64,
                nu: 0_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.009309666458941_f64,
                nu: 0_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.08536904764777566_f64,
                nu: 0_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.055620339089770016_f64,
                nu: 0_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.04956788495149425_f64,
                nu: 0_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.07977647585487775_f64,
                nu: 0_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.0028230995861232107_f64,
                nu: 0_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.07724431336508315_f64,
                nu: 0_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler018_33: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-10_f64 });
pub static mut data019_34: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -0.1819232112934385_f64,
                nu: 0.3333333333333333_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.1702011178826876_f64,
                nu: 0.3333333333333333_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.18540507541540796_f64,
                nu: 0.3333333333333333_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.028777707635715043_f64,
                nu: 0.3333333333333333_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.15829741864944163_f64,
                nu: 0.3333333333333333_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.05864577231670521_f64,
                nu: 0.3333333333333333_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.10294930308870617_f64,
                nu: 0.3333333333333333_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.10547870367098922_f64,
                nu: 0.3333333333333333_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.034334228816010816_f64,
                nu: 0.3333333333333333_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.1128348993303128_f64,
                nu: 0.3333333333333333_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.030007358986895105_f64,
                nu: 0.3333333333333333_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08669917329571812_f64,
                nu: 0.3333333333333333_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.07487557966887866_f64,
                nu: 0.3333333333333333_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.03932324637455268_f64,
                nu: 0.3333333333333333_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.09126353957447524_f64,
                nu: 0.3333333333333333_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.013358849535984318_f64,
                nu: 0.3333333333333333_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.0783735755378302_f64,
                nu: 0.3333333333333333_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.05581248288395594_f64,
                nu: 0.3333333333333333_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.04331038010699068_f64,
                nu: 0.3333333333333333_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.07690050496213656_f64,
                nu: 0.3333333333333333_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler019_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data020_36: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -0.10121770918510846_f64,
                nu: 0.5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.2117088663313981_f64,
                nu: 0.5_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.15650551590730855_f64,
                nu: 0.5_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.07280690478506194_f64,
                nu: 0.5_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.15817308404205055_f64,
                nu: 0.5_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.022470290598831138_f64,
                nu: 0.5_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.12187835265849535_f64,
                nu: 0.5_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08413865567639538_f64,
                nu: 0.5_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.0624826419330032_f64,
                nu: 0.5_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.10888475635053954_f64,
                nu: 0.5_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.0023805454010949376_f64,
                nu: 0.5_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.09810468373503792_f64,
                nu: 0.5_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.055663470218594434_f64,
                nu: 0.5_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.06039676788382487_f64,
                nu: 0.5_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.08492257892204687_f64,
                nu: 0.5_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.009847227192444128_f64,
                nu: 0.5_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08519064357434362_f64,
                nu: 0.5_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.03768497043715627_f64,
                nu: 0.5_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.0597729048560975_f64,
                nu: 0.5_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.06880309146872811_f64,
                nu: 0.5_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler020_37: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-11_f64 });
pub static mut data021_38: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -0.016050662643389627_f64,
                nu: 0.6666666666666666_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.23937232657540733_f64,
                nu: 0.6666666666666666_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.11762106604241235_f64,
                nu: 0.6666666666666666_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.11182254014899558_f64,
                nu: 0.6666666666666666_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.14756582982938804_f64,
                nu: 0.6666666666666666_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.015078692908077713_f64,
                nu: 0.6666666666666666_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.13260911815705795_f64,
                nu: 0.6666666666666666_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.0572175659896527_f64,
                nu: 0.6666666666666666_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.086373755152382_f64,
                nu: 0.6666666666666666_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.09762413920805162_f64,
                nu: 0.6666666666666666_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.025354902147023392_f64,
                nu: 0.6666666666666666_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.10288136476351206_f64,
                nu: 0.6666666666666666_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.0327283795601282_f64,
                nu: 0.6666666666666666_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.07736367273574782_f64,
                nu: 0.6666666666666666_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.07285587045829396_f64,
                nu: 0.6666666666666666_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.03235810604695354_f64,
                nu: 0.6666666666666666_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08624065153739423_f64,
                nu: 0.6666666666666666_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.01702960169728519_f64,
                nu: 0.6666666666666666_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.07217352056058468_f64,
                nu: 0.6666666666666666_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.05605733920407389_f64,
                nu: 0.6666666666666666_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler021_39: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data022_40: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: 0.147863143391227_f64,
                nu: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.24901542420695386_f64,
                nu: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.021073628036873522_f64,
                nu: 1_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.16551161436252115_f64,
                nu: 1_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.09882996478323741_f64,
                nu: 1_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08442557066174713_f64,
                nu: 1_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.1275127335455901_f64,
                nu: 1_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.005793505821549754_f64,
                nu: 1_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.11552517964639945_f64,
                nu: 1_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.05679566856201469_f64,
                nu: 1_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.07384626543257793_f64,
                nu: 1_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.09186960936986689_f64,
                nu: 1_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.017940374275377362_f64,
                nu: 1_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.09484465262571623_f64,
                nu: 1_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.03521378516058042_f64,
                nu: 1_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.06939591378458804_f64,
                nu: 1_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.07123318758274977_f64,
                nu: 1_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.026187238607768244_f64,
                nu: 1_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.08182795872450122_f64,
                nu: 1_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.020372312002759834_f64,
                nu: 1_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler022_41: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data023_42: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: 0.3676628826055244_f64,
                nu: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.005868082442208683_f64,
                nu: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.20265447896733507_f64,
                nu: 2_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.07919175824563617_f64,
                nu: 2_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.11934303508534717_f64,
                nu: 2_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.12292410306411394_f64,
                nu: 2_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.03851154527826483_f64,
                nu: 2_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.1262260923493384_f64,
                nu: 2_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.032194922192042195_f64,
                nu: 2_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.09579316872759654_f64,
                nu: 2_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08025449747341543_f64,
                nu: 2_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.04429663189712053_f64,
                nu: 2_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.09773556925634738_f64,
                nu: 2_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.012019513676818605_f64,
                nu: 2_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08443001337682685_f64,
                nu: 2_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.05735523693438472_f64,
                nu: 2_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.0478918099495472_f64,
                nu: 2_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.08035841449060593_f64,
                nu: 2_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.0011004057182389746_f64,
                nu: 2_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.07683686712502796_f64,
                nu: 2_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler023_43: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000017e-10_f64 });
pub static mut data024_44: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -0.45369482249110193_f64,
                nu: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.13540304768936234_f64,
                nu: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.16717271575940015_f64,
                nu: 5_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.1000357678895322_f64,
                nu: 5_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.14705799311372267_f64,
                nu: 5_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.0316273592892643_f64,
                nu: 5_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.13554781474770028_f64,
                nu: 5_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.03186944878085025_f64,
                nu: 5_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.10426932700176872_f64,
                nu: 5_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.0785484139130816_f64,
                nu: 5_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.055257033062858375_f64,
                nu: 5_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.09946463284045089_f64,
                nu: 5_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.00023860469499595305_f64,
                nu: 5_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.09186180221640605_f64,
                nu: 5_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.04838367129697004_f64,
                nu: 5_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.060293667104896316_f64,
                nu: 5_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.077506166682734_f64,
                nu: 5_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.015338764062239767_f64,
                nu: 5_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.08153150404551436_f64,
                nu: 5_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.029480196281661937_f64,
                nu: 5_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler024_45: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000013e-09_f64 });
pub static mut data025_46: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -25.129110095610095_f64,
                nu: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.3598141521834028_f64,
                nu: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.21997141360195577_f64,
                nu: 10_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.043894653515658105_f64,
                nu: 10_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.1487183904998065_f64,
                nu: 10_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.07505670212239701_f64,
                nu: 10_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.12222473135000546_f64,
                nu: 10_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.046723877232677985_f64,
                nu: 10_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.1173933900932218_f64,
                nu: 10_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.005723897182053593_f64,
                nu: 10_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.1073391012583163_f64,
                nu: 10_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.03629035055954548_f64,
                nu: 10_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.08323912769171567_f64,
                nu: 10_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.06963938413831486_f64,
                nu: 10_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.045798335061325066_f64,
                nu: 10_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.08626919506484446_f64,
                nu: 10_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.001823467412624874_f64,
                nu: 10_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.08206776237123128_f64,
                nu: 10_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.03879807475457809_f64,
                nu: 10_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.05833157423641491_f64,
                nu: 10_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler025_47: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000017e-10_f64 });
pub static mut data026_48: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -593396529.691432_f64,
                nu: 20_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1597.483848269626_f64,
                nu: 20_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.308733092473762_f64,
                nu: 20_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.2854894586002032_f64,
                nu: 20_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.19804074776289243_f64,
                nu: 20_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.16848153948742683_f64,
                nu: 20_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.10102784152594022_f64,
                nu: 20_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.045161820565805755_f64,
                nu: 20_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.12556489308015448_f64,
                nu: 20_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.016442633948115834_f64,
                nu: 20_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.10853448778255181_f64,
                nu: 20_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.0267214085206647_f64,
                nu: 20_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.09878042525632418_f64,
                nu: 20_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.016201957786018233_f64,
                nu: 20_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.09359119826506372_f64,
                nu: 20_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.00404844007372962_f64,
                nu: 20_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.08631492945992053_f64,
                nu: 20_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.02827411009723153_f64,
                nu: 20_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.07234952079163874_f64,
                nu: 20_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.051247973076188474_f64,
                nu: 20_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler026_49: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-10_f64 });
pub static mut data027_50: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -2.788837017583893e+42_f64,
                nu: 50_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.641066501800742e+27_f64,
                nu: 50_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -10929732912175415296_f64,
                nu: 50_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -15606426801663.734_f64,
                nu: 50_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -753573251.4466274_f64,
                nu: 50_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -386759.3260273476_f64,
                nu: 50_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1172.8690492895323_f64,
                nu: 50_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -15.615608873419944_f64,
                nu: 50_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.870583462041769_f64,
                nu: 50_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.21031655464397747_f64,
                nu: 50_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.09304824041299939_f64,
                nu: 50_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.008641769962674475_f64,
                nu: 50_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.025019788459222037_f64,
                nu: 50_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.0014815155191909152_f64,
                nu: 50_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.05033577473216412_f64,
                nu: 50_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.09292425096798723_f64,
                nu: 50_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.0873324630302057_f64,
                nu: 50_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.01616423770165186_f64,
                nu: 50_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.06889761382045793_f64,
                nu: 50_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.07650526394480305_f64,
                nu: 50_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler027_51: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000017e-10_f64 });
pub static mut data028_52: std::cell::LazyCell<[testcase_cyl_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -5.08486391601962e+115_f64,
                nu: 100_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -4.849148271180025e+85_f64,
                nu: 100_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.637595532319532e+68_f64,
                nu: 100_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -8.200264814467911e+55_f64,
                nu: 100_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.971221643256237e+46_f64,
                nu: 100_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -7.287528470824075e+38_f64,
                nu: 100_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.425107990210798e+32_f64,
                nu: 100_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1.4552439438102214e+27_f64,
                nu: 100_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3.4506612476220073e+22_f64,
                nu: 100_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -3293800188202547712_f64,
                nu: 100_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1005686182055520.4_f64,
                nu: 100_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -831892881402.0903_f64,
                nu: 100_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1650863778.0598392_f64,
                nu: 100_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -7192614.197609779_f64,
                nu: 100_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -64639.072261231595_f64,
                nu: 100_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -1152.5905185698466_f64,
                nu: 100_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -40.25076140210149_f64,
                nu: 100_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -2.8307771387185294_f64,
                nu: 100_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.4576220049590485_f64,
                nu: 100_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.16692141141757652_f64,
                nu: 100_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler028_53: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-11_f64 });
pub static mut data029_54: std::cell::LazyCell<[testcase_cyl_neumann_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_neumann_double_ {
                f0: -0.022438688257729954_f64,
                nu: 100_f64,
                x: 1000_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.02107759515981999_f64,
                nu: 100_f64,
                x: 1100_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.0035299439206692585_f64,
                nu: 100_f64,
                x: 1200_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.014250019326536615_f64,
                nu: 100_f64,
                x: 1300_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.021304679089735663_f64,
                nu: 100_f64,
                x: 1400_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.015734395077905267_f64,
                nu: 100_f64,
                x: 1500_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.0025544633636137774_f64,
                nu: 100_f64,
                x: 1600_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.010722045524849367_f64,
                nu: 100_f64,
                x: 1700_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.018036919243226864_f64,
                nu: 100_f64,
                x: 1800_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.016958415593079763_f64,
                nu: 100_f64,
                x: 1900_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.008878870456627667_f64,
                nu: 100_f64,
                x: 2000_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.002250440710841318_f64,
                nu: 100_f64,
                x: 2100_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.01183321524671225_f64,
                nu: 100_f64,
                x: 2200_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.016398784536343945_f64,
                nu: 100_f64,
                x: 2300_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.014675984403642338_f64,
                nu: 100_f64,
                x: 2400_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: 0.007752392045165423_f64,
                nu: 100_f64,
                x: 2500_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.001575982257600349_f64,
                nu: 100_f64,
                x: 2600_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.009931487740478709_f64,
                nu: 100_f64,
                x: 2700_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.014534495161704743_f64,
                nu: 100_f64,
                x: 2800_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.014059273497237509_f64,
                nu: 100_f64,
                x: 2900_f64,
                f: 0_f64,
            },
            testcase_cyl_neumann_double_ {
                f0: -0.008938515814960519_f64,
                nu: 100_f64,
                x: 3000_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler029_55: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-10_f64 });
pub unsafe fn test_56(data: *const [testcase_cyl_neumann_double_; 20], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 20_();
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
                __builtin_fprintf_57 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 802_() ) .into() , ( c"void test(const testcase_cyl_neumann<Ret> (&)[Num], Ret) [Ret = double, Num = 20U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_58(data: *const [testcase_cyl_neumann_double_; 21], mut toler: ()) {
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
                __builtin_fprintf_57 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 802_() ) .into() , ( c"void test(const testcase_cyl_neumann<Ret> (&)[Num], Ret) [Ret = double, Num = 21U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
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
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_11)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_13)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_14)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_15)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_16)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_17)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_18)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_19)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_20)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_21)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_22)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_23)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_24)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_25)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_26)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_27)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_28)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_29)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data017_30)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler017_31)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data018_32)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler018_33)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data019_34)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler019_35)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data020_36)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler020_37)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data021_38)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler021_39)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data022_40)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler022_41)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data023_42)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler023_43)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data024_44)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler024_45)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data025_46)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler025_47)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data026_48)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler026_49)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data027_50)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler027_51)),
        )
    });
    (unsafe {
        test_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data028_52)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler028_53)),
        )
    });
    (unsafe {
        test_58(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data029_54)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler029_55)),
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
        this.pid_ = ((unsafe { getpid_59() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_60(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_61(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_62(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_63(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_64(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_63(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_64(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_63((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_64(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_65(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_63(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_64(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_66(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_67(
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
pub unsafe fn run_tests_wrapped_locale_68(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_57 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_69(
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
        if ((self.pid_) == (unsafe { getpid_59() })) {
            (unsafe { semctl_61(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_70(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_70(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_71(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72));
}
static mut tmp_72: std::cell::LazyCell<::libc::tm> =
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
    std::cell::LazyCell::force(&*&raw const data028_52);
    std::cell::LazyCell::force(&*&raw const toler028_53);
    std::cell::LazyCell::force(&*&raw const data029_54);
    std::cell::LazyCell::force(&*&raw const toler029_55);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
