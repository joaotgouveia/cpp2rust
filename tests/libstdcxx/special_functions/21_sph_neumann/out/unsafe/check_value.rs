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
pub struct testcase_sph_neumann_double_ {
    pub f0: (),
    pub n: (),
    pub x: (),
    pub f: (),
}
pub static mut data001_10: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -3.875649686842579_f64,
                n: 0_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.7551651237807455_f64,
                n: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.9755851584984279_f64,
                n: 0_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.5403023058681398_f64,
                n: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.25225788991621495_f64,
                n: 0_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.04715813444513527_f64,
                n: 0_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.1018548889425669_f64,
                n: 0_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.2080734182735712_f64,
                n: 0_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.27918827676566177_f64,
                n: 0_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.3204574462187735_f64,
                n: 0_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.3361099558663504_f64,
                n: 0_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.3299974988668151_f64,
                n: 0_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.30588605417862963_f64,
                n: 0_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.2675590535116561_f64,
                n: 0_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.21881582862388288_f64,
                n: 0_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.16341090521590299_f64,
                n: 0_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.10496176233265714_f64,
                n: 0_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.04684351098461772_f64,
                n: 0_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.007916242713258222_f64,
                n: 0_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.05673243709264526_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data002_12: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -16.492214584388407_f64,
                n: 1_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.469181324769897_f64,
                n: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.209631891362349_f64,
                n: 1_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.3817732906760363_f64,
                n: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.9609940074174409_f64,
                n: 1_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.6964354140327931_f64,
                n: 1_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.5040748902464972_f64,
                n: 1_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.35061200427605527_f64,
                n: 1_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.2217266311654487_f64,
                n: 1_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.11120587915407318_f64,
                n: 1_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.016564013158538646_f64,
                n: 1_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.06295916360231597_f64,
                n: 1_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.12740959652576553_f64,
                n: 1_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.17666922320036457_f64,
                n: 1_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.21076723929766045_f64,
                n: 1_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.2300533501309578_f64,
                n: 1_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.23528261660264485_f64,
                n: 1_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.22763858414438104_f64,
                n: 1_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.2087108518446568_f64,
                n: 1_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.18043836751409864_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data003_14: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -194.03092532581832_f64,
                n: 2_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -25.059922824838637_f64,
                n: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.862942406950969_f64,
                n: 2_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.605017566159969_f64,
                n: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.054127727885643_f64,
                n: 2_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.3457126936204509_f64,
                n: 2_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.9659832722222763_f64,
                n: 2_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.733991424687654_f64,
                n: 2_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.5748237849862601_f64,
                n: 2_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.4539045012036613_f64,
                n: 2_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.35417978840293796_f64,
                n: 2_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.26703833526449916_f64,
                n: 2_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.18827719584715374_f64,
                n: 2_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.11612829076848646_f64,
                n: 2_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.0502020371857545_f64,
                n: 2_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.009129107382315343_f64,
                n: 2_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.06112008468097453_f64,
                n: 2_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.10491554511163632_f64,
                n: 2_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.13973362282567303_f64,
                n: 2_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.16499545760110443_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_16: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -3884190.062663773_f64,
                n: 5_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -61327.56316698064_f64,
                n: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -5478.952932319084_f64,
                n: 5_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -999.4403433922364_f64,
                n: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -270.4972050294236_f64,
                n: 5_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -94.23611008523247_f64,
                n: 5_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -39.18282778658433_f64,
                n: 5_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -18.591445311190984_f64,
                n: 5_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -9.782142020318227_f64,
                n: 5_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -5.599100154806323_f64,
                n: 5_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.4400655233636823_f64,
                n: 5_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.2470233284653904_f64,
                n: 5_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.549143994577916_f64,
                n: 5_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.1205896325654248_f64,
                n: 5_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.8459225560519484_f64,
                n: 5_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.6628012664504588_f64,
                n: 5_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.5358937443603853_f64,
                n: 5_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.4443032422909055_f64,
                n: 5_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.3752015723289989_f64,
                n: 5_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.3204650467497392_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_18: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -2750653598174213.5_f64,
                n: 10_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1349739281107.0554_f64,
                n: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -15733380424.95376_f64,
                n: 10_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -672215008.2562084_f64,
                n: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -58607405.98867945_f64,
                n: 10_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -8032728.814823481_f64,
                n: 10_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1505955.5720640516_f64,
                n: 10_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -355414.72008543846_f64,
                n: 10_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -100086.80374425423_f64,
                n: 10_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -32423.79408533442_f64,
                n: 10_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -11772.863161809979_f64,
                n: 10_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4699.859188811392_f64,
                n: 10_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2033.0183273853759_f64,
                n: 10_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -942.1907502842549_f64,
                n: 10_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -463.65206971202474_f64,
                n: 10_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -240.5355298798893_f64,
                n: 10_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -130.78478404631085_f64,
                n: 10_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -74.17066550173753_f64,
                n: 10_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -43.69824989818498_f64,
                n: 10_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -26.65611440571871_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data006_20: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -1.407759140254225e+36_f64,
                n: 20_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.728876183823471e+29_f64,
                n: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.3544611382105945e+26_f64,
                n: 20_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.239592218578983e+23_f64,
                n: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.009641671595306e+21_f64,
                n: 20_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -65999646851668172800_f64,
                n: 20_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2619336475307073536_f64,
                n: 20_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -160543649281522240_f64,
                n: 20_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -13719071872797762_f64,
                n: 20_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1524247248298953.8_f64,
                n: 20_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -209484650509384.06_f64,
                n: 20_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -34327545666696.49_f64,
                n: 20_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6522260876203.317_f64,
                n: 20_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1406018871897.2307_f64,
                n: 20_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -338025193731.7888_f64,
                n: 20_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -89381690326.01868_f64,
                n: 20_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -25701805899.474934_f64,
                n: 20_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7961859734.240776_f64,
                n: 20_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2636237230.085001_f64,
                n: 20_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -926795140.3057547_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data007_22: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -1.382374280800406e+109_f64,
                n: 50_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.144791292212169e+93_f64,
                n: 50_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.434849490890053e+84_f64,
                n: 50_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.739192284629757e+78_f64,
                n: 50_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.136503757329993e+73_f64,
                n: 50_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.8821098528635756e+69_f64,
                n: 50_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.1148255024189452e+66_f64,
                n: 50_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.235021944367097e+63_f64,
                n: 50_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.0565226939717125e+60_f64,
                n: 50_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.4262702131152733e+58_f64,
                n: 50_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.1118745474840939e+56_f64,
                n: 50_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.3243260716629503e+54_f64,
                n: 50_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.251947209412933e+52_f64,
                n: 50_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -5.186150720110329e+50_f64,
                n: 50_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.5513212909462263e+49_f64,
                n: 50_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -5.827647140790065e+47_f64,
                n: 50_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.6745414086543416e+46_f64,
                n: 50_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.4657308996352946e+45_f64,
                n: 50_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -9.410267436669065e+43_f64,
                n: 50_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.964109188270132e+42_f64,
                n: 50_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data008_24: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -4.285610946051641e+247_f64,
                n: 100_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.691172001175378e+217_f64,
                n: 100_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.7753107402139484e+199_f64,
                n: 100_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.683079463258677e+186_f64,
                n: 100_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.0906342369729277e+177_f64,
                n: 100_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.0993184254131119e+169_f64,
                n: 100_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.9071480498141315e+162_f64,
                n: 100_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.6559558301924957e+156_f64,
                n: 100_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.8154136926485787e+151_f64,
                n: 100_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.352763166211138e+146_f64,
                n: 100_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.880953701410059e+142_f64,
                n: 100_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.410222995302561e+138_f64,
                n: 100_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.365190415404474e+135_f64,
                n: 100_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.698074910106327e+131_f64,
                n: 100_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.279055349924666e+128_f64,
                n: 100_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.0796647795891517e+126_f64,
                n: 100_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.3785795774441247e+123_f64,
                n: 100_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.439159663194743e+120_f64,
                n: 100_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.1802258278275794e+118_f64,
                n: 100_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.799713982625566e+116_f64,
                n: 100_(),
                x: 5_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data009_26: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -0.05673243709264526_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.08390715290764525_f64,
                n: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.05064586085725475_f64,
                n: 0_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.020404103090669597_f64,
                n: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.03964811247453894_f64,
                n: 0_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.005141714996252802_f64,
                n: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.025819777288328762_f64,
                n: 0_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.016673451541306544_f64,
                n: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.011673821973727327_f64,
                n: 0_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.019299320569842265_f64,
                n: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.00040230465930828606_f64,
                n: 0_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.015873549673585938_f64,
                n: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.008653136172894954_f64,
                n: 0_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.00904741718694714_f64,
                n: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.012290016929663325_f64,
                n: 0_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0013798405479880944_f64,
                n: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.011580901686988727_f64,
                n: 0_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.004978595734768557_f64,
                n: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.007686037484155996_f64,
                n: 0_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.00862318872287684_f64,
                n: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-11_f64 });
