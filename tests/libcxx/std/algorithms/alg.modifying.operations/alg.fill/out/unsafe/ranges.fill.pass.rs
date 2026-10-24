extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_char__signed_char_ {
    pub unsafe fn new(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_char__signed_char_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    data_: (),
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n)
            > (unsafe {
                sized_allocator_unsigned_char__unsigned_char__signed_char_::max_size(self)
            }))
        {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_short__short_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_short__short_ {
    pub unsafe fn new(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_short__unsigned_short__short_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_short__short_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_short__unsigned_short__short_ {
    data_: (),
}
impl sized_allocator_unsigned_short__unsigned_short__short_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n)
            > (unsafe { sized_allocator_unsigned_short__unsigned_short__short_::max_size(self) }))
        {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_int__int_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_int__int_ {
    pub unsafe fn new(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_int__unsigned_int__int_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_int__int_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_int__unsigned_int__int_ {
    data_: (),
}
impl sized_allocator_unsigned_int__unsigned_int__int_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n) > (unsafe { sized_allocator_unsigned_int__unsigned_int__int_::max_size(self) })) {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_long__long_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_long__long_ {
    pub unsafe fn new(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_long__long_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_long__unsigned_long__long_ {
    data_: (),
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n) > (unsafe { sized_allocator_unsigned_long__unsigned_long__long_::max_size(self) }))
        {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
            it_: (*unimplemented!()),
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
pub unsafe fn operator_ne_1(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_2(i: *const forward_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn operator_ne_3(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_4(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn operator_sub_5(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn base_6(i: *const random_access_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn operator_sub_7(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn base_8(i: *const contiguous_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn base_9(mut i: *mut ()) -> *mut () {
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
pub struct sentinel_wrapper_OutputIteratorNotIndirectlyWritable_ {
    base_: OutputIteratorNotIndirectlyWritable,
}
impl sentinel_wrapper_OutputIteratorNotIndirectlyWritable_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_int_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp17_output_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp17_output_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_0(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp17_output_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_0(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_output_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_output_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_10(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_output_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_10(other) }));
    }
}
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
pub struct UncheckedRange_cpp20_output_iterator_int_ptr___SentinelForNotSemiregular_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_cpp20_output_iterator_int_ptr___SentinelForNotWeaklyEqualityComparableWith_
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
pub struct UncheckedRange_OutputIteratorNotIndirectlyWritable__sentinel_wrapper_OutputIteratorNotIndirectlyWritable__
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
const _: () = assert!(true, "HasFillIt<int*>");
const _: () = assert!(!(false), "!HasFillIt<OutputIteratorNotIndirectlyWritable>");
const _: () = assert!(
    !(false),
    "!HasFillIt<OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(!(false), "!HasFillIt<int*, SentinelForNotSemiregular>");
const _: () = assert!(
    !(false),
    "!HasFillIt<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(true, "HasFillR<UncheckedRange<int*>>");
const _: () = assert!(!(false), "!HasFillR<OutputRangeNotIndirectlyWritable>");
const _: () = assert!(!(false), "!HasFillR<OutputRangeNotInputOrOutputIterator>");
const _: () = assert!(!(false), "!HasFillR<OutputRangeNotSentinelSemiregular>");
const _: () = assert!(
    !(false),
    "!HasFillR<OutputRangeNotSentinelEqualityComparableWith>"
);
pub unsafe fn test_iterators_18() {
    {
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut ret: cpp17_output_iterator_int_ptr_ = (unsafe {
                let ___first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_0: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            a.as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_0 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut range: () = unimplemented!();
            let mut ret: cpp17_output_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut a: () = unimplemented!();
            let mut ret: cpp17_output_iterator_int_ptr_ = (unsafe {
                let ___first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
                let ___last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_1: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_1 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
        {
            let mut a: () = unimplemented!();
            let mut range: () = unimplemented!();
            let mut ret: cpp17_output_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_20() {
    {
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut ret: cpp20_output_iterator_int_ptr_ = (unsafe {
                let ___first: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_output_iterator_int_ptr__ = {
                    let mut __tmp_2: cpp20_output_iterator_int_ptr_ =
                        cpp20_output_iterator_int_ptr_::new({
                            a.as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ &mut __tmp_2 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut range: () = unimplemented!();
            let mut ret: cpp20_output_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut a: () = unimplemented!();
            let mut ret: cpp20_output_iterator_int_ptr_ = (unsafe {
                let ___first: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                let ___last: sentinel_wrapper_cpp20_output_iterator_int_ptr__ = {
                    let mut __tmp_3: cpp20_output_iterator_int_ptr_ =
                        cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                    sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ &mut __tmp_3 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
        {
            let mut a: () = unimplemented!();
            let mut range: () = unimplemented!();
            let mut ret: cpp20_output_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_21() {
    {
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut ret: forward_iterator_int_ptr_ = (unsafe {
                let ___first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((3_()) as isize) });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut range: () = unimplemented!();
            let mut ret: forward_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut a: () = unimplemented!();
            let mut ret: forward_iterator_int_ptr_ = (unsafe {
                let ___first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                let ___last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
        {
            let mut a: () = unimplemented!();
            let mut range: () = unimplemented!();
            let mut ret: forward_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_22() {
    {
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut ret: bidirectional_iterator_int_ptr_ = (unsafe {
                let ___first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        a.as_mut_ptr().offset((3_()) as isize)
                    });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut range: () = unimplemented!();
            let mut ret: bidirectional_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut a: () = unimplemented!();
            let mut ret: bidirectional_iterator_int_ptr_ = (unsafe {
                let ___first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                let ___last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
        {
            let mut a: () = unimplemented!();
            let mut range: () = unimplemented!();
            let mut ret: bidirectional_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_23() {
    {
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut ret: random_access_iterator_int_ptr_ = (unsafe {
                let ___first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        a.as_mut_ptr().offset((3_()) as isize)
                    });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut range: () = unimplemented!();
            let mut ret: random_access_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut a: () = unimplemented!();
            let mut ret: random_access_iterator_int_ptr_ = (unsafe {
                let ___first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                let ___last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
        {
            let mut a: () = unimplemented!();
            let mut range: () = unimplemented!();
            let mut ret: random_access_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_24() {
    {
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut ret: contiguous_iterator_int_ptr_ = (unsafe {
                let ___first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((3_()) as isize) });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut range: () = unimplemented!();
            let mut ret: contiguous_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut a: () = unimplemented!();
            let mut ret: contiguous_iterator_int_ptr_ = (unsafe {
                let ___first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                let ___last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
        {
            let mut a: () = unimplemented!();
            let mut range: () = unimplemented!();
            let mut ret: contiguous_iterator_int_ptr_ = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_25() {
    {
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut ret: *mut () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((3_()) as isize);
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 3] = [unimplemented!(); 3];
            let mut range: () = unimplemented!();
            let mut ret: *mut () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut a: () = unimplemented!();
            let mut ret: *mut () = (unsafe {
                let ___first: *mut () = unimplemented!();
                let ___last: *mut () = unimplemented!();
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
        {
            let mut a: () = unimplemented!();
            let mut range: () = unimplemented!();
            let mut ret: *mut () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut range, &mut ___value)
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_vector_bool_26(mut N: ()) -> () {
    {
        {
            let mut in_: () = unimplemented!();
            let mut expected: () = unimplemented!();
            unimplemented!();
            (unsafe {
                let mut ___range: () = unimplemented!();
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    &mut ___range,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut expected: () = unimplemented!();
            unimplemented!();
            (unsafe {
                let mut ___range: () = unimplemented!();
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    &mut ___range,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut expected: () = unimplemented!();
            unimplemented!();
            (unsafe {
                let mut ___range: () = unimplemented!();
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    &mut ___range,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
    }
    {
        {
            let mut in_: () = unimplemented!();
            let mut expected: () = unimplemented!();
            (unsafe {
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut in_, &mut ___value)
            });
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut expected: () = unimplemented!();
            (unsafe {
                let mut ___range: () = unimplemented!();
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    &mut ___range,
                    &mut ___value,
                )
            });
            (unsafe {
                let mut ___range: () = unimplemented!();
                let mut ___value: () = false;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    &mut ___range,
                    &mut ___value,
                )
            });
            (unsafe {
                let mut ___range: () = unimplemented!();
                let mut ___value: () = false;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    &mut ___range,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
    }
    return true;
}
pub unsafe fn test_deque_27() {
    let mut in_: () = unimplemented!();
    let mut expected: () = unimplemented!();
    (unsafe {
        let mut ___value: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut in_, &mut ___value)
    });
    unimplemented!();
}
pub unsafe fn test_28() -> () {
    (unsafe { test_iterators_18() });
    (unsafe { test_iterators_20() });
    (unsafe { test_iterators_21() });
    (unsafe { test_iterators_22() });
    (unsafe { test_iterators_23() });
    (unsafe { test_iterators_24() });
    (unsafe { test_iterators_25() });
    {
        {
            let mut a: [S_29; 5] = [<S_29>::default(); 5];
            (unsafe {
                let ___first: *mut S_29 = a.as_mut_ptr();
                let ___last: *mut S_29 = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: S_29 = S_29 { copied: true };
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &mut ___value,
                )
            });
            unimplemented!();
        }
        {
            let mut a: [S_29; 5] = [<S_29>::default(); 5];
            (unsafe {
                let mut ___value: S_29 = S_29 { copied: true };
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut a, &mut ___value)
            });
            unimplemented!();
        }
    }
    {
        let mut ret: () = (unsafe {
            let mut ___range: () = unimplemented!();
            let mut ___value: () = 1_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut ___range, &mut ___value)
        });
    }
    {
        let mut a: () = unimplemented!();
        let mut ret: () = (unsafe {
            let mut ___range: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut all_30))(&mut a) });
            let mut ___value: () = 1_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut ___range, &mut ___value)
        });
        unimplemented!();
    }
    {
        {
            let mut a: () = unimplemented!();
            let mut ret: *mut () = (unsafe {
                let ___first: *mut () = unimplemented!();
                let ___last: *mut () = unimplemented!();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    ___first,
                    ___last,
                    &std::mem::transmute(*b"long long string so no SSO\0"),
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: () = unimplemented!();
            let mut ret: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(
                    &mut a,
                    &std::mem::transmute(*b"long long string so no SSO\0"),
                )
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut expected: () = unimplemented!();
            (unsafe {
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut in_, &mut ___value)
            });
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut expected: () = unimplemented!();
            (unsafe {
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut in_, &mut ___value)
            });
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut expected: () = unimplemented!();
            (unsafe {
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut in_, &mut ___value)
            });
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut expected: () = unimplemented!();
            (unsafe {
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut in_, &mut ___value)
            });
            unimplemented!();
        }
    }
    {
        (unsafe { test_deque_27() });
    }
    {
        let mut v: () = unimplemented!();
        let mut jv: () = unimplemented!();
        (unsafe {
            let mut ___value: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fill_19))(&mut jv, &mut ___value)
        });
        'loop_: for vec_ in IterRange::new(unimplemented!(), unimplemented!()) {
            let vec_: *const () = &(*unimplemented!());
            'loop_: for n in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut n: () = (*unimplemented!());
                unimplemented!();
            }
        }
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct S_29 {
    pub copied: (),
}
impl S_29 {
    pub unsafe fn copy_assign(&mut self, _a0: *const S_29) -> *mut S_29 {
        self.copied = true;
        return &mut (*(self as *mut S_29));
    }
}
impl Default for S_29 {
    fn default() -> Self {
        unsafe { S_29 { copied: false } }
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
    (unsafe { test_28() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
