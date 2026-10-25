extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub unsafe fn base_0(i: *const cpp17_output_iterator_int_ptr_) -> *mut () {
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
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_output_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_output_iterator_int_ptr_));
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn base_1(i: *const random_access_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_inc();
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
pub unsafe fn base_2(mut i: *mut ()) -> *mut () {
    return i;
}
pub unsafe fn base_3(i: *const cpp20_input_iterator_int_ptr_) -> *mut () {
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
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp20_input_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp20_input_iterator_int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
pub unsafe fn base_4(i: *const cpp20_output_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
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
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp20_output_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp20_output_iterator_int_ptr_));
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
pub struct sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__ {
    base_: InputIteratorNotDerivedFromGeneric_int_,
}
impl sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_InputIteratorNotIndirectlyReadable_ {
    base_: InputIteratorNotIndirectlyReadable,
}
impl sentinel_wrapper_InputIteratorNotIndirectlyReadable_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_InputIteratorNotInputOrOutputIterator_ {
    base_: InputIteratorNotInputOrOutputIterator,
}
impl sentinel_wrapper_InputIteratorNotInputOrOutputIterator_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_ {
    base_: BidirectionalIteratorNotDerivedFrom,
}
impl sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_BidirectionalIteratorNotDecrementable_ {
    base_: BidirectionalIteratorNotDecrementable,
}
impl sentinel_wrapper_BidirectionalIteratorNotDecrementable_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_int_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_int_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_2((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_2((*other)) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_3(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_3(other) }));
    }
}
pub unsafe fn operator_eq_5(
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
            operator_eq_5(
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
pub unsafe fn operator_add_6(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_7(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_6(_i, n)
    });
}
pub unsafe fn iter_swap_8(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_9(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_10(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_11(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_11(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_int_ptr__int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_InputIteratorNotDerivedFromGeneric_int___sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_InputIteratorNotIndirectlyReadable__sentinel_wrapper_InputIteratorNotIndirectlyReadable__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_InputIteratorNotInputOrOutputIterator__sentinel_wrapper_InputIteratorNotInputOrOutputIterator__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__ {}
const _: () = assert!(
    true,
    "std::ranges::contiguous_range<UncheckedRange<int*, int*>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InputIteratorNotDerivedFromGeneric_int_ {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    true,
    "std::indirectly_readable<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotDerivedFrom>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InputIteratorNotIndirectlyReadable {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::indirectly_readable<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotIndirectlyReadable>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InputIteratorNotInputOrOutputIterator {}
const _: () = assert!(
    !(false),
    "!std::input_or_output_iterator<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::indirectly_readable<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotInputOrOutputIterator>"
);
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct IndirectUnaryPredicateNotCopyConstructible {}
const _: () = assert!(
    true,
    "std::predicate<IndirectUnaryPredicateNotCopyConstructible, int&>"
);
const _: () = assert!(
    !(false),
    "!std::indirect_unary_predicate<IndirectUnaryPredicateNotCopyConstructible, int*>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IndirectUnaryPredicateNotPredicate {}
const _: () = assert!(
    !(false),
    "!std::predicate<IndirectUnaryPredicateNotPredicate, int&>"
);
const _: () = assert!(
    !(false),
    "!std::indirect_unary_predicate<IndirectUnaryPredicateNotPredicate, int*>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SentinelForNotSemiregular {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<SentinelForNotSemiregular>"
);
const _: () = assert!(!(false), "!std::semiregular<SentinelForNotSemiregular>");
const _: () = assert!(
    !(false),
    "!std::sentinel_for<SentinelForNotSemiregular, cpp20_input_iterator<int*>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SentinelForNotWeaklyEqualityComparableWith {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    true,
    "std::semiregular<SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!std::sentinel_for<SentinelForNotWeaklyEqualityComparableWith, cpp20_input_iterator<int*>>"
);
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct WeaklyIncrementableNotMovable {}
const _: () = assert!(!(false), "!std::movable<WeaklyIncrementableNotMovable>");
const _: () = assert!(
    !(false),
    "!std::weakly_incrementable<WeaklyIncrementableNotMovable>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ForwardIteratorNotDerivedFrom {}
impl ForwardIteratorNotDerivedFrom {
    pub unsafe fn operator_eq(&self, _a0: *const ForwardIteratorNotDerivedFrom) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for ForwardIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ForwardIteratorNotDerivedFrom::operator_eq(
                self,
                other as *const ForwardIteratorNotDerivedFrom,
            )
        }
    }
}
impl std::cmp::Eq for ForwardIteratorNotDerivedFrom {}
const _: () = assert!(true, "std::input_iterator<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(true, "std::incrementable<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(
    true,
    "std::sentinel_for<ForwardIteratorNotDerivedFrom, ForwardIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::forward_iterator<ForwardIteratorNotDerivedFrom>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ForwardIteratorNotIncrementable {}
impl ForwardIteratorNotIncrementable {
    pub unsafe fn operator_eq(&self, _a0: *const ForwardIteratorNotIncrementable) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for ForwardIteratorNotIncrementable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ForwardIteratorNotIncrementable::operator_eq(
                self,
                other as *const ForwardIteratorNotIncrementable,
            )
        }
    }
}
impl std::cmp::Eq for ForwardIteratorNotIncrementable {}
const _: () = assert!(true, "std::input_iterator<ForwardIteratorNotIncrementable>");
const _: () = assert!(
    !(false),
    "!std::incrementable<ForwardIteratorNotIncrementable>"
);
const _: () = assert!(
    true,
    "std::sentinel_for<ForwardIteratorNotIncrementable, ForwardIteratorNotIncrementable>"
);
const _: () = assert!(
    !(false),
    "!std::forward_iterator<ForwardIteratorNotIncrementable>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BidirectionalIteratorNotDerivedFrom {}
impl BidirectionalIteratorNotDerivedFrom {
    pub unsafe fn operator_eq(&self, _a0: *const BidirectionalIteratorNotDerivedFrom) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for BidirectionalIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            BidirectionalIteratorNotDerivedFrom::operator_eq(
                self,
                other as *const BidirectionalIteratorNotDerivedFrom,
            )
        }
    }
}
impl std::cmp::Eq for BidirectionalIteratorNotDerivedFrom {}
const _: () = assert!(
    true,
    "std::forward_iterator<BidirectionalIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::bidirectional_iterator<BidirectionalIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::bidirectional_range<BidirectionalRangeNotDerivedFrom>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BidirectionalIteratorNotDecrementable {}
impl BidirectionalIteratorNotDecrementable {
    pub unsafe fn operator_eq(&self, _a0: *const BidirectionalIteratorNotDecrementable) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for BidirectionalIteratorNotDecrementable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            BidirectionalIteratorNotDecrementable::operator_eq(
                self,
                other as *const BidirectionalIteratorNotDecrementable,
            )
        }
    }
}
impl std::cmp::Eq for BidirectionalIteratorNotDecrementable {}
const _: () = assert!(
    true,
    "std::forward_iterator<BidirectionalIteratorNotDecrementable>"
);
const _: () = assert!(
    !(false),
    "!std::bidirectional_iterator<BidirectionalIteratorNotDecrementable>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::bidirectional_range<BidirectionalRangeNotDecrementable>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PermutableNotForwardIterator {}
const _: () = assert!(true, "std::input_iterator<PermutableNotForwardIterator>");
const _: () = assert!(
    !(false),
    "!std::forward_iterator<PermutableNotForwardIterator>"
);
const _: () = assert!(!(false), "!std::permutable<PermutableNotForwardIterator>");
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct PermutableNotSwappable_NotSwappable {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PermutableNotSwappable {}
impl PermutableNotSwappable {
    pub unsafe fn operator_eq(&self, _a0: *const PermutableNotSwappable) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for PermutableNotSwappable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { PermutableNotSwappable::operator_eq(self, other as *const PermutableNotSwappable) }
    }
}
impl std::cmp::Eq for PermutableNotSwappable {}
const _: () = assert!(true, "std::input_iterator<PermutableNotSwappable>");
const _: () = assert!(true, "std::forward_iterator<PermutableNotSwappable>");
const _: () = assert!(!(false), "!std::permutable<PermutableNotSwappable>");
const _: () = assert!(
    !(false),
    "!std::indirectly_swappable<PermutableNotSwappable>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OutputIteratorNotInputOrOutputIterator {}
const _: () = assert!(
    !(false),
    "!std::input_or_output_iterator<OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::indirectly_writable<OutputIteratorNotInputOrOutputIterator, int>"
);
const _: () = assert!(
    !(false),
    "!std::output_iterator<OutputIteratorNotInputOrOutputIterator, int>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::output_range<OutputRangeNotInputOrOutputIterator, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OutputIteratorNotIndirectlyWritable {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<OutputIteratorNotIndirectlyWritable>"
);
const _: () = assert!(
    !(false),
    "!std::indirectly_writable<OutputIteratorNotIndirectlyWritable, int>"
);
const _: () = assert!(
    !(false),
    "!std::output_iterator<OutputIteratorNotIndirectlyWritable, int>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::output_range<OutputIteratorNotIndirectlyWritable, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IndirectBinaryPredicateNotIndirectlyReadable {}
const _: () = assert!(
    !(false),
    "!std::indirect_binary_predicate<std::ranges::equal_to, IndirectBinaryPredicateNotIndirectlyReadable, int*>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RandomAccessIteratorNotDerivedFrom {}
impl RandomAccessIteratorNotDerivedFrom {
    pub unsafe fn operator_cmp(&self, _a0: *const RandomAccessIteratorNotDerivedFrom) -> () {
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const RandomAccessIteratorNotDerivedFrom) -> () {
        return true;
    }
}
impl std::cmp::Ord for RandomAccessIteratorNotDerivedFrom {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            RandomAccessIteratorNotDerivedFrom::operator_cmp(
                self,
                other as *const RandomAccessIteratorNotDerivedFrom,
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorNotDerivedFrom {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            RandomAccessIteratorNotDerivedFrom::operator_cmp(
                self,
                other as *const RandomAccessIteratorNotDerivedFrom,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorNotDerivedFrom {}
const _: () = assert!(
    true,
    "std::bidirectional_iterator<RandomAccessIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorNotDerivedFrom>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RandomAccessIteratorBadIndex {}
impl RandomAccessIteratorBadIndex {
    pub unsafe fn operator_cmp(&self, _a0: *const RandomAccessIteratorBadIndex) -> () {
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const RandomAccessIteratorBadIndex) -> () {
        return true;
    }
}
impl std::cmp::Ord for RandomAccessIteratorBadIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            RandomAccessIteratorBadIndex::operator_cmp(
                self,
                other as *const RandomAccessIteratorBadIndex,
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorBadIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorBadIndex {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            RandomAccessIteratorBadIndex::operator_cmp(
                self,
                other as *const RandomAccessIteratorBadIndex,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorBadIndex {}
const _: () = assert!(
    true,
    "std::bidirectional_iterator<RandomAccessIteratorBadIndex>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorBadIndex>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RandomAccessIteratorBadDifferenceType {}
impl RandomAccessIteratorBadDifferenceType {
    pub unsafe fn operator_cmp(&self, _a0: *const RandomAccessIteratorBadDifferenceType) -> () {
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const RandomAccessIteratorBadDifferenceType) -> () {
        return true;
    }
}
impl std::cmp::Ord for RandomAccessIteratorBadDifferenceType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            RandomAccessIteratorBadDifferenceType::operator_cmp(
                self,
                other as *const RandomAccessIteratorBadDifferenceType,
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorBadDifferenceType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorBadDifferenceType {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            RandomAccessIteratorBadDifferenceType::operator_cmp(
                self,
                other as *const RandomAccessIteratorBadDifferenceType,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorBadDifferenceType {}
const _: () = assert!(true, "std::regular<RandomAccessIteratorBadDifferenceType>");
const _: () = assert!(
    !(false),
    "!std::weakly_incrementable<RandomAccessIteratorBadDifferenceType>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorBadDifferenceType>"
);
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct counting_predicate__lambda_at_ranges_partition_copy_pass_cpp_273_26__ {
    pred_: FnPtr<fn(()) -> ()>,
    count_: *mut (),
}
impl counting_predicate__lambda_at_ranges_partition_copy_pass_cpp_273_26__ {
    pub unsafe fn new_1(mut pred: FnPtr<fn(()) -> ()>, count: *mut ()) -> Self {
        let mut this = Self {
            pred_: pred.move_from(),
            count_: (count),
        };
        this
    }
    pub unsafe fn operator_call_3_int_ref(&mut self, args: *mut ()) -> () {
        (*self.count_).prefix_inc();
        return (unsafe {
            let _x: () = (*args);
            self.pred_.call(_x)
        });
    }
    pub unsafe fn operator_call_3_const_int_ref(&mut self, args: *const ()) -> () {
        (*self.count_).prefix_inc();
        return (unsafe {
            let _x: () = (*args);
            self.pred_.call(_x)
        });
    }
}
impl Default for counting_predicate__lambda_at_ranges_partition_copy_pass_cpp_273_26__ {
    fn default() -> Self {
        unsafe {
            counting_predicate__lambda_at_ranges_partition_copy_pass_cpp_273_26__ {
                pred_: FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
                    unsafe {
                        return ((x) < (0_()));
                    }
                }),
                count_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct counting_projection_std_identity_ {
    proj_: (),
    count_: *mut (),
}
impl counting_projection_std_identity_ {
    pub unsafe fn new_1(count: *mut ()) -> Self {
        let mut this = Self {
            proj_: unimplemented!(),
            count_: (count),
        };
        this
    }
    pub unsafe fn operator_call_4_int_ref(&self, value: *mut ()) -> *mut () {
        (*self.count_).prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_4_const_int_ref(&self, value: *const ()) -> *const () {
        (*self.count_).prefix_inc();
        return unimplemented!();
    }
}
impl Default for counting_projection_std_identity_ {
    fn default() -> Self {
        unsafe {
            counting_projection_std_identity_ {
                proj_: unimplemented!(),
                count_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UnaryPred {}
const _: () = assert!(
    true,
    "HasPartitionCopyIter<int*, int*, int*, int*, UnaryPred>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyIter<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyIter<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyIter<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyIter<int*, SentinelForNotSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyIter<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyIter<int*, int*, WeaklyIncrementableNotMovable>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyIter<int*, int*, int*, WeaklyIncrementableNotMovable>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyIter<int*, int*, int*, int*, IndirectUnaryPredicateNotPredicate>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyIter<int*, int*, int*, int*, IndirectUnaryPredicateNotCopyConstructible>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Uncopyable {}
impl Uncopyable {}
const _: () = assert!(!(false), "!HasPartitionCopyIter<int*, int*, Uncopyable*>");
const _: () = assert!(
    !(false),
    "!HasPartitionCopyIter<int*, int*, int*, Uncopyable*>"
);
const _: () = assert!(
    true,
    "HasPartitionCopyRange<R<int*>, int*, int*, UnaryPred>"
);
const _: () = assert!(!(false), "!HasPartitionCopyRange<InputRangeNotDerivedFrom>");
const _: () = assert!(
    !(false),
    "!HasPartitionCopyRange<InputRangeNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyRange<InputRangeNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyRange<R<int*>, WeaklyIncrementableNotMovable>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyRange<R<int*>, int*, WeaklyIncrementableNotMovable>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyRange<R<int*>, int*, int*, IndirectUnaryPredicateNotPredicate>"
);
const _: () = assert!(
    !(false),
    "!HasPartitionCopyRange<R<int*>, int*, int*, IndirectUnaryPredicateNotCopyConstructible>"
);
const _: () = assert!(!(false), "!HasPartitionCopyRange<R<int*>, Uncopyable*>");
const _: () = assert!(
    !(false),
    "!HasPartitionCopyRange<R<int*>, int*, Uncopyable*>"
);
const _: () = assert!(
    true,
    "std::is_same_v<std::ranges::partition_copy_result<int, int, int>,\n    std::ranges::in_out_out_result<int, int, int>>"
);
pub unsafe fn test_one_12(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_0: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_0 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_14(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_1: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_1 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_15(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_2: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_2 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_16(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_3: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_3 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_17(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_4: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_4 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_18(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_5: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_5 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_19(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_6: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_6 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_20(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_7: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_7 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_21(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_8: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_8 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_22(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_9: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_9 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_23(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_10: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_10 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_24(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_11: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_11 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_25(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_12: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_12 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_26(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_13: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_13 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_27(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_14: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_14 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_28(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_15: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_15 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_29(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_16: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_16 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_30(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_17: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_17 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_31(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_18: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_18 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_32(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_19: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_19 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_33(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_20: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_20 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_34(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_21: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_21 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_35(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_22: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_22 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_36(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_23: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_23 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_37(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_24: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_24 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_38(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_25: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_25 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_39(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_26: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_26 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_40(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_27: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_27 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_41(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_28: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_28 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_42(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_29: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_29 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_43(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_30: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_30 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_44(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_31: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_31 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_45(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_32: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_32 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_46(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_33: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_33 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_47(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_34: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_34 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_48(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_35: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_35 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_49(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_36: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_36 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_50(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_37: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_37 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_51(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_38: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_38 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_52(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_39: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_39 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_53(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_40: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_40 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_54(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_41: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_41 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_55(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_42: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_42 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_56(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_43: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_43 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_57(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_44: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_44 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_58(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_45: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_45 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_59(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_46: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_46 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_60(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_47: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_47 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_61(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_48: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_48 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_62(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_49: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_49 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_63(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_50: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_50 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_64(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_51: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_51 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_65(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_52: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_52 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_66(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_53: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_53 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_67(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_54: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_54 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_68(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_55: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_55 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_69(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_56: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_56 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_70(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_57: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_57 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_71(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_58: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_58 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_72(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_59: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_59 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_73(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_60: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_60 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_74(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_61: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_61 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_75(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_62: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_62 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_76(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_63: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_63 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_77(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_64: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_64 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_78(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_65: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_65 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_79(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_66: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_66 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_80(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_67: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_67 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_81(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_68: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_68 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_82(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_69: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_69 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_83(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_70: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_70 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_84(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_71: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_71 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_85(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_72: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_72 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_86(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_73: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_73 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_87(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_74: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_74 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_88(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_75: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_75 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_89(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_76: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_76 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_90(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_77: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_77 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_91(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_78: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_78 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_92(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_79: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_79 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_93(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_80: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_80 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_94(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_81: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_81 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_95(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_82: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_82 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_96(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_83: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_83 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_97(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_84: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_84 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_98(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_85: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_85 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_99(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_86: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_86 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_100(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_87: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_87 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_101(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_88: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_88 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_102(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_89: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_89 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_103(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_90: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_90 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_104(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_91: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_91 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_105(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_92: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_92 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_106(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_93: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_93 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_107(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_94: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_94 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_108(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_95: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_95 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_109(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_96: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_96 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_110(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_97: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_97 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_111(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_98: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_98 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_112(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_99: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_99 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_113(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_100: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_100 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_114(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_101: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_101 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_115(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_102: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_102 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_116(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_103: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_103 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_117(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_104: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_104 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_118(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_105: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_105 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_119(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_106: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_106 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_120(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_107: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_107 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_121(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_108: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_108 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_122(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_109: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_109 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_123(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_110: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_110 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_124(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_111: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_111 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_125(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_112: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_112 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_126(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_113: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_113 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_127(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_114: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_114 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_128(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_115: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_115 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_129(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_116: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_116 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_130(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_117: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_117 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_131(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_118: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_118 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_132(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_119: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_119 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_133(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_120: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_120 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_134(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_121: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_121 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_135(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_122: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_122 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_136(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_123: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_123 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_137(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_124: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_124 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_138(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_125: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_125 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_139(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_126: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_126 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_140(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_127: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_127 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_141(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_128: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_128 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_142(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_129: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_129 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_143(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_130: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_130 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_144(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_131: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_131 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_145(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_146(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_147(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_148(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_149(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_150(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_151(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_152(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_153(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_154(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_155(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_156(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_157(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_158(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_159(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_160(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_161(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_162(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_163(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_164(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_165(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_166(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_167(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_168(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_169(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_170(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_171(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_172(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_173(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_174(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_175(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_176(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_177(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_178(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_179(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_180(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_181(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_182(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_183(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_184(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_185(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_186(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_187(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_188(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_189(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_190(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_191(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_192(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_193(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_194(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_195(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_196(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_197(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_198(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_199(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_200(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_201(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_202(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_203(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_204(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_205(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_206(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_207(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_208(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_209(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_210(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_211(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_212(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_213(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_214(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_215(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_216(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_217(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_218(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_219(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_220(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_221(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_222(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_223(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_224(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_225(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_226(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_227(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_228(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_229(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_230(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_231(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_232(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_233(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_234(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_235(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_236(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_237(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_238(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_239(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_240(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_241(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_242(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_243(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_244(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_245(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_246(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_247(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_248(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_249(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_250(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_251(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_252(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_253(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_254(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_255(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_256(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_257(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_258(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_259(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_260(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_261(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_262(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_263(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_264(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_265(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_266(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_267(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_268(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_269(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_270(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_271(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_272(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_273(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_274(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_275(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_276(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                end,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_277(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_132: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_132 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_278(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_133: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_133 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_279(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_134: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_134 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_280(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_135: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_135 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_281(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_136: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_136 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_282(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_137: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_137 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_283(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_138: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_138 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_284(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_139: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_139 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_285(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_140: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_140 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_286(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_141: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_141 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_287(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_142: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_142 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_288(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_143: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_143 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_289(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_144: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_144 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_290(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_145: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_145 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_291(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_146: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_146 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_292(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_147: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_147 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_293(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_148: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_148 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_294(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_149: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_149 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_295(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_150: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_150 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_296(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_151: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_151 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_297(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_152: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_152 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_298(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_153: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_153 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_299(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_154: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_154 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_300(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_155: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_155 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_301(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_156: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_156 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_302(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_157: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_157 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_303(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_158: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_158 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_304(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_159: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_159 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_305(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_160: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_160 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_306(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_161: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_161 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_307(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_162: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_162 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_308(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_163: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_163 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_309(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_164: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_164 })
                },
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp17_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_310(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_165: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_165 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_311(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_166: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_166 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_312(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_167: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_167 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_313(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_168: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_168 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_314(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_169: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_169 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_315(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_170: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_170 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_316(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_171: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_171 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_317(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_172: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_172 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_318(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_173: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_173 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_319(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_174: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_174 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_320(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_175: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_175 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_321(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_176: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_176 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_322(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_177: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_177 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_323(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_178: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_178 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_324(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_179: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_179 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_325(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_180: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_180 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_326(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_181: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_181 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_327(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_182: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_182 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_328(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_183: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_183 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_329(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_184: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_184 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_330(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_185: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_185 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_331(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_186: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_186 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_332(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_187: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_187 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_333(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_188: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_188 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_334(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_189: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_189 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_335(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_190: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_190 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_336(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_191: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_191 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_337(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_192: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_192 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_338(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_193: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_193 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_339(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_194: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_194 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_340(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_195: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_195 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_341(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_196: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_196 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_342(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_197: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_197 })
                },
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_343(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_198: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_198 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_344(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_199: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_199 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_345(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_200: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_200 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_346(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_201: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_201 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_347(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_202: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_202 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_348(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_203: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_203 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_349(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_204: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_204 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_350(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_205: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_205 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_351(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_206: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_206 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_352(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_207: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_207 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_353(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_208: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_208 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_354(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_209: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_209 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_355(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_210: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_210 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_356(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_211: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_211 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_357(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_212: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_212 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_358(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_213: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_213 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_359(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_214: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_214 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_360(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_215: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_215 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_361(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_216: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_216 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_362(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_217: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_217 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_363(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_218: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_218 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_364(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_219: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_219 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_365(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_220: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_220 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_366(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_221: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_221 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_367(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_222: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_222 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_368(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_223: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_223 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_369(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_224: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_224 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_370(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_225: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_225 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_371(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_226: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_226 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_372(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_227: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_227 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_373(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_228: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_228 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_374(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_229: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_229 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_375(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_230: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_230 })
                },
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_376(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_231: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_231 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_377(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_232: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_232 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_378(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_233: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_233 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_379(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_234: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_234 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_380(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_235: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_235 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_381(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_236: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_236 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_382(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_237: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_237 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_383(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_238: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_238 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_384(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_239: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_239 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_385(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_240: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_240 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_386(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_241: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_241 })
                },
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_387(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_242: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_242 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_388(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_243: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_243 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_389(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_244: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_244 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_390(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_245: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_245 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_391(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_246: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_246 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_392(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_247: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_247 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_393(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_248: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_248 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_394(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_249: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_249 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_395(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_250: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_250 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_396(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_251: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_251 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_397(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_252: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_252 })
                },
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                random_access_iterator_int_ptr_::new_1({ unimplemented!() }),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_398(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_253: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_253 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_399(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_254: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_254 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_400(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_255: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_255 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_401(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_256: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_256 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_402(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_257: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_257 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_403(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_258: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_258 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_404(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_259: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_259 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_405(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_260: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_260 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_406(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_261: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_261 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_407(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_262: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_262 })
                },
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            let ___out_true: *mut () = unimplemented!();
            let ___out_false: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                ___out_true,
                ___out_false,
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_408(
    mut input: (),
    mut pred: FnPtr<fn(()) -> ()>,
    mut expected_true: (),
    mut expected_false: (),
) {
    const _: () = assert!(true, "N2 + N3 == N1");;
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                begin,
                {
                    let mut __tmp_263: *mut () = end;
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_263 })
                },
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut out1: () = unimplemented!();
        let mut out2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                &mut range,
                unimplemented!(),
                unimplemented!(),
                pred.copy_from(),
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_iterators_in_sent_out1_out2_409() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_12(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_14(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_15(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_16(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_16(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_17(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_18(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_19(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_20(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_21(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_22(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_22(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_22(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_23(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_410() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_24(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_25(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_26(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_27(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_27(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_28(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_29(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_30(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_31(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_32(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_33(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_33(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_33(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_34(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_411() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_35(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_36(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_37(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_38(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_38(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_39(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_40(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_41(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_42(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_43(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_44(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_44(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_44(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_45(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_412() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_46(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_47(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_48(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_49(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_49(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_50(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_51(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_52(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_53(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_54(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_55(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_55(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_55(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_56(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_413() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_57(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_58(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_59(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_60(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_60(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_61(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_62(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_63(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_64(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_65(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_66(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_66(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_66(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_67(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_414() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_68(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_69(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_70(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_71(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_71(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_72(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_73(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_74(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_75(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_76(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_77(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_77(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_77(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_78(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_415() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_79(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_80(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_81(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_82(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_82(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_83(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_84(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_85(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_86(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_87(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_88(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_88(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_88(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_89(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_416() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_90(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_91(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_92(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_93(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_93(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_94(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_95(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_96(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_97(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_98(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_99(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_99(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_99(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_100(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_417() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_101(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_102(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_103(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_104(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_104(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_105(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_106(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_107(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_108(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_109(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_110(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_110(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_110(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_111(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_418() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_112(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_113(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_114(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_115(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_115(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_116(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_117(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_118(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_119(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_120(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_121(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_121(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_121(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_122(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_419() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_123(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_124(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_125(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_126(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_126(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_127(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_128(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_129(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_130(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_131(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_132(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_132(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_132(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_133(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_420() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_134(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_135(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_136(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_137(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_137(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_138(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_139(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_140(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_141(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_142(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_143(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_143(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_143(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_144(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_421() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_145(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_146(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_147(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_148(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_148(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_149(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_150(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_151(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_152(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_153(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_154(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_154(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_154(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_155(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_422() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_156(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_157(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_158(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_159(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_159(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_160(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_161(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_162(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_163(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_164(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_165(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_165(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_165(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_166(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_423() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_167(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_168(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_169(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_170(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_170(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_171(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_172(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_173(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_174(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_175(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_176(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_176(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_176(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_177(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_424() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_178(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_179(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_180(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_181(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_181(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_182(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_183(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_184(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_185(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_186(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_187(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_187(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_187(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_188(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_425() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_189(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_190(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_191(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_192(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_192(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_193(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_194(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_195(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_196(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_197(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_198(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_198(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_198(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_199(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_426() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_200(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_201(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_202(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_203(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_203(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_204(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_205(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_206(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_207(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_208(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_209(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_209(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_209(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_210(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_427() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_211(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_212(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_213(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_214(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_214(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_215(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_216(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_217(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_218(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_219(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_220(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_220(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_220(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_221(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_428() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_222(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_223(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_224(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_225(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_225(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_226(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_227(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_228(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_229(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_230(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_231(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_231(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_231(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_232(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_429() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_233(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_234(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_235(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_236(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_236(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_237(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_238(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_239(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_240(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_241(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_242(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_242(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_242(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_243(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_430() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_244(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_245(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_246(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_247(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_247(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_248(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_249(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_250(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_251(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_252(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_253(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_253(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_253(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_254(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_431() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_255(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_256(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_257(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_258(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_258(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_259(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_260(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_261(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_262(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_263(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_264(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_264(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_264(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_265(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_432() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_266(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_267(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_268(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_269(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_269(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_270(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_271(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_272(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_273(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_274(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_275(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_275(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_275(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_276(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_433() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_277(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_278(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_279(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_280(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_280(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_281(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_282(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_283(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_284(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_285(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_286(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_286(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_286(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_287(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_434() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_288(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_289(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_290(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_291(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_291(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_292(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_293(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_294(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_295(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_296(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_297(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_297(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_297(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_298(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_435() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_299(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_300(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_301(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_302(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_302(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_303(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_304(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_305(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_306(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_307(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_308(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_308(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_308(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_309(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_436() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_310(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_311(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_312(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_313(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_313(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_314(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_315(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_316(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_317(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_318(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_319(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_319(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_319(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_320(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_437() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_321(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_322(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_323(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_324(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_324(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_325(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_326(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_327(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_328(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_329(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_330(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_330(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_330(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_331(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_438() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_332(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_333(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_334(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_335(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_335(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_336(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_337(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_338(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_339(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_340(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_341(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_341(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_341(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_342(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_439() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_343(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_344(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_345(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_346(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_346(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_347(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_348(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_349(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_350(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_351(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_352(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_352(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_352(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_353(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_440() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_354(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_355(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_356(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_357(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_357(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_358(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_359(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_360(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_361(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_362(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_363(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_363(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_363(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_364(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_441() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_365(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_366(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_367(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_368(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_368(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_369(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_370(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_371(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_372(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_373(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_374(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_374(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_374(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_375(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_442() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_376(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_377(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_378(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_379(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_379(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_380(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_381(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_382(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_383(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_384(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_385(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_385(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_385(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_386(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_443() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_387(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_388(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_389(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_390(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_390(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_391(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_392(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_393(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_394(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_395(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_396(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_396(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_396(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_397(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_out2_444() {
    let mut is_odd: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) != (0_()));
        }
    });
    (unsafe {
        test_one_398(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_399(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_400(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_401(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_401(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_402(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_403(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_404(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_405(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_406(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_407(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_407(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_one_407(
            unimplemented!(),
            is_odd.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) < (0_()));
        }
    });
    (unsafe {
        test_one_408(
            unimplemented!(),
            is_negative.copy_from(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterators_in_sent_out1_445() {
    (unsafe { test_iterators_in_sent_out1_out2_409() });
    (unsafe { test_iterators_in_sent_out1_out2_410() });
    (unsafe { test_iterators_in_sent_out1_out2_411() });
}
pub unsafe fn test_iterators_in_sent_out1_446() {
    (unsafe { test_iterators_in_sent_out1_out2_412() });
    (unsafe { test_iterators_in_sent_out1_out2_413() });
    (unsafe { test_iterators_in_sent_out1_out2_414() });
}
pub unsafe fn test_iterators_in_sent_out1_447() {
    (unsafe { test_iterators_in_sent_out1_out2_415() });
    (unsafe { test_iterators_in_sent_out1_out2_416() });
    (unsafe { test_iterators_in_sent_out1_out2_417() });
}
pub unsafe fn test_iterators_in_sent_out1_448() {
    (unsafe { test_iterators_in_sent_out1_out2_418() });
    (unsafe { test_iterators_in_sent_out1_out2_419() });
    (unsafe { test_iterators_in_sent_out1_out2_420() });
}
pub unsafe fn test_iterators_in_sent_out1_449() {
    (unsafe { test_iterators_in_sent_out1_out2_421() });
    (unsafe { test_iterators_in_sent_out1_out2_422() });
    (unsafe { test_iterators_in_sent_out1_out2_423() });
}
pub unsafe fn test_iterators_in_sent_out1_450() {
    (unsafe { test_iterators_in_sent_out1_out2_424() });
    (unsafe { test_iterators_in_sent_out1_out2_425() });
    (unsafe { test_iterators_in_sent_out1_out2_426() });
}
pub unsafe fn test_iterators_in_sent_out1_451() {
    (unsafe { test_iterators_in_sent_out1_out2_427() });
    (unsafe { test_iterators_in_sent_out1_out2_428() });
    (unsafe { test_iterators_in_sent_out1_out2_429() });
}
pub unsafe fn test_iterators_in_sent_out1_452() {
    (unsafe { test_iterators_in_sent_out1_out2_430() });
    (unsafe { test_iterators_in_sent_out1_out2_431() });
    (unsafe { test_iterators_in_sent_out1_out2_432() });
}
pub unsafe fn test_iterators_in_sent_out1_453() {
    (unsafe { test_iterators_in_sent_out1_out2_433() });
    (unsafe { test_iterators_in_sent_out1_out2_434() });
    (unsafe { test_iterators_in_sent_out1_out2_435() });
}
pub unsafe fn test_iterators_in_sent_out1_454() {
    (unsafe { test_iterators_in_sent_out1_out2_436() });
    (unsafe { test_iterators_in_sent_out1_out2_437() });
    (unsafe { test_iterators_in_sent_out1_out2_438() });
}
pub unsafe fn test_iterators_in_sent_out1_455() {
    (unsafe { test_iterators_in_sent_out1_out2_439() });
    (unsafe { test_iterators_in_sent_out1_out2_440() });
    (unsafe { test_iterators_in_sent_out1_out2_441() });
}
pub unsafe fn test_iterators_in_sent_out1_456() {
    (unsafe { test_iterators_in_sent_out1_out2_442() });
    (unsafe { test_iterators_in_sent_out1_out2_443() });
    (unsafe { test_iterators_in_sent_out1_out2_444() });
}
pub unsafe fn test_iterators_in_sent_457() {
    (unsafe { test_iterators_in_sent_out1_445() });
    (unsafe { test_iterators_in_sent_out1_446() });
    (unsafe { test_iterators_in_sent_out1_447() });
    (unsafe { test_iterators_in_sent_out1_448() });
}
pub unsafe fn test_iterators_in_sent_458() {
    (unsafe { test_iterators_in_sent_out1_449() });
    (unsafe { test_iterators_in_sent_out1_450() });
    (unsafe { test_iterators_in_sent_out1_451() });
    (unsafe { test_iterators_in_sent_out1_452() });
}
pub unsafe fn test_iterators_in_sent_459() {
    (unsafe { test_iterators_in_sent_out1_453() });
    (unsafe { test_iterators_in_sent_out1_454() });
    (unsafe { test_iterators_in_sent_out1_455() });
    (unsafe { test_iterators_in_sent_out1_456() });
}
pub unsafe fn test_iterators_in_460() {
    {}
    (unsafe { test_iterators_in_sent_457() });
}
pub unsafe fn test_iterators_in_461() {
    {
        (unsafe { test_iterators_in_sent_458() });
    }
    (unsafe { test_iterators_in_sent_459() });
}
pub unsafe fn test_iterators_462() {
    (unsafe { test_iterators_in_460() });
    (unsafe { test_iterators_in_461() });
}
pub unsafe fn test_463() -> () {
    (unsafe { test_iterators_462() });
    {
        let in_: () = unimplemented!();
        let mut is_negative: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
            unsafe {
                return ((x) < (0_()));
            }
        });
        let mut negate: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
            unsafe {
                return -x;
            }
        });
        let expected_negative: () = unimplemented!();
        let expected_positive: () = unimplemented!();
        {
            {
                let mut out1: () = unimplemented!();
                let mut out2: () = unimplemented!();
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!();
                    let ___out_true: *mut () = unimplemented!();
                    let ___out_false: *mut () = unimplemented!();
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                        ___first,
                        ___last,
                        ___out_true,
                        ___out_false,
                        is_negative.copy_from(),
                        None,
                    )
                });
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out1: () = unimplemented!();
                let mut out2: () = unimplemented!();
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!();
                    let ___out_true: *mut () = unimplemented!();
                    let ___out_false: *mut () = unimplemented!();
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                        ___first,
                        ___last,
                        ___out_true,
                        ___out_false,
                        is_negative.copy_from(),
                        negate.copy_from(),
                    )
                });
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut out1: () = unimplemented!();
                let mut out2: () = unimplemented!();
                (unsafe {
                    let ___out_true: *mut () = unimplemented!();
                    let ___out_false: *mut () = unimplemented!();
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                        &in_,
                        ___out_true,
                        ___out_false,
                        is_negative.copy_from(),
                        None,
                    )
                });
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out1: () = unimplemented!();
                let mut out2: () = unimplemented!();
                (unsafe {
                    let ___out_true: *mut () = unimplemented!();
                    let ___out_false: *mut () = unimplemented!();
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                        &in_,
                        ___out_true,
                        ___out_false,
                        is_negative.copy_from(),
                        negate.copy_from(),
                    )
                });
                unimplemented!();
                unimplemented!();
            }
        }
    }
    {
        {
            let in_: () = unimplemented!();
            let mut is_negative: FnPtr<fn(()) -> ()> =
                FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
                    unsafe {
                        return ((x) < (0_()));
                    }
                });
            let mut out1: () = unimplemented!();
            let mut out2: () = unimplemented!();
            let mut pred_count: () = 0_();
            let mut proj_count: () = 0_();
            let mut pred: counting_predicate__lambda_at_ranges_partition_copy_pass_cpp_273_26__ =
                counting_predicate__lambda_at_ranges_partition_copy_pass_cpp_273_26__::new_1(
                    { is_negative.copy_from() },
                    { &mut pred_count },
                );
            let mut proj: counting_projection_std_identity_ =
                counting_projection_std_identity_::new_1({ &mut proj_count });
            let mut expected: () = unimplemented!();
            {
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!();
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                        ___first,
                        ___last,
                        unimplemented!(),
                        unimplemented!(),
                        pred.clone(),
                        proj.clone(),
                    )
                });
                unimplemented!();
                unimplemented!();
                pred_count = {
                    proj_count = 0_();
                    proj_count
                };
            }
            {
                (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_13))(
                        &in_,
                        unimplemented!(),
                        unimplemented!(),
                        pred.clone(),
                        proj.clone(),
                    )
                });
                unimplemented!();
                unimplemented!();
                pred_count = {
                    proj_count = 0_();
                    proj_count
                };
            }
        }
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
    (unsafe { test_463() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
