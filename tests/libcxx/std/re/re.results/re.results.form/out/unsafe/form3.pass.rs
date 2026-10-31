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
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_char_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_char_ {
    pub unsafe fn copy_from(a: *const test_allocator_char_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0))))
                && (!(c"copying from destroyed allocator".as_ptr()).is_null())
        );
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn move_from(a: *mut test_allocator_char_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0))))
                && (!(c"moving from destroyed allocator".as_ptr()).is_null())
        );
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_1));
        this
    }
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut libc::c_char {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                std::process::abort();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_add(n)) as i32;
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut libc::c_char, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
    }
}
impl Clone for test_allocator_char_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_char_::copy_from(self as *const test_allocator_char_) }
    }
}
impl Default for test_allocator_char_ {
    fn default() -> Self {
        unsafe {
            test_allocator_char_ {
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_wchar_t_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_wchar_t_ {
    pub unsafe fn copy_from(a: *const test_allocator_wchar_t_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0))))
                && (!(c"copying from destroyed allocator".as_ptr()).is_null())
        );
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn move_from(a: *mut test_allocator_wchar_t_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0))))
                && (!(c"moving from destroyed allocator".as_ptr()).is_null())
        );
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_1));
        this
    }
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                std::process::abort();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_add(n)) as i32;
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    }
}
impl Clone for test_allocator_wchar_t_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_wchar_t_::copy_from(self as *const test_allocator_wchar_t_) }
    }
}
impl Default for test_allocator_wchar_t_ {
    fn default() -> Self {
        unsafe {
            test_allocator_wchar_t_ {
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
    {
        let mut m: () = unimplemented!();
        let s: [libc::c_char; 12] = std::mem::transmute(*b"abcdefghijk\0");
        assert!(unimplemented!());
        let mut fmt: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2".as_ptr(),
            libcc2rs::c_str_len(c"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2".as_ptr())
                + 1,
        )
        .to_vec();
        let mut out: Vec<libc::c_char> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                c"prefix: ab, match: cdefghi, suffix: jk, m[1]: efg, m[2]: e".as_ptr(),
                libcc2rs::c_str_len(
                    c"prefix: ab, match: cdefghi, suffix: jk, m[1]: efg, m[2]: e".as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut m: () = unimplemented!();
        let s: [libc::c_char; 12] = std::mem::transmute(*b"abcdefghijk\0");
        assert!(unimplemented!());
        let mut fmt: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2".as_ptr(),
            libcc2rs::c_str_len(c"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2".as_ptr())
                + 1,
        )
        .to_vec();
        let mut out: Vec<libc::c_char> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                c"prefix: $`, match: $cdefghi, suffix: $\', m[1]: $1, m[2]: $2".as_ptr(),
                libcc2rs::c_str_len(
                    c"prefix: $`, match: $cdefghi, suffix: $\', m[1]: $1, m[2]: $2".as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut m: () = unimplemented!();
        let s: [libc::c_char; 12] = std::mem::transmute(*b"abcdefghijk\0");
        assert!(unimplemented!());
        let mut fmt: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"match: &, m[1]: \\1, m[2]: \\2".as_ptr(),
            libcc2rs::c_str_len(c"match: &, m[1]: \\1, m[2]: \\2".as_ptr()) + 1,
        )
        .to_vec();
        let mut out: Vec<libc::c_char> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                c"match: cdefghi, m[1]: efg, m[2]: e".as_ptr(),
                libcc2rs::c_str_len(c"match: cdefghi, m[1]: efg, m[2]: e".as_ptr()) + 1
            )
            .to_vec()
        );
    };
    {
        let mut m: () = unimplemented!();
        let s: [i32; 12] = [
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ];
        assert!(unimplemented!());
        let mut fmt: Vec<i32> = std::slice::from_raw_parts(
            [
                112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
                116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32, 38 as i32,
                44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32,
                120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32,
                109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32,
                49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32,
                58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32,
                    58 as i32, 32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32,
                    97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32,
                    38 as i32, 44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32,
                    102 as i32, 105 as i32, 120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32,
                    44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32,
                    32 as i32, 36 as i32, 49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32,
                    50 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut out: Vec<i32> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                [
                    112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32,
                    58 as i32, 32 as i32, 97 as i32, 98 as i32, 44 as i32, 32 as i32, 109 as i32,
                    97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 99 as i32,
                    100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32, 105 as i32,
                    44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32,
                    105 as i32, 120 as i32, 58 as i32, 32 as i32, 106 as i32, 107 as i32,
                    44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32,
                    32 as i32, 101 as i32, 102 as i32, 103 as i32, 44 as i32, 32 as i32,
                    109 as i32, 91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32,
                    0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32,
                        58 as i32, 32 as i32, 97 as i32, 98 as i32, 44 as i32, 32 as i32,
                        109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32,
                        32 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                        104 as i32, 105 as i32, 44 as i32, 32 as i32, 115 as i32, 117 as i32,
                        102 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32, 32 as i32,
                        106 as i32, 107 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32,
                        49 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32, 102 as i32,
                        103 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32,
                        93 as i32, 58 as i32, 32 as i32, 101 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut m: () = unimplemented!();
        let s: [i32; 12] = [
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ];
        assert!(unimplemented!());
        let mut fmt: Vec<i32> = std::slice::from_raw_parts(
            [
                112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
                116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32, 38 as i32,
                44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32,
                120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32,
                109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32,
                49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32,
                58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32,
                    58 as i32, 32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32,
                    97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32,
                    38 as i32, 44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32,
                    102 as i32, 105 as i32, 120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32,
                    44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32,
                    32 as i32, 36 as i32, 49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32,
                    50 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut out: Vec<i32> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                [
                    112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32,
                    58 as i32, 32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32,
                    97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32,
                    99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32,
                    105 as i32, 44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32,
                    102 as i32, 105 as i32, 120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32,
                    44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32,
                    32 as i32, 36 as i32, 49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32,
                    50 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32,
                        58 as i32, 32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32,
                        109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32,
                        32 as i32, 36 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                        103 as i32, 104 as i32, 105 as i32, 44 as i32, 32 as i32, 115 as i32,
                        117 as i32, 102 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                        32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32, 109 as i32,
                        91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32,
                        49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32,
                        93 as i32, 58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut m: () = unimplemented!();
        let s: [i32; 12] = [
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ];
        assert!(unimplemented!());
        let mut fmt: Vec<i32> = std::slice::from_raw_parts(
            [
                109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32,
                38 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32,
                58 as i32, 32 as i32, 92 as i32, 49 as i32, 44 as i32, 32 as i32, 109 as i32,
                91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 92 as i32, 50 as i32,
                0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32,
                    38 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32,
                    58 as i32, 32 as i32, 92 as i32, 49 as i32, 44 as i32, 32 as i32, 109 as i32,
                    91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 92 as i32, 50 as i32,
                    0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut out: Vec<i32> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                [
                    109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32,
                    99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32,
                    105 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32,
                    58 as i32, 32 as i32, 101 as i32, 102 as i32, 103 as i32, 44 as i32, 32 as i32,
                    109 as i32, 91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32,
                    0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32,
                        32 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                        104 as i32, 105 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32,
                        49 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32, 102 as i32,
                        103 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32,
                        93 as i32, 58 as i32, 32 as i32, 101 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
}
