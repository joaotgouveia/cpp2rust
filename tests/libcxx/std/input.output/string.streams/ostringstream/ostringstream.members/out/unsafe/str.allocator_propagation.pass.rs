extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(*const libc::c_char , to_pconstlibcc_char)]
#[cpp_into(*const i32 , to_pconsti32)]
#[cpp_into(*const u8 , to_pconstu8)]
#[cpp_into(*const u16 , to_pconstu16)]
#[cpp_into(*const u32 , to_pconstu32)]
pub struct MultiStringType {
    pub w_: *const i32,
    pub wn_: usize,
    pub u8_: *const u8,
    pub u8n_: usize,
    pub u16_: *const u16,
    pub u16n_: usize,
    pub u32_: *const u32,
    pub u32n_: usize,
    pub s_: *const libc::c_char,
    pub sn_: usize,
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const i32,
        mut wn: usize,
        mut u8: *const u8,
        mut u8n: usize,
        mut u16: *const u16,
        mut u16n: usize,
        mut u32: *const u32,
        mut u32n: usize,
        mut s: *const libc::c_char,
        mut sn: usize,
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
    pub unsafe fn as_ptr_2(&self, mut _a0: *const libc::c_char) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const i32) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const u8) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const u16) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const u32) -> *const u32 {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const libc::c_char) -> usize {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const i32) -> usize {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const u8) -> usize {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const u16) -> usize {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const u32) -> usize {
        return self.u32n_;
    }
    pub unsafe fn to_pconstlibcc_char(&self) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn to_pconsti32(&self) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn to_pconstu8(&self) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn to_pconstu16(&self) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn to_pconstu32(&self) -> *const u32 {
        return self.u32_;
    }
}
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
#[derive(FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
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
    pub count_: i32,
}
impl SocccAllocator_char_ {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { count_: i };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut libc::c_char {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut libc::c_char, mut n: usize) {
        unimplemented!();
    }
    pub unsafe fn select_on_container_copy_construction(&self) -> SocccAllocator_char_ {
        return SocccAllocator_char_::new({ ((self.count_) + (1)) });
    }
    pub unsafe fn operator_eq(&self, _a0: *const SocccAllocator_char_) -> bool {
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
        unsafe { SocccAllocator_char_ { count_: 0_i32 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct SocccAllocator_wchar_t_ {
    pub count_: i32,
}
impl SocccAllocator_wchar_t_ {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { count_: i };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        unimplemented!();
    }
    pub unsafe fn select_on_container_copy_construction(&self) -> SocccAllocator_wchar_t_ {
        return SocccAllocator_wchar_t_::new({ ((self.count_) + (1)) });
    }
    pub unsafe fn operator_eq(&self, _a0: *const SocccAllocator_wchar_t_) -> bool {
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
        unsafe { SocccAllocator_wchar_t_ { count_: 0_i32 } }
    }
}
pub unsafe fn test_soccc_behavior_4() {
    {
        let mut ss: () = unimplemented!();
        let mut copied: Vec<libc::c_char> = unimplemented!();
        assert!(((unimplemented!().count_) == (10)));
        assert!(((unimplemented!().count_) == (10)));
        assert!(Vec::len(&copied) <= 1);
        assert!(((unimplemented!().count_) == (11)));
        let mut moved: Vec<libc::c_char> = unimplemented!();
        assert!(((unimplemented!().count_) == (10)));
        assert!(((unimplemented!().count_) == (10)));
        assert!(Vec::len(&moved) <= 1);
    }
}
pub unsafe fn test_soccc_behavior_5() {
    {
        let mut ss: () = unimplemented!();
        let mut copied: Vec<i32> = unimplemented!();
        assert!(((unimplemented!().count_) == (10)));
        assert!(((unimplemented!().count_) == (10)));
        assert!(Vec::len(&copied) <= 1);
        assert!(((unimplemented!().count_) == (11)));
        let mut moved: Vec<i32> = unimplemented!();
        assert!(((unimplemented!().count_) == (10)));
        assert!(((unimplemented!().count_) == (10)));
        assert!(Vec::len(&moved) <= 1);
    }
}
pub unsafe fn test_allocation_is_pilfered_6() {
    let mut buf: [libc::c_char; 80] = [(0 as libc::c_char); 80];
    let mut initial: *const libc::c_char = (unsafe {
        MultiStringType::as_ptr_2(
            &MultiStringType::new(
                {
                    [
                        97 as i32, 32 as i32, 118 as i32, 101 as i32, 114 as i32, 121 as i32,
                        32 as i32, 108 as i32, 111 as i32, 110 as i32, 103 as i32, 32 as i32,
                        115 as i32, 116 as i32, 114 as i32, 105 as i32, 110 as i32, 103 as i32,
                        32 as i32, 116 as i32, 104 as i32, 97 as i32, 116 as i32, 32 as i32,
                        101 as i32, 120 as i32, 99 as i32, 101 as i32, 101 as i32, 100 as i32,
                        115 as i32, 32 as i32, 116 as i32, 104 as i32, 101 as i32, 32 as i32,
                        115 as i32, 109 as i32, 97 as i32, 108 as i32, 108 as i32, 32 as i32,
                        115 as i32, 116 as i32, 114 as i32, 105 as i32, 110 as i32, 103 as i32,
                        32 as i32, 111 as i32, 112 as i32, 116 as i32, 105 as i32, 109 as i32,
                        105 as i32, 122 as i32, 97 as i32, 116 as i32, 105 as i32, 111 as i32,
                        110 as i32, 32 as i32, 98 as i32, 117 as i32, 102 as i32, 102 as i32,
                        101 as i32, 114 as i32, 32 as i32, 108 as i32, 101 as i32, 110 as i32,
                        103 as i32, 116 as i32, 104 as i32, 0 as i32,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[i32; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u8, 32 as u8, 118 as u8, 101 as u8, 114 as u8, 121 as u8, 32 as u8,
                        108 as u8, 111 as u8, 110 as u8, 103 as u8, 32 as u8, 115 as u8, 116 as u8,
                        114 as u8, 105 as u8, 110 as u8, 103 as u8, 32 as u8, 116 as u8, 104 as u8,
                        97 as u8, 116 as u8, 32 as u8, 101 as u8, 120 as u8, 99 as u8, 101 as u8,
                        101 as u8, 100 as u8, 115 as u8, 32 as u8, 116 as u8, 104 as u8, 101 as u8,
                        32 as u8, 115 as u8, 109 as u8, 97 as u8, 108 as u8, 108 as u8, 32 as u8,
                        115 as u8, 116 as u8, 114 as u8, 105 as u8, 110 as u8, 103 as u8, 32 as u8,
                        111 as u8, 112 as u8, 116 as u8, 105 as u8, 109 as u8, 105 as u8,
                        122 as u8, 97 as u8, 116 as u8, 105 as u8, 111 as u8, 110 as u8, 32 as u8,
                        98 as u8, 117 as u8, 102 as u8, 102 as u8, 101 as u8, 114 as u8, 32 as u8,
                        108 as u8, 101 as u8, 110 as u8, 103 as u8, 116 as u8, 104 as u8, 0 as u8,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u8; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u16, 32 as u16, 118 as u16, 101 as u16, 114 as u16, 121 as u16,
                        32 as u16, 108 as u16, 111 as u16, 110 as u16, 103 as u16, 32 as u16,
                        115 as u16, 116 as u16, 114 as u16, 105 as u16, 110 as u16, 103 as u16,
                        32 as u16, 116 as u16, 104 as u16, 97 as u16, 116 as u16, 32 as u16,
                        101 as u16, 120 as u16, 99 as u16, 101 as u16, 101 as u16, 100 as u16,
                        115 as u16, 32 as u16, 116 as u16, 104 as u16, 101 as u16, 32 as u16,
                        115 as u16, 109 as u16, 97 as u16, 108 as u16, 108 as u16, 32 as u16,
                        115 as u16, 116 as u16, 114 as u16, 105 as u16, 110 as u16, 103 as u16,
                        32 as u16, 111 as u16, 112 as u16, 116 as u16, 105 as u16, 109 as u16,
                        105 as u16, 122 as u16, 97 as u16, 116 as u16, 105 as u16, 111 as u16,
                        110 as u16, 32 as u16, 98 as u16, 117 as u16, 102 as u16, 102 as u16,
                        101 as u16, 114 as u16, 32 as u16, 108 as u16, 101 as u16, 110 as u16,
                        103 as u16, 116 as u16, 104 as u16, 0 as u16,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u16; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u32, 32 as u32, 118 as u32, 101 as u32, 114 as u32, 121 as u32,
                        32 as u32, 108 as u32, 111 as u32, 110 as u32, 103 as u32, 32 as u32,
                        115 as u32, 116 as u32, 114 as u32, 105 as u32, 110 as u32, 103 as u32,
                        32 as u32, 116 as u32, 104 as u32, 97 as u32, 116 as u32, 32 as u32,
                        101 as u32, 120 as u32, 99 as u32, 101 as u32, 101 as u32, 100 as u32,
                        115 as u32, 32 as u32, 116 as u32, 104 as u32, 101 as u32, 32 as u32,
                        115 as u32, 109 as u32, 97 as u32, 108 as u32, 108 as u32, 32 as u32,
                        115 as u32, 116 as u32, 114 as u32, 105 as u32, 110 as u32, 103 as u32,
                        32 as u32, 111 as u32, 112 as u32, 116 as u32, 105 as u32, 109 as u32,
                        105 as u32, 122 as u32, 97 as u32, 116 as u32, 105 as u32, 111 as u32,
                        110 as u32, 32 as u32, 98 as u32, 117 as u32, 102 as u32, 102 as u32,
                        101 as u32, 114 as u32, 32 as u32, 108 as u32, 101 as u32, 110 as u32,
                        103 as u32, 116 as u32, 104 as u32, 0 as u32,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u32; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    c"a very long string that exceeds the small string optimization buffer length"
                        .as_ptr()
                },
                { (::std::mem::size_of::<[libc::c_char; 76]>() as usize).wrapping_sub(1_usize) },
            ),
            std::ptr::null(),
        )
    });
    {
        unimplemented!();
        let mut mr1: () = unimplemented!();
        let mut ss: () = unimplemented!();
        let mut s: () = unimplemented!();
        assert!(
            s == std::slice::from_raw_parts(initial, libcc2rs::c_str_len(initial) + 1).to_vec()
        );
    }
}
pub unsafe fn test_allocation_is_pilfered_7() {
    let mut buf: [libc::c_char; 320] = [(0 as libc::c_char); 320];
    let mut initial: *const i32 = (unsafe {
        MultiStringType::as_ptr_3(
            &MultiStringType::new(
                {
                    [
                        97 as i32, 32 as i32, 118 as i32, 101 as i32, 114 as i32, 121 as i32,
                        32 as i32, 108 as i32, 111 as i32, 110 as i32, 103 as i32, 32 as i32,
                        115 as i32, 116 as i32, 114 as i32, 105 as i32, 110 as i32, 103 as i32,
                        32 as i32, 116 as i32, 104 as i32, 97 as i32, 116 as i32, 32 as i32,
                        101 as i32, 120 as i32, 99 as i32, 101 as i32, 101 as i32, 100 as i32,
                        115 as i32, 32 as i32, 116 as i32, 104 as i32, 101 as i32, 32 as i32,
                        115 as i32, 109 as i32, 97 as i32, 108 as i32, 108 as i32, 32 as i32,
                        115 as i32, 116 as i32, 114 as i32, 105 as i32, 110 as i32, 103 as i32,
                        32 as i32, 111 as i32, 112 as i32, 116 as i32, 105 as i32, 109 as i32,
                        105 as i32, 122 as i32, 97 as i32, 116 as i32, 105 as i32, 111 as i32,
                        110 as i32, 32 as i32, 98 as i32, 117 as i32, 102 as i32, 102 as i32,
                        101 as i32, 114 as i32, 32 as i32, 108 as i32, 101 as i32, 110 as i32,
                        103 as i32, 116 as i32, 104 as i32, 0 as i32,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[i32; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u8, 32 as u8, 118 as u8, 101 as u8, 114 as u8, 121 as u8, 32 as u8,
                        108 as u8, 111 as u8, 110 as u8, 103 as u8, 32 as u8, 115 as u8, 116 as u8,
                        114 as u8, 105 as u8, 110 as u8, 103 as u8, 32 as u8, 116 as u8, 104 as u8,
                        97 as u8, 116 as u8, 32 as u8, 101 as u8, 120 as u8, 99 as u8, 101 as u8,
                        101 as u8, 100 as u8, 115 as u8, 32 as u8, 116 as u8, 104 as u8, 101 as u8,
                        32 as u8, 115 as u8, 109 as u8, 97 as u8, 108 as u8, 108 as u8, 32 as u8,
                        115 as u8, 116 as u8, 114 as u8, 105 as u8, 110 as u8, 103 as u8, 32 as u8,
                        111 as u8, 112 as u8, 116 as u8, 105 as u8, 109 as u8, 105 as u8,
                        122 as u8, 97 as u8, 116 as u8, 105 as u8, 111 as u8, 110 as u8, 32 as u8,
                        98 as u8, 117 as u8, 102 as u8, 102 as u8, 101 as u8, 114 as u8, 32 as u8,
                        108 as u8, 101 as u8, 110 as u8, 103 as u8, 116 as u8, 104 as u8, 0 as u8,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u8; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u16, 32 as u16, 118 as u16, 101 as u16, 114 as u16, 121 as u16,
                        32 as u16, 108 as u16, 111 as u16, 110 as u16, 103 as u16, 32 as u16,
                        115 as u16, 116 as u16, 114 as u16, 105 as u16, 110 as u16, 103 as u16,
                        32 as u16, 116 as u16, 104 as u16, 97 as u16, 116 as u16, 32 as u16,
                        101 as u16, 120 as u16, 99 as u16, 101 as u16, 101 as u16, 100 as u16,
                        115 as u16, 32 as u16, 116 as u16, 104 as u16, 101 as u16, 32 as u16,
                        115 as u16, 109 as u16, 97 as u16, 108 as u16, 108 as u16, 32 as u16,
                        115 as u16, 116 as u16, 114 as u16, 105 as u16, 110 as u16, 103 as u16,
                        32 as u16, 111 as u16, 112 as u16, 116 as u16, 105 as u16, 109 as u16,
                        105 as u16, 122 as u16, 97 as u16, 116 as u16, 105 as u16, 111 as u16,
                        110 as u16, 32 as u16, 98 as u16, 117 as u16, 102 as u16, 102 as u16,
                        101 as u16, 114 as u16, 32 as u16, 108 as u16, 101 as u16, 110 as u16,
                        103 as u16, 116 as u16, 104 as u16, 0 as u16,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u16; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u32, 32 as u32, 118 as u32, 101 as u32, 114 as u32, 121 as u32,
                        32 as u32, 108 as u32, 111 as u32, 110 as u32, 103 as u32, 32 as u32,
                        115 as u32, 116 as u32, 114 as u32, 105 as u32, 110 as u32, 103 as u32,
                        32 as u32, 116 as u32, 104 as u32, 97 as u32, 116 as u32, 32 as u32,
                        101 as u32, 120 as u32, 99 as u32, 101 as u32, 101 as u32, 100 as u32,
                        115 as u32, 32 as u32, 116 as u32, 104 as u32, 101 as u32, 32 as u32,
                        115 as u32, 109 as u32, 97 as u32, 108 as u32, 108 as u32, 32 as u32,
                        115 as u32, 116 as u32, 114 as u32, 105 as u32, 110 as u32, 103 as u32,
                        32 as u32, 111 as u32, 112 as u32, 116 as u32, 105 as u32, 109 as u32,
                        105 as u32, 122 as u32, 97 as u32, 116 as u32, 105 as u32, 111 as u32,
                        110 as u32, 32 as u32, 98 as u32, 117 as u32, 102 as u32, 102 as u32,
                        101 as u32, 114 as u32, 32 as u32, 108 as u32, 101 as u32, 110 as u32,
                        103 as u32, 116 as u32, 104 as u32, 0 as u32,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u32; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    c"a very long string that exceeds the small string optimization buffer length"
                        .as_ptr()
                },
                { (::std::mem::size_of::<[libc::c_char; 76]>() as usize).wrapping_sub(1_usize) },
            ),
            std::ptr::null(),
        )
    });
    {
        unimplemented!();
        let mut mr1: () = unimplemented!();
        let mut ss: () = unimplemented!();
        let mut s: () = unimplemented!();
        assert!(
            s == std::slice::from_raw_parts(initial, libcc2rs::c_str_len(initial) + 1).to_vec()
        );
    }
}
pub unsafe fn test_no_foreign_allocations_8() {
    let mut initial: *const libc::c_char = (unsafe {
        MultiStringType::as_ptr_2(
            &MultiStringType::new(
                {
                    [
                        97 as i32, 32 as i32, 118 as i32, 101 as i32, 114 as i32, 121 as i32,
                        32 as i32, 108 as i32, 111 as i32, 110 as i32, 103 as i32, 32 as i32,
                        115 as i32, 116 as i32, 114 as i32, 105 as i32, 110 as i32, 103 as i32,
                        32 as i32, 116 as i32, 104 as i32, 97 as i32, 116 as i32, 32 as i32,
                        101 as i32, 120 as i32, 99 as i32, 101 as i32, 101 as i32, 100 as i32,
                        115 as i32, 32 as i32, 116 as i32, 104 as i32, 101 as i32, 32 as i32,
                        115 as i32, 109 as i32, 97 as i32, 108 as i32, 108 as i32, 32 as i32,
                        115 as i32, 116 as i32, 114 as i32, 105 as i32, 110 as i32, 103 as i32,
                        32 as i32, 111 as i32, 112 as i32, 116 as i32, 105 as i32, 109 as i32,
                        105 as i32, 122 as i32, 97 as i32, 116 as i32, 105 as i32, 111 as i32,
                        110 as i32, 32 as i32, 98 as i32, 117 as i32, 102 as i32, 102 as i32,
                        101 as i32, 114 as i32, 32 as i32, 108 as i32, 101 as i32, 110 as i32,
                        103 as i32, 116 as i32, 104 as i32, 0 as i32,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[i32; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u8, 32 as u8, 118 as u8, 101 as u8, 114 as u8, 121 as u8, 32 as u8,
                        108 as u8, 111 as u8, 110 as u8, 103 as u8, 32 as u8, 115 as u8, 116 as u8,
                        114 as u8, 105 as u8, 110 as u8, 103 as u8, 32 as u8, 116 as u8, 104 as u8,
                        97 as u8, 116 as u8, 32 as u8, 101 as u8, 120 as u8, 99 as u8, 101 as u8,
                        101 as u8, 100 as u8, 115 as u8, 32 as u8, 116 as u8, 104 as u8, 101 as u8,
                        32 as u8, 115 as u8, 109 as u8, 97 as u8, 108 as u8, 108 as u8, 32 as u8,
                        115 as u8, 116 as u8, 114 as u8, 105 as u8, 110 as u8, 103 as u8, 32 as u8,
                        111 as u8, 112 as u8, 116 as u8, 105 as u8, 109 as u8, 105 as u8,
                        122 as u8, 97 as u8, 116 as u8, 105 as u8, 111 as u8, 110 as u8, 32 as u8,
                        98 as u8, 117 as u8, 102 as u8, 102 as u8, 101 as u8, 114 as u8, 32 as u8,
                        108 as u8, 101 as u8, 110 as u8, 103 as u8, 116 as u8, 104 as u8, 0 as u8,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u8; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u16, 32 as u16, 118 as u16, 101 as u16, 114 as u16, 121 as u16,
                        32 as u16, 108 as u16, 111 as u16, 110 as u16, 103 as u16, 32 as u16,
                        115 as u16, 116 as u16, 114 as u16, 105 as u16, 110 as u16, 103 as u16,
                        32 as u16, 116 as u16, 104 as u16, 97 as u16, 116 as u16, 32 as u16,
                        101 as u16, 120 as u16, 99 as u16, 101 as u16, 101 as u16, 100 as u16,
                        115 as u16, 32 as u16, 116 as u16, 104 as u16, 101 as u16, 32 as u16,
                        115 as u16, 109 as u16, 97 as u16, 108 as u16, 108 as u16, 32 as u16,
                        115 as u16, 116 as u16, 114 as u16, 105 as u16, 110 as u16, 103 as u16,
                        32 as u16, 111 as u16, 112 as u16, 116 as u16, 105 as u16, 109 as u16,
                        105 as u16, 122 as u16, 97 as u16, 116 as u16, 105 as u16, 111 as u16,
                        110 as u16, 32 as u16, 98 as u16, 117 as u16, 102 as u16, 102 as u16,
                        101 as u16, 114 as u16, 32 as u16, 108 as u16, 101 as u16, 110 as u16,
                        103 as u16, 116 as u16, 104 as u16, 0 as u16,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u16; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u32, 32 as u32, 118 as u32, 101 as u32, 114 as u32, 121 as u32,
                        32 as u32, 108 as u32, 111 as u32, 110 as u32, 103 as u32, 32 as u32,
                        115 as u32, 116 as u32, 114 as u32, 105 as u32, 110 as u32, 103 as u32,
                        32 as u32, 116 as u32, 104 as u32, 97 as u32, 116 as u32, 32 as u32,
                        101 as u32, 120 as u32, 99 as u32, 101 as u32, 101 as u32, 100 as u32,
                        115 as u32, 32 as u32, 116 as u32, 104 as u32, 101 as u32, 32 as u32,
                        115 as u32, 109 as u32, 97 as u32, 108 as u32, 108 as u32, 32 as u32,
                        115 as u32, 116 as u32, 114 as u32, 105 as u32, 110 as u32, 103 as u32,
                        32 as u32, 111 as u32, 112 as u32, 116 as u32, 105 as u32, 109 as u32,
                        105 as u32, 122 as u32, 97 as u32, 116 as u32, 105 as u32, 111 as u32,
                        110 as u32, 32 as u32, 98 as u32, 117 as u32, 102 as u32, 102 as u32,
                        101 as u32, 114 as u32, 32 as u32, 108 as u32, 101 as u32, 110 as u32,
                        103 as u32, 116 as u32, 104 as u32, 0 as u32,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u32; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    c"a very long string that exceeds the small string optimization buffer length"
                        .as_ptr()
                },
                { (::std::mem::size_of::<[libc::c_char; 76]>() as usize).wrapping_sub(1_usize) },
            ),
            std::ptr::null(),
        )
    });
    {
        unimplemented!();
        let mut mr1: () = unimplemented!();
        let mut ss: () = unimplemented!();
        assert!(((unimplemented!()) == (&mut mr1 as *mut ())));
        let mut copied: () = unimplemented!();
        assert!(((unimplemented!()) == (&mut mr1 as *mut ())));
        assert!(((unimplemented!()) == (&mut mr1 as *mut ())));
        assert!(
            copied
                == std::slice::from_raw_parts(initial, libcc2rs::c_str_len(initial) + 1).to_vec()
        );
        let mut moved: () = unimplemented!();
        assert!(((unimplemented!()) == (&mut mr1 as *mut ())));
        assert!(((unimplemented!()) == (&mut mr1 as *mut ())));
        assert!(
            moved == std::slice::from_raw_parts(initial, libcc2rs::c_str_len(initial) + 1).to_vec()
        );
    }
}
pub unsafe fn test_no_foreign_allocations_9() {
    let mut initial: *const i32 = (unsafe {
        MultiStringType::as_ptr_3(
            &MultiStringType::new(
                {
                    [
                        97 as i32, 32 as i32, 118 as i32, 101 as i32, 114 as i32, 121 as i32,
                        32 as i32, 108 as i32, 111 as i32, 110 as i32, 103 as i32, 32 as i32,
                        115 as i32, 116 as i32, 114 as i32, 105 as i32, 110 as i32, 103 as i32,
                        32 as i32, 116 as i32, 104 as i32, 97 as i32, 116 as i32, 32 as i32,
                        101 as i32, 120 as i32, 99 as i32, 101 as i32, 101 as i32, 100 as i32,
                        115 as i32, 32 as i32, 116 as i32, 104 as i32, 101 as i32, 32 as i32,
                        115 as i32, 109 as i32, 97 as i32, 108 as i32, 108 as i32, 32 as i32,
                        115 as i32, 116 as i32, 114 as i32, 105 as i32, 110 as i32, 103 as i32,
                        32 as i32, 111 as i32, 112 as i32, 116 as i32, 105 as i32, 109 as i32,
                        105 as i32, 122 as i32, 97 as i32, 116 as i32, 105 as i32, 111 as i32,
                        110 as i32, 32 as i32, 98 as i32, 117 as i32, 102 as i32, 102 as i32,
                        101 as i32, 114 as i32, 32 as i32, 108 as i32, 101 as i32, 110 as i32,
                        103 as i32, 116 as i32, 104 as i32, 0 as i32,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[i32; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u8, 32 as u8, 118 as u8, 101 as u8, 114 as u8, 121 as u8, 32 as u8,
                        108 as u8, 111 as u8, 110 as u8, 103 as u8, 32 as u8, 115 as u8, 116 as u8,
                        114 as u8, 105 as u8, 110 as u8, 103 as u8, 32 as u8, 116 as u8, 104 as u8,
                        97 as u8, 116 as u8, 32 as u8, 101 as u8, 120 as u8, 99 as u8, 101 as u8,
                        101 as u8, 100 as u8, 115 as u8, 32 as u8, 116 as u8, 104 as u8, 101 as u8,
                        32 as u8, 115 as u8, 109 as u8, 97 as u8, 108 as u8, 108 as u8, 32 as u8,
                        115 as u8, 116 as u8, 114 as u8, 105 as u8, 110 as u8, 103 as u8, 32 as u8,
                        111 as u8, 112 as u8, 116 as u8, 105 as u8, 109 as u8, 105 as u8,
                        122 as u8, 97 as u8, 116 as u8, 105 as u8, 111 as u8, 110 as u8, 32 as u8,
                        98 as u8, 117 as u8, 102 as u8, 102 as u8, 101 as u8, 114 as u8, 32 as u8,
                        108 as u8, 101 as u8, 110 as u8, 103 as u8, 116 as u8, 104 as u8, 0 as u8,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u8; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u16, 32 as u16, 118 as u16, 101 as u16, 114 as u16, 121 as u16,
                        32 as u16, 108 as u16, 111 as u16, 110 as u16, 103 as u16, 32 as u16,
                        115 as u16, 116 as u16, 114 as u16, 105 as u16, 110 as u16, 103 as u16,
                        32 as u16, 116 as u16, 104 as u16, 97 as u16, 116 as u16, 32 as u16,
                        101 as u16, 120 as u16, 99 as u16, 101 as u16, 101 as u16, 100 as u16,
                        115 as u16, 32 as u16, 116 as u16, 104 as u16, 101 as u16, 32 as u16,
                        115 as u16, 109 as u16, 97 as u16, 108 as u16, 108 as u16, 32 as u16,
                        115 as u16, 116 as u16, 114 as u16, 105 as u16, 110 as u16, 103 as u16,
                        32 as u16, 111 as u16, 112 as u16, 116 as u16, 105 as u16, 109 as u16,
                        105 as u16, 122 as u16, 97 as u16, 116 as u16, 105 as u16, 111 as u16,
                        110 as u16, 32 as u16, 98 as u16, 117 as u16, 102 as u16, 102 as u16,
                        101 as u16, 114 as u16, 32 as u16, 108 as u16, 101 as u16, 110 as u16,
                        103 as u16, 116 as u16, 104 as u16, 0 as u16,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u16; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    [
                        97 as u32, 32 as u32, 118 as u32, 101 as u32, 114 as u32, 121 as u32,
                        32 as u32, 108 as u32, 111 as u32, 110 as u32, 103 as u32, 32 as u32,
                        115 as u32, 116 as u32, 114 as u32, 105 as u32, 110 as u32, 103 as u32,
                        32 as u32, 116 as u32, 104 as u32, 97 as u32, 116 as u32, 32 as u32,
                        101 as u32, 120 as u32, 99 as u32, 101 as u32, 101 as u32, 100 as u32,
                        115 as u32, 32 as u32, 116 as u32, 104 as u32, 101 as u32, 32 as u32,
                        115 as u32, 109 as u32, 97 as u32, 108 as u32, 108 as u32, 32 as u32,
                        115 as u32, 116 as u32, 114 as u32, 105 as u32, 110 as u32, 103 as u32,
                        32 as u32, 111 as u32, 112 as u32, 116 as u32, 105 as u32, 109 as u32,
                        105 as u32, 122 as u32, 97 as u32, 116 as u32, 105 as u32, 111 as u32,
                        110 as u32, 32 as u32, 98 as u32, 117 as u32, 102 as u32, 102 as u32,
                        101 as u32, 114 as u32, 32 as u32, 108 as u32, 101 as u32, 110 as u32,
                        103 as u32, 116 as u32, 104 as u32, 0 as u32,
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[u32; 76]>() as usize)
                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                        as usize)
                        .wrapping_sub(1_usize)
                },
                {
                    c"a very long string that exceeds the small string optimization buffer length"
                        .as_ptr()
                },
                { (::std::mem::size_of::<[libc::c_char; 76]>() as usize).wrapping_sub(1_usize) },
            ),
            std::ptr::null(),
        )
    });
    {
        unimplemented!();
        let mut mr1: () = unimplemented!();
        let mut ss: () = unimplemented!();
        assert!(((unimplemented!()) == (&mut mr1 as *mut ())));
        let mut copied: () = unimplemented!();
        assert!(((unimplemented!()) == (&mut mr1 as *mut ())));
        assert!(((unimplemented!()) == (&mut mr1 as *mut ())));
        assert!(
            copied
                == std::slice::from_raw_parts(initial, libcc2rs::c_str_len(initial) + 1).to_vec()
        );
        let mut moved: () = unimplemented!();
        assert!(((unimplemented!()) == (&mut mr1 as *mut ())));
        assert!(((unimplemented!()) == (&mut mr1 as *mut ())));
        assert!(
            moved == std::slice::from_raw_parts(initial, libcc2rs::c_str_len(initial) + 1).to_vec()
        );
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_soccc_behavior_4() });
    (unsafe { test_allocation_is_pilfered_6() });
    (unsafe { test_no_foreign_allocations_8() });
    (unsafe { test_soccc_behavior_5() });
    (unsafe { test_allocation_is_pilfered_7() });
    (unsafe { test_no_foreign_allocations_9() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
}
