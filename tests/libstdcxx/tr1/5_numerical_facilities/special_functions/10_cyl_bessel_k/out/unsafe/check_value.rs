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
pub struct testcase_cyl_bessel_k_double_ {
    pub f0: (),
    pub nu: (),
    pub x: (),
    pub f: (),
}
pub static mut data007_10: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 1.5415067512483025_f64,
                nu: 0_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.9244190712276656_f64,
                nu: 0_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.6105824221164643_f64,
                nu: 0_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.42102443824070834_f64,
                nu: 0_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.2976030890841059_f64,
                nu: 0_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.21380556264752565_f64,
                nu: 0_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.1553798123866036_f64,
                nu: 0_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.1138938727495336_f64,
                nu: 0_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.08404311197465819_f64,
                nu: 0_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.06234755320036617_f64,
                nu: 0_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.046454901308760774_f64,
                nu: 0_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.034739504386279256_f64,
                nu: 0_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.026058755255154966_f64,
                nu: 0_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.0195988971703685_f64,
                nu: 0_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.014774250877128707_f64,
                nu: 0_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.01115967608585303_f64,
                nu: 0_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.008444387724542965_f64,
                nu: 0_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.006399857243233976_f64,
                nu: 0_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.004857204557887952_f64,
                nu: 0_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.0036910983340425947_f64,
                nu: 0_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_12: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 1.7144912564234513_f64,
                nu: 0.3333333333333333_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.9890310742467242_f64,
                nu: 0.3333333333333333_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.6421689966728299_f64,
                nu: 0.3333333333333333_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.43843063344153443_f64,
                nu: 0.3333333333333333_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.3078819241494504_f64,
                nu: 0.3333333333333333_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.22015769026776688_f64,
                nu: 0.3333333333333333_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.15943413057311243_f64,
                nu: 0.3333333333333333_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.11654496129616534_f64,
                nu: 0.3333333333333333_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.08580960930643967_f64,
                nu: 0.3333333333333333_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.06354253745473339_f64,
                nu: 0.3333333333333333_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.04727335418479551_f64,
                nu: 0.3333333333333333_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.03530590490216259_f64,
                nu: 0.3333333333333333_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.02645418689277317_f64,
                nu: 0.3333333333333333_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.019877061407943805_f64,
                nu: 0.3333333333333333_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.014971213514760216_f64,
                nu: 0.3333333333333333_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.011299947573672166_f64,
                nu: 0.3333333333333333_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.008544795954611047_f64,
                nu: 0.3333333333333333_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.0064720581217078245_f64,
                nu: 0.3333333333333333_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.004909334280327526_f64,
                nu: 0.3333333333333333_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.0037288750960535887_f64,
                nu: 0.3333333333333333_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data009_14: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 1.9521640631515476_f64,
                nu: 0.5_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.0750476034999195_f64,
                nu: 0.5_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.6836100603495243_f64,
                nu: 0.5_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.46106850444789443_f64,
                nu: 0.5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.32117137397144746_f64,
                nu: 0.5_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.2283350522282655_f64,
                nu: 0.5_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.1646362899738086_f64,
                nu: 0.5_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.11993777196806145_f64,
                nu: 0.5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.08806555880365045_f64,
                nu: 0.5_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.06506594315400999_f64,
                nu: 0.5_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.048315198301417825_f64,
                nu: 0.5_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.03602598513176459_f64,
                nu: 0.5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.02695635653244335_f64,
                nu: 0.5_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.020229969578139294_f64,
                nu: 0.5_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.015220888252975568_f64,
                nu: 0.5_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.011477624576608053_f64,
                nu: 0.5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.008671893295697834_f64,
                nu: 0.5_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.006563394564634542_f64,
                nu: 0.5_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.004975243542126229_f64,
                nu: 0.5_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.0037766133746428825_f64,
                nu: 0.5_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data010_16: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 2.32890607455441_f64,
                nu: 0.6666666666666666_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.2059304647203353_f64,
                nu: 0.6666666666666666_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.7454723297664723_f64,
                nu: 0.6666666666666666_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.4944750621042081_f64,
                nu: 0.6666666666666666_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.3406299481351425_f64,
                nu: 0.6666666666666666_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.2402404524031558_f64,
                nu: 0.6666666666666666_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.17217716908452307_f64,
                nu: 0.6666666666666666_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.12483892748812841_f64,
                nu: 0.6666666666666666_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.09131529607962105_f64,
                nu: 0.6666666666666666_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.06725532217162336_f64,
                nu: 0.6666666666666666_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.049809546542402224_f64,
                nu: 0.6666666666666666_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.03705707449518853_f64,
                nu: 0.6666666666666666_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.02767436550488673_f64,
                nu: 0.6666666666666666_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.020733915836010912_f64,
                nu: 0.6666666666666666_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.015577015510251336_f64,
                nu: 0.6666666666666666_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.011730801456525337_f64,
                nu: 0.6666666666666666_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.008852834320465885_f64,
                nu: 0.6666666666666666_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.006693319091577557_f64,
                nu: 0.6666666666666666_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.005068929210625548_f64,
                nu: 0.6666666666666666_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.0038444246344968226_f64,
                nu: 0.6666666666666666_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data011_18: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 3.7470259744407115_f64,
                nu: 1_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.6564411200033007_f64,
                nu: 1_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.9495804669621402_f64,
                nu: 1_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.6019072301972346_f64,
                nu: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.4021240797841954_f64,
                nu: 1_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.27738780045684375_f64,
                nu: 1_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.19547745347439305_f64,
                nu: 1_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.13986588181652262_f64,
                nu: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.10121630256832535_f64,
                nu: 1_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.07389081634774704_f64,
                nu: 1_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.05431852275891986_f64,
                nu: 1_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.0401564311281942_f64,
                nu: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.029825529796040143_f64,
                nu: 1_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.022239392925923845_f64,
                nu: 1_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.016638191754688916_f64,
                nu: 1_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.012483498887268437_f64,
                nu: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.009389680656043259_f64,
                nu: 1_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.007078094908968092_f64,
                nu: 1_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.005345921817822839_f64,
                nu: 1_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.0040446134454521655_f64,
                nu: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data012_20: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 31.517714546774_f64,
                nu: 2_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.5501835512408695_f64,
                nu: 2_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.1427970006821715_f64,
                nu: 2_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.6248388986351774_f64,
                nu: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.9410016167388185_f64,
                nu: 2_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.5836559632566507_f64,
                nu: 2_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.3787826163573385_f64,
                nu: 2_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.2537597545660562_f64,
                nu: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.1740131587020585_f64,
                nu: 2_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.12146020627856381_f64,
                nu: 2_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.08595928149706614_f64,
                nu: 2_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.06151045847174206_f64,
                nu: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.044412927437333515_f64,
                nu: 2_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.03230712169946784_f64,
                nu: 2_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.02364795314629613_f64,
                nu: 2_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.017401425529487247_f64,
                nu: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.012863060974445674_f64,
                nu: 2_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.00954567720277535_f64,
                nu: 2_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.007108119007497569_f64,
                nu: 2_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.005308943712223461_f64,
                nu: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data013_22: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 391683.9896233489_f64,
                nu: 5_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 12097.979476096394_f64,
                nu: 5_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1562.5870339691098_f64,
                nu: 5_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 360.9605896012407_f64,
                nu: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 114.29321426334016_f64,
                nu: 5_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 44.067781159301056_f64,
                nu: 5_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 19.42656868773029_f64,
                nu: 5_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.431049100596482_f64,
                nu: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.9221270549918685_f64,
                nu: 5_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.7168842907865423_f64,
                nu: 5_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.5677685890536335_f64,
                nu: 5_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.9377736023868082_f64,
                nu: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.5777553473678511_f64,
                nu: 5_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.3648244020845198_f64,
                nu: 5_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.2352029062008226_f64,
                nu: 5_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.15434254872599726_f64,
                nu: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.10283347176876455_f64,
                nu: 5_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.06942364315088177_f64,
                nu: 5_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.04741061691794221_f64,
                nu: 5_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.032706273712031865_f64,
                nu: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data014_24: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 194481817927839.88_f64,
                nu: 10_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 188937569319.9003_f64,
                nu: 10_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3248187687.8018155_f64,
                nu: 10_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 180713289.90102944_f64,
                nu: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 19104425.94525218_f64,
                nu: 10_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3027483.5236822353_f64,
                nu: 10_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 633724.7155508787_f64,
                nu: 10_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 162482.40397955943_f64,
                nu: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 48602.44608774979_f64,
                nu: 10_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 16406.916416341937_f64,
                nu: 10_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6104.172074590961_f64,
                nu: 10_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2459.620422056948_f64,
                nu: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1059.235844370338_f64,
                nu: 10_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 482.5358209666476_f64,
                nu: 10_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 230.64249314993782_f64,
                nu: 10_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 114.91408364049623_f64,
                nu: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 59.36161363270648_f64,
                nu: 10_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 31.652958759229872_f64,
                nu: 10_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 17.3577239664174_f64,
                nu: 10_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.758562829177812_f64,
                nu: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data015_26: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 7.006598366164118e+34_f64,
                nu: 20_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6.665549874417159e+28_f64,
                nu: 20_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.996298961538038e+25_f64,
                nu: 20_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6.2943693604245335e+22_f64,
                nu: 20_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 720345119200741818368_f64,
                nu: 20_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 18620549984645545984_f64,
                nu: 20_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 844156053039524864_f64,
                nu: 20_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 57708568527002520_f64,
                nu: 20_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5396824209986879_f64,
                nu: 20_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 645996884063683.6_f64,
                nu: 20_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 94387401970996.33_f64,
                nu: 20_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 16254643952204.371_f64,
                nu: 20_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3212694836166.4053_f64,
                nu: 20_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 713857897923.7407_f64,
                nu: 20_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 175423421958.35925_f64,
                nu: 20_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 47050078926.29809_f64,
                nu: 20_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 13625066095.067503_f64,
                nu: 20_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4222179870.681066_f64,
                nu: 20_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1389634112.7516634_f64,
                nu: 20_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 482700052.0621487_f64,
                nu: 20_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler015_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data016_28: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 4.3394604622138714e+107_f64,
                nu: 50_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.8505298918269003e+92_f64,
                nu: 50_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6.029275689484279e+83_f64,
                nu: 50_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.4068968541617e+77_f64,
                nu: 50_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.848552736503905e+72_f64,
                nu: 50_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.309171757490792e+68_f64,
                nu: 50_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.3762245257445824e+65_f64,
                nu: 50_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.979981739604927e+62_f64,
                nu: 50_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.207943123348858e+59_f64,
                nu: 50_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.20465282129875e+57_f64,
                nu: 50_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.5578676911884825e+55_f64,
                nu: 50_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.5559542293221535e+53_f64,
                nu: 50_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.2606735967629e+51_f64,
                nu: 50_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.0139406747903812e+50_f64,
                nu: 50_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6.336872783748461e+48_f64,
                nu: 50_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.4897317389325757e+47_f64,
                nu: 50_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.18889581730397e+46_f64,
                nu: 50_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6.747259364814855e+44_f64,
                nu: 50_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.46642665859307e+43_f64,
                nu: 50_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.394322243430163e+42_f64,
                nu: 50_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler016_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data017_30: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 9.50394281158099e+245_f64,
                nu: 100_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.493739931353311e+215_f64,
                nu: 100_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.84174710207307e+198_f64,
                nu: 100_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.900333183638641e+185_f64,
                nu: 100_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.200213093557695e+176_f64,
                nu: 100_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.4467044226487075e+168_f64,
                nu: 100_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.916149841149764e+161_f64,
                nu: 100_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.6194159776013925e+155_f64,
                nu: 100_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.533212158354173e+150_f64,
                nu: 100_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.356609723103994e+145_f64,
                nu: 100_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6.767228361513453e+141_f64,
                nu: 100_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.1219630864949494e+138_f64,
                nu: 100_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.73297236999909e+134_f64,
                nu: 100_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.2476893883855163e+131_f64,
                nu: 100_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.25645593198832e+128_f64,
                nu: 100_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.535334049962646e+125_f64,
                nu: 100_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.189843921301023e+122_f64,
                nu: 100_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.68237441107268e+120_f64,
                nu: 100_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.1963963615212274e+118_f64,
                nu: 100_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.0398601930616815e+115_f64,
                nu: 100_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler017_31: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-12_f64 });
