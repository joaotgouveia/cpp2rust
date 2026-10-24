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
pub struct types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__
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
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr____types_type_list_bidirectional_iterator_const_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr____types_type_list_forward_iterator_const_int_ptr___
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
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr____types_type_list_cpp20_output_iterator_int_ptr___
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr__,
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
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_2(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__,
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
        swallow_3(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn swallow_3(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
) {
}
pub unsafe fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: (), mut _a4: ()) {}
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
pub unsafe fn operator_ne_5(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_6(i: *const forward_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct forward_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn base_7(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn operator_ne_8(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_9(i: *const bidirectional_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct bidirectional_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn operator_ne_10(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_11(i: *const random_access_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct random_access_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
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
pub unsafe fn base_12(i: *const random_access_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn operator_add_13(
    mut x: contiguous_iterator_const_int_ptr_,
    mut n: (),
) -> contiguous_iterator_const_int_ptr_ {
    (unsafe { contiguous_iterator_const_int_ptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_const_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_ne_14(
    x: *const contiguous_iterator_const_int_ptr_,
    y: *const contiguous_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_15(i: *const contiguous_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_const_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut contiguous_iterator_const_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn operator_add_16(
    mut x: contiguous_iterator_int_ptr_,
    mut n: (),
) -> contiguous_iterator_int_ptr_ {
    (unsafe { contiguous_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn base_17(i: *const contiguous_iterator_int_ptr_) -> *mut () {
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
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
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
}
pub unsafe fn base_18(mut i: *mut ()) -> *mut () {
    return i;
}
pub unsafe fn base_19(mut i: *const ()) -> *const () {
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
pub unsafe fn base_20(i: *const cpp20_output_iterator_int_ptr_) -> *mut () {
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
pub struct sentinel_wrapper_ForwardIteratorNotDerivedFrom_ {
    base_: ForwardIteratorNotDerivedFrom,
}
impl sentinel_wrapper_ForwardIteratorNotDerivedFrom_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_ForwardIteratorNotIncrementable_ {
    base_: ForwardIteratorNotIncrementable,
}
impl sentinel_wrapper_ForwardIteratorNotIncrementable_ {}
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
pub unsafe fn operator_eq_21(
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
            operator_eq_21(
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
pub unsafe fn operator_add_22(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_23(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_22(_i, n)
    });
}
pub unsafe fn iter_swap_24(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_25(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*unimplemented!())).cast::<()>();
}
pub unsafe fn operator_eq_26(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_27(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_27(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_26(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub struct UncheckedRange_ForwardIteratorNotDerivedFrom__sentinel_wrapper_ForwardIteratorNotDerivedFrom__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_ForwardIteratorNotIncrementable__sentinel_wrapper_ForwardIteratorNotIncrementable__
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
const _: () = assert!(true, "HasRotateCopyIt<int*>");
const _: () = assert!(!(false), "!HasRotateCopyIt<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(
    !(false),
    "!HasRotateCopyIt<ForwardIteratorNotIncrementable>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyIt<int*, SentinelForNotSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyIt<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyIt<int*, OutputIteratorNotIndirectlyWritable>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyIt<int*, OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(true, "HasRotateCopyR<UncheckedRange<int*>>");
const _: () = assert!(!(false), "!HasRotateCopyR<ForwardRangeNotDerivedFrom>");
const _: () = assert!(!(false), "!HasRotateCopyR<ForwardRangeNotIncrementable>");
const _: () = assert!(
    !(false),
    "!HasRotateCopyR<UncheckedRange<int*, SentinelForNotSemiregular>>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyR<UncheckedRange<int*>, OutputIteratorNotIndirectlyWritable>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyR<UncheckedRange<int*>, OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::is_same_v<std::ranges::rotate_copy_result<int, int>, std::ranges::in_out_result<int, int>>"
);
pub unsafe fn test_28(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_30(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_31(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_32(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_33(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_34(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_35(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_36(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_37(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_38(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_39(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_40(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_41(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_42(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_43(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_44(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_45(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_46(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_47(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_48(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_49(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_50(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_51(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_52(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_53(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_54(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_55(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_56(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_57(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_58(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: *const () = unimplemented!();
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___last: *const () = unimplemented!().offset((unimplemented!()) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: *const () = unimplemented!().offset((middle) as isize);
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_59(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_60(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_61(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_62(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_63(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_64(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_65(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_66(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_67(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_68(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_69(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_70(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_71(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_72(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_73(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_74(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_75(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_76(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_77(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_78(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_79(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_80(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_81(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_82(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_83(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_84(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_85(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_86(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_87(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_88(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_89(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_90(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_91(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_92(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_93(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_94(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_95(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_96(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_97(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_98(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_99(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_100(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_101(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_102(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_103(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_104(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_105(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_106(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_107(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_108(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_109(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_110(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_111(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_112(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_113(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_114(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_115(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_116(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_117(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_118(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_119(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_120(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_121(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_122(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_123(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_124(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_125(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_126(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_127(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_128(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_129(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_130(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_131(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_132(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_133(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_134(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_135(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_136(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_137(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_138(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_139(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_140(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_141(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_142(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_143(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_144(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_145(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_146(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_147(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_148(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_149(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_150(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_151(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_152(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_153(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: *mut () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_154(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_155(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_156(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_157(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_158(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_159(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_160(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_161(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_162(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_163(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_164(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_165(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_166(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_167(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_168(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_169(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_170(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_171(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_172(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_173(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_174(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_175(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_176(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_177(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_178(mut value: (), mut middle: (), mut expected: ()) {
    {
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() });
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___last: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                ___first, ___middle, ___last, ___result,
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
            let ___middle: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (unimplemented!().offset((middle) as isize)).cast_const()
                });
            let ___result: cpp20_output_iterator_int_ptr_ =
                cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                &mut range, ___middle, ___result,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_iterators_179() {
    (unsafe { test_28(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_30(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_31(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_32(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_33(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_33(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_33(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_33(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_180() {
    (unsafe { test_34(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_35(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_36(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_37(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_38(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_38(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_38(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_38(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_181() {
    (unsafe { test_39(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_40(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_41(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_42(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_43(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_43(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_43(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_43(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_182() {
    (unsafe { test_44(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_45(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_46(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_47(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_48(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_48(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_48(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_48(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_183() {
    (unsafe { test_49(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_50(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_51(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_52(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_53(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_53(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_53(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_53(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_184() {
    (unsafe { test_54(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_55(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_56(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_57(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_58(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_58(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_58(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_58(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_185() {
    (unsafe { test_59(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_60(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_61(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_62(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_63(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_63(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_63(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_63(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_186() {
    (unsafe { test_64(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_65(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_66(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_67(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_68(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_68(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_68(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_68(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_187() {
    (unsafe { test_69(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_70(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_71(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_72(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_73(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_73(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_73(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_73(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_188() {
    (unsafe { test_74(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_75(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_76(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_77(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_78(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_78(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_78(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_78(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_189() {
    (unsafe { test_79(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_80(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_81(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_82(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_83(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_83(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_83(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_83(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_190() {
    (unsafe { test_84(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_85(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_86(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_87(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_88(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_88(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_88(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_88(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_191() {
    (unsafe { test_89(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_90(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_91(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_92(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_93(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_93(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_93(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_93(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_192() {
    (unsafe { test_94(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_95(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_96(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_97(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_98(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_98(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_98(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_98(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_193() {
    (unsafe { test_99(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_100(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_101(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_102(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_103(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_103(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_103(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_103(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_194() {
    (unsafe { test_104(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_105(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_106(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_107(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_108(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_108(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_108(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_108(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_195() {
    (unsafe { test_109(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_110(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_111(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_112(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_113(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_113(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_113(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_113(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_196() {
    (unsafe { test_114(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_115(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_116(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_117(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_118(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_118(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_118(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_118(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_197() {
    (unsafe { test_119(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_120(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_121(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_122(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_123(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_123(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_123(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_123(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_198() {
    (unsafe { test_124(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_125(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_126(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_127(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_128(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_128(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_128(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_128(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_199() {
    (unsafe { test_129(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_130(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_131(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_132(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_133(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_133(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_133(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_133(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_200() {
    (unsafe { test_134(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_135(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_136(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_137(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_138(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_138(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_138(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_138(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_201() {
    (unsafe { test_139(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_140(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_141(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_142(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_143(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_143(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_143(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_143(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_202() {
    (unsafe { test_144(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_145(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_146(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_147(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_148(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_148(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_148(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_148(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_203() {
    (unsafe { test_149(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_150(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_151(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_152(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_153(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_153(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_153(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_153(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_204() {
    (unsafe { test_154(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_155(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_156(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_157(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_158(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_158(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_158(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_158(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_205() {
    (unsafe { test_159(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_160(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_161(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_162(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_163(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_163(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_163(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_163(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_206() {
    (unsafe { test_164(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_165(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_166(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_167(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_168(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_168(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_168(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_168(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_207() {
    (unsafe { test_169(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_170(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_171(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_172(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_173(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_173(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_173(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_173(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iterators_208() {
    (unsafe { test_174(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_175(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_176(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_177(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_178(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_178(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_178(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_178(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_209() -> () {
    (unsafe {
        for_each_0 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_2 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_iterators_179 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_180 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_181 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_182 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_183 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_184 ( ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_210 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_iterators_185 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_186 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_187 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_188 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_189 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_190 ( ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_211 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_iterators_191 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_192 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_193 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_194 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_195 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_196 ( ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_212 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_iterators_197 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_198 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_199 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_200 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_201 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_202 ( ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_213 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_iterators_203 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_204 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_205 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_206 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_207 ( ) } )  ;
 } , | | { ( unsafe { test_iterators_208 ( ) } )  ;
 } ) , ) } )  ;
 } ) , )
    });
    {
        {
            let mut c: () = 0_();
            let mut a: [AssignmentCounter_214; 4] = [
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
            ];
            let mut b: [AssignmentCounter_214; 4] = [
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
            ];
            (unsafe {
                let ___first: *mut AssignmentCounter_214 = a.as_mut_ptr();
                let ___middle: *mut AssignmentCounter_214 = a.as_mut_ptr().offset((2_()) as isize);
                let ___last: *mut AssignmentCounter_214 = a.as_mut_ptr().offset((4_()) as isize);
                (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                    ___first,
                    ___middle,
                    ___last,
                    b.as_mut_ptr(),
                )
            });
            unimplemented!();
        }
        {
            let mut c: () = 0_();
            let mut a: [AssignmentCounter_214; 4] = [
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
            ];
            let mut b: [AssignmentCounter_214; 4] = [
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
                AssignmentCounter_214::new({ (&mut c as *mut ()) }),
            ];
            (unsafe {
                let ___range: *mut [AssignmentCounter_214; 4] = &mut a;
                let ___middle: *mut AssignmentCounter_214 = a.as_mut_ptr().offset((2_()) as isize);
                (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_29))(
                    ___range,
                    ___middle,
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
pub struct AssignmentCounter_214 {
    pub counter: *mut (),
}
impl AssignmentCounter_214 {
    pub unsafe fn new(mut counter_: *mut ()) -> Self {
        let mut this = Self { counter: counter_ };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const AssignmentCounter_214,
    ) -> *mut AssignmentCounter_214 {
        (*self.counter).prefix_inc();
        return &mut (*(self as *mut AssignmentCounter_214));
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
    (unsafe { test_209() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
