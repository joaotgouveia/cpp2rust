extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type check_unqualified_lookup_unscoped_enum = ();
pub const check_unqualified_lookup_unscoped_enum_a: check_unqualified_lookup_unscoped_enum = 0;
pub const check_unqualified_lookup_unscoped_enum_b: check_unqualified_lookup_unscoped_enum = 1;
pub const check_unqualified_lookup_unscoped_enum_c: check_unqualified_lookup_unscoped_enum = 2;
pub unsafe fn iter_move_0(
    e: *mut check_unqualified_lookup_unscoped_enum,
) -> check_unqualified_lookup_unscoped_enum {
    return (*e);
}
pub type check_unqualified_lookup_scoped_enum = ();
pub const check_unqualified_lookup_scoped_enum_a: check_unqualified_lookup_scoped_enum = 0;
pub const check_unqualified_lookup_scoped_enum_b: check_unqualified_lookup_scoped_enum = 1;
pub const check_unqualified_lookup_scoped_enum_c: check_unqualified_lookup_scoped_enum = 2;
pub unsafe fn iter_move_1(
    _a0: *mut check_unqualified_lookup_scoped_enum,
) -> *mut check_unqualified_lookup_scoped_enum {
    return std::ptr::null_mut();
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union check_unqualified_lookup_some_union {
    pub x: (),
    pub y: (),
}
impl Default for check_unqualified_lookup_some_union {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub unsafe fn iter_move_2(u: *mut check_unqualified_lookup_some_union) -> () {
    return (*u).x;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct move_tracker {
    moves_: (),
}
impl move_tracker {
    pub unsafe fn move_from(other: *mut move_tracker) -> Self {
        let mut this = Self {
            moves_: (((*other).moves_) + (1_())),
        };
        (*other).moves_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, other: *mut move_tracker) -> *mut move_tracker {
        self.moves_ = (((*other).moves_) + (1_()));
        (*other).moves_ = 0_();
        return &mut (*(self as *mut move_tracker));
    }
    pub unsafe fn moves(&self) -> () {
        return self.moves_;
    }
}
impl Default for move_tracker {
    fn default() -> Self {
        unsafe { move_tracker { moves_: 0_() } }
    }
}
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
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct forward_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
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
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
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
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
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
pub unsafe fn operator_eq_3(
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
            operator_eq_3(
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
pub unsafe fn operator_add_4(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_5(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_4(_i, n)
    });
}
pub unsafe fn iter_swap_6(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_7(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_8(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_9(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_9(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn iter_swap_10(a: *mut HasIterSwap, b: *mut HasIterSwap) {
    (*(*a).value_) = 1_();
    (*(*b).value_) = 1_();
}
pub unsafe fn iter_swap_11(a: *mut HasIterSwap, b: *mut ()) {
    (*(*a).value_) = 2_();
    (*b) = 2_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasIterSwap {
    pub value_: *mut (),
}
impl HasIterSwap {
    pub unsafe fn new(value: *mut ()) -> Self {
        let mut this = Self { value_: value };
        unimplemented!();
        this
    }
}
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, HasIterSwap&, HasIterSwap&>"
);
const _: () = assert!(true, "std::is_invocable_v<IterSwapT, HasIterSwap&, int&>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, int&, HasIterSwap&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT&, HasIterSwap&, HasIterSwap&>"
);
const _: () = assert!(true, "std::is_invocable_v<IterSwapT&, HasIterSwap&, int&>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT&, int&, HasIterSwap&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT&&, HasIterSwap&, HasIterSwap&>"
);
const _: () = assert!(true, "std::is_invocable_v<IterSwapT&&, HasIterSwap&, int&>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<IterSwapT&&, int&, HasIterSwap&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StructWithNotMoreSpecializedIterSwap {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<IterSwapT, StructWithNotMoreSpecializedIterSwap&, StructWithNotMoreSpecializedIterSwap&>"
);
pub unsafe fn iter_swap_12(_a0: *mut NodiscardIterSwap, _a1: *mut NodiscardIterSwap) -> () {
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NodiscardIterSwap {}
pub unsafe fn ensureVoidCast_13(a: *mut NodiscardIterSwap, b: *mut NodiscardIterSwap) {
    (unsafe {
        let ___x: *mut NodiscardIterSwap = a;
        let ___y: *mut NodiscardIterSwap = b;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(___x, ___y)
    });
}
pub unsafe fn swap_15(a: *mut HasRangesSwap, b: *mut HasRangesSwap) {
    (*(*a).value_) = 1_();
    (*(*b).value_) = 1_();
}
pub unsafe fn swap_16(a: *mut HasRangesSwap, b: *mut ()) {
    (*(*a).value_) = 2_();
    (*b) = 2_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasRangesSwap {
    pub value_: *mut (),
}
impl HasRangesSwap {
    pub unsafe fn new(value: *mut ()) -> Self {
        let mut this = Self { value_: value };
        unimplemented!();
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasRangesSwapWrapper {
    pub value_: *mut HasRangesSwap,
}
impl HasRangesSwapWrapper {
    pub unsafe fn new(value: *mut HasRangesSwap) -> Self {
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut HasRangesSwap {
        return self.value_;
    }
}
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, HasRangesSwapWrapper&, HasRangesSwapWrapper&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<IterSwapT, HasRangesSwapWrapper&, int&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<IterSwapT, int&, HasRangesSwapWrapper&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct A {
    pub value: (),
}
impl A {
    pub unsafe fn operator_assign_1(&mut self, _a0: *const B) -> *mut A {
        self.value = true;
        return &mut (*(self as *mut A));
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A { value: false } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct B {
    pub value: (),
}
impl B {
    pub unsafe fn operator_assign_1(&mut self, _a0: *const A) -> *mut B {
        self.value = true;
        return &mut (*(self as *mut B));
    }
}
impl Default for B {
    fn default() -> Self {
        unsafe { B { value: false } }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly1 {
    pub value: (),
}
impl MoveOnly1 {
    pub unsafe fn move_from(_a0: *mut MoveOnly1) -> Self {
        let mut this = Self {
            value: (*_a0).value,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveOnly1) -> *mut MoveOnly1 {
        self.value = (*_a0).value;
        return &mut (*(self as *mut MoveOnly1));
    }
    pub unsafe fn operator_assign_6(&mut self, _a0: *mut MoveOnly2) -> *mut MoveOnly1 {
        self.value = true;
        return &mut (*(self as *mut MoveOnly1));
    }
}
impl Default for MoveOnly1 {
    fn default() -> Self {
        unsafe { MoveOnly1 { value: false } }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly2 {
    pub value: (),
}
impl MoveOnly2 {
    pub unsafe fn move_from(_a0: *mut MoveOnly2) -> Self {
        let mut this = Self {
            value: (*_a0).value,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveOnly2) -> *mut MoveOnly2 {
        self.value = (*_a0).value;
        return &mut (*(self as *mut MoveOnly2));
    }
    pub unsafe fn operator_assign_6(&mut self, _a0: *mut MoveOnly1) -> *mut MoveOnly2 {
        self.value = true;
        return &mut (*(self as *mut MoveOnly2));
    }
}
impl Default for MoveOnly2 {
    fn default() -> Self {
        unsafe { MoveOnly2 { value: false } }
    }
}
pub unsafe fn test_17() -> () {
    {
        let mut value1: () = 0_();
        let mut value2: () = 0_();
        let mut a: HasIterSwap = HasIterSwap::new({ &mut value1 });
        let mut b: HasIterSwap = HasIterSwap::new({ &mut value2 });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut a, &mut b) });
        unimplemented!();
    }
    {
        let mut value1: () = 0_();
        let mut value2: () = 0_();
        let mut c: HasRangesSwap = HasRangesSwap::new({ &mut value1 });
        let mut d: HasRangesSwap = HasRangesSwap::new({ &mut value2 });
        let mut cWrapper: HasRangesSwapWrapper = HasRangesSwapWrapper::new({ &mut c });
        let mut dWrapper: HasRangesSwapWrapper = HasRangesSwapWrapper::new({ &mut d });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(
                &mut cWrapper,
                &mut dWrapper,
            )
        });
        unimplemented!();
    }
    {
        let mut value1: () = 0_();
        let mut value2: () = 0_();
        let mut c: HasRangesSwap = HasRangesSwap::new({ &mut value1 });
        let mut d: HasRangesSwap = HasRangesSwap::new({ &mut value2 });
        (unsafe {
            let mut ___x: HasRangesSwapWrapper = HasRangesSwapWrapper::new({ &mut c });
            let mut ___y: HasRangesSwapWrapper = HasRangesSwapWrapper::new({ &mut d });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ___x, &mut ___y)
        });
        unimplemented!();
    }
    {
        let mut e: A = <A>::default();
        let mut f: B = <B>::default();
        let mut ePtr: *mut A = (&mut e as *mut A);
        let mut fPtr: *mut B = (&mut f as *mut B);
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ePtr, &mut fPtr)
        });
        unimplemented!();
    }
    {
        let mut g: MoveOnly1 = <MoveOnly1>::default();
        let mut h: MoveOnly2 = <MoveOnly2>::default();
        (unsafe {
            let mut ___x: *mut MoveOnly1 = (&mut g as *mut MoveOnly1);
            let mut ___y: *mut MoveOnly2 = (&mut h as *mut MoveOnly2);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ___x, &mut ___y)
        });
        unimplemented!();
    }
    {
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___x: *mut move_tracker = unimplemented!();
            let mut ___y: *mut move_tracker = unimplemented!().offset((1_()) as isize);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ___x, &mut ___y)
        });
        {
            unimplemented!();
        }
    }
    {
        let mut buff: [(); 2] = [1_(), 2_()];
        (unsafe {
            let mut ___x: *mut () = buff.as_mut_ptr().offset((0_()) as isize);
            let mut ___y: *mut () = buff.as_mut_ptr().offset((1_()) as isize);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ___x, &mut ___y)
        });
        unimplemented!();
        (unsafe {
            let mut ___x: cpp20_input_iterator_int_ptr_ =
                cpp20_input_iterator_int_ptr_::new({ buff.as_mut_ptr() });
            let mut ___y: cpp20_input_iterator_int_ptr_ =
                cpp20_input_iterator_int_ptr_::new({ buff.as_mut_ptr().offset((1_()) as isize) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ___x, &mut ___y)
        });
        unimplemented!();
        (unsafe {
            let mut ___x: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ buff.as_mut_ptr() });
            let mut ___y: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({
                    buff.as_mut_ptr().offset((1_()) as isize)
                });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ___x, &mut ___y)
        });
        unimplemented!();
        (unsafe {
            let mut ___x: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ buff.as_mut_ptr() });
            let mut ___y: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ buff.as_mut_ptr().offset((1_()) as isize) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ___x, &mut ___y)
        });
        unimplemented!();
        (unsafe {
            let mut ___x: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ buff.as_mut_ptr() });
            let mut ___y: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    buff.as_mut_ptr().offset((1_()) as isize)
                });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ___x, &mut ___y)
        });
        unimplemented!();
        (unsafe {
            let mut ___x: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ buff.as_mut_ptr() });
            let mut ___y: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    buff.as_mut_ptr().offset((1_()) as isize)
                });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ___x, &mut ___y)
        });
        unimplemented!();
        (unsafe {
            let mut ___x: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ buff.as_mut_ptr() });
            let mut ___y: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ buff.as_mut_ptr().offset((1_()) as isize) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_14))(&mut ___x, &mut ___y)
        });
        unimplemented!();
    }
    return true;
}
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, int*>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, int*, int*, int*>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, int, int*>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, int*, int>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, void*, void*>");
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, Holder<Incomplete>**, Holder<Incomplete>**>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, Holder<Incomplete>**, Holder<Incomplete>**&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, Holder<Incomplete>**&, Holder<Incomplete>**>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, Holder<Incomplete>**&, Holder<Incomplete>**&>"
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
    (unsafe { test_17() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