pub static mut data018_32: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 0.0036910983340425947_f64,
                nu: 0_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.7780062316167647e-05_f64,
                nu: 0_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.819536482396433e-08_f64,
                nu: 0_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.741237815336524e-10_f64,
                nu: 0_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.464161562213115e-12_f64,
                nu: 0_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.1324774964630566e-14_f64,
                nu: 0_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.3310351491429464e-16_f64,
                nu: 0_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.39286110009957e-19_f64,
                nu: 0_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.333456122618725e-21_f64,
                nu: 0_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.4101677497894956e-23_f64,
                nu: 0_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.1913102183534147e-25_f64,
                nu: 0_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.4138978405591074e-27_f64,
                nu: 0_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.154467321003005e-30_f64,
                nu: 0_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.944661337292501e-32_f64,
                nu: 0_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.870117045586911e-34_f64,
                nu: 0_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.525119842505472e-36_f64,
                nu: 0_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.6507623579783908e-38_f64,
                nu: 0_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.0810242556984256e-40_f64,
                nu: 0_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.090124969900128e-43_f64,
                nu: 0_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.6566282291759025e-45_f64,
                nu: 0_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler018_33: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data019_34: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 0.0037288750960535887_f64,
                nu: 0.3333333333333333_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.7874608271055336e-05_f64,
                nu: 0.3333333333333333_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.854834156879832e-08_f64,
                nu: 0.3333333333333333_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.756827824779087e-10_f64,
                nu: 0.3333333333333333_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.471720142490706e-12_f64,
                nu: 0.3333333333333333_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.136366473661119e-14_f64,
                nu: 0.3333333333333333_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.3331202314165813e-16_f64,
                nu: 0.3333333333333333_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.404383776948093e-19_f64,
                nu: 0.3333333333333333_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.339973126102495e-21_f64,
                nu: 0.3333333333333333_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.413921781358363e-23_f64,
                nu: 0.3333333333333333_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.1935050179185627e-25_f64,
                nu: 0.3333333333333333_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.4151968805623662e-27_f64,
                nu: 0.3333333333333333_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.162235721701904e-30_f64,
                nu: 0.3333333333333333_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.949347970346131e-32_f64,
                nu: 0.3333333333333333_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.872966001105595e-34_f64,
                nu: 0.3333333333333333_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.5268631828013874e-36_f64,
                nu: 0.3333333333333333_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.6518353676138867e-38_f64,
                nu: 0.3333333333333333_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.0816880942511494e-40_f64,
                nu: 0.3333333333333333_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.094250859923151e-43_f64,
                nu: 0.3333333333333333_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.659203157021345e-45_f64,
                nu: 0.3333333333333333_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler019_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data020_36: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 0.0037766133746428825_f64,
                nu: 0.5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.7993478093705178e-05_f64,
                nu: 0.5_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.899131203287724e-08_f64,
                nu: 0.5_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.776373974707445e-10_f64,
                nu: 0.5_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.481191276840695e-12_f64,
                nu: 0.5_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.141237565956011e-14_f64,
                nu: 0.5_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.3357311366035824e-16_f64,
                nu: 0.5_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.418809194948905e-19_f64,
                nu: 0.5_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.348130500251741e-21_f64,
                nu: 0.5_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.4186200954570754e-23_f64,
                nu: 0.5_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.1962515908772453e-25_f64,
                nu: 0.5_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.4168223500353693e-27_f64,
                nu: 0.5_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.171955447325689e-30_f64,
                nu: 0.5_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.955211433778893e-32_f64,
                nu: 0.5_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.876530132140943e-34_f64,
                nu: 0.5_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.5290440439442907e-36_f64,
                nu: 0.5_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.653177606760598e-38_f64,
                nu: 0.5_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.0825184636529955e-40_f64,
                nu: 0.5_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.099411587325882e-43_f64,
                nu: 0.5_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.662423812634671e-45_f64,
                nu: 0.5_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler020_37: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data021_38: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 0.0038444246344968226_f64,
                nu: 0.6666666666666666_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.8161187569530204e-05_f64,
                nu: 0.6666666666666666_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.961475154230557e-08_f64,
                nu: 0.6666666666666666_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.803848427192581e-10_f64,
                nu: 0.6666666666666666_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.4944937498488603e-12_f64,
                nu: 0.6666666666666666_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.148075564557772e-14_f64,
                nu: 0.6666666666666666_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.339394919015216e-16_f64,
                nu: 0.6666666666666666_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.439046055364299e-19_f64,
                nu: 0.6666666666666666_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.359571614362209e-21_f64,
                nu: 0.6666666666666666_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.425208530143375e-23_f64,
                nu: 0.6666666666666666_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.200102537798231e-25_f64,
                nu: 0.6666666666666666_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.4191011274172078e-27_f64,
                nu: 0.6666666666666666_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.185580302026976e-30_f64,
                nu: 0.6666666666666666_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.963429947257876e-32_f64,
                nu: 0.6666666666666666_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.88152540264785e-34_f64,
                nu: 0.6666666666666666_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.5321003991943847e-36_f64,
                nu: 0.6666666666666666_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.6550585670593067e-38_f64,
                nu: 0.6666666666666666_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.0836820479428605e-40_f64,
                nu: 0.6666666666666666_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.106642891628536e-43_f64,
                nu: 0.6666666666666666_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.666936458728046e-45_f64,
                nu: 0.6666666666666666_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler021_39: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data022_40: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 0.0040446134454521655_f64,
                nu: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.864877345382558e-05_f64,
                nu: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.014172936976209e-07_f64,
                nu: 1_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.883057969557038e-10_f64,
                nu: 1_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.5327780731999345e-12_f64,
                nu: 1_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.1677320018915498e-14_f64,
                nu: 1_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.3499178340011053e-16_f64,
                nu: 1_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.4971319548610435e-19_f64,
                nu: 1_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.3923945937225035e-21_f64,
                nu: 1_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.4441022267175555e-23_f64,
                nu: 1_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.2111422716117463e-25_f64,
                nu: 1_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.4256320265171041e-27_f64,
                nu: 1_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.224619527890616e-30_f64,
                nu: 1_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.986973673913855e-32_f64,
                nu: 1_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.895832946742191e-34_f64,
                nu: 1_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.5408531275211705e-36_f64,
                nu: 1_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.660444494856757e-38_f64,
                nu: 1_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.0870134457498335e-40_f64,
                nu: 1_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.127344232990724e-43_f64,
                nu: 1_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.679853735636909e-45_f64,
                nu: 1_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler022_41: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data023_42: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 0.005308943712223461_f64,
                nu: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.1509817006932763e-05_f64,
                nu: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.1171767065031378e-07_f64,
                nu: 2_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6.329543612292228e-10_f64,
                nu: 2_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.74678380806911e-12_f64,
                nu: 2_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.2769929632558265e-14_f64,
                nu: 2_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.4081733110858665e-16_f64,
                nu: 2_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.817717697842622e-19_f64,
                nu: 2_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.573118104561947e-21_f64,
                nu: 2_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.547931838858198e-23_f64,
                nu: 2_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.271715391866569e-25_f64,
                nu: 2_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.4614189081096777e-27_f64,
                nu: 2_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.438301768015023e-30_f64,
                nu: 2_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6.115717727975754e-32_f64,
                nu: 2_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.974005924166703e-34_f64,
                nu: 2_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.5886411706935012e-36_f64,
                nu: 2_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.6898316402103142e-38_f64,
                nu: 2_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.1051801100484218e-40_f64,
                nu: 2_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.240174322173618e-43_f64,
                nu: 2_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.7502253038886407e-45_f64,
                nu: 2_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler023_43: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data024_44: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 0.032706273712031865_f64,
                nu: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.7541849985312275e-05_f64,
                nu: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.1878261369258224e-07_f64,
                nu: 5_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.0538660139974233e-09_f64,
                nu: 5_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.648592136528416e-12_f64,
                nu: 5_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.2103335105890266e-14_f64,
                nu: 5_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.8919208406439644e-16_f64,
                nu: 5_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.1423814375953188e-18_f64,
                nu: 5_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.01812168222041e-21_f64,
                nu: 5_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.367182254100986e-23_f64,
                nu: 5_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.744496764035787e-25_f64,
                nu: 5_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.7382232741886986e-27_f64,
                nu: 5_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.1078474298959669e-29_f64,
                nu: 5_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.097453708179442e-32_f64,
                nu: 5_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.5667269500061064e-34_f64,
                nu: 5_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.9491764420206146e-36_f64,
                nu: 5_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.9105685973117463e-38_f64,
                nu: 5_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.2411034311592645e-40_f64,
                nu: 5_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.081421133137915e-43_f64,
                nu: 5_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.27325611329295e-45_f64,
                nu: 5_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler024_45: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data025_46: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 9.758562829177812_f64,
                nu: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.0016142553003906698_f64,
                nu: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.2605303776606435e-06_f64,
                nu: 10_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6.31621452832158e-09_f64,
                nu: 10_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.4076769602801233e-11_f64,
                nu: 10_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.0842816942222975e-13_f64,
                nu: 10_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.397677042977719e-16_f64,
                nu: 10_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.868029311367193e-18_f64,
                nu: 10_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.59398719001696e-20_f64,
                nu: 10_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.150988209987996e-23_f64,
                nu: 10_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.382384624959286e-25_f64,
                nu: 10_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.2253408700563144e-27_f64,
                nu: 10_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.9613367530075138e-29_f64,
                nu: 10_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.2068471495933484e-31_f64,
                nu: 10_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.497915264944964e-34_f64,
                nu: 10_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.695728583049053e-36_f64,
                nu: 10_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.960632334703408e-38_f64,
                nu: 10_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.8773542561131613e-40_f64,
                nu: 10_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.196289952784635e-42_f64,
                nu: 10_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.6554279773881e-45_f64,
                nu: 10_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler025_47: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data026_48: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 482700052.0621487_f64,
                nu: 20_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 178.74427820770543_f64,
                nu: 20_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.012141257729731143_f64,
                nu: 20_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.5431116361258155e-06_f64,
                nu: 20_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 6.374402933035211e-09_f64,
                nu: 20_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.2304516475442478e-11_f64,
                nu: 20_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.267313647980902e-14_f64,
                nu: 20_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.0703023799997383e-16_f64,
                nu: 20_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.0549953175660457e-19_f64,
                nu: 20_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.706148379722035e-21_f64,
                nu: 20_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.761700811565941e-24_f64,
                nu: 20_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.748295400687472e-26_f64,
                nu: 20_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.8966880763956576e-28_f64,
                nu: 20_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.961576347999886e-31_f64,
                nu: 20_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.3921623063091066e-33_f64,
                nu: 20_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.9920407657642266e-35_f64,
                nu: 20_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.6948662723618255e-37_f64,
                nu: 20_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.768914964296304e-40_f64,
                nu: 20_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.7143603019220823e-42_f64,
                nu: 20_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.3852054148901695e-44_f64,
                nu: 20_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler026_49: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data027_50: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 3.394322243430163e+42_f64,
                nu: 50_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.0613737753892554e+27_f64,
                nu: 50_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1726773697451918848_f64,
                nu: 50_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 411711209122.0179_f64,
                nu: 50_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1972478.7419813874_f64,
                nu: 50_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 58.77068625800727_f64,
                nu: 50_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.005865939118253518_f64,
                nu: 50_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.3634854128794101e-06_f64,
                nu: 50_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.865239636216082e-10_f64,
                nu: 50_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.0060134766400893e-13_f64,
                nu: 50_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.9062324485711016e-16_f64,
                nu: 50_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 5.038929808517651e-19_f64,
                nu: 50_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.130534425011042e-22_f64,
                nu: 50_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.573281623494899e-24_f64,
                nu: 50_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.534985499387441e-27_f64,
                nu: 50_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.994010100318947e-30_f64,
                nu: 50_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.5403205503080723e-32_f64,
                nu: 50_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.839759648671572e-35_f64,
                nu: 50_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.609890065132954e-37_f64,
                nu: 50_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.274522653613324e-40_f64,
                nu: 50_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler027_51: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data028_52: std::cell::LazyCell<[testcase_cyl_bessel_k_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_cyl_bessel_k_double_ {
                f0: 7.0398601930616815e+115_f64,
                nu: 100_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 4.596674084269523e+85_f64,
                nu: 100_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.256555224265395e+67_f64,
                nu: 100_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.708135645687604e+55_f64,
                nu: 100_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.985802812878061e+45_f64,
                nu: 100_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.2131584253026677e+37_f64,
                nu: 100_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.1016916354696688e+30_f64,
                nu: 100_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.007402329777571e+23_f64,
                nu: 100_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1923664395847089408_f64,
                nu: 100_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 16394035276269.25_f64,
                nu: 100_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 343254952.89495474_f64,
                nu: 100_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 14870.012754946298_f64,
                nu: 100_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.1708099078572216_f64,
                nu: 100_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 0.00015161193930722313_f64,
                nu: 100_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 2.985023438162344e-08_f64,
                nu: 100_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 8.392871072464911e-12_f64,
                nu: 100_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 3.2033435630927732e-15_f64,
                nu: 100_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 1.5922281431788096e-18_f64,
                nu: 100_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 9.958945457767413e-22_f64,
                nu: 100_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_k_double_ {
                f0: 7.617129630494083e-25_f64,
                nu: 100_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler028_53: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub unsafe fn test_54(data: *const [testcase_cyl_bessel_k_double_; 20], mut toler: ()) {
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
                __builtin_fprintf_55 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 769_() ) .into() , ( c"void test(const testcase_cyl_bessel_k<Ret> (&)[Num], Ret) [Ret = double, Num = 20U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
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
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_11)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_13)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_14)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_15)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_16)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_17)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_18)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_19)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_20)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_21)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_22)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_23)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_24)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_25)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_26)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_27)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_28)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_29)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data017_30)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler017_31)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data018_32)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler018_33)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data019_34)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler019_35)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data020_36)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler020_37)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data021_38)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler021_39)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data022_40)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler022_41)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data023_42)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler023_43)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data024_44)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler024_45)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data025_46)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler025_47)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data026_48)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler026_49)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data027_50)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler027_51)),
        )
    });
    (unsafe {
        test_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data028_52)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler028_53)),
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
        this.pid_ = ((unsafe { getpid_56() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_57(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_58(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_59(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_60(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_61(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_60(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_61(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_60((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_61(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_62(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_60(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_61(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_63(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_64(
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
    let mut w: () = unimplemented!();
    if unimplemented!() {
        (unsafe { __throw_runtime_error_3(s) });
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_65(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                (unsafe {
                    __builtin_fprintf_55 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        (unsafe { __throw_runtime_error_3(unimplemented!()) });
    }
}
pub unsafe fn run_tests_wrapped_env_66(
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
        if ((self.pid_) == (unsafe { getpid_56() })) {
            (unsafe { semctl_58(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_67(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_67(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_68(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69));
}
static mut tmp_69: std::cell::LazyCell<::libc::tm> =
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
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
