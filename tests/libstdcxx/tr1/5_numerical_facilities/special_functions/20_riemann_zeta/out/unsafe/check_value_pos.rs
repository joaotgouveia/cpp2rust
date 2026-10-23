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
    pub x: (),
    pub f: (),
}
pub static mut data001_10: std::cell::LazyCell<[testcase_riemann_zeta_double_; 145]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_riemann_zeta_double_ {
                f0: 5.59158244117775_f64,
                x: 1.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 3.1055472779775792_f64,
                x: 1.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 2.2857656656801324_f64,
                x: 1.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.882229618102822_f64,
                x: 1.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.6449340668482275_f64,
                x: 2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.4905432565068937_f64,
                x: 2.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.383342858840736_f64,
                x: 2.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.3054778090727803_f64,
                x: 2.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.247031422317254_f64,
                x: 2.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.2020569031595945_f64,
                x: 3_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.1667733709844674_f64,
                x: 3.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.138663775728042_f64,
                x: 3.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.1159890791233376_f64,
                x: 3.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0975105764590047_f64,
                x: 3.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0823232337111381_f64,
                x: 4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0697514772338095_f64,
                x: 4.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0592817259798355_f64,
                x: 4.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0505173825665735_f64,
                x: 4.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.043148013335179_f64,
                x: 4.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.03692775514337_f64,
                x: 5_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0316598766779168_f64,
                x: 5.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0271855389203537_f64,
                x: 5.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.02337547922703_f64,
                x: 5.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0201237683883446_f64,
                x: 5.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0173430619844492_f64,
                x: 6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0149609451852233_f64,
                x: 6.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.012917088712184_f64,
                x: 6.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0111610141542708_f64,
                x: 6.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.009650322344712_f64,
                x: 6.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.008349277381923_f64,
                x: 7_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.007227666480717_f64,
                x: 7.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0062598756930512_f64,
                x: 7.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0054241359879634_f64,
                x: 7.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0047019048164696_f64,
                x: 7.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0040773561979444_f64,
                x: 8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0035369583062013_f64,
                x: 8.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0030691220374448_f64,
                x: 8.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0026639074861505_f64,
                x: 8.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.002312777909822_f64,
                x: 8.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0020083928260823_f64,
                x: 9_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0017444334995897_f64,
                x: 9.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0015154553480514_f64,
                x: 9.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0013167628052648_f64,
                x: 9.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0011443029840295_f64,
                x: 9.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0009945751278182_f64,
                x: 10_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0008645533615086_f64,
                x: 10.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0007516206744649_f64,
                x: 10.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0006535124140847_f64,
                x: 10.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0005682678503411_f64,
                x: 10.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0004941886041194_f64,
                x: 11_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0004298029239944_f64,
                x: 11.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0003738349551168_f64,
                x: 11.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0003251782761946_f64,
                x: 11.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000282873090999_f64,
                x: 11.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000246086553308_f64,
                x: 12_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000214095781875_f64,
                x: 12.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0001862731874056_f64,
                x: 12.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000162073788746_f64,
                x: 12.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0001410242422089_f64,
                x: 12.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0001227133475783_f64,
                x: 13_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0001067838280169_f64,
                x: 13.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000929252097515_f64,
                x: 13.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000808676518718_f64,
                x: 13.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000703765974504_f64,
                x: 13.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000612481350588_f64,
                x: 14_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000533049750668_f64,
                x: 14.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000463929582293_f64,
                x: 14.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000403780253397_f64,
                x: 14.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000351435864272_f64,
                x: 14.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000030588236307_f64,
                x: 15_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000266237704787_f64,
                x: 15.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000231734615617_f64,
                x: 15.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000201705617975_f64,
                x: 15.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000017557001761_f64,
                x: 15.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000152822594086_f64,
                x: 16_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000133023770337_f64,
                x: 16.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000011579106683_f64,
                x: 16.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000100791671644_f64,
                x: 16.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000008773598001_f64,
                x: 16.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000076371976379_f64,
                x: 17_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000066480348633_f64,
                x: 17.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000057870238734_f64,
                x: 17.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000050375546607_f64,
                x: 17.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000043851715013_f64,
                x: 17.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000038172932648_f64,
                x: 18_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000033229700953_f64,
                x: 18.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000028926717153_f64,
                x: 18.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000002518103242_f64,
                x: 18.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000021920449287_f64,
                x: 18.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000019082127167_f64,
                x: 19_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000016611368951_f64,
                x: 19.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000014460565094_f64,
                x: 19.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000012588274738_f64,
                x: 19.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000010958426055_f64,
                x: 19.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000009539620338_f64,
                x: 20_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000008304526344_f64,
                x: 20.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000007229353187_f64,
                x: 20.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000006293391575_f64,
                x: 20.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000005478614529_f64,
                x: 20.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000004769329869_f64,
                x: 21_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000415187772_f64,
                x: 21.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000003614367254_f64,
                x: 21.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000003146447527_f64,
                x: 21.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000273910802_f64,
                x: 21.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000238450503_f64,
                x: 22_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000002075810521_f64,
                x: 22.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001807080625_f64,
                x: 22.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001573141093_f64,
                x: 22.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001369487659_f64,
                x: 22.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001192199262_f64,
                x: 23_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000103786252_f64,
                x: 23.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000903506006_f64,
                x: 23.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000078654301_f64,
                x: 23.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000684721728_f64,
                x: 23.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000059608189_f64,
                x: 24_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000051891702_f64,
                x: 24.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000451741575_f64,
                x: 24.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000393262332_f64,
                x: 24.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.00000003423535_f64,
                x: 24.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000298035037_f64,
                x: 25_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000259453767_f64,
                x: 25.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000225866978_f64,
                x: 25.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000019662811_f64,
                x: 25.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000171174297_f64,
                x: 25.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000149015549_f64,
                x: 26_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000129725304_f64,
                x: 26.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000112932221_f64,
                x: 26.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000098313035_f64,
                x: 26.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000085586331_f64,
                x: 26.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000074507118_f64,
                x: 27_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000064862125_f64,
                x: 27.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000056465688_f64,
                x: 27.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000004915618_f64,
                x: 27.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000042792894_f64,
                x: 27.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000003725334_f64,
                x: 28_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000032430887_f64,
                x: 28.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000028232703_f64,
                x: 28.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000024577977_f64,
                x: 28.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000021396356_f64,
                x: 28.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000018626598_f64,
                x: 29_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000016215385_f64,
                x: 29.2_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000014116306_f64,
                x: 29.4_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000012288952_f64,
                x: 29.6_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000010698147_f64,
                x: 29.8_f64,
                f: unimplemented!(),
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000009313275_f64,
                x: 30_f64,
                f: unimplemented!(),
            },
        ]
    });
pub unsafe fn test001_11() {
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = ((::std::mem::size_of::<[testcase_riemann_zeta_double_; 145]>() as ())
        .wrapping_div((::std::mem::size_of::<testcase_riemann_zeta_double_>() as ())));
    let mut i: () = 0_();
    'loop_: while ((i) < (num_datum)) {
        let f: () = unimplemented!();
        let f0: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut data001_10))[(i) as usize].f0;
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
        if !((max_abs_frac) < (2.500000000000002e-13_f64)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"check_value_pos.cc".as_ptr()).into(),
                        (220_()).into(),
                        (c"void test001() [Tp = double]".as_ptr()).into(),
                        (c"max_abs_frac < Tp(2.5000000000000020e-13)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
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
    (unsafe { test001_11() });
    return 0_();
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_13() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_14(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_15(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_16(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_17(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_18(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_17(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_18(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_17((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_18(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_19(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_17(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_18(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_20(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_21(
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
pub unsafe fn run_tests_wrapped_locale_22(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_12 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_23(
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
        if ((self.pid_) == (unsafe { getpid_13() })) {
            (unsafe { semctl_15(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_24(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_24(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_25(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26));
}
static mut tmp_26: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const data001_10);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