pub static mut data010_28: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: 0.18043836751409864_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0627928263797015_f64,
                n: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.03997613195332415_f64,
                n: 1_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.046667467690914864_f64,
                n: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0037081455049293634_f64,
                n: 1_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.032762996969886965_f64,
                n: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012971498479556563_f64,
                n: 1_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.018210992723451058_f64,
                n: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.01916838547795213_f64,
                n: 1_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.00486151066268173_f64,
                n: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.018170052158169303_f64,
                n: 1_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.005344736179596711_f64,
                n: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.012587316051033977_f64,
                n: 1_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.01118482998206909_f64,
                n: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.005006554913063562_f64,
                n: 1_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012440856180892041_f64,
                n: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.002207723783947951_f64,
                n: 1_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.009877978531842104_f64,
                n: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.007273134233897652_f64,
                n: 1_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.00497742452386882_f64,
                n: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data011_30: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: 0.16499545760110443_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.06506930499373478_f64,
                n: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.058641087247919575_f64,
                n: 2_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.01340398293703237_f64,
                n: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.04009308993513046_f64,
                n: 2_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.008418014693241499_f64,
                n: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.024707934561509628_f64,
                n: 2_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.018039275995565374_f64,
                n: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010395929608530518_f64,
                n: 2_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.01959101120960317_f64,
                n: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.001393398413390248_f64,
                n: 2_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.015606312864606101_f64,
                n: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.009234089221404215_f64,
                n: 2_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.008568067330572752_f64,
                n: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012490279126185866_f64,
                n: 2_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.0009133084412046427_f64,
                n: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.01150298202402586_f64,
                n: 2_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.005307861685829961_f64,
                n: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.00745635956098028_f64,
                n: 2_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.008772511458592905_f64,
                n: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_31: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-11_f64 });
