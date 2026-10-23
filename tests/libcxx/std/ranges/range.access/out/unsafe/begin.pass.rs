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
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
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
const _: () = assert!(true, "!std::is_invocable_v<RangeBeginT, int (&&)[10]>");
const _: () = assert!(true, "std::is_invocable_v<RangeBeginT, int (&)[10]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeBeginT, int (&&)[]>");
const _: () = assert!(true, "std::is_invocable_v<RangeBeginT, int (&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCBeginT, int (&&)[10]>");
const _: () = assert!(true, "std::is_invocable_v<RangeCBeginT, int (&)[10]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCBeginT, int (&&)[]>");
const _: () = assert!(true, "std::is_invocable_v<RangeCBeginT, int (&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeBeginT, Incomplete(&&)[]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, const Incomplete(&&)[]>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeCBeginT, Incomplete(&&)[]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, const Incomplete(&&)[]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, Incomplete(&&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, const Incomplete(&&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, Incomplete(&&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, const Incomplete(&&)[10]>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeBeginT, Incomplete(&)[]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, const Incomplete(&)[]>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeCBeginT, Incomplete(&)[]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, const Incomplete(&)[]>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeBeginT, Incomplete(&)[10]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, const Incomplete(&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, Incomplete(&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, const Incomplete(&)[10]>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginMember {
    pub x: (),
}
impl BeginMember {
    pub unsafe fn begin(&self) -> *const () {
        return (&self.x as *const ());
    }
}
const _: () = assert!(true, "std::is_invocable_v<RangeBeginT, BeginMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeBeginT, BeginMember &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeBeginT, BeginMember const&>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, BeginMember const&&>"
);
const _: () = assert!(true, "std::is_invocable_v<RangeCBeginT, BeginMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCBeginT, BeginMember &&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCBeginT, BeginMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, BeginMember const&&>"
);
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
    let mut c: [BeginMember; 2] = [<BeginMember>::default(); 2];
    unimplemented!();
    unimplemented!();
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginMemberReturnsInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, BeginMemberReturnsInt const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginMemberReturnsVoidPtr {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, BeginMemberReturnsVoidPtr const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EmptyBeginMember_iterator {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EmptyBeginMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, EmptyBeginMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrConvertibleBeginMember_iterator {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrConvertibleBeginMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, PtrConvertibleBeginMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonConstBeginMember {
    pub x: (),
}
impl NonConstBeginMember {
    pub unsafe fn begin(&mut self) -> *mut () {
        return (&mut self.x as *mut ());
    }
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeBeginT,  NonConstBeginMember &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT,  NonConstBeginMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, NonConstBeginMember &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, NonConstBeginMember const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EnabledBorrowingBeginMember {}
impl EnabledBorrowingBeginMember {
    pub unsafe fn begin(&self) -> *mut () {
        return (&mut (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_13))
            [(0_()) as usize] as *mut ());
    }
}
pub static mut enable_borrowed_range_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginMemberFunction {
    pub x: (),
}
impl BeginMemberFunction {
    pub unsafe fn begin(&self) -> *const () {
        return (&self.x as *const ());
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EmptyPtrBeginMember_Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EmptyPtrBeginMember {
    pub x: EmptyPtrBeginMember_Empty,
}
impl EmptyPtrBeginMember {
    pub unsafe fn begin(&self) -> *const EmptyPtrBeginMember_Empty {
        return (&self.x as *const EmptyPtrBeginMember_Empty);
    }
}
pub unsafe fn testBeginMember_18() -> () {
    let mut a: BeginMember = <BeginMember>::default();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "!std::is_invocable_v<RangeBeginT, BeginMember&&>");;
    const _: () = assert!(true, "!std::is_invocable_v<RangeCBeginT, BeginMember&&>");;
    let mut b: NonConstBeginMember = <NonConstBeginMember>::default();
    unimplemented!();
    const _: () = assert!(
        true,
        "!std::is_invocable_v<RangeCBeginT, NonConstBeginMember&>"
    );;
    let mut c: EnabledBorrowingBeginMember = <EnabledBorrowingBeginMember>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d: BeginMemberFunction = <BeginMemberFunction>::default();
    unimplemented!();
    unimplemented!();
    let mut e: EmptyPtrBeginMember = <EmptyPtrBeginMember>::default();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn begin_19(bf: *const BeginFunction) -> *const () {
    return (&(*bf).x as *const ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunction {
    pub x: (),
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeBeginT,  BeginFunction const&>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeBeginT,  BeginFunction &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeBeginT, BeginFunction&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCBeginT, BeginFunction const&>"
);
const _: () = assert!(true, "std::is_invocable_v<RangeCBeginT, BeginFunction &>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunctionReturnsInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, BeginFunctionReturnsInt const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunctionReturnsVoidPtr {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, BeginFunctionReturnsVoidPtr const&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunctionReturnsPtrConvertible_iterator {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunctionReturnsPtrConvertible {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, BeginFunctionReturnsPtrConvertible const&>"
);
pub unsafe fn begin_20(mut _a0: BeginFunctionByValue) -> *mut () {
    return (&mut (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_13))[(1_()) as usize]
        as *mut ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunctionByValue {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, BeginFunctionByValue>"
);
pub unsafe fn begin_21(mut _a0: BeginFunctionEnabledBorrowing) -> *mut () {
    return (&mut (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_13))[(2_()) as usize]
        as *mut ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunctionEnabledBorrowing {}
pub static mut enable_borrowed_range_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub unsafe fn begin_23(
    bf: *const BeginFunctionReturnsEmptyPtr,
) -> *const BeginFunctionReturnsEmptyPtr_Empty {
    return (&(*bf).x as *const BeginFunctionReturnsEmptyPtr_Empty);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunctionReturnsEmptyPtr_Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunctionReturnsEmptyPtr {
    pub x: BeginFunctionReturnsEmptyPtr_Empty,
}
pub unsafe fn begin_24(bf: *const BeginFunctionWithDataMember) -> *const () {
    return (&(*bf).x as *const ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunctionWithDataMember {
    pub x: (),
    pub begin: (),
}
pub unsafe fn begin_25(bf: *const BeginFunctionWithPrivateBeginMember) -> *const () {
    return (&(*bf).y as *const ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginFunctionWithPrivateBeginMember {
    pub y: (),
}
pub unsafe fn testBeginFunction_26() -> () {
    let mut a: BeginFunction = <BeginFunction>::default();
    let aa: BeginFunction = <BeginFunction>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b: BeginFunctionByValue = <BeginFunctionByValue>::default();
    let bb: BeginFunctionByValue = <BeginFunctionByValue>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c: BeginFunctionEnabledBorrowing = <BeginFunctionEnabledBorrowing>::default();
    let cc: BeginFunctionEnabledBorrowing = <BeginFunctionEnabledBorrowing>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d: BeginFunctionReturnsEmptyPtr = <BeginFunctionReturnsEmptyPtr>::default();
    let dd: BeginFunctionReturnsEmptyPtr = <BeginFunctionReturnsEmptyPtr>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut e: BeginFunctionWithDataMember = <BeginFunctionWithDataMember>::default();
    let ee: BeginFunctionWithDataMember = <BeginFunctionWithDataMember>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut f: BeginFunctionWithPrivateBeginMember =
        <BeginFunctionWithPrivateBeginMember>::default();
    let ff: BeginFunctionWithPrivateBeginMember = <BeginFunctionWithPrivateBeginMember>::default();
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
pub struct NoThrowMemberBegin {}
pub static mut ntmb_27: std::cell::LazyCell<NoThrowMemberBegin> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowMemberBegin>::default() });
const _: () = assert!(true, "noexcept(std::ranges::begin(ntmb))");
const _: () = assert!(true, "noexcept(std::ranges::cbegin(ntmb))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowADLBegin {}
pub static mut ntab_28: std::cell::LazyCell<NoThrowADLBegin> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowADLBegin>::default() });
const _: () = assert!(true, "noexcept(std::ranges::begin(ntab))");
const _: () = assert!(true, "noexcept(std::ranges::cbegin(ntab))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowMemberBeginReturnsRef {}
pub static mut ntmbrr_29: std::cell::LazyCell<NoThrowMemberBeginReturnsRef> =
    std::cell::LazyCell::new(|| unsafe { <NoThrowMemberBeginReturnsRef>::default() });
const _: () = assert!(true, "!noexcept(std::ranges::begin(ntmbrr))");
const _: () = assert!(true, "!noexcept(std::ranges::cbegin(ntmbrr))");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BeginReturnsArrayRef {}
pub static mut brar_30: std::cell::LazyCell<BeginReturnsArrayRef> =
    std::cell::LazyCell::new(|| unsafe { <BeginReturnsArrayRef>::default() });
const _: () = assert!(true, "noexcept(std::ranges::begin(brar))");
const _: () = assert!(true, "noexcept(std::ranges::cbegin(brar))");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, Holder<Incomplete>*>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeBeginT, Holder<Incomplete>*&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, Holder<Incomplete>*>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCBeginT, Holder<Incomplete>*&>"
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
    (unsafe { testBeginMember_18() });
    const _: () = assert!(true, "testBeginMember()");;
    (unsafe { testBeginFunction_26() });
    const _: () = assert!(true, "testBeginFunction()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const globalBuff_13);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_17);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_22);
    std::cell::LazyCell::force(&*&raw const ntmb_27);
    std::cell::LazyCell::force(&*&raw const ntab_28);
    std::cell::LazyCell::force(&*&raw const ntmbrr_29);
    std::cell::LazyCell::force(&*&raw const brar_30);
}
