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
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__
{}
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
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr____types_type_list_bidirectional_iterator_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr____types_type_list_forward_iterator_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr____types_type_list_cpp17_input_iterator_int_ptr__int_ptr___
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn swallow_1(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
) {
}
pub unsafe fn base_2(i: *const cpp17_output_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn base_3(i: *const cpp17_input_iterator_int_ptr__int_ptr_) -> *mut () {
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
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
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
pub unsafe fn base_4(i: *const forward_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn base_5(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn base_7(i: *const contiguous_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn base_8(mut i: *mut ()) -> *mut () {
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
pub unsafe fn base_9(i: *const cpp20_output_iterator_int_ptr_) -> *mut () {
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
            base_: (unsafe { base_8((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_8((*other)) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_contiguous_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_7(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_7(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_random_access_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_random_access_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_6(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_6(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_5(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_5(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_forward_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_4(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_4(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_3(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp17_input_iterator_int_ptr__int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_3(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp17_output_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp17_output_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_2(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp17_output_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_2(other) }));
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
            base_: (unsafe { base_9(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_output_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_9(other) }));
    }
}
pub unsafe fn operator_sub_10(s: *const sized_sentinel_int_ptr_, i: *const *mut ()) -> () {
    return (((*s).base_ as usize - (unsafe { base_8((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_11(i: *const *mut (), s: *const sized_sentinel_int_ptr_) -> () {
    return (((unsafe { base_8((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<()>()) as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_int_ptr_ {
    base_: *mut (),
}
impl sized_sentinel_int_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_8((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        return ((self.base_) == (unsafe { base_8((*other)) }));
    }
}
pub unsafe fn operator_eq_12(
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
            operator_eq_12(
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
pub unsafe fn operator_add_13(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_14(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_13(_i, n)
    });
}
pub unsafe fn iter_swap_15(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_16(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*unimplemented!());
}
pub unsafe fn operator_eq_17(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_18(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_18(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self as *const adl_Iterator, other as *const adl_Iterator) }
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
const _: () = assert!(true, "HasIotaIter<int*, int*, int>");
const _: () = assert!(
    !(false),
    "!HasIotaIter<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(!(false), "!HasIotaIter<int*, SentinelForNotSemiregular>");
const _: () = assert!(
    !(false),
    "!HasIotaIter<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasIotaIter<int*, int*, WeaklyIncrementableNotMovable>"
);
const _: () = assert!(
    !(false),
    "!HasIotaIter<OutputIteratorNotIndirectlyWritable, int*, int>"
);
const _: () = assert!(true, "HasIotaRange<UncheckedRange<int*>, int>");
const _: () = assert!(
    !(false),
    "!HasIotaRange<UncheckedRange<int*>, WeaklyIncrementableNotMovable>"
);
const _: () = assert!(
    !(false),
    "!HasIotaRange<UncheckedRange<int*>, OutputIteratorNotIndirectlyWritable>"
);
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct DangerousCopyAssign {
    pub val: (),
}
impl DangerousCopyAssign {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn copy_from(other: *const DangerousCopyAssign) -> Self {
        let mut this = Self {
            val: unimplemented!(),
        };
        this.val = (*other).val;
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        a: *const DangerousCopyAssign,
    ) -> *mut DangerousCopyAssign {
        self.val = (*a).val;
        return &mut (*(self as *mut DangerousCopyAssign));
    }
    pub unsafe fn operator_eq(&mut self, rhs: *const DangerousCopyAssign) -> () {
        return ((self.val) == ((*rhs).val));
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut DangerousCopyAssign {
        (self.val).prefix_inc();
        return &mut (*(self as *mut DangerousCopyAssign));
    }
    pub unsafe fn operator_post_inc_7(&mut self, mut _a0: ()) -> DangerousCopyAssign {
        let mut tmp: DangerousCopyAssign =
            DangerousCopyAssign::copy_from({ &(*(self as *mut DangerousCopyAssign)) });
        self.val.prefix_inc();
        return DangerousCopyAssign::copy_from({ &tmp });
    }
}
impl std::cmp::PartialEq for DangerousCopyAssign {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            DangerousCopyAssign::operator_eq(
                &mut *(&raw const *self).cast_mut(),
                other as *const DangerousCopyAssign,
            )
        }
    }
}
impl std::cmp::Eq for DangerousCopyAssign {}
impl Clone for DangerousCopyAssign {
    fn clone(&self) -> Self {
        unsafe { DangerousCopyAssign::copy_from(self as *const DangerousCopyAssign) }
    }
}
pub unsafe fn test_result_19(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_0: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_0 })
        };
        let mut result: () = (unsafe {
            let ___first: *mut () = (*unimplemented!());
            let ___value: () = starting_value;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                ___first,
                (*unimplemented!()),
                ___value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_1: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_1 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_21(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_2: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_2 })
        };
        let mut result: () = (unsafe {
            let ___first: *mut () = (*unimplemented!());
            let ___value: () = starting_value;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                ___first,
                (*unimplemented!()),
                ___value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_3: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_3 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_22(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_4: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_4 })
        };
        let mut result: () = (unsafe {
            let ___first: *mut () = (*unimplemented!());
            let ___value: () = starting_value;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                ___first,
                (*unimplemented!()),
                ___value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_5: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_5 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_23(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_6: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_6 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                contiguous_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_7: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_7 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_24(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_8: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_8 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                contiguous_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_9: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_9 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_25(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_10: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_10 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                contiguous_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_11: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_11 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_26(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_12: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_12 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                random_access_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_13: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_13 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_27(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_14: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_14 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                random_access_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_15: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_15 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_28(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_16: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_16 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                random_access_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_17: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_17 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_29(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_18: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_18 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                bidirectional_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_19: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_19 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_30(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_20: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_20 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                bidirectional_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_21: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_21 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_31(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_22: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_22 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                bidirectional_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_23: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_23 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_32(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_24: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_24 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                forward_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_25: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_25 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_33(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_26: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_26 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                forward_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_27: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_27 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_34(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_28: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_28 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                forward_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut in_end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_29: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_29 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_35(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_30: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_30 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_31: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_31 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_36(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_32: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_32 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_33: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_33 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_37(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_34: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_34 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_35: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_35 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_38(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
            let mut __tmp_36: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_36 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                cpp17_output_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
            let mut __tmp_37: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_37 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_39(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
            let mut __tmp_38: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_38 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                cpp17_output_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
            let mut __tmp_39: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_39 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_40(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
            let mut __tmp_40: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_40 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                cpp17_output_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
            let mut __tmp_41: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_41 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_41(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp20_output_iterator_int_ptr__ = {
            let mut __tmp_42: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ &mut __tmp_42 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                cpp20_output_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp20_output_iterator_int_ptr__ = {
            let mut __tmp_43: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ &mut __tmp_43 })
        };
        let mut range : () = std_ranges_subrange_cpp20_output_iterator_int_ptr___sentinel_wrapper_cpp20_output_iterator_int_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_output_iterator_int_ptr_ :: move_from ( { & mut ( * unimplemented!()  )  } , )  } , { ( * unimplemented!()  )  } , )  ;
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_42(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp20_output_iterator_int_ptr__ = {
            let mut __tmp_44: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ &mut __tmp_44 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                cpp20_output_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp20_output_iterator_int_ptr__ = {
            let mut __tmp_45: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ &mut __tmp_45 })
        };
        let mut range : () = std_ranges_subrange_cpp20_output_iterator_int_ptr___sentinel_wrapper_cpp20_output_iterator_int_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_output_iterator_int_ptr_ :: move_from ( { & mut ( * unimplemented!()  )  } , )  } , { ( * unimplemented!()  )  } , )  ;
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_43(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp20_output_iterator_int_ptr__ = {
            let mut __tmp_46: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ &mut __tmp_46 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                cpp20_output_iterator_int_ptr_::move_from({ &mut (*unimplemented!()) }),
                (*unimplemented!()),
                starting_value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let mut in_end: sentinel_wrapper_cpp20_output_iterator_int_ptr__ = {
            let mut __tmp_47: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ &mut __tmp_47 })
        };
        let mut range : () = std_ranges_subrange_cpp20_output_iterator_int_ptr___sentinel_wrapper_cpp20_output_iterator_int_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_output_iterator_int_ptr_ :: move_from ( { & mut ( * unimplemented!()  )  } , )  } , { ( * unimplemented!()  )  } , )  ;
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_44(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sized_sentinel_int_ptr_ = {
            let mut __tmp_48: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sized_sentinel_int_ptr_::new_1({ &mut __tmp_48 })
        };
        let mut result: () = (unsafe {
            let ___first: *mut () = (*unimplemented!());
            let ___value: () = starting_value;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                ___first,
                (*unimplemented!()),
                ___value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sized_sentinel_int_ptr_ = {
            let mut __tmp_49: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sized_sentinel_int_ptr_::new_1({ &mut __tmp_49 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_45(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sized_sentinel_int_ptr_ = {
            let mut __tmp_50: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sized_sentinel_int_ptr_::new_1({ &mut __tmp_50 })
        };
        let mut result: () = (unsafe {
            let ___first: *mut () = (*unimplemented!());
            let ___value: () = starting_value;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                ___first,
                (*unimplemented!()),
                ___value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sized_sentinel_int_ptr_ = {
            let mut __tmp_51: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sized_sentinel_int_ptr_::new_1({ &mut __tmp_51 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_result_46(mut input: (), mut starting_value: (), expected: ()) {
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sized_sentinel_int_ptr_ = {
            let mut __tmp_52: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sized_sentinel_int_ptr_::new_1({ &mut __tmp_52 })
        };
        let mut result: () = (unsafe {
            let ___first: *mut () = (*unimplemented!());
            let ___value: () = starting_value;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
                ___first,
                (*unimplemented!()),
                ___value,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: *mut () = unimplemented!();
        let mut in_end: sized_sentinel_int_ptr_ = {
            let mut __tmp_53: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sized_sentinel_int_ptr_::new_1({ &mut __tmp_53 })
        };
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(&mut range, starting_value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_results_47() {
    (unsafe { test_result_19(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_21(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_22(unimplemented!(), 0_(), unimplemented!()) });
}
pub unsafe fn test_results_48() {
    (unsafe { test_result_23(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_24(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_25(unimplemented!(), 0_(), unimplemented!()) });
}
pub unsafe fn test_results_49() {
    (unsafe { test_result_26(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_27(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_28(unimplemented!(), 0_(), unimplemented!()) });
}
pub unsafe fn test_results_50() {
    (unsafe { test_result_29(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_30(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_31(unimplemented!(), 0_(), unimplemented!()) });
}
pub unsafe fn test_results_51() {
    (unsafe { test_result_32(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_33(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_34(unimplemented!(), 0_(), unimplemented!()) });
}
pub unsafe fn test_results_52() {
    (unsafe { test_result_35(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_36(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_37(unimplemented!(), 0_(), unimplemented!()) });
}
pub unsafe fn test_results_53() {
    (unsafe { test_result_38(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_39(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_40(unimplemented!(), 0_(), unimplemented!()) });
}
pub unsafe fn test_results_54() {
    (unsafe { test_result_41(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_42(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_43(unimplemented!(), 0_(), unimplemented!()) });
}
pub unsafe fn test_results_55() {
    (unsafe { test_result_44(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_45(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_result_46(unimplemented!(), 0_(), unimplemented!()) });
}
pub unsafe fn test_user_defined_type_56() {
    let mut a: () = unimplemented!();
    let mut expected: () = unimplemented!();
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
            &mut a,
            UserDefinedType_57::new({ 0_() }),
        )
    });
    let mut proj_val: FnPtr<fn(*const UserDefinedType_57) -> ()> =
        FnPtr::<fn(*const UserDefinedType_57) -> ()>::new(|el: *const UserDefinedType_57| -> () {
            unsafe {
                return (*el).val;
            }
        });
    unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct UserDefinedType_57 {
    pub val: (),
}
impl UserDefinedType_57 {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn copy_from(other: *const UserDefinedType_57) -> Self {
        let mut this = Self {
            val: unimplemented!(),
        };
        this.val = (*other).val;
        this
    }
    pub unsafe fn copy_assign(&mut self, a: *const UserDefinedType_57) -> *mut UserDefinedType_57 {
        self.val = (*a).val;
        return &mut (*(self as *mut UserDefinedType_57));
    }
    pub unsafe fn operator_inc_4(&mut self) -> *mut UserDefinedType_57 {
        (self.val).prefix_inc();
        return &mut (*(self as *mut UserDefinedType_57));
    }
    pub unsafe fn operator_post_inc_5(&mut self, mut _a0: ()) -> UserDefinedType_57 {
        let mut tmp: UserDefinedType_57 =
            UserDefinedType_57::copy_from({ &(*(self as *mut UserDefinedType_57)) });
        self.val.prefix_inc();
        return UserDefinedType_57::copy_from({ &tmp });
    }
}
impl Clone for UserDefinedType_57 {
    fn clone(&self) -> Self {
        unsafe { UserDefinedType_57::copy_from(self as *const UserDefinedType_57) }
    }
}
pub unsafe fn test_dangerous_copy_assign_58() {
    let mut aa: () = unimplemented!();
    let mut expected: () = unimplemented!();
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_20))(
            &mut aa,
            DangerousCopyAssign::new({ 0_() }),
        )
    });
    let mut proj_val: FnPtr<fn(*const DangerousCopyAssign) -> ()> =
        FnPtr::<fn(*const DangerousCopyAssign) -> ()>::new(
            |el: *const DangerousCopyAssign| -> () {
                unsafe {
                    return (*el).val;
                }
            },
        );
    unimplemented!();
}
pub unsafe fn test_results_59() -> () {
    (unsafe {
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_results_47 ( ) } )  ;
 } , | | { ( unsafe { test_results_48 ( ) } )  ;
 } , | | { ( unsafe { test_results_49 ( ) } )  ;
 } , | | { ( unsafe { test_results_50 ( ) } )  ;
 } , | | { ( unsafe { test_results_51 ( ) } )  ;
 } , | | { ( unsafe { test_results_52 ( ) } )  ;
 } ) , )
    });
    (unsafe { test_results_53() });
    (unsafe { test_results_54() });
    (unsafe { test_results_55() });
    (unsafe { test_user_defined_type_56() });
    (unsafe { test_dangerous_copy_assign_58() });
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
    (unsafe { test_results_59() });
    const _: () = assert!(true, "test_results()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
