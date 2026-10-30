extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: i32,
    pub throw_after: i32,
    pub count: i32,
    pub alloc_count: i32,
    pub allocated_size: i32,
    pub construct_count: i32,
    pub destroy_count: i32,
    pub copied: i32,
    pub moved: i32,
    pub converted: i32,
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        assert!(
            ((self.count) == (0)) && (!(c"clearing leaking allocator data?".as_ptr()).is_null())
        );
        self.count = 0;
        self.time_to_throw = 0;
        self.alloc_count = 0;
        self.allocated_size = 0;
        self.construct_count = 0;
        self.destroy_count = 0;
        self.throw_after = 2147483647;
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0;
        self.moved = 0;
        self.converted = 0;
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
pub static mut destructed_value_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_2(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_3(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_2(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: i32, mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: i32, mut id: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: i32,
        mut id: i32,
        mut stats: *mut test_allocator_statistics,
    ) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: stats,
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_void_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn get_id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const test_allocator_void_,
    ) -> *mut test_allocator_void_ {
        self.data_ = (*_a0).data_;
        self.id_ = (*_a0).id_;
        self.stats_ = (*_a0).stats_;
        return &mut (*(self as *mut test_allocator_void_));
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
                self as *const test_allocator_void_,
                other as *const test_allocator_void_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_void_::copy_from(self as *const test_allocator_void_) }
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        unsafe {
            test_allocator_void_ {
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Tag_X {}
impl Tag_X {
    pub unsafe fn new_1(mut _a0: Ctor_Tag) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: Ctor_Tag, _a1: *const Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_3(mut _a0: Ctor_Tag, _a1: *mut Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
}
pub unsafe fn operator_eq_5(x: *const DefaultOnly, y: *const DefaultOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_lt_6(x: *const DefaultOnly, y: *const DefaultOnly) -> bool {
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_dec();
    }
}
impl std::cmp::Ord for DefaultOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_6(self as *const DefaultOnly, other as *const DefaultOnly) {
                std::cmp::Ordering::Less
            } else if operator_lt_6(other as *const DefaultOnly, self as *const DefaultOnly) {
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
        unsafe { operator_eq_5(self as *const DefaultOnly, other as *const DefaultOnly) }
    }
}
impl std::cmp::Eq for DefaultOnly {}
impl Default for DefaultOnly {
    fn default() -> Self {
        unsafe { DefaultOnly::new() }
    }
}
pub static mut count_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_8))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_9)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_8))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_9))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_10)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_7)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_8)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_9)) = 0_usize;
    }
}
pub static mut outstanding_bytes_7: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_8: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_9: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_10: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_DefaultOnly_ {}
impl min_allocator_DefaultOnly_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_DefaultOnly_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe { <std_allocator_DefaultOnly_>::default().deallocate(p.ptr_, n) });
    }
}
pub unsafe fn operator_eq_11(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_int_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe { <std_allocator_int_>::default().deallocate(p.ptr_, n) });
    }
}
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
impl min_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_DefaultOnly_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( unsafe { <std_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___ >::default() . allocate  ( n , ) } )  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            <std_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___ >::default() . deallocate  ( p . ptr_  , n , )
        });
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( unsafe { <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ >::default() . allocate  ( n , ) } )  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>::default()
                .deallocate(p.ptr_, n)
        });
    }
}
impl min_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
pub unsafe fn test2_12(mut n: u32) {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)) == (0)));
    {
        let mut d: std_deque_DefaultOnly__min_allocator_DefaultOnly__ = {
            let mut __tmp_0: min_allocator_DefaultOnly_ = <min_allocator_DefaultOnly_>::default();
            std_deque_DefaultOnly__min_allocator_DefaultOnly__::new_1({ (n as u64) }, {
                Some(&mut __tmp_0)
            })
        };
        assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)) as u32) == (n)));
        assert!(((unsafe { d.size() }) == (n as u64)));
        assert!(
            (((unsafe {
                let ___first: UnsafeRandomAccessIterator<DefaultOnly> = (unsafe { d.begin_24() });
                let ___last: UnsafeRandomAccessIterator<DefaultOnly> = (unsafe { d.end_26() });
                distance_13(___first, ___last)
            }) as usize)
                == ((unsafe { d.size() }) as usize))
        );
        const _: () = assert!(true, "");;
        let mut i: UnsafeRandomAccessIterator<DefaultOnly> =
            <UnsafeRandomAccessIterator<DefaultOnly>>::from((unsafe { d.begin_24() }));
        let mut e: UnsafeRandomAccessIterator<DefaultOnly> =
            <UnsafeRandomAccessIterator<DefaultOnly>>::from((unsafe { d.end_26() }));
        'loop_: while !(i == e) {
            assert!(
                (unsafe {
                    let _x: *const DefaultOnly = &(*i.iter_deref());
                    let mut _y: DefaultOnly = DefaultOnly::new();
                    operator_eq_5(_x, &mut _y)
                })
            );
            i.iter_inc();
        }
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)) == (0)));
}
pub unsafe fn test1_14(mut n: u32) {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)) == (0)));
    {
        let mut d: std_deque_DefaultOnly__min_allocator_DefaultOnly__ =
            std_deque_DefaultOnly__min_allocator_DefaultOnly__::new_1({ (n as u64) }, None);
        assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)) as u32) == (n)));
        assert!(((unsafe { d.size() }) == (n as u64)));
        assert!(
            (((unsafe {
                let ___first: UnsafeRandomAccessIterator<DefaultOnly> = (unsafe { d.begin_24() });
                let ___last: UnsafeRandomAccessIterator<DefaultOnly> = (unsafe { d.end_26() });
                distance_13(___first, ___last)
            }) as usize)
                == ((unsafe { d.size() }) as usize))
        );
        const _: () = assert!(true, "");;
        let mut i: UnsafeRandomAccessIterator<DefaultOnly> =
            <UnsafeRandomAccessIterator<DefaultOnly>>::from((unsafe { d.begin_24() }));
        let mut e: UnsafeRandomAccessIterator<DefaultOnly> =
            <UnsafeRandomAccessIterator<DefaultOnly>>::from((unsafe { d.end_26() }));
        'loop_: while !(i == e) {
            assert!(
                (unsafe {
                    let _x: *const DefaultOnly = &(*i.iter_deref());
                    let mut _y: DefaultOnly = DefaultOnly::new();
                    operator_eq_5(_x, &mut _y)
                })
            );
            i.iter_inc();
        }
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)) == (0)));
}
pub unsafe fn test3_15(mut n: u32, alloc: Option<*const min_allocator_int_>) {
    let mut __alloc_default: Option<min_allocator_int_> = None;
    let mut alloc: *const min_allocator_int_ = alloc.unwrap_or_else(|| unsafe {
        __alloc_default.insert(<min_allocator_int_>::default()) as *const min_allocator_int_
    });
    {
        let mut d: std_deque_int__min_allocator_int__ =
            std_deque_int__min_allocator_int__::new_1({ (n as u64) }, { alloc });
        assert!(((unsafe { d.size() }) == (n as u64)));
        assert!(
            (unsafe {
                let _arg0: min_allocator_int_ = (unsafe { d.get_allocator() });
                let _arg1: min_allocator_int_ = (*alloc);
                operator_eq_11(_arg0, _arg1)
            })
        );
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_16(mut n: u32) {
    (unsafe { test1_14(n) });
    (unsafe { test2_12(n) });
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
    (unsafe { test_17(0_u32) });
    (unsafe { test_17(1_u32) });
    (unsafe { test_17(10_u32) });
    (unsafe { test_17(1023_u32) });
    (unsafe { test_17(1024_u32) });
    (unsafe { test_17(1025_u32) });
    (unsafe { test_17(2047_u32) });
    (unsafe { test_17(2048_u32) });
    (unsafe { test_17(2049_u32) });
    (unsafe { test_17(4095_u32) });
    (unsafe { test_17(4096_u32) });
    (unsafe { test_17(4097_u32) });
    const _: () = assert!(true, "");;
    (unsafe { test_16(4095_u32) });
    (unsafe { test3_18(1023_u32, None) });
    (unsafe { test3_19(1_u32, None) });
    (unsafe { test3_15(3_u32, None) });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
    std::cell::LazyCell::force(&*&raw const count_4);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_7);
    std::cell::LazyCell::force(&*&raw const alloc_count_8);
    std::cell::LazyCell::force(&*&raw const dealloc_count_9);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_10);
}
