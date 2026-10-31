extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_1(x: *const DefaultOnly, y: *const DefaultOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const DefaultOnly, y: *const DefaultOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct DefaultOnly {
    data_: i32,
}
impl DefaultOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self { data_: -1_i32 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_dec();
    }
}
impl std::cmp::Ord for DefaultOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_2(self as *const DefaultOnly, other as *const DefaultOnly) {
                std::cmp::Ordering::Less
            } else if operator_lt_2(other as *const DefaultOnly, self as *const DefaultOnly) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for DefaultOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const DefaultOnly, other as *const DefaultOnly) }
    }
}
impl std::cmp::Eq for DefaultOnly {}
impl Default for DefaultOnly {
    fn default() -> Self {
        unsafe { DefaultOnly::new() }
    }
}
pub static mut count_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_6)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_3)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) = 0_usize;
    }
}
pub static mut outstanding_bytes_3: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_4: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_5: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_6: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn operator_eq_7(LHS: *const detail_TypeID, RHS: *const detail_TypeID) -> bool {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_8(LHS: *const detail_TypeID, RHS: *const detail_TypeID) -> bool {
    return (((*LHS).m_id) != ((*RHS).m_id));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_TypeID {
    m_id: *const i32,
}
impl detail_TypeID {
    unsafe fn new(mut xid: *const i32) -> Self {
        let mut this = Self { m_id: xid };
        this
    }
}
impl std::cmp::PartialEq for detail_TypeID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_7(self as *const detail_TypeID, other as *const detail_TypeID) }
    }
}
impl std::cmp::Eq for detail_TypeID {}
static mut dummy_addr_9: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 42 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_TypeInfo_detail_ArgumentListID_const_int_ref__ {}
impl detail_TypeInfo_detail_ArgumentListID_const_int_ref__ {
    pub unsafe fn GetID() -> *const detail_TypeID {
        return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_10));
    }
}
pub unsafe fn makeTypeID_11() -> *const detail_TypeID {
    return (unsafe { detail_TypeInfo_detail_ArgumentListID_const_int_ref__::GetID() });
}
pub unsafe fn makeArgumentID_12() -> *const detail_TypeID {
    return (unsafe { makeTypeID_11() });
}
static mut id_10: std::cell::LazyCell<detail_TypeID> = std::cell::LazyCell::new(|| unsafe {
    detail_TypeID::new({
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_addr_9)) as *const i32)
    })
});
#[repr(C)]
#[derive(FnPtrArg)]
pub struct AllocatorConstructController {
    pub m_expected_args: *const detail_TypeID,
    pub m_allow_constructions: bool,
    pub m_allow_unchecked: bool,
    pub m_expected_count: i32,
}
impl AllocatorConstructController {
    pub unsafe fn clear(&mut self) {
        self.m_expected_args = std::ptr::null();
        self.m_expected_count = -1_i32;
    }
    pub unsafe fn check(&mut self, tid: *const detail_TypeID) -> bool {
        if !(!(self.m_expected_args).is_null()) {
            assert!(self.m_allow_unchecked);
            return self.m_allow_unchecked;
        }
        let mut res: bool = (unsafe {
            let _LHS: *const detail_TypeID = &(*self.m_expected_args);
            let _RHS: *const detail_TypeID = tid;
            operator_eq_7(_LHS, _RHS)
        });
        if ((self.m_expected_count) == (-1_i32))
            || ((self.m_expected_count.prefix_dec()) == (-1_i32))
        {
            self.m_expected_args = std::ptr::null();
        }
        return res;
    }
    pub unsafe fn unchecked(&mut self) -> bool {
        return !((self.m_expected_args).is_null());
    }
    pub unsafe fn expect(&mut self, tid: *const detail_TypeID) {
        assert!(!(unsafe { AllocatorConstructController::unchecked(self,) }));
        self.m_expected_args = (tid);
    }
    pub unsafe fn isInAllocatorConstruct(&self) -> bool {
        return self.m_allow_constructions;
    }
    pub unsafe fn inAllocatorConstruct(&mut self, mut value: Option<bool>) {
        let mut value: bool = value.unwrap_or_else(|| unsafe { true });
        self.m_allow_constructions = value;
    }
    pub unsafe fn allowUnchecked(&mut self, mut value: Option<bool>) {
        let mut value: bool = value.unwrap_or_else(|| unsafe { true });
        self.m_allow_unchecked = value;
    }
    pub unsafe fn reset(&mut self) {
        self.m_allow_constructions = false;
        self.m_expected_args = std::ptr::null();
        self.m_allow_unchecked = false;
        self.m_expected_count = -1_i32;
    }
    unsafe fn new() -> Self {
        let mut this = Self {
            m_expected_args: std::ptr::null(),
            m_allow_constructions: false,
            m_allow_unchecked: false,
            m_expected_count: 0_i32,
        };
        (unsafe { AllocatorConstructController::reset(&mut this) });
        this
    }
    pub unsafe fn expect_5_const_int_ref(&mut self, mut times: i32) {
        assert!(!(unsafe { AllocatorConstructController::unchecked(self,) }));
        assert!(((times) > (0)));
        self.m_expected_count = ((times) - (1));
        self.m_expected_args = (unsafe { makeArgumentID_12() });
    }
    pub unsafe fn check_6_const_int_ref(&mut self) -> bool {
        return (unsafe {
            AllocatorConstructController::check(self, (unsafe { makeArgumentID_12() }))
        });
    }
}
impl Default for AllocatorConstructController {
    fn default() -> Self {
        unsafe { AllocatorConstructController::new() }
    }
}
pub unsafe fn getConstructController_13() -> *mut AllocatorConstructController {
    return (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_14))
        as *mut AllocatorConstructController);
}
static mut c_14: std::cell::LazyCell<AllocatorConstructController> =
    std::cell::LazyCell::new(|| unsafe { AllocatorConstructController::new() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ContainerTestAllocator_int__int_ {
    pub construct_called: i32,
    pub destroy_called: i32,
    pub controller: *mut AllocatorConstructController,
}
impl ContainerTestAllocator_int__int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ContainerTestAllocator_float__int_ {
    pub construct_called: i32,
    pub destroy_called: i32,
    pub controller: *mut AllocatorConstructController,
}
impl ContainerTestAllocator_float__int_ {}
const _: () = assert!(true, "std::is_same<A1T::rebind_traits<float>, A2T>::value");
const _: () = assert!(true, "std::is_same<A2T::rebind_traits<int>, A1T>::value");
pub unsafe fn test_15() -> bool {
    {
        let t: [i32; 5] = [0, 1, 2, 3, 4];
        let mut c: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (3_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (0)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        unimplemented!();
        assert!(((unimplemented!()) == (6_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (0)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (10)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()) == (10)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()) == (10)));
        unimplemented!();
        assert!(((unimplemented!()) == (6_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (0)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (10)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()) == (10)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()) == (10)));
    }
    {
        let t: [i32; 5] = [0, 1, 2, 3, 4];
        let mut c: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (3_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (0)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        unimplemented!();
        assert!(((unimplemented!()) == (6_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (0)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (10)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()) == (10)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()) == (10)));
        unimplemented!();
        assert!(((unimplemented!()) == (6_isize)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (0)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (10)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()) == (10)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()) == (10)));
    }
    {
        let mut cc: *mut AllocatorConstructController = (unsafe { getConstructController_13() });
        (unsafe { AllocatorConstructController::reset(&mut (*cc)) });
        {
            let mut c: () = unimplemented!();
            (unsafe { AllocatorConstructController::expect_5_const_int_ref(&mut (*cc), 6) });
            unimplemented!();
            assert!(!(unsafe { AllocatorConstructController::unchecked(&mut (*cc),) }));
        }
    }
    return true;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    assert!((unsafe { test_15() }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_3);
    std::cell::LazyCell::force(&*&raw const alloc_count_4);
    std::cell::LazyCell::force(&*&raw const dealloc_count_5);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_6);
    std::cell::LazyCell::force(&*&raw const dummy_addr_9);
}
