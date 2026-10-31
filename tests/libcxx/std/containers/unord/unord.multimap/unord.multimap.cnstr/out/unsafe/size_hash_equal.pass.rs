extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NotConstructible {}
impl NotConstructible {}
impl std::cmp::PartialEq for NotConstructible {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const NotConstructible,
                other as *const NotConstructible,
            )
        }
    }
}
impl std::cmp::Eq for NotConstructible {}
pub unsafe fn operator_eq_0(_a0: *const NotConstructible, _a1: *const NotConstructible) -> bool {
    return true;
}
pub unsafe fn operator_eq_1(
    a: *const test_equal_to_NotConstructible_,
    b: *const test_equal_to_NotConstructible_,
) -> bool {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_equal_to_NotConstructible_ {
    pub data_: i32,
}
impl test_equal_to_NotConstructible_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self { data_: data };
        this
    }
}
impl std::cmp::PartialEq for test_equal_to_NotConstructible_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_1(
                self as *const test_equal_to_NotConstructible_,
                other as *const test_equal_to_NotConstructible_,
            )
        }
    }
}
impl std::cmp::Eq for test_equal_to_NotConstructible_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_hash_NotConstructible_ {
    data_: i32,
}
impl test_hash_NotConstructible_ {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_eq(&self, c: *const test_hash_NotConstructible_) -> bool {
        return ((self.data_) == ((*c).data_));
    }
}
impl std::cmp::PartialEq for test_hash_NotConstructible_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            test_hash_NotConstructible_::operator_eq(
                self,
                other as *const test_hash_NotConstructible_,
            )
        }
    }
}
impl std::cmp::Eq for test_hash_NotConstructible_ {}
impl Default for test_hash_NotConstructible_ {
    fn default() -> Self {
        unsafe { test_hash_NotConstructible_::new(None) }
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
pub static mut destructed_value_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_4(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_5(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_4(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
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
            operator_eq_4(
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
pub unsafe fn operator_eq_6(
    x: *const test_allocator_std_pair_const_NotConstructible__NotConstructible__,
    y: *const test_allocator_std_pair_const_NotConstructible__NotConstructible__,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_NotConstructible__NotConstructible___rebind_std_pair_const_NotConstructible__NotConstructible__
{}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_NotConstructible__NotConstructible__ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_NotConstructible__NotConstructible__ {
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_NotConstructible__NotConstructible__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const test_allocator_std_pair_const_NotConstructible__NotConstructible__,
                other as *const test_allocator_std_pair_const_NotConstructible__NotConstructible__,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_std_pair_const_NotConstructible__NotConstructible__ {}
impl Default for test_allocator_std_pair_const_NotConstructible__NotConstructible__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_NotConstructible__NotConstructible__ {
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
pub struct min_pointer_std_pair_const_NotConstructible__NotConstructible___std_integral_constant_unsigned_long__0__
{
    ptr_: *mut (NotConstructible, NotConstructible),
}
impl min_pointer_std_pair_const_NotConstructible__NotConstructible___std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_11(
    mut _a0: min_allocator_std_pair_const_NotConstructible__NotConstructible__,
    mut _a1: min_allocator_std_pair_const_NotConstructible__NotConstructible__,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_NotConstructible__NotConstructible__ {}
impl std::cmp::PartialEq for min_allocator_std_pair_const_NotConstructible__NotConstructible__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_NotConstructible__NotConstructible__ {}
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
        let mut c: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            (unsafe {
                let mut _c: test_hash_NotConstructible_ =
                    test_hash_NotConstructible_::new({ Some(8) });
                test_hash_NotConstructible_::operator_eq(&unimplemented!(), &mut _c)
            })
        );
        assert!(
            (unsafe {
                let mut _a: test_equal_to_NotConstructible_ = unimplemented!();
                let mut _b: test_equal_to_NotConstructible_ =
                    test_equal_to_NotConstructible_::new_1({ 9 });
                operator_eq_1(&mut _a, &mut _b)
            })
        );
        assert!(
            (unsafe {
                let mut _x: test_allocator_std_pair_const_NotConstructible__NotConstructible__ =
                    unimplemented!();
                let mut _y: test_allocator_std_pair_const_NotConstructible__NotConstructible__ =
                    (<test_allocator_std_pair_const_NotConstructible__NotConstructible__>::default(
                    ));
                operator_eq_6(&mut _x, &mut _y)
            })
        );
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!());
        assert!(((unimplemented!()) == (0_isize)));
        assert!(((unimplemented!()) == (0_f32)));
        assert!(((unimplemented!()) == (1_f32)));
    }
    {
        let mut c: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            (unsafe {
                let mut _c: test_hash_NotConstructible_ =
                    test_hash_NotConstructible_::new({ Some(8) });
                test_hash_NotConstructible_::operator_eq(&unimplemented!(), &mut _c)
            })
        );
        assert!(
            (unsafe {
                let mut _a: test_equal_to_NotConstructible_ = unimplemented!();
                let mut _b: test_equal_to_NotConstructible_ =
                    test_equal_to_NotConstructible_::new_1({ 9 });
                operator_eq_1(&mut _a, &mut _b)
            })
        );
        assert!(
            (unsafe {
                let _arg0: min_allocator_std_pair_const_NotConstructible__NotConstructible__ =
                    unimplemented!();
                operator_eq_11(
                    _arg0,
                    (<min_allocator_std_pair_const_NotConstructible__NotConstructible__>::default(
                    )),
                )
            })
        );
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!());
        assert!(((unimplemented!()) == (0_isize)));
        assert!(((unimplemented!()) == (0_f32)));
        assert!(((unimplemented!()) == (1_f32)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_2);
    std::cell::LazyCell::force(&*&raw const moved_value_3);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_7);
    std::cell::LazyCell::force(&*&raw const alloc_count_8);
    std::cell::LazyCell::force(&*&raw const dealloc_count_9);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_10);
}
