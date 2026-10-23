extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_span_int__18446744073709551615___std_span_int__18446744073709551615__
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_span_const_int__18446744073709551615___std_span_const_int__18446744073709551615__
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized___std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized__
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_subrange_random_access_iterator_const_int_ptr___random_access_iterator_const_int_ptr___std_ranges_subrange_kind_sized___std_ranges_subrange_random_access_iterator_const_int_ptr___random_access_iterator_const_int_ptr___std_ranges_subrange_kind_sized__
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_subrange_std_counted_iterator_bidirectional_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized___std_ranges_subrange_std_counted_iterator_bidirectional_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized__
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_subrange_std_counted_iterator_bidirectional_iterator_const_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized___std_ranges_subrange_std_counted_iterator_bidirectional_iterator_const_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized__
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_subrange_std_counted_iterator_cpp17_output_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized___std_ranges_subrange_std_counted_iterator_cpp17_output_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized__
{}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_subrange_std_counted_iterator_cpp17_input_iterator_int_ptr__int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized___std_ranges_subrange_std_counted_iterator_cpp17_input_iterator_int_ptr__int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized__
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_subrange_std_counted_iterator_cpp20_input_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized___std_ranges_subrange_std_counted_iterator_cpp20_input_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct support_double_move_tracker {
    moved_from_: (),
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        unimplemented!();
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        unimplemented!();
        (*other).moved_from_ = true;
        self.moved_from_ = false;
        return &mut (*(self as *mut support_double_move_tracker));
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        unsafe { support_double_move_tracker::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
pub unsafe fn base_9(i: *const cpp17_output_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_post_inc_6(&mut self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
        return cpp17_output_iterator_int_ptr_::new({ self.it_.postfix_inc() });
    }
    pub unsafe fn move_from(_a0: *mut cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
pub unsafe fn base_10(i: *const cpp17_input_iterator_int_ptr__int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_post_inc_6(
        &mut self,
        mut _a0: (),
    ) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        return cpp17_input_iterator_int_ptr__int_ptr_::new({ self.it_.postfix_inc() });
    }
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_11(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct bidirectional_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_11(
                self as *const bidirectional_iterator_int_ptr_,
                other as *const bidirectional_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
impl Default for bidirectional_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { bidirectional_iterator_int_ptr_::new_1() }
    }
}
pub unsafe fn operator_eq_12(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct bidirectional_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const bidirectional_iterator_const_int_ptr_,
                other as *const bidirectional_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
impl Default for bidirectional_iterator_const_int_ptr_ {
    fn default() -> Self {
        unsafe { bidirectional_iterator_const_int_ptr_::new_1() }
    }
}
pub unsafe fn operator_add_13(
    mut x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_eq_14(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_14(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl Default for random_access_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { random_access_iterator_int_ptr_::new_1() }
    }
}
pub unsafe fn operator_add_15(
    mut x: random_access_iterator_const_int_ptr_,
    mut n: (),
) -> random_access_iterator_const_int_ptr_ {
    (unsafe { random_access_iterator_const_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_const_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_eq_16(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct random_access_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_16(
                self as *const random_access_iterator_const_int_ptr_,
                other as *const random_access_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
impl Default for random_access_iterator_const_int_ptr_ {
    fn default() -> Self {
        unsafe { random_access_iterator_const_int_ptr_::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.it_.offset((n) as isize));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct contiguous_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl Default for contiguous_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { contiguous_iterator_int_ptr_::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct contiguous_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl Default for contiguous_iterator_const_int_ptr_ {
    fn default() -> Self {
        unsafe { contiguous_iterator_const_int_ptr_::new_1() }
    }
}
pub unsafe fn base_17(i: *const cpp20_input_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_int_ptr_,
    ) -> *mut cpp20_input_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_post_inc_6(&mut self, mut _a0: ()) {
        self.it_.prefix_inc();
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_int_ptr_,
    ) -> *mut cpp20_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IteratorOpCounts {
    pub increments: (),
    pub decrements: (),
    pub zero_moves: (),
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
pub unsafe fn operator_eq_18(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> () {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut (),
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_18(
                self as *const forward_sized_iterator_int_ptr_,
                other as *const forward_sized_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            forward_sized_iterator_int_ptr_ {
                it_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub unsafe fn operator_add_19(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_20(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_19(_i, n)
    });
}
pub unsafe fn iter_swap_21(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_22(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
}
pub unsafe fn operator_eq_23(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_24(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut (),
    iter_moves_: *mut (),
    iter_swaps_: *mut (),
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut (), mut iter_moves: *mut (), mut iter_swaps: *mut ()) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut (), iter_moves: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut (), iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> () {
        unimplemented!();
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> () {
        unimplemented!();
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<()>()) as ()
            as ());
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_24(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_23(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        unsafe {
            adl_Iterator {
                ptr_: std::ptr::null_mut(),
                iter_moves_: std::ptr::null_mut(),
                iter_swaps_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rvalue_iterator_int_ptr_ {
    it_: *mut *mut (),
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        return (((*(self as *const rvalue_iterator_int_ptr_)).it_) == ((*_a0).it_));
    }
}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct RvalueConvertible {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct LvalueConvertible {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OnlyExplicitlyConvertible {}
pub unsafe fn test_25() -> () {
    let mut buffer: [(); 8] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_()];
    {
        const _: () = assert!(
            true,
            "std::addressof(std::views::counted) == std::addressof(std::ranges::views::counted)"
        );;
        const _: () = assert!(true, "CountedInvocable<int*, std::size_t>");;
        const _: () = assert!(!(false), "!CountedInvocable<int*, LvalueConvertible>");;
        const _: () = assert!(true, "CountedInvocable<int*, LvalueConvertible&>");;
        const _: () = assert!(true, "CountedInvocable<int*, RvalueConvertible>");;
        const _: () = assert!(!(false), "!CountedInvocable<int*, RvalueConvertible&>");;
        const _: () = assert!(
            !(false),
            "!CountedInvocable<int*, OnlyExplicitlyConvertible>"
        );;
        const _: () = assert!(!(false), "!CountedInvocable<int*, int*>");;
        const _: () = assert!(!(false), "!CountedInvocable<int*>");;
        const _: () = assert!(!(false), "!CountedInvocable<std::size_t>");;
        const _: () = assert!(!(false), "!CountedInvocable<>");;
    }
    {
        let mut c1: () = (unsafe {
            let mut ___count: () = 3_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26))(&mut buffer, &mut ___count)
        });
        let mut c2: () = (unsafe {
            let mut ___count: () = 3_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26))(
                &(*unimplemented!()),
                &mut ___count,
            )
        });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut it: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_2({ buffer.as_mut_ptr() });
        let mut cit: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_2({ (buffer.as_mut_ptr()).cast_const() });
        let mut c1: () = (unsafe {
            let mut ___count: () = 3_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26))(&mut it, &mut ___count)
        });
        let mut c2: () = (unsafe {
            let mut ___count: () = 3_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26))(
                &(*unimplemented!()),
                &mut ___count,
            )
        });
        let mut c3: () = (unsafe {
            let mut ___count: () = 3_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26))(&mut it, &mut ___count)
        });
        let mut c4: () = (unsafe {
            let mut ___it: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ buffer.as_mut_ptr() });
            let mut ___count: () = 3_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26))(&mut ___it, &mut ___count)
        });
        let mut c5: () = (unsafe {
            let mut ___count: () = 3_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26))(&mut cit, &mut ___count)
        });
        let mut c6: () = (unsafe {
            let mut ___count: () = 3_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26))(
                &(*unimplemented!()),
                &mut ___count,
            )
        });
        let mut c7: () = (unsafe {
            let mut ___count: () = 3_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26))(&mut cit, &mut ___count)
        });
        let mut c8: () = (unsafe {
            let mut ___it: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_2({ (buffer.as_mut_ptr()).cast_const() });
            let mut ___count: () = 3_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26))(&mut ___it, &mut ___count)
        });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut it: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_2({ buffer.as_mut_ptr() });
        let mut cit: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_2({ (buffer.as_mut_ptr()).cast_const() });
        let mut it_copy: random_access_iterator_int_ptr_ = it;
        let mut cit_copy: random_access_iterator_const_int_ptr_ = cit;
        let mut c1 : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut it , & mut ___count , ) } ) ;
        let mut c2 : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & ( * unimplemented!()  )  , & mut ___count , ) } ) ;
        let mut c3 : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut it_copy , & mut ___count , ) } ) ;
        let mut c4 : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___it : random_access_iterator_int_ptr_  = random_access_iterator_int_ptr_ :: new_2 ( { buffer .as_mut_ptr() } , )  ; let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut ___it , & mut ___count , ) } ) ;
        let mut c5 : std_ranges_subrange_random_access_iterator_const_int_ptr___random_access_iterator_const_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut cit , & mut ___count , ) } ) ;
        let mut c6 : std_ranges_subrange_random_access_iterator_const_int_ptr___random_access_iterator_const_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & ( * unimplemented!()  )  , & mut ___count , ) } ) ;
        let mut c7 : std_ranges_subrange_random_access_iterator_const_int_ptr___random_access_iterator_const_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut cit_copy , & mut ___count , ) } ) ;
        let mut c8 : std_ranges_subrange_random_access_iterator_const_int_ptr___random_access_iterator_const_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___it : random_access_iterator_const_int_ptr_  = random_access_iterator_const_int_ptr_ :: new_2 ( { ( buffer .as_mut_ptr() ) .cast_const() } , )  ; let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut ___it , & mut ___count , ) } ) ;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut it: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_2({ buffer.as_mut_ptr() });
        let mut cit: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_2({ (buffer.as_mut_ptr()).cast_const() });
        let mut it_copy: bidirectional_iterator_int_ptr_ = it;
        let mut cit_copy: bidirectional_iterator_const_int_ptr_ = cit;
        let mut c1 : std_ranges_subrange_std_counted_iterator_bidirectional_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut it , & mut ___count , ) } ) ;
        let mut c2 : std_ranges_subrange_std_counted_iterator_bidirectional_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & ( * unimplemented!()  )  , & mut ___count , ) } ) ;
        let mut c3 : std_ranges_subrange_std_counted_iterator_bidirectional_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut it_copy , & mut ___count , ) } ) ;
        let mut c4 : std_ranges_subrange_std_counted_iterator_bidirectional_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___it : bidirectional_iterator_int_ptr_  = bidirectional_iterator_int_ptr_ :: new_2 ( { buffer .as_mut_ptr() } , )  ; let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut ___it , & mut ___count , ) } ) ;
        let mut c5 : std_ranges_subrange_std_counted_iterator_bidirectional_iterator_const_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut cit , & mut ___count , ) } ) ;
        let mut c6 : std_ranges_subrange_std_counted_iterator_bidirectional_iterator_const_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & ( * unimplemented!()  )  , & mut ___count , ) } ) ;
        let mut c7 : std_ranges_subrange_std_counted_iterator_bidirectional_iterator_const_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut cit_copy , & mut ___count , ) } ) ;
        let mut c8 : std_ranges_subrange_std_counted_iterator_bidirectional_iterator_const_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___it : bidirectional_iterator_const_int_ptr_  = bidirectional_iterator_const_int_ptr_ :: new_2 ( { ( buffer .as_mut_ptr() ) .cast_const() } , )  ; let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut ___it , & mut ___count , ) } ) ;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut it: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ buffer.as_mut_ptr() });
        let mut it_copy: cpp17_output_iterator_int_ptr_ = it;
        let mut c1 : std_ranges_subrange_std_counted_iterator_cpp17_output_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut it , & mut ___count , ) } ) ;
        let mut c2 : std_ranges_subrange_std_counted_iterator_cpp17_output_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & ( * unimplemented!()  )  , & mut ___count , ) } ) ;
        let mut c3 : std_ranges_subrange_std_counted_iterator_cpp17_output_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut it_copy , & mut ___count , ) } ) ;
        let mut c4 : std_ranges_subrange_std_counted_iterator_cpp17_output_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___it : cpp17_output_iterator_int_ptr_  = cpp17_output_iterator_int_ptr_ :: new ( { buffer .as_mut_ptr() } , )  ; let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut ___it , & mut ___count , ) } ) ;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut it: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ buffer.as_mut_ptr() });
        let mut it_copy: cpp17_input_iterator_int_ptr__int_ptr_ = it;
        let mut c1 : std_ranges_subrange_std_counted_iterator_cpp17_input_iterator_int_ptr__int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut it , & mut ___count , ) } ) ;
        let mut c2 : std_ranges_subrange_std_counted_iterator_cpp17_input_iterator_int_ptr__int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & ( * unimplemented!()  )  , & mut ___count , ) } ) ;
        let mut c3 : std_ranges_subrange_std_counted_iterator_cpp17_input_iterator_int_ptr__int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut it_copy , & mut ___count , ) } ) ;
        let mut c4 : std_ranges_subrange_std_counted_iterator_cpp17_input_iterator_int_ptr__int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___it : cpp17_input_iterator_int_ptr__int_ptr_  = cpp17_input_iterator_int_ptr__int_ptr_ :: new ( { buffer .as_mut_ptr() } , )  ; let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut ___it , & mut ___count , ) } ) ;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut it: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ buffer.as_mut_ptr() });
        const _: () = assert!(!(false), "!std::copyable<cpp20_input_iterator<int*>>");;
        const _: () = assert!(
            !(false),
            "!CountedInvocable<cpp20_input_iterator<int*>&, int>"
        );;
        const _: () = assert!(
            !(false),
            "!CountedInvocable<const cpp20_input_iterator<int*>&, int>"
        );;
        let mut c3 : std_ranges_subrange_std_counted_iterator_cpp20_input_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut it , & mut ___count , ) } ) ;
        let mut c4 : std_ranges_subrange_std_counted_iterator_cpp20_input_iterator_int_ptr____std_default_sentinel_t__std_ranges_subrange_kind_sized_ = ( unsafe { let mut ___it : cpp20_input_iterator_int_ptr_  = cpp20_input_iterator_int_ptr_ :: new ( { buffer .as_mut_ptr() } , )  ; let mut ___count : ()  = 3_() ; (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_26)) ( & mut ___it , & mut ___count , ) } ) ;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_25() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
