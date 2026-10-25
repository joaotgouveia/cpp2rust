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
pub struct test_macros_detail_is_same_std_reverse_iterator_int_ptrptr___std_reverse_iterator_int_ptrptr__
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator_int_ptrconst_ptr___std_reverse_iterator_int_ptrconst_ptr__
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator_int__ptr_arr2arr___std_reverse_iterator_int__ptr_arr2arr__
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator_const_int__ptr_arr2arr___std_reverse_iterator_const_int__ptr_arr2arr__
{}
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
pub struct forward_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {}
pub unsafe fn base_6(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bidirectional_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
}
pub unsafe fn base_7(i: *const bidirectional_iterator_const_char_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bidirectional_iterator_const_char_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_char_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bidirectional_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {}
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
pub type ThrowingIterator_int__ThrowingAction = ();
pub const ThrowingIterator_int__ThrowingAction_TAIncrement: ThrowingIterator_int__ThrowingAction =
    0;
pub const ThrowingIterator_int__ThrowingAction_TADecrement: ThrowingIterator_int__ThrowingAction =
    1;
pub const ThrowingIterator_int__ThrowingAction_TADereference: ThrowingIterator_int__ThrowingAction =
    2;
pub const ThrowingIterator_int__ThrowingAction_TAAssignment: ThrowingIterator_int__ThrowingAction =
    3;
pub const ThrowingIterator_int__ThrowingAction_TAComparison: ThrowingIterator_int__ThrowingAction =
    4;
#[repr(C)]
#[derive(VaArg, FnPtrArg, Default)]
pub struct ThrowingIterator_int_ {
    begin_: *const (),
    end_: *const (),
    current_: *const (),
    action_: ThrowingIterator_int__ThrowingAction,
    index_: (),
}
impl ThrowingIterator_int_ {
    pub unsafe fn copy_from(rhs: *const ThrowingIterator_int_) -> Self {
        let mut this = Self {
            begin_: (*rhs).begin_,
            end_: (*rhs).end_,
            current_: (*rhs).current_,
            action_: (*rhs).action_,
            index_: (*rhs).index_,
        };
        this
    }
}
impl Clone for ThrowingIterator_int_ {
    fn clone(&self) -> Self {
        unsafe { ThrowingIterator_int_::copy_from(self as *const ThrowingIterator_int_) }
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
pub unsafe fn operator_eq_8(
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
            operator_eq_8(
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
pub unsafe fn operator_add_9(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_10(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_9(_i, n)
    });
}
pub unsafe fn iter_swap_11(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_12(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_13(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_14(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_14(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_13(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub static mut globalBuff_15: std::cell::LazyCell<[(); 8]> =
    std::cell::LazyCell::new(|| unsafe { [unimplemented!(); 8] });
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, int (&&)[10]>");
const _: () = assert!(true, "std::is_invocable_v<RangeRBeginT, int (&)[10]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, int (&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, int (&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, int (&&)[10]>");
const _: () = assert!(true, "std::is_invocable_v<RangeCRBeginT, int (&)[10]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, int (&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, int (&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, Incomplete(&&)[]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, const Incomplete(&&)[]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, Incomplete(&&)[]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, const Incomplete(&&)[]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, Incomplete(&&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, const Incomplete(&&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, Incomplete(&&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, const Incomplete(&&)[10]>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, Incomplete(&)[]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, const Incomplete(&)[]>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, Incomplete(&)[]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, const Incomplete(&)[]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, Incomplete(&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, const Incomplete(&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, Incomplete(&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, const Incomplete(&)[10]>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginMember {
    pub x: (),
}
impl RBeginMember {
    pub unsafe fn rbegin(&self) -> *const () {
        return (&self.x as *const ());
    }
}
const _: () = assert!(true, "std::is_invocable_v<RangeRBeginT, RBeginMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, RBeginMember &&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, RBeginMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginMember const&&>"
);
const _: () = assert!(true, "std::is_invocable_v<RangeCRBeginT, RBeginMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, RBeginMember &&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, RBeginMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, RBeginMember const&&>"
);
pub unsafe fn testReturnTypes_16() -> () {
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
        let mut x: Different_17 = <Different_17>::default();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Different_17 {}
pub unsafe fn testArray_18() -> () {
    let mut a: [(); 2] = [unimplemented!(); 2];
    unimplemented!();
    unimplemented!();
    let mut b: [[(); 2]; 2] = [[unimplemented!(); 2]; 2];
    unimplemented!();
    unimplemented!();
    let mut c: [RBeginMember; 2] = [<RBeginMember>::default(); 2];
    unimplemented!();
    unimplemented!();
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginMemberReturnsInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginMemberReturnsInt const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginMemberReturnsVoidPtr {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginMemberReturnsVoidPtr const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrConvertibleRBeginMember_iterator {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrConvertibleRBeginMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, PtrConvertibleRBeginMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonConstRBeginMember {
    pub x: (),
}
impl NonConstRBeginMember {
    pub unsafe fn rbegin(&mut self) -> *mut () {
        return (&mut self.x as *mut ());
    }
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT,  NonConstRBeginMember &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT,  NonConstRBeginMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, NonConstRBeginMember &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, NonConstRBeginMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EnabledBorrowingRBeginMember {}
impl EnabledBorrowingRBeginMember {
    pub unsafe fn rbegin(&self) -> *mut () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_15)).as_mut_ptr();
    }
}
pub static mut enable_borrowed_range_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginMemberFunction {
    pub x: (),
}
impl RBeginMemberFunction {
    pub unsafe fn rbegin(&self) -> *const () {
        return (&self.x as *const ());
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EmptyPtrRBeginMember_Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EmptyPtrRBeginMember {
    pub x: EmptyPtrRBeginMember_Empty,
}
impl EmptyPtrRBeginMember {
    pub unsafe fn rbegin(&self) -> *const EmptyPtrRBeginMember_Empty {
        return (&self.x as *const EmptyPtrRBeginMember_Empty);
    }
}
pub unsafe fn testRBeginMember_20() -> () {
    let mut a: RBeginMember = <RBeginMember>::default();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, RBeginMember&&>");;
    const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, RBeginMember&&>");;
    let mut b: NonConstRBeginMember = <NonConstRBeginMember>::default();
    unimplemented!();
    const _: () = assert!(
        true,
        "!std::is_invocable_v<RangeCRBeginT, NonConstRBeginMember&>"
    );;
    let mut c: EnabledBorrowingRBeginMember = <EnabledBorrowingRBeginMember>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d: RBeginMemberFunction = <RBeginMemberFunction>::default();
    unimplemented!();
    unimplemented!();
    let mut e: EmptyPtrRBeginMember = <EmptyPtrRBeginMember>::default();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn rbegin_21(bf: *const RBeginFunction) -> *const () {
    return (&(*bf).x as *const ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunction {
    pub x: (),
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT,  RBeginFunction const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT,  RBeginFunction &&>"
);
const _: () = assert!(true, "std::is_invocable_v<RangeRBeginT, RBeginFunction&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, RBeginFunction const&>"
);
const _: () = assert!(true, "std::is_invocable_v<RangeCRBeginT, RBeginFunction &>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionReturnsInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginFunctionReturnsInt const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionReturnsVoidPtr {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginFunctionReturnsVoidPtr const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionReturnsEmpty_Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionReturnsEmpty {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginFunctionReturnsEmpty const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionReturnsPtrConvertible_iterator {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionReturnsPtrConvertible {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginFunctionReturnsPtrConvertible const&>"
);
pub unsafe fn rbegin_22(mut _a0: RBeginFunctionByValue) -> *mut () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_15))
        .as_mut_ptr()
        .offset((1_()) as isize);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionByValue {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, RBeginFunctionByValue>"
);
pub unsafe fn rbegin_23(mut _a0: RBeginFunctionEnabledBorrowing) -> *mut () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_15))
        .as_mut_ptr()
        .offset((2_()) as isize);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionEnabledBorrowing {}
pub static mut enable_borrowed_range_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub unsafe fn rbegin_25(
    bf: *const RBeginFunctionReturnsEmptyPtr,
) -> *const RBeginFunctionReturnsEmptyPtr_Empty {
    return (&(*bf).x as *const RBeginFunctionReturnsEmptyPtr_Empty);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionReturnsEmptyPtr_Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionReturnsEmptyPtr {
    pub x: RBeginFunctionReturnsEmptyPtr_Empty,
}
pub unsafe fn rbegin_26(bf: *const RBeginFunctionWithDataMember) -> *const () {
    return (&(*bf).x as *const ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionWithDataMember {
    pub x: (),
    pub rbegin: (),
}
pub unsafe fn rbegin_27(bf: *const RBeginFunctionWithPrivateBeginMember) -> *const () {
    return (&(*bf).y as *const ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginFunctionWithPrivateBeginMember {
    pub y: (),
}
pub unsafe fn testRBeginFunction_28() -> () {
    let mut a: RBeginFunction = <RBeginFunction>::default();
    let aa: RBeginFunction = <RBeginFunction>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b: RBeginFunctionByValue = <RBeginFunctionByValue>::default();
    let bb: RBeginFunctionByValue = <RBeginFunctionByValue>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c: RBeginFunctionEnabledBorrowing = <RBeginFunctionEnabledBorrowing>::default();
    let cc: RBeginFunctionEnabledBorrowing = <RBeginFunctionEnabledBorrowing>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d: RBeginFunctionReturnsEmptyPtr = <RBeginFunctionReturnsEmptyPtr>::default();
    let dd: RBeginFunctionReturnsEmptyPtr = <RBeginFunctionReturnsEmptyPtr>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut e: RBeginFunctionWithDataMember = <RBeginFunctionWithDataMember>::default();
    let ee: RBeginFunctionWithDataMember = <RBeginFunctionWithDataMember>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut f: RBeginFunctionWithPrivateBeginMember =
        <RBeginFunctionWithPrivateBeginMember>::default();
    let ff: RBeginFunctionWithPrivateBeginMember =
        <RBeginFunctionWithPrivateBeginMember>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MemberBeginEnd {
    pub b: (),
    pub e: (),
    pub cb: (),
    pub ce: (),
}
impl MemberBeginEnd {
    pub unsafe fn begin_1(&mut self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ (&mut self.b as *mut ()) });
    }
    pub unsafe fn end_2(&mut self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ (&mut self.e as *mut ()) });
    }
    pub unsafe fn begin_3(&self) -> bidirectional_iterator_const_char_ptr_ {
        return bidirectional_iterator_const_char_ptr_::new_1({ (&self.cb as *const ()) });
    }
    pub unsafe fn end_4(&self) -> bidirectional_iterator_const_char_ptr_ {
        return bidirectional_iterator_const_char_ptr_::new_1({ (&self.ce as *const ()) });
    }
}
const _: () = assert!(true, "std::is_invocable_v<RangeRBeginT, MemberBeginEnd&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, MemberBeginEnd const&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, MemberBeginEnd const&>"
);
pub unsafe fn begin_29(v: *mut FunctionBeginEnd) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (&mut (*v).b as *mut ()) });
}
pub unsafe fn end_30(v: *mut FunctionBeginEnd) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (&mut (*v).e as *mut ()) });
}
pub unsafe fn begin_31(v: *const FunctionBeginEnd) -> bidirectional_iterator_const_char_ptr_ {
    return bidirectional_iterator_const_char_ptr_::new_1({ (&(*v).cb as *const ()) });
}
pub unsafe fn end_32(v: *const FunctionBeginEnd) -> bidirectional_iterator_const_char_ptr_ {
    return bidirectional_iterator_const_char_ptr_::new_1({ (&(*v).ce as *const ()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FunctionBeginEnd {
    pub b: (),
    pub e: (),
    pub cb: (),
    pub ce: (),
}
const _: () = assert!(true, "std::is_invocable_v<RangeRBeginT, FunctionBeginEnd&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, FunctionBeginEnd const&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, FunctionBeginEnd const&>"
);
pub unsafe fn end_33(v: *mut MemberBeginFunctionEnd) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (&mut (*v).e as *mut ()) });
}
pub unsafe fn end_34(v: *const MemberBeginFunctionEnd) -> bidirectional_iterator_const_char_ptr_ {
    return bidirectional_iterator_const_char_ptr_::new_1({ (&(*v).ce as *const ()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MemberBeginFunctionEnd {
    pub b: (),
    pub e: (),
    pub cb: (),
    pub ce: (),
}
impl MemberBeginFunctionEnd {
    pub unsafe fn begin_1(&mut self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ (&mut self.b as *mut ()) });
    }
    pub unsafe fn begin_2(&self) -> bidirectional_iterator_const_char_ptr_ {
        return bidirectional_iterator_const_char_ptr_::new_1({ (&self.cb as *const ()) });
    }
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, MemberBeginFunctionEnd&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, MemberBeginFunctionEnd const&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, MemberBeginFunctionEnd const&>"
);
pub unsafe fn begin_35(v: *mut FunctionBeginMemberEnd) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (&mut (*v).b as *mut ()) });
}
pub unsafe fn begin_36(v: *const FunctionBeginMemberEnd) -> bidirectional_iterator_const_char_ptr_ {
    return bidirectional_iterator_const_char_ptr_::new_1({ (&(*v).cb as *const ()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FunctionBeginMemberEnd {
    pub b: (),
    pub e: (),
    pub cb: (),
    pub ce: (),
}
impl FunctionBeginMemberEnd {
    pub unsafe fn end_1(&mut self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ (&mut self.e as *mut ()) });
    }
    pub unsafe fn end_2(&self) -> bidirectional_iterator_const_char_ptr_ {
        return bidirectional_iterator_const_char_ptr_::new_1({ (&self.ce as *const ()) });
    }
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, FunctionBeginMemberEnd&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, FunctionBeginMemberEnd const&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, FunctionBeginMemberEnd const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MemberBeginEndDifferentTypes {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, MemberBeginEndDifferentTypes&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, MemberBeginEndDifferentTypes&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FunctionBeginEndDifferentTypes {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, FunctionBeginEndDifferentTypes&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, FunctionBeginEndDifferentTypes&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MemberBeginEndForwardIterators {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, MemberBeginEndForwardIterators&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, MemberBeginEndForwardIterators&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FunctionBeginEndForwardIterators {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, FunctionBeginEndForwardIterators&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, FunctionBeginEndForwardIterators&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MemberBeginOnly {}
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, MemberBeginOnly&>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, MemberBeginOnly&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FunctionBeginOnly {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, FunctionBeginOnly&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, FunctionBeginOnly&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MemberEndOnly {}
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, MemberEndOnly&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, MemberEndOnly&>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FunctionEndOnly {}
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, FunctionEndOnly&>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, FunctionEndOnly&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MemberBeginAndRBegin {}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, MemberBeginAndRBegin&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, MemberBeginAndRBegin&>"
);
const _: () = assert!(
    true,
    "std::same_as<std::invoke_result_t<RangeRBeginT, MemberBeginAndRBegin&>, int*>"
);
const _: () = assert!(
    true,
    "std::same_as<std::invoke_result_t<RangeCRBeginT, MemberBeginAndRBegin&>, int*>"
);
pub unsafe fn testBeginEnd_37() -> () {
    let mut a: MemberBeginEnd = <MemberBeginEnd>::default();
    let aa: MemberBeginEnd = <MemberBeginEnd>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b: FunctionBeginEnd = <FunctionBeginEnd>::default();
    let bb: FunctionBeginEnd = <FunctionBeginEnd>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c: MemberBeginFunctionEnd = <MemberBeginFunctionEnd>::default();
    let cc: MemberBeginFunctionEnd = <MemberBeginFunctionEnd>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d: FunctionBeginMemberEnd = <FunctionBeginMemberEnd>::default();
    let dd: FunctionBeginMemberEnd = <FunctionBeginMemberEnd>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
const _: () = assert!(true, "");
const _: () = assert!(true, "");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowMemberRBegin {}
pub static mut ntmb_38: std::cell::LazyCell<NoThrowMemberRBegin> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowMemberRBegin>::default() });
const _: () = assert!(true, "noexcept(std::ranges::rbegin(ntmb))");
const _: () = assert!(true, "noexcept(std::ranges::crbegin(ntmb))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowADLRBegin {}
pub static mut ntab_39: std::cell::LazyCell<NoThrowADLRBegin> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowADLRBegin>::default() });
const _: () = assert!(true, "noexcept(std::ranges::rbegin(ntab))");
const _: () = assert!(true, "noexcept(std::ranges::crbegin(ntab))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowMemberRBeginReturnsRef {}
pub static mut ntmbrr_40: std::cell::LazyCell<NoThrowMemberRBeginReturnsRef> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowMemberRBeginReturnsRef>::default() });
const _: () = assert!(true, "!noexcept(std::ranges::rbegin(ntmbrr))");
const _: () = assert!(true, "!noexcept(std::ranges::crbegin(ntmbrr))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RBeginReturnsArrayRef {}
pub static mut brar_41: std::cell::LazyCell<RBeginReturnsArrayRef> =
    std::cell::LazyCell::new(|| unsafe { <RBeginReturnsArrayRef>::default() });
const _: () = assert!(true, "noexcept(std::ranges::rbegin(brar))");
const _: () = assert!(true, "noexcept(std::ranges::crbegin(brar))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowBeginThrowingEnd {}
pub static mut ntbte_42: std::cell::LazyCell<NoThrowBeginThrowingEnd> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowBeginThrowingEnd>::default() });
const _: () = assert!(true, "!noexcept(std::ranges::rbegin(ntbte))");
const _: () = assert!(true, "!noexcept(std::ranges::crbegin(ntbte))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowEndThrowingBegin {}
pub static mut ntetb_43: std::cell::LazyCell<NoThrowEndThrowingBegin> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowEndThrowingBegin>::default() });
const _: () = assert!(true, "noexcept(std::ranges::rbegin(ntetb))");
const _: () = assert!(true, "noexcept(std::ranges::crbegin(ntetb))");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, Holder<Incomplete>*>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, Holder<Incomplete>*&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, Holder<Incomplete>*>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, Holder<Incomplete>*&>"
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
    (unsafe { testArray_18() });
    const _: () = assert!(true, "testArray()");;
    (unsafe { testRBeginMember_20() });
    const _: () = assert!(true, "testRBeginMember()");;
    (unsafe { testRBeginFunction_28() });
    const _: () = assert!(true, "testRBeginFunction()");;
    (unsafe { testBeginEnd_37() });
    const _: () = assert!(true, "testBeginEnd()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const globalBuff_15);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_19);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_24);
    std::cell::LazyCell::force(&*&raw const ntmb_38);
    std::cell::LazyCell::force(&*&raw const ntab_39);
    std::cell::LazyCell::force(&*&raw const ntmbrr_40);
    std::cell::LazyCell::force(&*&raw const brar_41);
    std::cell::LazyCell::force(&*&raw const ntbte_42);
    std::cell::LazyCell::force(&*&raw const ntetb_43);
}
