extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const ()) -> *const () {
    c"";
    return value;
}
pub unsafe fn is_string_asan_correct_1(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_2(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_3(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_4(_a0: *const ()) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_8)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_5)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7)) = 0_();
    }
}
pub static mut outstanding_bytes_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_char_ {}
impl safe_allocator_char_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        let mut memory: *mut () = unimplemented!();
        {
            (unsafe {
                memset_9(
                    (memory as ()),
                    0_(),
                    (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                )
            });
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_9(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_char_ {
    fn default() -> Self {
        unsafe { safe_allocator_char_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: (),
    pub throw_after: (),
    pub count: (),
    pub alloc_count: (),
    pub allocated_size: (),
    pub construct_count: (),
    pub destroy_count: (),
    pub copied: (),
    pub moved: (),
    pub converted: (),
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        unimplemented!();
        self.count = 0_();
        self.time_to_throw = 0_();
        self.alloc_count = 0_();
        self.allocated_size = 0_();
        self.construct_count = 0_();
        self.destroy_count = 0_();
        self.throw_after = 2147483647_();
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0_();
        self.moved = 0_();
        self.converted = 0_();
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
pub static mut destructed_value_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_12(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_13(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_12(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0_(),
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: (), mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: (),
        mut id: (),
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_10));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_10));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> () {
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
            operator_eq_12(
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
                data_: 0_(),
                id_: 0_(),
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct limited_alloc_handle_96_ {
    pub outstanding_: (),
    pub last_alloc_: (),
}
impl limited_alloc_handle_96_ {
    pub unsafe fn allocate_1_char(&mut self, mut N: ()) -> *mut () {
        if (((N).wrapping_add(self.outstanding_)) > (96_())) {
            unimplemented!();
        }
        let mut alloc: *mut () = unimplemented!();
        self.last_alloc_ = (alloc as ());
        self.outstanding_ = (self.outstanding_).wrapping_add(N);
        return alloc;
    }
    pub unsafe fn deallocate_2_char(&mut self, mut ptr: *mut (), mut N: ()) {
        if ((ptr as ()) == (self.last_alloc_)) {
            self.last_alloc_ = unimplemented!();
            unimplemented!();
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        unimplemented!();
    }
}
impl Default for limited_alloc_handle_96_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_96_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
pub unsafe fn make_thread_unsafe_shared_14()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__ {
    let mut ptr: detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__ =
        <detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__>::default();
    let _dtor_ptr = ScopedDestructorUnsafe::new(
        &raw mut ptr,
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__::destructor,
    );
    let mut alloc: () = unimplemented!();
    ptr.block = unimplemented!();
    (unsafe {
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_96___control_block__::construct ( & mut alloc , ptr . block  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__::copy_from({ &ptr });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_96___control_block {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_96_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_96___control_block {
    pub ref_count: (),
    pub content: limited_alloc_handle_96_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_96___control_block {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_96___control_block {
                ref_count: 1_(),
                content: <limited_alloc_handle_96_>::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__ {
    block: *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_96___control_block,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__ {
    pub unsafe fn copy_from(
        other: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__,
    ) -> Self {
        let mut this = Self {
            block: (*other).block,
        };
        (*this.block).ref_count.prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.block).ref_count.prefix_dec();
        if (((*self.block).ref_count) != (0_())) {
            return;
        };
        let mut alloc: () = unimplemented!();
        (unsafe {
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_96___control_block__::destroy ( & mut alloc , self . block  , )
        });
        unimplemented!();
    }
    pub unsafe fn operator_arrow_7(&mut self) -> *mut limited_alloc_handle_96_ {
        return (&mut (*self.block).content as *mut limited_alloc_handle_96_);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__,
    ) -> *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__ {
        self.block = (*_a0).block;
        return &mut (*(self as *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__));
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__ {
    fn clone(&self) -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__::copy_from(
                self as *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__,
            )
        }
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__ {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__ {
                block: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_char__96__rebind_char_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct limited_allocator_char__96_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__,
}
impl limited_allocator_char__96_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            handle_: (unsafe { make_thread_unsafe_shared_14() }),
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return (unsafe {
            limited_alloc_handle_96_::allocate_1_char(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                n,
            )
        });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        (unsafe {
            limited_alloc_handle_96_::deallocate_2_char(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                p,
                n,
            )
        });
    }
    pub unsafe fn max_size(&self) -> () {
        return 96_();
    }
}
impl limited_allocator_char__96_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_96__::destructor(&mut self.handle_);
    }
}
impl Default for limited_allocator_char__96_ {
    fn default() -> Self {
        unsafe { limited_allocator_char__96_::new() }
    }
}
pub unsafe fn test_15(mut s: (), mut str: *const (), mut pos: (), mut n: (), mut expected: ()) {
    let mut str_len: () = unimplemented!();
    if ((pos) <= (str_len)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_16(mut s: (), mut str: *const (), mut pos: (), mut n: (), mut expected: ()) {
    let mut str_len: () = unimplemented!();
    if ((pos) <= (str_len)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_17(mut s: (), mut str: *const (), mut pos: (), mut n: (), mut expected: ()) {
    let mut str_len: () = unimplemented!();
    if ((pos) <= (str_len)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_18(mut s: (), mut str: *const (), mut pos: (), mut n: (), mut expected: ()) {
    let mut str_len: () = unimplemented!();
    if ((pos) <= (str_len)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_string_19() {
    (unsafe { test_15(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe { test_15(unimplemented!(), c"".as_ptr(), 1_(), 0_(), unimplemented!()) });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345".as_ptr(),
            0_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345".as_ptr(),
            3_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345".as_ptr(),
            5_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345".as_ptr(),
            6_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            2_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            12_(),
            13_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            21_(),
            13_(),
            unimplemented!(),
        )
    });
    (unsafe { test_15(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345".as_ptr(),
            2_(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"1234567890".as_ptr(),
            0_(),
            100_(),
            unimplemented!(),
        )
    });
    (unsafe { test_15(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe { test_15(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_15(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_15 ( unimplemented!() , c"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890" .as_ptr() , 5_() , 10_() , unimplemented!() , )
    });
}
pub unsafe fn test_string_20() {
    (unsafe { test_16(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe { test_16(unimplemented!(), c"".as_ptr(), 1_(), 0_(), unimplemented!()) });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345".as_ptr(),
            0_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345".as_ptr(),
            3_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345".as_ptr(),
            5_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345".as_ptr(),
            6_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            2_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            12_(),
            13_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            21_(),
            13_(),
            unimplemented!(),
        )
    });
    (unsafe { test_16(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345".as_ptr(),
            2_(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"1234567890".as_ptr(),
            0_(),
            100_(),
            unimplemented!(),
        )
    });
    (unsafe { test_16(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe { test_16(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_16(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_16 ( unimplemented!() , c"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890" .as_ptr() , 5_() , 10_() , unimplemented!() , )
    });
}
pub unsafe fn test_string_21() {
    (unsafe { test_17(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe { test_17(unimplemented!(), c"".as_ptr(), 1_(), 0_(), unimplemented!()) });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345".as_ptr(),
            0_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345".as_ptr(),
            3_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345".as_ptr(),
            5_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345".as_ptr(),
            6_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            2_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            12_(),
            13_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            21_(),
            13_(),
            unimplemented!(),
        )
    });
    (unsafe { test_17(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345".as_ptr(),
            2_(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"1234567890".as_ptr(),
            0_(),
            100_(),
            unimplemented!(),
        )
    });
    (unsafe { test_17(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe { test_17(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_17(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_17 ( unimplemented!() , c"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890" .as_ptr() , 5_() , 10_() , unimplemented!() , )
    });
}
pub unsafe fn test_string_22() {
    (unsafe { test_18(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe { test_18(unimplemented!(), c"".as_ptr(), 1_(), 0_(), unimplemented!()) });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345".as_ptr(),
            0_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345".as_ptr(),
            3_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345".as_ptr(),
            5_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345".as_ptr(),
            6_(),
            15_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            2_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            12_(),
            13_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            21_(),
            13_(),
            unimplemented!(),
        )
    });
    (unsafe { test_18(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345".as_ptr(),
            2_(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"1234567890".as_ptr(),
            0_(),
            100_(),
            unimplemented!(),
        )
    });
    (unsafe { test_18(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345678901234567890".as_ptr(),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe { test_18(unimplemented!(), c"".as_ptr(), 0_(), 0_(), unimplemented!()) });
    (unsafe {
        test_18(
            unimplemented!(),
            c"12345".as_ptr(),
            1_(),
            3_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_18 ( unimplemented!() , c"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890" .as_ptr() , 5_() , 10_() , unimplemented!() , )
    });
}
pub unsafe fn test_23() -> () {
    (unsafe { test_string_19() });
    (unsafe { test_string_20() });
    (unsafe { test_string_21() });
    (unsafe { test_string_22() });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_23() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_5);
    std::cell::LazyCell::force(&*&raw const alloc_count_6);
    std::cell::LazyCell::force(&*&raw const dealloc_count_7);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_8);
    std::cell::LazyCell::force(&*&raw const destructed_value_10);
    std::cell::LazyCell::force(&*&raw const moved_value_11);
}