pub static mut data012_32: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -0.3204650467497392_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.09383354167869182_f64,
                n: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.02047569828185906_f64,
                n: 5_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.04817234775737278_f64,
                n: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.018309489232548347_f64,
                n: 5_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.026639390496569996_f64,
                n: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.02200603898557621_f64,
                n: 5_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.011268975348057965_f64,
                n: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.02177038837227486_f64,
                n: 5_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.000697113196458537_f64,
                n: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0174395894502209_f64,
                n: 5_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.008869917091934309_f64,
                n: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.010421334444951861_f64,
                n: 5_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.012746769858008553_f64,
                n: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0026282888028967737_f64,
                n: 5_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.01247765858132419_f64,
                n: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.004077181681818264_f64,
                n: 5_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.008977775957057982_f64,
                n: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.008318455789667615_f64,
                n: 5_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0037206784862748965_f64,
                n: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_33: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-11_f64 });
pub static mut data013_34: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -26.65611440571871_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.17245367208805784_f64,
                n: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.07846168984964258_f64,
                n: 10_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.03684341049628996_f64,
                n: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.021158339301097475_f64,
                n: 10_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.03121959106475494_f64,
                n: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012840593422414807_f64,
                n: 10_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.021803068636888072_f64,
                n: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.014071636804469044_f64,
                n: 10_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.013524687511158758_f64,
                n: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.015684932653180595_f64,
                n: 10_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.005635689556726212_f64,
                n: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.015364490270315362_f64,
                n: 10_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.0014525575672261295_f64,
                n: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012648951699549433_f64,
                n: 10_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.006857160806112037_f64,
                n: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.008015115294140146_f64,
                n: 10_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.009813974221901915_f64,
                n: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.002500285407231495_f64,
                n: 10_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010025777373636155_f64,
                n: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-11_f64 });
