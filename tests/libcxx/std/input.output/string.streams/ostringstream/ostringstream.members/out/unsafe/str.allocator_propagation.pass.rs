extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MultiStringType {
    pub w_: *const (),
    pub wn_: (),
    pub u8_: *const (),
    pub u8n_: (),
    pub u16_: *const (),
    pub u16n_: (),
    pub u32_: *const (),
    pub u32n_: (),
    pub s_: *const (),
    pub sn_: (),
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const (),
        mut wn: (),
        mut u8: *const (),
        mut u8n: (),
        mut u16: *const (),
        mut u16n: (),
        mut u32: *const (),
        mut u32n: (),
        mut s: *const (),
        mut sn: (),
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const ()) -> *const () {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const ()) -> *const () {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const ()) -> *const () {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const ()) -> *const () {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const ()) -> *const () {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const ()) -> () {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const ()) -> () {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const ()) -> () {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const ()) -> () {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const ()) -> () {
        return self.u32n_;
    }
    pub unsafe fn to_pconst_12(&self) -> *const () {
        return self.s_;
    }
    pub unsafe fn to_pconst_13(&self) -> *const () {
        return self.w_;
    }
    pub unsafe fn to_pconst_14(&self) -> *const () {
        return self.u8_;
    }
    pub unsafe fn to_pconst_15(&self) -> *const () {
        return self.u16_;
    }
    pub unsafe fn to_pconst_16(&self) -> *const () {
        return self.u32_;
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
pub static mut destructed_value_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_2(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_3(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_2(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
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
pub struct SocccAllocator_char_ {
    pub count_: (),
}
impl SocccAllocator_char_ {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { count_: i };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn select_on_container_copy_construction(&self) -> SocccAllocator_char_ {
        return SocccAllocator_char_::new({ ((self.count_) + (1_())) });
    }
    pub unsafe fn operator_eq(&self, _a0: *const SocccAllocator_char_) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for SocccAllocator_char_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { SocccAllocator_char_::operator_eq(self, other as *const SocccAllocator_char_) }
    }
}
impl std::cmp::Eq for SocccAllocator_char_ {}
impl Default for SocccAllocator_char_ {
    fn default() -> Self {
        unsafe {
            SocccAllocator_char_ {
                count_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct SocccAllocator_wchar_t_ {
    pub count_: (),
}
impl SocccAllocator_wchar_t_ {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { count_: i };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn select_on_container_copy_construction(&self) -> SocccAllocator_wchar_t_ {
        return SocccAllocator_wchar_t_::new({ ((self.count_) + (1_())) });
    }
    pub unsafe fn operator_eq(&self, _a0: *const SocccAllocator_wchar_t_) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for SocccAllocator_wchar_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            SocccAllocator_wchar_t_::operator_eq(self, other as *const SocccAllocator_wchar_t_)
        }
    }
}
impl std::cmp::Eq for SocccAllocator_wchar_t_ {}
impl Default for SocccAllocator_wchar_t_ {
    fn default() -> Self {
        unsafe {
            SocccAllocator_wchar_t_ {
                count_: unimplemented!(),
            }
        }
    }
}
pub unsafe fn test_soccc_behavior_4() {
    {
        let mut ss: () = unimplemented!();
        let mut copied: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut moved: () = (unsafe { (*unimplemented!()).str_16() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_soccc_behavior_5() {
    {
        let mut ss: () = unimplemented!();
        let mut copied: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut moved: () = (unsafe { (*unimplemented!()).str_16() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_allocation_is_pilfered_6() {
    let mut buf: [(); 80] = [unimplemented!(); 80];
    let mut initial: *const () = (unsafe {
        MultiStringType::as_ptr_2(
            &MultiStringType::new(
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    c"a very long string that exceeds the small string optimization buffer length"
                        .as_ptr()
                },
                { (::std::mem::size_of::<[(); 76]>() as ()).wrapping_sub(1_()) },
            ),
            std::ptr::null(),
        )
    });
    {
        unimplemented!();
        let mut mr1: () = unimplemented!();
        let mut ss: () = unimplemented!();
        let mut s: () = (unsafe { (*unimplemented!()).str_16() });
        unimplemented!();
    }
}
pub unsafe fn test_allocation_is_pilfered_7() {
    let mut buf: [(); 320] = [unimplemented!(); 320];
    let mut initial: *const () = (unsafe {
        MultiStringType::as_ptr_3(
            &MultiStringType::new(
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    c"a very long string that exceeds the small string optimization buffer length"
                        .as_ptr()
                },
                { (::std::mem::size_of::<[(); 76]>() as ()).wrapping_sub(1_()) },
            ),
            std::ptr::null(),
        )
    });
    {
        unimplemented!();
        let mut mr1: () = unimplemented!();
        let mut ss: () = unimplemented!();
        let mut s: () = (unsafe { (*unimplemented!()).str_16() });
        unimplemented!();
    }
}
pub unsafe fn test_no_foreign_allocations_8() {
    let mut initial: *const () = (unsafe {
        MultiStringType::as_ptr_2(
            &MultiStringType::new(
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    c"a very long string that exceeds the small string optimization buffer length"
                        .as_ptr()
                },
                { (::std::mem::size_of::<[(); 76]>() as ()).wrapping_sub(1_()) },
            ),
            std::ptr::null(),
        )
    });
    {
        unimplemented!();
        let mut mr1: () = unimplemented!();
        let mut ss: () = unimplemented!();
        unimplemented!();
        let mut copied: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut moved: () = (unsafe { (*unimplemented!()).str_16() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_no_foreign_allocations_9() {
    let mut initial: *const () = (unsafe {
        MultiStringType::as_ptr_3(
            &MultiStringType::new(
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        97 as (), 32 as (), 118 as (), 101 as (), 114 as (), 121 as (), 32 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 32 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 32 as (), 116 as (), 104 as (),
                        97 as (), 116 as (), 32 as (), 101 as (), 120 as (), 99 as (), 101 as (),
                        101 as (), 100 as (), 115 as (), 32 as (), 116 as (), 104 as (), 101 as (),
                        32 as (), 115 as (), 109 as (), 97 as (), 108 as (), 108 as (), 32 as (),
                        115 as (), 116 as (), 114 as (), 105 as (), 110 as (), 103 as (), 32 as (),
                        111 as (), 112 as (), 116 as (), 105 as (), 109 as (), 105 as (),
                        122 as (), 97 as (), 116 as (), 105 as (), 111 as (), 110 as (), 32 as (),
                        98 as (), 117 as (), 102 as (), 102 as (), 101 as (), 114 as (), 32 as (),
                        108 as (), 101 as (), 110 as (), 103 as (), 116 as (), 104 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 76]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    c"a very long string that exceeds the small string optimization buffer length"
                        .as_ptr()
                },
                { (::std::mem::size_of::<[(); 76]>() as ()).wrapping_sub(1_()) },
            ),
            std::ptr::null(),
        )
    });
    {
        unimplemented!();
        let mut mr1: () = unimplemented!();
        let mut ss: () = unimplemented!();
        unimplemented!();
        let mut copied: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut moved: () = (unsafe { (*unimplemented!()).str_16() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_soccc_behavior_4() });
    (unsafe { test_allocation_is_pilfered_6() });
    (unsafe { test_no_foreign_allocations_8() });
    (unsafe { test_soccc_behavior_5() });
    (unsafe { test_allocation_is_pilfered_7() });
    (unsafe { test_no_foreign_allocations_9() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
}
