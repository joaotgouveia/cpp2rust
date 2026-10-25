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
pub unsafe fn base_16(i: *const contiguous_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn base_17(mut i: *mut ()) -> *mut () {
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
            base_: (unsafe { base_17((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_17((*other)) }));
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
            base_: (unsafe { base_16(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_16(other) }));
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
pub unsafe fn operator_sub_18(s: *const sized_sentinel_int_ptr_, i: *const *mut ()) -> () {
    return (((*s).base_ as usize - (unsafe { base_17((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_19(i: *const *mut (), s: *const sized_sentinel_int_ptr_) -> () {
    return (((unsafe { base_17((*i)) }) as usize - (*s).base_ as usize)
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
            base_: (unsafe { base_17((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        return ((self.base_) == (unsafe { base_17((*other)) }));
    }
}
pub unsafe fn operator_sub_20(
    s: *const sized_sentinel_contiguous_iterator_int_ptr__,
    i: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_16(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_21(
    i: *const contiguous_iterator_int_ptr_,
    s: *const sized_sentinel_contiguous_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_16(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_int_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_contiguous_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_16(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        return ((self.base_) == (unsafe { base_16(other) }));
    }
}
pub unsafe fn operator_sub_22(
    s: *const sized_sentinel_random_access_iterator_int_ptr__,
    i: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_12(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_23(
    i: *const random_access_iterator_int_ptr_,
    s: *const sized_sentinel_random_access_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_12(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_random_access_iterator_int_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_random_access_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_12(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_int_ptr_) -> () {
        return ((self.base_) == (unsafe { base_12(other) }));
    }
}
pub unsafe fn operator_sub_24(
    s: *const sized_sentinel_bidirectional_iterator_int_ptr__,
    i: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_7(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_25(
    i: *const bidirectional_iterator_int_ptr_,
    s: *const sized_sentinel_bidirectional_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_7(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_bidirectional_iterator_int_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_bidirectional_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_7(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_int_ptr_) -> () {
        return ((self.base_) == (unsafe { base_7(other) }));
    }
}
pub unsafe fn operator_sub_26(
    s: *const sized_sentinel_forward_iterator_int_ptr__,
    i: *const forward_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_4(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_27(
    i: *const forward_iterator_int_ptr_,
    s: *const sized_sentinel_forward_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_4(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_forward_iterator_int_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_forward_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_4(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_int_ptr_) -> () {
        return ((self.base_) == (unsafe { base_4(other) }));
    }
}
pub unsafe fn operator_eq_28(
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
            operator_eq_28(
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
pub unsafe fn operator_add_29(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_30(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_29(_i, n)
    });
}
pub unsafe fn iter_swap_31(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_32(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_33(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_34(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_34(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_33(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn operator_eq_35(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_36(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_37(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_38(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_39(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_40(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_41(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: (),
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0_();
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_41(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_35(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_42(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_43(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_44(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_45(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_46(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_47(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_48(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct TrivialMoveOnly {
    data_: (),
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_48(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_42(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InvalidDifferenceT {}
const _: () = assert!(true, "HasShiftLeftIt<int*>");
const _: () = assert!(true, "HasShiftLeftIt<int*, sentinel_wrapper<int*>>");
const _: () = assert!(true, "HasShiftLeftIt<int*, sized_sentinel<int*>>");
const _: () = assert!(!(false), "!HasShiftLeftIt<int*, int*, InvalidDifferenceT>");
const _: () = assert!(!(false), "!HasShiftLeftIt<int*, int, int>");
const _: () = assert!(!(false), "!HasShiftLeftIt<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(!(false), "!HasShiftLeftIt<PermutableNotForwardIterator>");
const _: () = assert!(!(false), "!HasShiftLeftIt<PermutableNotSwappable>");
const _: () = assert!(true, "HasShiftLeftR<UncheckedRange<int*>>");
const _: () = assert!(
    !(false),
    "!HasShiftLeftR<UncheckedRange<int*>, InvalidDifferenceT>"
);
const _: () = assert!(!(false), "!HasShiftLeftR<ForwardRangeNotDerivedFrom>");
const _: () = assert!(
    !(false),
    "!HasShiftLeftR<PermutableRangeNotForwardIterator>"
);
const _: () = assert!(!(false), "!HasShiftLeftR<PermutableRangeNotSwappable>");
pub unsafe fn operator_add_49(mut x: DiffTypeIter, mut n: ()) -> DiffTypeIter {
    (unsafe { DiffTypeIter::operator_add_assign(&mut x, n) });
    return x;
}
pub unsafe fn operator_add_50(mut n: (), mut x: DiffTypeIter) -> DiffTypeIter {
    (unsafe { DiffTypeIter::operator_add_assign(&mut x, n) });
    return x;
}
pub unsafe fn operator_sub_51(mut x: DiffTypeIter, mut n: ()) -> DiffTypeIter {
    (unsafe { DiffTypeIter::operator_sub_assign(&mut x, n) });
    return x;
}
pub unsafe fn operator_sub_52(mut x: DiffTypeIter, mut y: DiffTypeIter) -> () {
    return ((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_eq_53(x: *const DiffTypeIter, y: *const DiffTypeIter) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_54(x: *const DiffTypeIter, y: *const DiffTypeIter) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_55(x: *const DiffTypeIter, y: *const DiffTypeIter) -> () {
    return (((*x).it_) < ((*y).it_));
}
pub unsafe fn operator_le_56(x: *const DiffTypeIter, y: *const DiffTypeIter) -> () {
    return (((*x).it_) <= ((*y).it_));
}
pub unsafe fn operator_gt_57(x: *const DiffTypeIter, y: *const DiffTypeIter) -> () {
    return (((*x).it_) > ((*y).it_));
}
pub unsafe fn operator_ge_58(x: *const DiffTypeIter, y: *const DiffTypeIter) -> () {
    return (((*x).it_) >= ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct DiffTypeIter {
    pub it_: *mut (),
}
impl DiffTypeIter {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut ()) -> Self {
        let mut this = Self { it_: it };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut DiffTypeIter {
        self.it_.prefix_inc();
        return &mut (*(self as *mut DiffTypeIter));
    }
    pub unsafe fn operator_dec_6(&mut self) -> *mut DiffTypeIter {
        self.it_.prefix_dec();
        return &mut (*(self as *mut DiffTypeIter));
    }
    pub unsafe fn operator_post_inc_7(&mut self, mut _a0: ()) -> DiffTypeIter {
        return DiffTypeIter::new_2({ self.it_.postfix_inc() });
    }
    pub unsafe fn operator_post_dec_8(&mut self, mut _a0: ()) -> DiffTypeIter {
        return DiffTypeIter::new_2({ self.it_.postfix_dec() });
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut DiffTypeIter {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut DiffTypeIter));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut DiffTypeIter {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut DiffTypeIter));
    }
}
impl std::cmp::Ord for DiffTypeIter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_55(self as *const DiffTypeIter, other as *const DiffTypeIter) {
                std::cmp::Ordering::Less
            } else if operator_lt_55(other as *const DiffTypeIter, self as *const DiffTypeIter) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for DiffTypeIter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DiffTypeIter {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_53(self as *const DiffTypeIter, other as *const DiffTypeIter) }
    }
}
impl std::cmp::Eq for DiffTypeIter {}
impl Default for DiffTypeIter {
    fn default() -> Self {
        unsafe { DiffTypeIter::new_1() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct TrackCopyMove {
    pub copy_count: (),
    pub move_count: (),
}
impl TrackCopyMove {
    pub unsafe fn copy_from(other: *const TrackCopyMove) -> Self {
        let mut this = Self {
            copy_count: (*other).copy_count,
            move_count: (*other).move_count,
        };
        this.copy_count.prefix_inc();
        (*(&raw const (*other).copy_count).cast_mut()).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut TrackCopyMove) -> Self {
        let mut this = Self {
            copy_count: (*other).copy_count,
            move_count: (*other).move_count,
        };
        this.move_count.prefix_inc();
        (*other).move_count.prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, other: *const TrackCopyMove) -> *mut TrackCopyMove {
        self.copy_count.prefix_inc();
        (*(&raw const (*other).copy_count).cast_mut()).prefix_inc();
        return &mut (*(self as *mut TrackCopyMove));
    }
    pub unsafe fn move_assign(&mut self, other: *mut TrackCopyMove) -> *mut TrackCopyMove {
        self.move_count.prefix_inc();
        (*other).move_count.prefix_inc();
        return &mut (*(self as *mut TrackCopyMove));
    }
}
impl Clone for TrackCopyMove {
    fn clone(&self) -> Self {
        unsafe { TrackCopyMove::copy_from(self as *const TrackCopyMove) }
    }
}
impl Default for TrackCopyMove {
    fn default() -> Self {
        unsafe {
            TrackCopyMove {
                copy_count: 0_(),
                move_count: 0_(),
            }
        }
    }
}
pub unsafe fn test_iter_sent_59() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: *mut () = unimplemented!();
                let mut end: *mut () = unimplemented!().offset((n) as isize);
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: *mut () = unimplemented!();
                let mut end: *mut () = unimplemented!().offset((n) as isize);
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: *mut () = unimplemented!();
            let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: *mut () = unimplemented!();
            let mut end: *mut () = unimplemented!().offset((unimplemented!()) as isize);
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_62() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: *mut () = unimplemented!();
                let mut end: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_0: *mut () = unimplemented!().offset((n) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_0 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: *mut () = unimplemented!();
                let mut end: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_1: *mut () = unimplemented!().offset((n) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_1 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let mut __tmp_2: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_2 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let mut __tmp_3: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_3 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let mut __tmp_4: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_4 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let mut __tmp_5: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_5 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let mut __tmp_6: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_6 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let mut __tmp_7: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_7 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: *mut () = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let mut __tmp_8: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_8 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: *mut () = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let mut __tmp_9: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_9 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_63() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: *mut () = unimplemented!();
                let mut end: sized_sentinel_int_ptr_ = {
                    let mut __tmp_10: *mut () = unimplemented!().offset((n) as isize);
                    sized_sentinel_int_ptr_::new_1({ &mut __tmp_10 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: *mut () = unimplemented!();
                let mut end: sized_sentinel_int_ptr_ = {
                    let mut __tmp_11: *mut () = unimplemented!().offset((n) as isize);
                    sized_sentinel_int_ptr_::new_1({ &mut __tmp_11 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let mut __tmp_12: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sized_sentinel_int_ptr_::new_1({ &mut __tmp_12 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let mut __tmp_13: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sized_sentinel_int_ptr_::new_1({ &mut __tmp_13 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let mut __tmp_14: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sized_sentinel_int_ptr_::new_1({ &mut __tmp_14 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let mut __tmp_15: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sized_sentinel_int_ptr_::new_1({ &mut __tmp_15 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let mut __tmp_16: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sized_sentinel_int_ptr_::new_1({ &mut __tmp_16 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: *mut () = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let mut __tmp_17: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sized_sentinel_int_ptr_::new_1({ &mut __tmp_17 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: *mut () = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let mut __tmp_18: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sized_sentinel_int_ptr_::new_1({ &mut __tmp_18 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: *mut () = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let mut __tmp_19: *mut () = unimplemented!().offset((unimplemented!()) as isize);
                sized_sentinel_int_ptr_::new_1({ &mut __tmp_19 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_64() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_65() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_20: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_20 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_21: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_21 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let mut __tmp_22: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_22 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let mut __tmp_23: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_23 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let mut __tmp_24: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_24 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let mut __tmp_25: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_25 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let mut __tmp_26: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_26 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let mut __tmp_27: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_27 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let mut __tmp_28: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_28 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let mut __tmp_29: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_29 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_66() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_30: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_30 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_31: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_31 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let mut __tmp_32: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_32 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let mut __tmp_33: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_33 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let mut __tmp_34: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_34 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let mut __tmp_35: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_35 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let mut __tmp_36: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_36 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let mut __tmp_37: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_37 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let mut __tmp_38: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_38 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let mut __tmp_39: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_39 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_67() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((n) as isize)
                    });
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((n) as isize)
                    });
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_68() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_40: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_40 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_41: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_41 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let mut __tmp_42: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_42 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let mut __tmp_43: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_43 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let mut __tmp_44: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_44 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let mut __tmp_45: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_45 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let mut __tmp_46: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_46 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let mut __tmp_47: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_47 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let mut __tmp_48: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_48 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let mut __tmp_49: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_49 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_69() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_50: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_50 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_51: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_51 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let mut __tmp_52: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_52 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let mut __tmp_53: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_53 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let mut __tmp_54: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_54 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let mut __tmp_55: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_55 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let mut __tmp_56: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_56 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let mut __tmp_57: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_57 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let mut __tmp_58: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_58 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let mut __tmp_59: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_59 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_70() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((n) as isize)
                    });
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((n) as isize)
                    });
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_71() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_60: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_60 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_61: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_61 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_62: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_62 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_63: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_63 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_64: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_64 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_65: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_65 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_66: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_66 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_67: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_67 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_68: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_68 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_69: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_69 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_72() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_70: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_70 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_71: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_71 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_72: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_72 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_73: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_73 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_74: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_74 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_75: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_75 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_76: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_76 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_77: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_77 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_78: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_78 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let mut __tmp_79: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_79 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_73() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_74() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_80: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_80 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_81: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_81 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let mut __tmp_82: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_82 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let mut __tmp_83: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_83 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let mut __tmp_84: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_84 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let mut __tmp_85: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_85 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let mut __tmp_86: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_86 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let mut __tmp_87: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_87 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let mut __tmp_88: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_88 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let mut __tmp_89: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_89 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_75() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_90: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_90 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_91: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_91 })
                };
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let mut __tmp_92: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_92 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let mut __tmp_93: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_93 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let mut __tmp_94: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_94 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let mut __tmp_95: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_95 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let mut __tmp_96: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_96 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let mut __tmp_97: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_97 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let mut __tmp_98: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_98 })
            };
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let mut __tmp_99: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
                sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_99 })
            };
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_iter_sent_76() {
    {
        let original: () = unimplemented!();
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: DiffTypeIter = DiffTypeIter::new_2({ unimplemented!() });
                let mut end: DiffTypeIter =
                    DiffTypeIter::new_2({ unimplemented!().offset((n) as isize) });
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while ((n) <= (unimplemented!())) {
            let mut k: () = 0_();
            'loop_: while ((k) <= ((n).wrapping_add(2_()))) {
                let mut scratch: () = unimplemented!();
                let mut begin: DiffTypeIter = DiffTypeIter::new_2({ unimplemented!() });
                let mut end: DiffTypeIter =
                    DiffTypeIter::new_2({ unimplemented!().offset((n) as isize) });
                (unsafe {
                    let ___first: *const () = unimplemented!();
                    let ___last: *const () = unimplemented!().offset((n) as isize);
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_60))(
                        ___first, ___last, begin,
                    )
                });
                let mut range: () = unimplemented!();
                let mut result: () = (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, k)
                });
                unimplemented!();
                if ((k) < (n)) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: DiffTypeIter = DiffTypeIter::new_2({ unimplemented!() });
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(begin, end, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: DiffTypeIter = DiffTypeIter::new_2({ unimplemented!() });
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut range, 0_())
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: DiffTypeIter = DiffTypeIter::new_2({ unimplemented!() });
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: DiffTypeIter = DiffTypeIter::new_2({ unimplemented!() });
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    unimplemented!(),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = input.clone();
            let mut begin: DiffTypeIter = DiffTypeIter::new_2({ unimplemented!() });
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = input.clone();
            let mut begin: DiffTypeIter = DiffTypeIter::new_2({ unimplemented!() });
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!() as ()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut input: () = unimplemented!();
        let expected: () = unimplemented!();
        {
            let mut in_: () = unimplemented!();
            let mut begin: DiffTypeIter = DiffTypeIter::new_2({ unimplemented!() });
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    begin,
                    end,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: DiffTypeIter = DiffTypeIter::new_2({ unimplemented!() });
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    &mut range,
                    (unimplemented!()).wrapping_add(1_()),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_77() -> () {
    (unsafe {
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_iter_sent_59 ( ) } )  ;
 ( unsafe { test_iter_sent_62 ( ) } )  ;
 ( unsafe { test_iter_sent_63 ( ) } )  ;
 } , | | { ( unsafe { test_iter_sent_64 ( ) } )  ;
 ( unsafe { test_iter_sent_65 ( ) } )  ;
 ( unsafe { test_iter_sent_66 ( ) } )  ;
 } , | | { ( unsafe { test_iter_sent_67 ( ) } )  ;
 ( unsafe { test_iter_sent_68 ( ) } )  ;
 ( unsafe { test_iter_sent_69 ( ) } )  ;
 } , | | { ( unsafe { test_iter_sent_70 ( ) } )  ;
 ( unsafe { test_iter_sent_71 ( ) } )  ;
 ( unsafe { test_iter_sent_72 ( ) } )  ;
 } , | | { ( unsafe { test_iter_sent_73 ( ) } )  ;
 ( unsafe { test_iter_sent_74 ( ) } )  ;
 ( unsafe { test_iter_sent_75 ( ) } )  ;
 } ) , )
    });
    (unsafe { test_iter_sent_76() });
    {
        let length: () = 100_();
        let n: () = ((length) / (2_()));
        let mut make_vec: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                let mut vec_: () = unimplemented!();
                unimplemented!();
                let mut i: () = 0_();
                'loop_: while ((i) < (100_())) {
                    unimplemented!();
                    i.prefix_inc();
                }
                return unimplemented!();
            }
        });
        {
            let mut input: () = (unsafe { make_vec.call() });
            let mut result: () = (unsafe {
                let ___first: () = unimplemented!();
                let ___last: () = unimplemented!();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(
                    ___first, ___last, n,
                )
            });
            unimplemented!();
            unimplemented!();
            let mut total_copies: () = 0_();
            let mut total_moves: () = 0_();
            let mut it: () = unimplemented!();
            'loop_: while !(unimplemented!()) {
                let item: *const TrackCopyMove = &(*unimplemented!());
                total_copies += (*item).copy_count;
                total_moves += (*item).move_count;
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
        {
            let mut input: () = (unsafe { make_vec.call() });
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_61))(&mut input, n)
            });
            unimplemented!();
            unimplemented!();
            let mut total_copies: () = 0_();
            let mut total_moves: () = 0_();
            let mut it: () = unimplemented!();
            'loop_: while !(unimplemented!()) {
                let item: *const TrackCopyMove = &(*unimplemented!());
                total_copies += (*item).copy_count;
                total_moves += (*item).move_count;
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
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
    (unsafe { test_77() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
