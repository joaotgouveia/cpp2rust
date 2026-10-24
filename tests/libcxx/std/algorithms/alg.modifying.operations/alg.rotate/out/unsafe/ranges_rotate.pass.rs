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
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__
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
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__,
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
pub unsafe fn operator_eq_2(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_3(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_4(i: *const forward_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
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
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_int_ptr_,
    ) -> *mut forward_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
                self as *const forward_iterator_int_ptr_,
                other as *const forward_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_int_ptr_ {}
pub unsafe fn operator_eq_5(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_6(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_7(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
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
impl std::cmp::PartialEq for bidirectional_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const bidirectional_iterator_int_ptr_,
                other as *const bidirectional_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
pub unsafe fn operator_add_8(
    mut x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_9(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_10(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_11(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_12(i: *const random_access_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
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
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
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
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
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
    mut x: contiguous_iterator_int_ptr_,
    mut n: (),
) -> contiguous_iterator_int_ptr_ {
    (unsafe { contiguous_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_14(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_15(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_16(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_17(i: *const contiguous_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
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
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
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
impl std::cmp::PartialEq for contiguous_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_15(
                self as *const contiguous_iterator_int_ptr_,
                other as *const contiguous_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_int_ptr_ {}
pub unsafe fn base_18(mut i: *mut ()) -> *mut () {
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
pub struct sentinel_wrapper_PermutableNotForwardIterator_ {
    base_: PermutableNotForwardIterator,
}
impl sentinel_wrapper_PermutableNotForwardIterator_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_PermutableNotSwappable_ {
    base_: PermutableNotSwappable,
}
impl sentinel_wrapper_PermutableNotSwappable_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_int_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_int_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_18((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_18((*other)) }));
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
            base_: (unsafe { base_17(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_17(other) }));
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
            base_: (unsafe { base_12(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_12(other) }));
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
            base_: (unsafe { base_7(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_7(other) }));
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
pub unsafe fn operator_eq_19(
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
            operator_eq_19(
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
pub unsafe fn operator_add_20(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_21(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_20(_i, n)
    });
}
pub unsafe fn iter_swap_22(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_23(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*unimplemented!())).cast::<()>();
}
pub unsafe fn operator_eq_24(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_25(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_25(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_24(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub struct UncheckedRange_forward_iterator_int_ptr___SentinelForNotSemiregular_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_forward_iterator_int_ptr___SentinelForNotWeaklyEqualityComparableWith_ {
}
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
pub struct UncheckedRange_PermutableNotForwardIterator__sentinel_wrapper_PermutableNotForwardIterator__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_PermutableNotSwappable__sentinel_wrapper_PermutableNotSwappable__ {}
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
const _: () = assert!(true, "HasRotateIter<int*, int*>");
const _: () = assert!(!(false), "!HasRotateIter<PermutableNotForwardIterator>");
const _: () = assert!(!(false), "!HasRotateIter<PermutableNotSwappable>");
const _: () = assert!(!(false), "!HasRotateIter<int*, SentinelForNotSemiregular>");
const _: () = assert!(
    !(false),
    "!HasRotateIter<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(true, "HasRotateRange<R<int*>>");
const _: () = assert!(!(false), "!HasRotateRange<ForwardRangeNotDerivedFrom>");
const _: () = assert!(!(false), "!HasRotateRange<ForwardRangeNotIncrementable>");
const _: () = assert!(
    !(false),
    "!HasRotateRange<ForwardRangeNotSentinelSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasRotateRange<ForwardRangeNotSentinelEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasRotateRange<PermutableRangeNotForwardIterator>"
);
const _: () = assert!(!(false), "!HasRotateRange<PermutableRangeNotSwappable>");
pub unsafe fn test_one_26(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
        let mut range: () = unimplemented!();
        let mut result: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(&mut range, mid)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_28(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
        let mut range: () = unimplemented!();
        let mut result: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(&mut range, mid)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_29(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
        let mut range: () = unimplemented!();
        let mut result: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(&mut range, mid)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_30(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
        let mut range: () = unimplemented!();
        let mut result: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(&mut range, mid)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_31(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
        let mut range: () = unimplemented!();
        let mut result: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(&mut range, mid)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_32(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_0: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_0 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_1: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_1 })
        };
        let mut range: () = unimplemented!();
        let mut result: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(&mut range, mid)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_33(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_2: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_2 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_3: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_3 })
        };
        let mut range: () = unimplemented!();
        let mut result: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(&mut range, mid)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_34(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_4: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_4 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_5: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_5 })
        };
        let mut range: () = unimplemented!();
        let mut result: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(&mut range, mid)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_35(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_6: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_6 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_7: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_7 })
        };
        let mut range: () = unimplemented!();
        let mut result: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(&mut range, mid)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_36(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_8: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_8 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: *mut () = unimplemented!();
        let mut mid: *mut () = unimplemented!().offset((mid_index) as isize);
        let mut end: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_9: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_9 })
        };
        let mut range: () = unimplemented!();
        let mut result: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(&mut range, mid)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_37(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_38(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_39(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_40(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_41(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_42(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_10: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_10 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_11: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_11 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_43(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_12: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_12 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_13: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_13 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_44(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_14: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_14 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_15: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_15 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_45(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_16: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_16 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_17: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_17 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_46(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_18: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_18 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_19: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_19 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_47(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_48(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_49(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_50(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_51(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_52(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_20: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_20 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_21: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_21 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_53(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_22: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_22 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_23: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_23 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_54(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_24: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_24 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_25: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_25 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_55(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_26: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_26 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_27: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_27 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_56(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_28: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_28 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_29: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_29 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_57(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_58(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_59(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_60(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_61(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_62(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_30: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_30 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_31: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_31 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_63(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_32: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_32 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_33: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_33 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_64(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_34: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_34 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_35: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_35 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_65(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_36: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_36 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_37: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_37 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_66(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_38: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_38 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: bidirectional_iterator_int_ptr_ = bidirectional_iterator_int_ptr_::new_1({
            unimplemented!().offset((mid_index) as isize)
        });
        let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_39: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_39 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_67(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_forward_iterator_int_ptr___forward_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_68(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_forward_iterator_int_ptr___forward_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_69(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_forward_iterator_int_ptr___forward_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_70(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_forward_iterator_int_ptr___forward_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_71(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_forward_iterator_int_ptr___forward_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_72(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_40: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_40 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_41: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_41 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_forward_iterator_int_ptr___forward_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_73(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_42: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_42 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_43: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_43 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_forward_iterator_int_ptr___forward_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_74(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_44: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_44 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_45: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_45 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_forward_iterator_int_ptr___forward_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_75(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_46: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_46 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_47: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_47 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_forward_iterator_int_ptr___forward_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_76(input: (), mut mid_index: (), mut expected: ()) {
    unimplemented!();
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_48: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_48 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(begin, mid, end)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = input.clone();
        let mut begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut mid: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!().offset((mid_index) as isize) });
        let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_49: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_49 })
        };
        let mut range: () = unimplemented!();
        let mut result : std_ranges_subrange_forward_iterator_int_ptr___forward_iterator_int_ptr___std_ranges_subrange_kind_unsized_ = ( unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27)) ( & mut range , mid , ) } ) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_iter_sent_77() {
    (unsafe { test_one_26(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_28(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_29(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_30(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_30(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_31(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_31(unimplemented!(), 3_(), unimplemented!()) });
    (unsafe { test_one_31(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_31(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_31(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_one_31(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iter_sent_78() {
    (unsafe { test_one_32(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_33(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_34(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_35(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_35(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_36(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_36(unimplemented!(), 3_(), unimplemented!()) });
    (unsafe { test_one_36(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_36(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_36(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_one_36(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iter_sent_79() {
    (unsafe { test_one_37(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_38(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_39(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_40(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_40(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_41(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_41(unimplemented!(), 3_(), unimplemented!()) });
    (unsafe { test_one_41(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_41(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_41(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_one_41(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iter_sent_80() {
    (unsafe { test_one_42(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_43(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_44(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_45(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_45(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_46(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_46(unimplemented!(), 3_(), unimplemented!()) });
    (unsafe { test_one_46(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_46(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_46(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_one_46(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iter_sent_81() {
    (unsafe { test_one_47(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_48(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_49(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_50(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_50(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_51(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_51(unimplemented!(), 3_(), unimplemented!()) });
    (unsafe { test_one_51(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_51(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_51(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_one_51(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iter_sent_82() {
    (unsafe { test_one_52(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_53(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_54(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_55(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_55(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_56(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_56(unimplemented!(), 3_(), unimplemented!()) });
    (unsafe { test_one_56(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_56(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_56(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_one_56(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iter_sent_83() {
    (unsafe { test_one_57(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_58(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_59(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_60(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_60(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_61(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_61(unimplemented!(), 3_(), unimplemented!()) });
    (unsafe { test_one_61(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_61(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_61(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_one_61(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iter_sent_84() {
    (unsafe { test_one_62(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_63(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_64(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_65(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_65(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_66(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_66(unimplemented!(), 3_(), unimplemented!()) });
    (unsafe { test_one_66(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_66(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_66(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_one_66(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iter_sent_85() {
    (unsafe { test_one_67(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_68(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_69(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_70(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_70(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_71(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_71(unimplemented!(), 3_(), unimplemented!()) });
    (unsafe { test_one_71(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_71(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_71(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_one_71(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_iter_sent_86() {
    (unsafe { test_one_72(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_73(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_74(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_75(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_75(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_76(unimplemented!(), 2_(), unimplemented!()) });
    (unsafe { test_one_76(unimplemented!(), 3_(), unimplemented!()) });
    (unsafe { test_one_76(unimplemented!(), 0_(), unimplemented!()) });
    (unsafe { test_one_76(unimplemented!(), 1_(), unimplemented!()) });
    (unsafe { test_one_76(unimplemented!(), 6_(), unimplemented!()) });
    (unsafe { test_one_76(unimplemented!(), 7_(), unimplemented!()) });
}
pub unsafe fn test_vector_bool_87() -> () {
    let mut offset: () = -4_();
    'loop_: while ((offset) <= (4_())) {
        let mut a: () = unimplemented!();
        let mut mid: () = ((8_() as ()).wrapping_div(2_()) as ()).wrapping_add(offset);
        let mut i: () = mid;
        'loop_: while ((i) < (8_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___first: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut begin_88))(&mut a) });
            let ___middle: () = unimplemented!();
            let ___last: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut end_89))(&mut a) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(
                ___first, ___middle, ___last,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (8_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___range: *mut () = &mut a;
            let ___middle: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(___range, ___middle)
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (8_())) {
            unimplemented!();
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_90() -> () {
    let mut offset: () = -4_();
    'loop_: while ((offset) <= (4_())) {
        let mut a: () = unimplemented!();
        let mut mid: () = ((19_() as ()).wrapping_div(2_()) as ()).wrapping_add(offset);
        let mut i: () = mid;
        'loop_: while ((i) < (19_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___first: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut begin_88))(&mut a) });
            let ___middle: () = unimplemented!();
            let ___last: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut end_89))(&mut a) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(
                ___first, ___middle, ___last,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (19_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___range: *mut () = &mut a;
            let ___middle: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(___range, ___middle)
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (19_())) {
            unimplemented!();
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_91() -> () {
    let mut offset: () = -4_();
    'loop_: while ((offset) <= (4_())) {
        let mut a: () = unimplemented!();
        let mut mid: () = ((32_() as ()).wrapping_div(2_()) as ()).wrapping_add(offset);
        let mut i: () = mid;
        'loop_: while ((i) < (32_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___first: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut begin_88))(&mut a) });
            let ___middle: () = unimplemented!();
            let ___last: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut end_89))(&mut a) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(
                ___first, ___middle, ___last,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (32_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___range: *mut () = &mut a;
            let ___middle: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(___range, ___middle)
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (32_())) {
            unimplemented!();
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_92() -> () {
    let mut offset: () = -4_();
    'loop_: while ((offset) <= (4_())) {
        let mut a: () = unimplemented!();
        let mut mid: () = ((49_() as ()).wrapping_div(2_()) as ()).wrapping_add(offset);
        let mut i: () = mid;
        'loop_: while ((i) < (49_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___first: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut begin_88))(&mut a) });
            let ___middle: () = unimplemented!();
            let ___last: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut end_89))(&mut a) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(
                ___first, ___middle, ___last,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (49_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___range: *mut () = &mut a;
            let ___middle: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(___range, ___middle)
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (49_())) {
            unimplemented!();
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_93() -> () {
    let mut offset: () = -4_();
    'loop_: while ((offset) <= (4_())) {
        let mut a: () = unimplemented!();
        let mut mid: () = ((64_() as ()).wrapping_div(2_()) as ()).wrapping_add(offset);
        let mut i: () = mid;
        'loop_: while ((i) < (64_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___first: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut begin_88))(&mut a) });
            let ___middle: () = unimplemented!();
            let ___last: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut end_89))(&mut a) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(
                ___first, ___middle, ___last,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (64_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___range: *mut () = &mut a;
            let ___middle: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(___range, ___middle)
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (64_())) {
            unimplemented!();
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_94() -> () {
    let mut offset: () = -4_();
    'loop_: while ((offset) <= (4_())) {
        let mut a: () = unimplemented!();
        let mut mid: () = ((199_() as ()).wrapping_div(2_()) as ()).wrapping_add(offset);
        let mut i: () = mid;
        'loop_: while ((i) < (199_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___first: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut begin_88))(&mut a) });
            let ___middle: () = unimplemented!();
            let ___last: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut end_89))(&mut a) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(
                ___first, ___middle, ___last,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (199_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___range: *mut () = &mut a;
            let ___middle: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(___range, ___middle)
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (199_())) {
            unimplemented!();
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_95() -> () {
    let mut offset: () = -4_();
    'loop_: while ((offset) <= (4_())) {
        let mut a: () = unimplemented!();
        let mut mid: () = ((256_() as ()).wrapping_div(2_()) as ()).wrapping_add(offset);
        let mut i: () = mid;
        'loop_: while ((i) < (256_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___first: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut begin_88))(&mut a) });
            let ___middle: () = unimplemented!();
            let ___last: () =
                (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut end_89))(&mut a) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(
                ___first, ___middle, ___last,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (256_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let ___range: *mut () = &mut a;
            let ___middle: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(___range, ___middle)
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (256_())) {
            unimplemented!();
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_96() -> () {
    (unsafe {
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_iter_sent_77 ( ) } )  ;
 ( unsafe { test_iter_sent_78 ( ) } )  ;
 } , | | { ( unsafe { test_iter_sent_79 ( ) } )  ;
 ( unsafe { test_iter_sent_80 ( ) } )  ;
 } , | | { ( unsafe { test_iter_sent_81 ( ) } )  ;
 ( unsafe { test_iter_sent_82 ( ) } )  ;
 } , | | { ( unsafe { test_iter_sent_83 ( ) } )  ;
 ( unsafe { test_iter_sent_84 ( ) } )  ;
 } , | | { ( unsafe { test_iter_sent_85 ( ) } )  ;
 ( unsafe { test_iter_sent_86 ( ) } )  ;
 } ) , )
    });
    {
        let input: () = unimplemented!();
        let mut expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut swaps: () = 0_();
            let mut begin: adl_Iterator =
                (unsafe { adl_Iterator::TrackSwaps_5(unimplemented!(), &mut swaps) });
            let mut end: adl_Iterator = (unsafe {
                adl_Iterator::TrackSwaps_5(
                    unimplemented!().offset((unimplemented!()) as isize),
                    &mut swaps,
                )
            });
            let mut mid: () = 0_();
            'loop_: while ((mid) != (unimplemented!())) {
                swaps = 0_();
                (unsafe {
                    let ___first: adl_Iterator = begin;
                    let ___middle: adl_Iterator = (unsafe {
                        let _i: adl_Iterator = begin;
                        operator_add_20(_i, mid)
                    });
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(
                        ___first, ___middle, end,
                    )
                });
                unimplemented!();
                mid.prefix_inc();
            }
        }
        {
            let mut in_: () = input.clone();
            let mut swaps: () = 0_();
            let mut begin: adl_Iterator =
                (unsafe { adl_Iterator::TrackSwaps_5(unimplemented!(), &mut swaps) });
            let mut end: adl_Iterator = (unsafe {
                adl_Iterator::TrackSwaps_5(
                    unimplemented!().offset((unimplemented!()) as isize),
                    &mut swaps,
                )
            });
            let mut range: () = unimplemented!();
            let mut mid: () = 0_();
            'loop_: while ((mid) != (unimplemented!())) {
                swaps = 0_();
                (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_27))(
                        &mut range,
                        (unsafe {
                            let _i: adl_Iterator = begin;
                            operator_add_20(_i, mid)
                        }),
                    )
                });
                unimplemented!();
                mid.prefix_inc();
            }
        }
    }
    (unsafe { test_vector_bool_87() });
    (unsafe { test_vector_bool_90() });
    (unsafe { test_vector_bool_91() });
    (unsafe { test_vector_bool_92() });
    (unsafe { test_vector_bool_93() });
    (unsafe { test_vector_bool_94() });
    (unsafe { test_vector_bool_95() });
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
    (unsafe { test_96() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
