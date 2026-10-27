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
pub struct test_macros_detail_is_same_int_ptrptr__int_ptrptr_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptrconst_ptr__int_ptrconst_ptr_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int__ptr_arr2arr__int__ptr_arr2arr_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int__ptr_arr2arr__const_int__ptr_arr2arr_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_short_ptr__short_ptr_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct random_access_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {}
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct contiguous_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct contiguous_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
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
pub unsafe fn operator_eq_7(
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
            operator_eq_7(
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
pub unsafe fn operator_add_8(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_9(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_8(_i, n)
    });
}
pub unsafe fn iter_swap_10(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_11(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_12(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_13(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_13(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_12(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub static mut globalBuff_14: std::cell::LazyCell<[(); 2]> =
    std::cell::LazyCell::new(|| unsafe { [unimplemented!(); 2] });
const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, Incomplete[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, Incomplete(&&)[2]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, Incomplete(&&)[2][2]>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, int [1]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, int (&&)[1]>");
const _: () = assert!(true, "std::is_invocable_v<RangeDataT, int (&)[1]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCDataT, Incomplete[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCDataT, Incomplete(&&)[2]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, Incomplete(&&)[2][2]>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeCDataT, int [1]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCDataT, int (&&)[1]>");
const _: () = assert!(true, "std::is_invocable_v<RangeCDataT, int (&)[1]>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DataMember {
    pub x: (),
}
impl DataMember {
    pub unsafe fn data(&self) -> *const () {
        return (&self.x as *const ());
    }
}
const _: () = assert!(true, "std::is_invocable_v<RangeDataT, DataMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, DataMember &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeDataT, DataMember const&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, DataMember const&&>");
const _: () = assert!(true, "std::is_invocable_v<RangeCDataT, DataMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCDataT, DataMember &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeCDataT, DataMember const&>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, DataMember const&&>"
);
pub unsafe fn testReturnTypes_15() -> () {
    {
        let mut x: [*mut (); 2] = [std::ptr::null_mut(); 2];
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let mut x: [[(); 2]; 2] = [[unimplemented!(); 2]; 2];
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, D&&>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, const D&&>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_invocable_v<RangeCDataT, D&&>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_invocable_v<RangeCDataT, const D&&>");;
    }
    {
        const _: () = assert!(!(false), "!std::ranges::contiguous_range<NC>");;
        const _: () = assert!(true, "std::ranges::contiguous_range<const NC>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, NC&&>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, const NC&&>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_invocable_v<RangeCDataT, NC&&>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_invocable_v<RangeCDataT, const NC&&>");;
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D_16 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NC_17 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct VoidDataMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, VoidDataMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, VoidDataMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EmptyDataMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, EmptyDataMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, EmptyDataMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrConvertibleDataMember_Ptr {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrConvertibleDataMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, PtrConvertibleDataMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, PtrConvertibleDataMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonConstDataMember {
    pub x: (),
}
impl NonConstDataMember {
    pub unsafe fn data(&mut self) -> *mut () {
        return (&mut self.x as *mut ());
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EnabledBorrowingDataMember {}
impl EnabledBorrowingDataMember {
    pub unsafe fn data(&mut self) -> *mut () {
        return (&mut (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_14))
            [(0_()) as usize] as *mut ());
    }
}
pub static mut enable_borrowed_range_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DataMemberAndBegin {
    pub x: (),
}
impl DataMemberAndBegin {
    pub unsafe fn data(&self) -> *const () {
        return (&self.x as *const ());
    }
}
pub unsafe fn testDataMember_19() -> () {
    let mut a: DataMember = <DataMember>::default();
    unimplemented!();
    unimplemented!();
    let mut b: NonConstDataMember = <NonConstDataMember>::default();
    unimplemented!();
    const _: () = assert!(true, "!std::is_invocable_v<RangeCDataT, decltype((b))>");;
    let mut c: EnabledBorrowingDataMember = <EnabledBorrowingDataMember>::default();
    unimplemented!();
    const _: () = assert!(
        true,
        "!std::is_invocable_v<RangeCDataT, decltype(std::move(c))>"
    );;
    let mut d: DataMemberAndBegin = <DataMemberAndBegin>::default();
    unimplemented!();
    unimplemented!();
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginMemberContiguousIterator {
    pub buff: [(); 8],
}
impl BeginMemberContiguousIterator {
    pub unsafe fn begin(&self) -> contiguous_iterator_const_int_ptr_ {
        return contiguous_iterator_const_int_ptr_::new_1({ self.buff.as_ptr() });
    }
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeDataT, BeginMemberContiguousIterator &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberContiguousIterator &&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeDataT, BeginMemberContiguousIterator const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberContiguousIterator const&&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCDataT, BeginMemberContiguousIterator &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberContiguousIterator &&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCDataT, BeginMemberContiguousIterator const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberContiguousIterator const&&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginMemberRandomAccess {
    pub buff: [(); 8],
}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberRandomAccess&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberRandomAccess&&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, const BeginMemberRandomAccess&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, const BeginMemberRandomAccess&&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberRandomAccess&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberRandomAccess&&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, const BeginMemberRandomAccess&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, const BeginMemberRandomAccess&&>"
);
pub unsafe fn begin_20(
    iter: *const BeginFriendContiguousIterator,
) -> contiguous_iterator_const_int_ptr_ {
    return contiguous_iterator_const_int_ptr_::new_1({ (*iter).buff.as_ptr() });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFriendContiguousIterator {
    pub buff: [(); 8],
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeDataT, BeginMemberContiguousIterator &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberContiguousIterator &&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeDataT, BeginMemberContiguousIterator const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberContiguousIterator const&&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCDataT, BeginMemberContiguousIterator &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberContiguousIterator &&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCDataT, BeginMemberContiguousIterator const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberContiguousIterator const&&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFriendRandomAccess {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginFriendRandomAccess&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginFriendRandomAccess&&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, const BeginFriendRandomAccess&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, const BeginFriendRandomAccess&&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginFriendRandomAccess&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginFriendRandomAccess&&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, const BeginFriendRandomAccess&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, const BeginFriendRandomAccess&&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginMemberRvalue {
    pub buff: [(); 8],
}
const _: () = assert!(true, "!std::is_invocable_v<RangeDataT, BeginMemberRvalue&>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberRvalue&&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberRvalue const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberRvalue const&&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberRvalue&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberRvalue&&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberRvalue const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberRvalue const&&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginMemberBorrowingEnabled {}
impl BeginMemberBorrowingEnabled {
    pub unsafe fn begin(&mut self) -> contiguous_iterator_int_ptr_ {
        return contiguous_iterator_int_ptr_::new_1({
            (&mut (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_14))[(1_()) as usize]
                as *mut ())
        });
    }
}
pub static mut enable_borrowed_range_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeDataT, BeginMemberBorrowingEnabled &>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeDataT, BeginMemberBorrowingEnabled &&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberBorrowingEnabled const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, BeginMemberBorrowingEnabled const&&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberBorrowingEnabled &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberBorrowingEnabled &&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberBorrowingEnabled const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, BeginMemberBorrowingEnabled const&&>"
);
pub unsafe fn testViaRangesBegin_22() -> () {
    let mut arr: [(); 2] = [unimplemented!(); 2];
    unimplemented!();
    unimplemented!();
    let mut a: BeginMemberContiguousIterator = <BeginMemberContiguousIterator>::default();
    unimplemented!();
    unimplemented!();
    let b: BeginFriendContiguousIterator = <BeginFriendContiguousIterator>::default();
    unimplemented!();
    unimplemented!();
    let mut c: BeginMemberBorrowingEnabled = <BeginMemberBorrowingEnabled>::default();
    unimplemented!();
    const _: () = assert!(
        true,
        "!std::is_invocable_v<RangeCDataT, decltype(std::move(c))>"
    );;
    return true;
}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, Holder<Incomplete>*>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, Holder<Incomplete>*&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, Holder<Incomplete>*>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, Holder<Incomplete>*&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RandomButNotContiguous {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, RandomButNotContiguous>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeDataT, RandomButNotContiguous&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, RandomButNotContiguous>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCDataT, RandomButNotContiguous&>"
);
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
    const _: () = assert!(true, "testReturnTypes()");;
    (unsafe { testDataMember_19() });
    const _: () = assert!(true, "testDataMember()");;
    (unsafe { testViaRangesBegin_22() });
    const _: () = assert!(true, "testViaRangesBegin()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const globalBuff_14);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_18);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_21);
}
