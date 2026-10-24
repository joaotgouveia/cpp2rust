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
pub unsafe fn base_0(i: *const forward_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn operator_ne_1(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_2(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
    pub unsafe fn operator_dec_7(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_int_ptr_,
    ) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
}
impl Default for bidirectional_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { bidirectional_iterator_int_ptr_::new_1() }
    }
}
pub unsafe fn operator_ne_3(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_4(i: *const random_access_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
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
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_int_ptr_,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
}
impl Default for random_access_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { random_access_iterator_int_ptr_::new_1() }
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
pub unsafe fn operator_add_5(
    mut x: contiguous_iterator_int_ptr_,
    mut n: (),
) -> contiguous_iterator_int_ptr_ {
    (unsafe { contiguous_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_ne_6(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_7(i: *const contiguous_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
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
    pub unsafe fn operator_dec_10(&mut self) -> *mut contiguous_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut contiguous_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_int_ptr_,
    ) -> *mut contiguous_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
}
impl Default for contiguous_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { contiguous_iterator_int_ptr_::new_1() }
    }
}
pub unsafe fn base_8(mut i: *mut ()) -> *mut () {
    return i;
}
pub unsafe fn base_9(mut i: *const ()) -> *const () {
    return i;
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
pub unsafe fn base_10(i: *const cpp20_output_iterator_int_ptr_) -> *mut () {
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
impl sentinel_wrapper_int_ptr_ {}
pub unsafe fn operator_eq_11(
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
            operator_eq_11(
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
pub unsafe fn operator_add_12(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_13(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_12(_i, n)
    });
}
pub unsafe fn iter_swap_14(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_15(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*unimplemented!());
}
pub unsafe fn operator_eq_16(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_17(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_17(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_16(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_int_ptr__SentinelForNotSemiregular_ {}
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
const _: () = assert!(true, "HasReverseCopyIt<int*>");
const _: () = assert!(
    !(false),
    "!HasReverseCopyIt<BidirectionalIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasReverseCopyIt<BidirectionalIteratorNotDecrementable>"
);
const _: () = assert!(
    !(false),
    "!HasReverseCopyIt<int*, SentinelForNotSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasReverseCopyIt<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasReverseCopyIt<int*, OutputIteratorNotIndirectlyWritable>"
);
const _: () = assert!(
    !(false),
    "!HasReverseCopyIt<int*, OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(true, "HasReverseCopyR<UncheckedRange<int*>>");
const _: () = assert!(
    !(false),
    "!HasReverseCopyR<BidirectionalRangeNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasReverseCopyR<BidirectionalRangeNotDecrementable>"
);
const _: () = assert!(
    !(false),
    "!HasReverseCopyR<UncheckedRange<int*, SentinelForNotSemiregular>>"
);
const _: () = assert!(
    !(false),
    "!HasReverseCopyR<UncheckedRange<int*>, OutputIteratorNotIndirectlyWritable>"
);
const _: () = assert!(
    !(false),
    "!HasReverseCopyR<UncheckedRange<int*>, OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::is_same_v<std::ranges::reverse_copy_result<int, int>, std::ranges::in_out_result<int, int>>"
);
pub unsafe fn test_18(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_20(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_21(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_22(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_23(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_24(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_25(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_26(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_27(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_28(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_29(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_30(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_31(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_32(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_33(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_34(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_35(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_36(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_37(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_38(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_39(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_40(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_41(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_42(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_43(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_44(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_45(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_46(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_47(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_48(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_49(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_50(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_51(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_52(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_53(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_54(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_55(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_56(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_57(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_58(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_59(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_60(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_61(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_62(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_63(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_64(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_65(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_66(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_67(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_68(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_69(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_70(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_71(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_72(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_73(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_74(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_75(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_76(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_77(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_78(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_79(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_80(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_81(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_82(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_83(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_84(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_85(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_86(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_87(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_88(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_89(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_90(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            let ___last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_91(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_92(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_93(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_94(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_95(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_96(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_97(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_98(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_99(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_100(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_101(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_102(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_103(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_104(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_105(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_106(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_107(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_108(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_109(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_110(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_111(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_112(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_113(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_114(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *mut () = unimplemented!();
            let ___last: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_115(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_116(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_117(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_118(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_119(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_120(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_121(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_122(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                forward_iterator_int_ptr_::new_1({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_123(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_124(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_125(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_126(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                bidirectional_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_127(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_128(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_129(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_130(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                random_access_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_131(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_132(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_133(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_134(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                contiguous_iterator_int_ptr_::new_2({ unimplemented!() }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_135(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_136(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_137(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_138(mut value: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                ___first, ___last, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                &mut range,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_iterators_139() {
    (unsafe { test_18(unimplemented!(), unimplemented!()) });
    (unsafe { test_20(unimplemented!(), unimplemented!()) });
    (unsafe { test_21(unimplemented!(), unimplemented!()) });
    (unsafe { test_22(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_140() {
    (unsafe { test_23(unimplemented!(), unimplemented!()) });
    (unsafe { test_24(unimplemented!(), unimplemented!()) });
    (unsafe { test_25(unimplemented!(), unimplemented!()) });
    (unsafe { test_26(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_141() {
    (unsafe { test_27(unimplemented!(), unimplemented!()) });
    (unsafe { test_28(unimplemented!(), unimplemented!()) });
    (unsafe { test_29(unimplemented!(), unimplemented!()) });
    (unsafe { test_30(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_142() {
    (unsafe { test_31(unimplemented!(), unimplemented!()) });
    (unsafe { test_32(unimplemented!(), unimplemented!()) });
    (unsafe { test_33(unimplemented!(), unimplemented!()) });
    (unsafe { test_34(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_143() {
    (unsafe { test_35(unimplemented!(), unimplemented!()) });
    (unsafe { test_36(unimplemented!(), unimplemented!()) });
    (unsafe { test_37(unimplemented!(), unimplemented!()) });
    (unsafe { test_38(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_144() {
    (unsafe { test_39(unimplemented!(), unimplemented!()) });
    (unsafe { test_40(unimplemented!(), unimplemented!()) });
    (unsafe { test_41(unimplemented!(), unimplemented!()) });
    (unsafe { test_42(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_145() {
    (unsafe { test_43(unimplemented!(), unimplemented!()) });
    (unsafe { test_44(unimplemented!(), unimplemented!()) });
    (unsafe { test_45(unimplemented!(), unimplemented!()) });
    (unsafe { test_46(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_146() {
    (unsafe { test_47(unimplemented!(), unimplemented!()) });
    (unsafe { test_48(unimplemented!(), unimplemented!()) });
    (unsafe { test_49(unimplemented!(), unimplemented!()) });
    (unsafe { test_50(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_147() {
    (unsafe { test_51(unimplemented!(), unimplemented!()) });
    (unsafe { test_52(unimplemented!(), unimplemented!()) });
    (unsafe { test_53(unimplemented!(), unimplemented!()) });
    (unsafe { test_54(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_148() {
    (unsafe { test_55(unimplemented!(), unimplemented!()) });
    (unsafe { test_56(unimplemented!(), unimplemented!()) });
    (unsafe { test_57(unimplemented!(), unimplemented!()) });
    (unsafe { test_58(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_149() {
    (unsafe { test_59(unimplemented!(), unimplemented!()) });
    (unsafe { test_60(unimplemented!(), unimplemented!()) });
    (unsafe { test_61(unimplemented!(), unimplemented!()) });
    (unsafe { test_62(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_150() {
    (unsafe { test_63(unimplemented!(), unimplemented!()) });
    (unsafe { test_64(unimplemented!(), unimplemented!()) });
    (unsafe { test_65(unimplemented!(), unimplemented!()) });
    (unsafe { test_66(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_151() {
    (unsafe { test_67(unimplemented!(), unimplemented!()) });
    (unsafe { test_68(unimplemented!(), unimplemented!()) });
    (unsafe { test_69(unimplemented!(), unimplemented!()) });
    (unsafe { test_70(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_152() {
    (unsafe { test_71(unimplemented!(), unimplemented!()) });
    (unsafe { test_72(unimplemented!(), unimplemented!()) });
    (unsafe { test_73(unimplemented!(), unimplemented!()) });
    (unsafe { test_74(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_153() {
    (unsafe { test_75(unimplemented!(), unimplemented!()) });
    (unsafe { test_76(unimplemented!(), unimplemented!()) });
    (unsafe { test_77(unimplemented!(), unimplemented!()) });
    (unsafe { test_78(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_154() {
    (unsafe { test_79(unimplemented!(), unimplemented!()) });
    (unsafe { test_80(unimplemented!(), unimplemented!()) });
    (unsafe { test_81(unimplemented!(), unimplemented!()) });
    (unsafe { test_82(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_155() {
    (unsafe { test_83(unimplemented!(), unimplemented!()) });
    (unsafe { test_84(unimplemented!(), unimplemented!()) });
    (unsafe { test_85(unimplemented!(), unimplemented!()) });
    (unsafe { test_86(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_156() {
    (unsafe { test_87(unimplemented!(), unimplemented!()) });
    (unsafe { test_88(unimplemented!(), unimplemented!()) });
    (unsafe { test_89(unimplemented!(), unimplemented!()) });
    (unsafe { test_90(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_157() {
    (unsafe { test_91(unimplemented!(), unimplemented!()) });
    (unsafe { test_92(unimplemented!(), unimplemented!()) });
    (unsafe { test_93(unimplemented!(), unimplemented!()) });
    (unsafe { test_94(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_158() {
    (unsafe { test_95(unimplemented!(), unimplemented!()) });
    (unsafe { test_96(unimplemented!(), unimplemented!()) });
    (unsafe { test_97(unimplemented!(), unimplemented!()) });
    (unsafe { test_98(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_159() {
    (unsafe { test_99(unimplemented!(), unimplemented!()) });
    (unsafe { test_100(unimplemented!(), unimplemented!()) });
    (unsafe { test_101(unimplemented!(), unimplemented!()) });
    (unsafe { test_102(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_160() {
    (unsafe { test_103(unimplemented!(), unimplemented!()) });
    (unsafe { test_104(unimplemented!(), unimplemented!()) });
    (unsafe { test_105(unimplemented!(), unimplemented!()) });
    (unsafe { test_106(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_161() {
    (unsafe { test_107(unimplemented!(), unimplemented!()) });
    (unsafe { test_108(unimplemented!(), unimplemented!()) });
    (unsafe { test_109(unimplemented!(), unimplemented!()) });
    (unsafe { test_110(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_162() {
    (unsafe { test_111(unimplemented!(), unimplemented!()) });
    (unsafe { test_112(unimplemented!(), unimplemented!()) });
    (unsafe { test_113(unimplemented!(), unimplemented!()) });
    (unsafe { test_114(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_163() {
    (unsafe { test_115(unimplemented!(), unimplemented!()) });
    (unsafe { test_116(unimplemented!(), unimplemented!()) });
    (unsafe { test_117(unimplemented!(), unimplemented!()) });
    (unsafe { test_118(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_164() {
    (unsafe { test_119(unimplemented!(), unimplemented!()) });
    (unsafe { test_120(unimplemented!(), unimplemented!()) });
    (unsafe { test_121(unimplemented!(), unimplemented!()) });
    (unsafe { test_122(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_165() {
    (unsafe { test_123(unimplemented!(), unimplemented!()) });
    (unsafe { test_124(unimplemented!(), unimplemented!()) });
    (unsafe { test_125(unimplemented!(), unimplemented!()) });
    (unsafe { test_126(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_166() {
    (unsafe { test_127(unimplemented!(), unimplemented!()) });
    (unsafe { test_128(unimplemented!(), unimplemented!()) });
    (unsafe { test_129(unimplemented!(), unimplemented!()) });
    (unsafe { test_130(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_167() {
    (unsafe { test_131(unimplemented!(), unimplemented!()) });
    (unsafe { test_132(unimplemented!(), unimplemented!()) });
    (unsafe { test_133(unimplemented!(), unimplemented!()) });
    (unsafe { test_134(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_iterators_168() {
    (unsafe { test_135(unimplemented!(), unimplemented!()) });
    (unsafe { test_136(unimplemented!(), unimplemented!()) });
    (unsafe { test_137(unimplemented!(), unimplemented!()) });
    (unsafe { test_138(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_out_iterators_169() {
    (unsafe { test_iterators_139() });
    (unsafe { test_iterators_140() });
    (unsafe { test_iterators_141() });
    (unsafe { test_iterators_142() });
    (unsafe { test_iterators_143() });
    (unsafe { test_iterators_144() });
}
pub unsafe fn test_out_iterators_170() {
    (unsafe { test_iterators_145() });
    (unsafe { test_iterators_146() });
    (unsafe { test_iterators_147() });
    (unsafe { test_iterators_148() });
    (unsafe { test_iterators_149() });
    (unsafe { test_iterators_150() });
}
pub unsafe fn test_out_iterators_171() {
    (unsafe { test_iterators_151() });
    (unsafe { test_iterators_152() });
    (unsafe { test_iterators_153() });
    (unsafe { test_iterators_154() });
    (unsafe { test_iterators_155() });
    (unsafe { test_iterators_156() });
}
pub unsafe fn test_out_iterators_172() {
    (unsafe { test_iterators_157() });
    (unsafe { test_iterators_158() });
    (unsafe { test_iterators_159() });
    (unsafe { test_iterators_160() });
    (unsafe { test_iterators_161() });
    (unsafe { test_iterators_162() });
}
pub unsafe fn test_out_iterators_173() {
    (unsafe { test_iterators_163() });
    (unsafe { test_iterators_164() });
    (unsafe { test_iterators_165() });
    (unsafe { test_iterators_166() });
    (unsafe { test_iterators_167() });
    (unsafe { test_iterators_168() });
}
pub unsafe fn test_174() -> () {
    (unsafe { test_out_iterators_169() });
    (unsafe { test_out_iterators_170() });
    (unsafe { test_out_iterators_171() });
    (unsafe { test_out_iterators_172() });
    (unsafe { test_out_iterators_173() });
    {
        {
            let mut c: () = 0_();
            let mut a: [AssignmentCounter_175; 4] = [
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
            ];
            let mut b: [AssignmentCounter_175; 4] = [
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
            ];
            (unsafe {
                let ___first: *mut AssignmentCounter_175 = a.as_mut_ptr();
                let ___last: *mut AssignmentCounter_175 = a.as_mut_ptr().offset((4_()) as isize);
                (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                    ___first,
                    ___last,
                    b.as_mut_ptr(),
                )
            });
            unimplemented!();
        }
        {
            let mut c: () = 0_();
            let mut a: [AssignmentCounter_175; 4] = [
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
            ];
            let mut b: [AssignmentCounter_175; 4] = [
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
                AssignmentCounter_175::new({ (&mut c as *mut ()) }),
            ];
            (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_19))(
                    &mut a,
                    b.as_mut_ptr(),
                )
            });
            unimplemented!();
        }
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct AssignmentCounter_175 {
    pub counter: *mut (),
}
impl AssignmentCounter_175 {
    pub unsafe fn new(mut counter_: *mut ()) -> Self {
        let mut this = Self { counter: counter_ };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const AssignmentCounter_175,
    ) -> *mut AssignmentCounter_175 {
        (*self.counter).prefix_inc();
        return &mut (*(self as *mut AssignmentCounter_175));
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_174() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