pub static mut data014_36: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -926795140.3057547_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1211.2106053526036_f64,
                n: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.5559965765652175_f64,
                n: 20_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.0934011322509144_f64,
                n: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.04403198567527646_f64,
                n: 20_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.03607803360661391_f64,
                n: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.029828405631319645_f64,
                n: 20_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.004841481098676076_f64,
                n: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.020504694681516944_f64,
                n: 20_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.013759531302541216_f64,
                n: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012783038861734196_f64,
                n: 20_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.013117009421906418_f64,
                n: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.010338106075674407_f64,
                n: 20_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010538610814111244_f64,
                n: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010200029094273744_f64,
                n: 20_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.007312345094561712_f64,
                n: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.010581510354950906_f64,
                n: 20_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0036866374015298723_f64,
                n: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.01049838431833827_f64,
                n: 20_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 5.631729378833498e-05_f64,
                n: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_37: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000007e-09_f64 });
pub static mut data015_38: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -6.964109188270132e+42_f64,
                n: 50_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.528227272351331e+27_f64,
                n: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -9000490264588767232_f64,
                n: 50_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -9542541667002.576_f64,
                n: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -363518140.7102718_f64,
                n: 50_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -152551.57233157742_f64,
                n: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -386.2659918620869_f64,
                n: 50_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.32905079472911_f64,
                n: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.19968460851503717_f64,
                n: 50_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.04190000150460776_f64,
                n: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010696040672421926_f64,
                n: 50_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.007819876855526828_f64,
                n: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.010088474938191183_f64,
                n: 50_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0062423671279823656_f64,
                n: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0011284242794942282_f64,
                n: 50_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.00939342660374863_f64,
                n: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.013108079602843421_f64,
                n: 50_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.007539660722572193_f64,
                n: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.00426057035528373_f64,
                n: 50_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010747822973682467_f64,
                n: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler015_39: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data016_40: std::cell::LazyCell<[testcase_sph_neumann_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_sph_neumann_double_ {
                f0: -1.799713982625566e+116_f64,
                n: 100_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -8.573226309328894e+85_f64,
                n: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.9270658593710037e+68_f64,
                n: 100_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.220889358295033e+55_f64,
                n: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.086875261300736e+46_f64,
                n: 100_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.249612402361085e+38_f64,
                n: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.704289834891003e+32_f64,
                n: 100_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.302156526072277e+26_f64,
                n: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.3199917400493998e+22_f64,
                n: 100_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1125692891326591104_f64,
                n: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -309801083340345.56_f64,
                n: 100_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -232585620046.64404_f64,
                n: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -421135935.93755186_f64,
                n: 100_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1680637.4531202628_f64,
                n: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -13868.302591128748_f64,
                n: 100_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -227.24385709173205_f64,
                n: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.280703878713856_f64,
                n: 100_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.4664815444825085_f64,
                n: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.06727077272065451_f64,
                n: 100_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.02298385049156227_f64,
                n: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler016_41: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub unsafe fn test_42(data: *const [testcase_sph_neumann_double_; 20], mut toler: ()) {
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
                __builtin_fprintf_43 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 577_() ) .into() , ( c"void test(const testcase_sph_neumann<Ret> (&)[Num], Ret) [Ret = double, Num = 20U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
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
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_11)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_13)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_14)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_15)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_16)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_17)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_18)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_19)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_20)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_21)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_22)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_23)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_24)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_25)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_26)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_27)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_28)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_29)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_30)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_31)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_32)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_33)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_34)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_35)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_36)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_37)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_38)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_39)),
        )
    });
    (unsafe {
        test_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_40)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_41)),
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
        this.pid_ = ((unsafe { getpid_44() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_45(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_46(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_47(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_48(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_49(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_48(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_49(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_48((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_49(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_50(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_48(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_49(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_51(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_52(
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
pub unsafe fn run_tests_wrapped_locale_53(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_43 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_54(
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
        if ((self.pid_) == (unsafe { getpid_44() })) {
            (unsafe { semctl_46(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_55(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_55(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_56(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_57)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_57)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_57)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_57)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_57)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_57)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_57)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_57)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_57)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_57));
}
static mut tmp_57: std::cell::LazyCell<::libc::tm> =
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
    std::cell::LazyCell::force(&*&raw const data015_38);
    std::cell::LazyCell::force(&*&raw const toler015_39);
    std::cell::LazyCell::force(&*&raw const data016_40);
    std::cell::LazyCell::force(&*&raw const toler016_41);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
