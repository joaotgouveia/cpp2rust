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
pub struct test_macros_detail_is_same_sentinel_wrapper_char_ptr___sentinel_wrapper_char_ptr__ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_sentinel_wrapper_short_ptr___sentinel_wrapper_short_ptr__ {}
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_char_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_char_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_short_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_short_ptr_ {}
pub unsafe fn operator_eq_6(
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
            operator_eq_6(
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
pub unsafe fn operator_add_7(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_8(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_7(_i, n)
    });
}
pub unsafe fn iter_swap_9(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_10(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*unimplemented!())).cast::<()>();
}
pub unsafe fn operator_eq_11(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_12(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_12(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub static mut globalBuff_13: std::cell::LazyCell<[(); 8]> =
    std::cell::LazyCell::new(|| unsafe { [unimplemented!(); 8] });
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, int (&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, int (&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, int (&&)[10]>");
const _: () = assert!(true, "std::is_invocable_v<RangeEndT, int (&)[10]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, int (&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, int (&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, int (&&)[10]>");
const _: () = assert!(true, "std::is_invocable_v<RangeCEndT, int (&)[10]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, Incomplete(&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, Incomplete(&&)[42]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, Incomplete(&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, Incomplete(&&)[42]>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndMember {
    pub x: (),
}
impl EndMember {
    pub unsafe fn end(&self) -> *const () {
        return (&self.x as *const ());
    }
}
const _: () = assert!(true, "std::is_invocable_v<RangeEndT, EndMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, EndMember &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeEndT, EndMember const&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, EndMember const&&>");
const _: () = assert!(true, "std::is_invocable_v<RangeCEndT, EndMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, EndMember &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeCEndT, EndMember const&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, EndMember const&&>");
pub unsafe fn testReturnTypes_14() -> () {
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
        let mut x: Different_15 = <Different_15>::default();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Different_15 {}
pub unsafe fn testArray_16() -> () {
    let mut a: [(); 2] = [unimplemented!(); 2];
    unimplemented!();
    unimplemented!();
    let mut b: [[(); 2]; 2] = [[unimplemented!(); 2]; 2];
    unimplemented!();
    unimplemented!();
    let mut c: [EndMember; 2] = [<EndMember>::default(); 2];
    unimplemented!();
    unimplemented!();
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndMemberReturnsInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndMemberReturnsInt const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndMemberReturnsVoidPtr {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndMemberReturnsVoidPtr const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrConvertible {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrConvertibleEndMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, PtrConvertibleEndMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoBeginMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, NoBeginMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonConstEndMember {
    pub x: (),
}
impl NonConstEndMember {
    pub unsafe fn begin(&mut self) -> *mut () {
        return std::ptr::null_mut();
    }
    pub unsafe fn end(&mut self) -> *mut () {
        return (&mut self.x as *mut ());
    }
}
const _: () = assert!(true, "std::is_invocable_v<RangeEndT,  NonConstEndMember &>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT,  NonConstEndMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCEndT, NonConstEndMember &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCEndT, NonConstEndMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EnabledBorrowingEndMember {}
impl EnabledBorrowingEndMember {
    pub unsafe fn begin(&self) -> *mut () {
        return std::ptr::null_mut();
    }
    pub unsafe fn end(&self) -> *mut () {
        return (&mut (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_13))
            [(0_()) as usize] as *mut ());
    }
}
pub static mut enable_borrowed_range_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndMemberFunction {
    pub x: (),
}
impl EndMemberFunction {
    pub unsafe fn begin(&self) -> *const () {
        return std::ptr::null();
    }
    pub unsafe fn end(&self) -> *const () {
        return (&self.x as *const ());
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EmptyEndMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EmptyEndMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EmptyPtrEndMember {
    pub x: Empty,
}
impl EmptyPtrEndMember {
    pub unsafe fn begin(&self) -> *const Empty {
        return std::ptr::null();
    }
    pub unsafe fn end(&self) -> *const Empty {
        return (&self.x as *const Empty);
    }
}
pub unsafe fn testEndMember_18() -> () {
    let mut a: EndMember = <EndMember>::default();
    unimplemented!();
    unimplemented!();
    let mut b: NonConstEndMember = <NonConstEndMember>::default();
    unimplemented!();
    const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, decltype((b))>");;
    let mut c: EnabledBorrowingEndMember = <EnabledBorrowingEndMember>::default();
    unimplemented!();
    unimplemented!();
    let mut d: EndMemberFunction = <EndMemberFunction>::default();
    unimplemented!();
    unimplemented!();
    let mut e: EmptyPtrEndMember = <EmptyPtrEndMember>::default();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn begin_19(_a0: *const EndFunction) -> *const () {
    return std::ptr::null();
}
pub unsafe fn end_20(bf: *const EndFunction) -> *const () {
    return (&(*bf).x as *const ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndFunction {
    pub x: (),
}
const _: () = assert!(true, "std::is_invocable_v<RangeEndT, EndFunction const&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, EndFunction &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeEndT,  EndFunction const&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT,  EndFunction &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeEndT, EndFunction&>");
const _: () = assert!(true, "std::is_invocable_v<RangeCEndT, EndFunction const&>");
const _: () = assert!(true, "std::is_invocable_v<RangeCEndT, EndFunction &>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndFunctionReturnsInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndFunctionReturnsInt const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndFunctionReturnsVoidPtr {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndFunctionReturnsVoidPtr const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndFunctionReturnsEmpty {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndFunctionReturnsEmpty const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndFunctionReturnsPtrConvertible {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndFunctionReturnsPtrConvertible const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoBeginFunction {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, NoBeginFunction const&>"
);
pub unsafe fn begin_21(mut _a0: EndFunctionByValue) -> *mut () {
    return std::ptr::null_mut();
}
pub unsafe fn end_22(mut _a0: EndFunctionByValue) -> *mut () {
    return (&mut (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_13))[(1_()) as usize]
        as *mut ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndFunctionByValue {}
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, EndFunctionByValue>");
pub unsafe fn begin_23(mut _a0: EndFunctionEnabledBorrowing) -> *mut () {
    return std::ptr::null_mut();
}
pub unsafe fn end_24(mut _a0: EndFunctionEnabledBorrowing) -> *mut () {
    return (&mut (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_13))[(2_()) as usize]
        as *mut ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndFunctionEnabledBorrowing {}
pub static mut enable_borrowed_range_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub unsafe fn begin_26(_a0: *const EndFunctionReturnsEmptyPtr) -> *const Empty {
    return std::ptr::null();
}
pub unsafe fn end_27(bf: *const EndFunctionReturnsEmptyPtr) -> *const Empty {
    return (&(*bf).x as *const Empty);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndFunctionReturnsEmptyPtr {
    pub x: Empty,
}
pub unsafe fn begin_28(_a0: *const EndFunctionWithDataMember) -> *const () {
    return std::ptr::null();
}
pub unsafe fn end_29(bf: *const EndFunctionWithDataMember) -> *const () {
    return (&(*bf).x as *const ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndFunctionWithDataMember {
    pub x: (),
    pub end: (),
}
pub unsafe fn begin_30(_a0: *const EndFunctionWithPrivateEndMember) -> *const () {
    return std::ptr::null();
}
pub unsafe fn end_31(bf: *const EndFunctionWithPrivateEndMember) -> *const () {
    return (&(*bf).y as *const ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndFunctionWithPrivateEndMember {
    pub y: (),
}
pub unsafe fn end_32(bf: *const BeginMemberEndFunction) -> *const () {
    return (&(*bf).x as *const ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginMemberEndFunction {
    pub x: (),
}
impl BeginMemberEndFunction {
    pub unsafe fn begin(&self) -> *const () {
        return std::ptr::null();
    }
}
pub unsafe fn testEndFunction_33() -> () {
    let a: EndFunction = <EndFunction>::default();
    unimplemented!();
    unimplemented!();
    let mut aa: EndFunction = <EndFunction>::default();
    unimplemented!();
    unimplemented!();
    let mut b: EndFunctionByValue = <EndFunctionByValue>::default();
    unimplemented!();
    unimplemented!();
    let mut c: EndFunctionEnabledBorrowing = <EndFunctionEnabledBorrowing>::default();
    unimplemented!();
    unimplemented!();
    let d: EndFunctionReturnsEmptyPtr = <EndFunctionReturnsEmptyPtr>::default();
    unimplemented!();
    unimplemented!();
    let mut dd: EndFunctionReturnsEmptyPtr = <EndFunctionReturnsEmptyPtr>::default();
    unimplemented!();
    unimplemented!();
    let e: EndFunctionWithDataMember = <EndFunctionWithDataMember>::default();
    unimplemented!();
    unimplemented!();
    let mut ee: EndFunctionWithDataMember = <EndFunctionWithDataMember>::default();
    unimplemented!();
    unimplemented!();
    let f: EndFunctionWithPrivateEndMember = <EndFunctionWithPrivateEndMember>::default();
    unimplemented!();
    unimplemented!();
    let mut ff: EndFunctionWithPrivateEndMember = <EndFunctionWithPrivateEndMember>::default();
    unimplemented!();
    unimplemented!();
    let g: BeginMemberEndFunction = <BeginMemberEndFunction>::default();
    unimplemented!();
    unimplemented!();
    let mut gg: BeginMemberEndFunction = <BeginMemberEndFunction>::default();
    unimplemented!();
    unimplemented!();
    return true;
}
const _: () = assert!(true, "");
const _: () = assert!(true, "");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowMemberEnd {}
pub static mut ntme_34: std::cell::LazyCell<NoThrowMemberEnd> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowMemberEnd>::default() });
const _: () = assert!(true, "noexcept(std::ranges::end(ntme))");
const _: () = assert!(true, "noexcept(std::ranges::cend(ntme))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowADLEnd {}
pub static mut ntae_35: std::cell::LazyCell<NoThrowADLEnd> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowADLEnd>::default() });
const _: () = assert!(true, "noexcept(std::ranges::end(ntae))");
const _: () = assert!(true, "noexcept(std::ranges::cend(ntae))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowMemberEndReturnsRef {}
pub static mut ntmerr_36: std::cell::LazyCell<NoThrowMemberEndReturnsRef> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowMemberEndReturnsRef>::default() });
const _: () = assert!(true, "!noexcept(std::ranges::end(ntmerr))");
const _: () = assert!(true, "!noexcept(std::ranges::cend(ntmerr))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EndReturnsArrayRef {}
pub static mut erar_37: std::cell::LazyCell<EndReturnsArrayRef> =
    std::cell::LazyCell::new(|| unsafe { <EndReturnsArrayRef>::default() });
const _: () = assert!(true, "noexcept(std::ranges::end(erar))");
const _: () = assert!(true, "noexcept(std::ranges::cend(erar))");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, Holder<Incomplete>*>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, Holder<Incomplete>*&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCEndT, Holder<Incomplete>*>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCEndT, Holder<Incomplete>*&>"
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
    (unsafe { testArray_16() });
    const _: () = assert!(true, "testArray()");;
    (unsafe { testEndMember_18() });
    const _: () = assert!(true, "testEndMember()");;
    (unsafe { testEndFunction_33() });
    const _: () = assert!(true, "testEndFunction()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const globalBuff_13);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_17);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_25);
    std::cell::LazyCell::force(&*&raw const ntme_34);
    std::cell::LazyCell::force(&*&raw const ntae_35);
    std::cell::LazyCell::force(&*&raw const ntmerr_36);
    std::cell::LazyCell::force(&*&raw const erar_37);
}
