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
pub struct test_macros_detail_is_same_void__void_ {}
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
pub unsafe fn base_1(i: *const cpp17_input_iterator_int_ptr__int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveAssignUnsafe, Default)]
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
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_int_ptr__int_ptr_,
    ) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_2(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn base_3(i: *const forward_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveAssignUnsafe, Default)]
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
pub unsafe fn operator_eq_4(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn base_5(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveAssignUnsafe, Default)]
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
            operator_eq_4(
                self as *const bidirectional_iterator_int_ptr_,
                other as *const bidirectional_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
pub unsafe fn operator_eq_6(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn base_7(i: *const random_access_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveAssignUnsafe, Default)]
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
    pub unsafe fn operator_index(&self, mut n: ()) -> () {
        return &mut (*self.it_.offset((n) as isize));
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
            operator_eq_6(
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
pub unsafe fn operator_eq_8(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn base_9(i: *const contiguous_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveAssignUnsafe, Default)]
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
    pub unsafe fn operator_index(&self, mut n: ()) -> () {
        return &mut (*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut contiguous_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
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
            operator_eq_8(
                self as *const contiguous_iterator_int_ptr_,
                other as *const contiguous_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_int_ptr_ {}
pub unsafe fn base_10(mut i: *mut ()) -> *mut () {
    return i;
}
pub unsafe fn base_11(i: *const cpp20_input_iterator_int_ptr_) -> *mut () {
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct iter_value_or_void_forward_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct iter_value_or_void_bidirectional_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct iter_value_or_void_random_access_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct iter_value_or_void_contiguous_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct iter_value_or_void_int_ptr_ {}
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
pub unsafe fn base_12(
    it: *const operation_counting_iterator_forward_iterator_int_ptr__,
) -> forward_iterator_int_ptr_ {
    return forward_iterator_int_ptr_::new_1({ (*it).base_ });
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe)]
pub struct operation_counting_iterator_forward_iterator_int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_forward_iterator_int_ptr__ {
    pub unsafe fn new_1(
        it: *const forward_iterator_int_ptr_,
        mut counts: Option<*mut IteratorOpCounts>,
    ) -> Self {
        let mut counts: *mut IteratorOpCounts =
            counts.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
        let mut this = Self {
            base_: (unsafe { base_3(it) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut operation_counting_iterator_forward_iterator_int_ptr__,
    ) -> *mut operation_counting_iterator_forward_iterator_int_ptr__ {
        return &mut {
            (*(self as *mut operation_counting_iterator_forward_iterator_int_ptr__)) = (*o).clone();
            (*(self as *mut operation_counting_iterator_forward_iterator_int_ptr__)).clone()
        };
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*(unsafe {
            forward_iterator_int_ptr_::operator_deref(&forward_iterator_int_ptr_::new_1({
                self.base_
            }))
        }));
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut operation_counting_iterator_forward_iterator_int_ptr__ {
        let mut tmp: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_3(&(*(unsafe { forward_iterator_int_ptr_::operator_inc_6(&mut tmp) })))
        });
        (unsafe { operation_counting_iterator_forward_iterator_int_ptr__::moved_by(self, 1_()) });
        return &mut (*(self as *mut operation_counting_iterator_forward_iterator_int_ptr__));
    }
    pub unsafe fn record_equality_comparison(&self) {
        if !((self.counts_).is_null()) {
            (*self.counts_).equal_cmps.prefix_inc();
        }
    }
    unsafe fn moved_by(&mut self, mut n: ()) {
        if (self.counts_).is_null() {
            return;
        }
        if ((n) > (0_())) {
            (*self.counts_).increments.prefix_inc();
        } else if ((n) < (0_())) {
            (*self.counts_).decrements.prefix_inc();
        } else {
            (*self.counts_).zero_moves.prefix_inc();
        }
    }
}
impl Default for operation_counting_iterator_forward_iterator_int_ptr__ {
    fn default() -> Self {
        unsafe {
            operation_counting_iterator_forward_iterator_int_ptr__ {
                base_: std::ptr::null_mut(),
                counts_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn base_13(
    it: *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (*it).base_ });
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe)]
pub struct operation_counting_iterator_bidirectional_iterator_int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_bidirectional_iterator_int_ptr__ {
    pub unsafe fn new_1(
        it: *const bidirectional_iterator_int_ptr_,
        mut counts: Option<*mut IteratorOpCounts>,
    ) -> Self {
        let mut counts: *mut IteratorOpCounts =
            counts.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
        let mut this = Self {
            base_: (unsafe { base_5(it) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut operation_counting_iterator_bidirectional_iterator_int_ptr__,
    ) -> *mut operation_counting_iterator_bidirectional_iterator_int_ptr__ {
        return &mut {
            (*(self as *mut operation_counting_iterator_bidirectional_iterator_int_ptr__)) =
                (*o).clone();
            (*(self as *mut operation_counting_iterator_bidirectional_iterator_int_ptr__)).clone()
        };
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*(unsafe {
            bidirectional_iterator_int_ptr_::operator_deref(
                &bidirectional_iterator_int_ptr_::new_1({ self.base_ }),
            )
        }));
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut operation_counting_iterator_bidirectional_iterator_int_ptr__ {
        let mut tmp: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_5(&(*(unsafe { bidirectional_iterator_int_ptr_::operator_inc_6(&mut tmp) })))
        });
        (unsafe {
            operation_counting_iterator_bidirectional_iterator_int_ptr__::moved_by(self, 1_())
        });
        return &mut (*(self as *mut operation_counting_iterator_bidirectional_iterator_int_ptr__));
    }
    pub unsafe fn operator_dec_12(
        &mut self,
    ) -> *mut operation_counting_iterator_bidirectional_iterator_int_ptr__ {
        let mut tmp: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_5(&(*(unsafe { bidirectional_iterator_int_ptr_::operator_dec_7(&mut tmp) })))
        });
        (unsafe {
            operation_counting_iterator_bidirectional_iterator_int_ptr__::moved_by(self, -1_())
        });
        return &mut (*(self as *mut operation_counting_iterator_bidirectional_iterator_int_ptr__));
    }
    pub unsafe fn record_equality_comparison(&self) {
        if !((self.counts_).is_null()) {
            (*self.counts_).equal_cmps.prefix_inc();
        }
    }
    unsafe fn moved_by(&mut self, mut n: ()) {
        if (self.counts_).is_null() {
            return;
        }
        if ((n) > (0_())) {
            (*self.counts_).increments.prefix_inc();
        } else if ((n) < (0_())) {
            (*self.counts_).decrements.prefix_inc();
        } else {
            (*self.counts_).zero_moves.prefix_inc();
        }
    }
}
impl Default for operation_counting_iterator_bidirectional_iterator_int_ptr__ {
    fn default() -> Self {
        unsafe {
            operation_counting_iterator_bidirectional_iterator_int_ptr__ {
                base_: std::ptr::null_mut(),
                counts_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn base_14(
    it: *const operation_counting_iterator_random_access_iterator_int_ptr__,
) -> random_access_iterator_int_ptr_ {
    return random_access_iterator_int_ptr_::new_1({ (*it).base_ });
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe)]
pub struct operation_counting_iterator_random_access_iterator_int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_random_access_iterator_int_ptr__ {
    pub unsafe fn new_1(
        it: *const random_access_iterator_int_ptr_,
        mut counts: Option<*mut IteratorOpCounts>,
    ) -> Self {
        let mut counts: *mut IteratorOpCounts =
            counts.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
        let mut this = Self {
            base_: (unsafe { base_7(it) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut operation_counting_iterator_random_access_iterator_int_ptr__,
    ) -> *mut operation_counting_iterator_random_access_iterator_int_ptr__ {
        return &mut {
            (*(self as *mut operation_counting_iterator_random_access_iterator_int_ptr__)) =
                (*o).clone();
            (*(self as *mut operation_counting_iterator_random_access_iterator_int_ptr__)).clone()
        };
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*(unsafe {
            random_access_iterator_int_ptr_::operator_deref(
                &random_access_iterator_int_ptr_::new_1({ self.base_ }),
            )
        }));
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*(unsafe {
            random_access_iterator_int_ptr_::operator_index(
                &random_access_iterator_int_ptr_::new_1({ self.base_ }),
                (n as ()),
            )
        }));
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut operation_counting_iterator_random_access_iterator_int_ptr__ {
        let mut tmp: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_7(&(*(unsafe { random_access_iterator_int_ptr_::operator_inc_7(&mut tmp) })))
        });
        (unsafe {
            operation_counting_iterator_random_access_iterator_int_ptr__::moved_by(self, 1_())
        });
        return &mut (*(self as *mut operation_counting_iterator_random_access_iterator_int_ptr__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        n: (),
    ) -> *mut operation_counting_iterator_random_access_iterator_int_ptr__ {
        let mut tmp: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_7(
                &(*(unsafe {
                    random_access_iterator_int_ptr_::operator_add_assign(&mut tmp, (n as ()))
                })),
            )
        });
        (unsafe {
            operation_counting_iterator_random_access_iterator_int_ptr__::moved_by(self, n)
        });
        return &mut (*(self as *mut operation_counting_iterator_random_access_iterator_int_ptr__));
    }
    pub unsafe fn record_equality_comparison(&self) {
        if !((self.counts_).is_null()) {
            (*self.counts_).equal_cmps.prefix_inc();
        }
    }
    unsafe fn moved_by(&mut self, mut n: ()) {
        if (self.counts_).is_null() {
            return;
        }
        if ((n) > (0_())) {
            (*self.counts_).increments.prefix_inc();
        } else if ((n) < (0_())) {
            (*self.counts_).decrements.prefix_inc();
        } else {
            (*self.counts_).zero_moves.prefix_inc();
        }
    }
}
impl Default for operation_counting_iterator_random_access_iterator_int_ptr__ {
    fn default() -> Self {
        unsafe {
            operation_counting_iterator_random_access_iterator_int_ptr__ {
                base_: std::ptr::null_mut(),
                counts_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn base_15(
    it: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
) -> contiguous_iterator_int_ptr_ {
    return contiguous_iterator_int_ptr_::new_1({ (*it).base_ });
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe)]
pub struct operation_counting_iterator_contiguous_iterator_int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_contiguous_iterator_int_ptr__ {
    pub unsafe fn new_1(
        it: *const contiguous_iterator_int_ptr_,
        mut counts: Option<*mut IteratorOpCounts>,
    ) -> Self {
        let mut counts: *mut IteratorOpCounts =
            counts.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
        let mut this = Self {
            base_: (unsafe { base_9(it) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut operation_counting_iterator_contiguous_iterator_int_ptr__,
    ) -> *mut operation_counting_iterator_contiguous_iterator_int_ptr__ {
        return &mut {
            (*(self as *mut operation_counting_iterator_contiguous_iterator_int_ptr__)) =
                (*o).clone();
            (*(self as *mut operation_counting_iterator_contiguous_iterator_int_ptr__)).clone()
        };
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*(unsafe {
            contiguous_iterator_int_ptr_::operator_deref(&contiguous_iterator_int_ptr_::new_1({
                self.base_
            }))
        }));
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*(unsafe {
            contiguous_iterator_int_ptr_::operator_index(
                &contiguous_iterator_int_ptr_::new_1({ self.base_ }),
                (n as ()),
            )
        }));
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut operation_counting_iterator_contiguous_iterator_int_ptr__ {
        let mut tmp: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_9(&(*(unsafe { contiguous_iterator_int_ptr_::operator_inc_9(&mut tmp) })))
        });
        (unsafe {
            operation_counting_iterator_contiguous_iterator_int_ptr__::moved_by(self, 1_())
        });
        return &mut (*(self as *mut operation_counting_iterator_contiguous_iterator_int_ptr__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        n: (),
    ) -> *mut operation_counting_iterator_contiguous_iterator_int_ptr__ {
        let mut tmp: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_9(
                &(*(unsafe {
                    contiguous_iterator_int_ptr_::operator_add_assign(&mut tmp, (n as ()))
                })),
            )
        });
        (unsafe { operation_counting_iterator_contiguous_iterator_int_ptr__::moved_by(self, n) });
        return &mut (*(self as *mut operation_counting_iterator_contiguous_iterator_int_ptr__));
    }
    pub unsafe fn record_equality_comparison(&self) {
        if !((self.counts_).is_null()) {
            (*self.counts_).equal_cmps.prefix_inc();
        }
    }
    unsafe fn moved_by(&mut self, mut n: ()) {
        if (self.counts_).is_null() {
            return;
        }
        if ((n) > (0_())) {
            (*self.counts_).increments.prefix_inc();
        } else if ((n) < (0_())) {
            (*self.counts_).decrements.prefix_inc();
        } else {
            (*self.counts_).zero_moves.prefix_inc();
        }
    }
}
impl Default for operation_counting_iterator_contiguous_iterator_int_ptr__ {
    fn default() -> Self {
        unsafe {
            operation_counting_iterator_contiguous_iterator_int_ptr__ {
                base_: std::ptr::null_mut(),
                counts_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn base_16(it: *const operation_counting_iterator_int_ptr_) -> *mut () {
    return (*it).base_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe)]
pub struct operation_counting_iterator_int_ptr_ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_int_ptr_ {
    pub unsafe fn new_1(it: *const *mut (), mut counts: Option<*mut IteratorOpCounts>) -> Self {
        let mut counts: *mut IteratorOpCounts =
            counts.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
        let mut this = Self {
            base_: (unsafe { base_10((*it)) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut operation_counting_iterator_int_ptr_,
    ) -> *mut operation_counting_iterator_int_ptr_ {
        return &mut {
            (*(self as *mut operation_counting_iterator_int_ptr_)) = (*o).clone();
            (*(self as *mut operation_counting_iterator_int_ptr_)).clone()
        };
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.base_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.base_.offset((n) as isize));
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut operation_counting_iterator_int_ptr_ {
        let mut tmp: *mut () = self.base_;
        self.base_ = (unsafe { base_10(tmp.prefix_inc()) });
        (unsafe { operation_counting_iterator_int_ptr_::moved_by(self, 1_()) });
        return &mut (*(self as *mut operation_counting_iterator_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        n: (),
    ) -> *mut operation_counting_iterator_int_ptr_ {
        let mut tmp: *mut () = self.base_;
        self.base_ = (unsafe {
            base_10({
                tmp = (tmp).wrapping_add(n as usize);
                tmp
            })
        });
        (unsafe { operation_counting_iterator_int_ptr_::moved_by(self, n) });
        return &mut (*(self as *mut operation_counting_iterator_int_ptr_));
    }
    pub unsafe fn record_equality_comparison(&self) {
        if !((self.counts_).is_null()) {
            (*self.counts_).equal_cmps.prefix_inc();
        }
    }
    unsafe fn moved_by(&mut self, mut n: ()) {
        if (self.counts_).is_null() {
            return;
        }
        if ((n) > (0_())) {
            (*self.counts_).increments.prefix_inc();
        } else if ((n) < (0_())) {
            (*self.counts_).decrements.prefix_inc();
        } else {
            (*self.counts_).zero_moves.prefix_inc();
        }
    }
}
impl Default for operation_counting_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            operation_counting_iterator_int_ptr_ {
                base_: std::ptr::null_mut(),
                counts_: std::ptr::null_mut(),
            }
        }
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
            base_: (unsafe { base_1(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp17_input_iterator_int_ptr__int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_1(other) }));
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
            base_: (unsafe { base_11(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_11(other) }));
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
            base_: (unsafe { base_3(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_3(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_operation_counting_iterator_forward_iterator_int_ptr___ {
    base_: forward_iterator_int_ptr_,
}
impl sentinel_wrapper_operation_counting_iterator_forward_iterator_int_ptr___ {
    pub unsafe fn new_1(it: *const operation_counting_iterator_forward_iterator_int_ptr__) -> Self {
        let mut this = Self {
            base_: (unsafe { base_12(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const operation_counting_iterator_forward_iterator_int_ptr__,
    ) -> () {
        {
            (unsafe {
                operation_counting_iterator_forward_iterator_int_ptr__::record_equality_comparison(
                    &(*other),
                )
            });
        }
        return (unsafe {
            let _x: *const forward_iterator_int_ptr_ = &self.base_;
            let mut _y: forward_iterator_int_ptr_ = (unsafe { base_12(other) });
            operator_eq_2(_x, &mut _y)
        });
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
pub struct sentinel_wrapper_operation_counting_iterator_bidirectional_iterator_int_ptr___ {
    base_: bidirectional_iterator_int_ptr_,
}
impl sentinel_wrapper_operation_counting_iterator_bidirectional_iterator_int_ptr___ {
    pub unsafe fn new_1(
        it: *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_13(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
    ) -> () {
        {
            (unsafe {
                operation_counting_iterator_bidirectional_iterator_int_ptr__ :: record_equality_comparison ( & ( * other )  , )
            });
        }
        return (unsafe {
            let _x: *const bidirectional_iterator_int_ptr_ = &self.base_;
            let mut _y: bidirectional_iterator_int_ptr_ = (unsafe { base_13(other) });
            operator_eq_4(_x, &mut _y)
        });
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
            base_: (unsafe { base_7(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_7(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_operation_counting_iterator_random_access_iterator_int_ptr___ {
    base_: random_access_iterator_int_ptr_,
}
impl sentinel_wrapper_operation_counting_iterator_random_access_iterator_int_ptr___ {
    pub unsafe fn new_1(
        it: *const operation_counting_iterator_random_access_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_14(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const operation_counting_iterator_random_access_iterator_int_ptr__,
    ) -> () {
        {
            (unsafe {
                operation_counting_iterator_random_access_iterator_int_ptr__ :: record_equality_comparison ( & ( * other )  , )
            });
        }
        return (unsafe {
            let _x: *const random_access_iterator_int_ptr_ = &self.base_;
            let mut _y: random_access_iterator_int_ptr_ = (unsafe { base_14(other) });
            operator_eq_6(_x, &mut _y)
        });
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
            base_: (unsafe { base_9(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_9(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_operation_counting_iterator_contiguous_iterator_int_ptr___ {
    base_: contiguous_iterator_int_ptr_,
}
impl sentinel_wrapper_operation_counting_iterator_contiguous_iterator_int_ptr___ {
    pub unsafe fn new_1(
        it: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_15(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
    ) -> () {
        {
            (unsafe {
                operation_counting_iterator_contiguous_iterator_int_ptr__ :: record_equality_comparison ( & ( * other )  , )
            });
        }
        return (unsafe {
            let _x: *const contiguous_iterator_int_ptr_ = &self.base_;
            let mut _y: contiguous_iterator_int_ptr_ = (unsafe { base_15(other) });
            operator_eq_8(_x, &mut _y)
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_int_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_int_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_10((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_10((*other)) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_operation_counting_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_operation_counting_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const operation_counting_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_16(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const operation_counting_iterator_int_ptr_) -> () {
        {
            (unsafe {
                operation_counting_iterator_int_ptr_::record_equality_comparison(&(*other))
            });
        }
        return ((self.base_) == (unsafe { base_16(other) }));
    }
}
pub unsafe fn operator_eq_17(
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
            operator_eq_17(
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
pub unsafe fn operator_add_18(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_19(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_18(_i, n)
    });
}
pub unsafe fn iter_swap_20(mut a: adl_Iterator, mut b: adl_Iterator) {
    (unsafe {
        let ___x: *mut *mut () = &mut a.ptr_;
        let ___y: *mut *mut () = &mut b.ptr_;
        swap_21(___x, ___y)
    });
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_22(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
}
pub unsafe fn operator_eq_23(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_24(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_24(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_23(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn operator_sub_25(
    x: distance_apriori_sentinel,
    _a1: *const cpp17_input_iterator_int_ptr__int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_26(
    x: distance_apriori_sentinel,
    _a1: *const cpp20_input_iterator_int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_27(
    x: distance_apriori_sentinel,
    _a1: *const forward_iterator_int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_28(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_forward_iterator_int_ptr__,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_29(
    x: distance_apriori_sentinel,
    _a1: *const bidirectional_iterator_int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_30(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_31(
    x: distance_apriori_sentinel,
    _a1: *const random_access_iterator_int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_32(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_random_access_iterator_int_ptr__,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_33(
    x: distance_apriori_sentinel,
    _a1: *const contiguous_iterator_int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_34(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_35(x: distance_apriori_sentinel, _a1: *const *mut ()) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_36(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_int_ptr_,
) -> () {
    return x.count_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct distance_apriori_sentinel {
    count_: (),
}
impl distance_apriori_sentinel {
    pub unsafe fn new_1(count: ()) -> Self {
        let mut this = Self { count_: count };
        this
    }
}
impl Default for distance_apriori_sentinel {
    fn default() -> Self {
        unsafe { distance_apriori_sentinel { count_: 0_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ {
    base_: *mut (),
}
impl assignable_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_1(it) }),
        };
        this
    }
    pub unsafe fn to_cpp17_input_iterator_int_ptr__int_ptr_(
        &self,
    ) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        return cpp17_input_iterator_int_ptr__int_ptr_::new({ self.base_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
}
impl assignable_sentinel_cpp20_input_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_11(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_int_ptr_) -> () {
        return ((self.base_) == (unsafe { base_11(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_forward_iterator_int_ptr__ {
    base_: *mut (),
}
impl assignable_sentinel_forward_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_3(it) }),
        };
        this
    }
    pub unsafe fn to_forward_iterator_int_ptr_(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({ self.base_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_operation_counting_iterator_forward_iterator_int_ptr___ {
    base_: forward_iterator_int_ptr_,
}
impl assignable_sentinel_operation_counting_iterator_forward_iterator_int_ptr___ {
    pub unsafe fn new_1(it: *const operation_counting_iterator_forward_iterator_int_ptr__) -> Self {
        let mut this = Self {
            base_: (unsafe { base_12(it) }),
        };
        this
    }
    pub unsafe fn to_operation_counting_iterator_forward_iterator_int_ptr__(
        &self,
    ) -> operation_counting_iterator_forward_iterator_int_ptr__ {
        return operation_counting_iterator_forward_iterator_int_ptr__::new_1(
            { &self.base_ },
            None,
        );
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_bidirectional_iterator_int_ptr__ {
    base_: *mut (),
}
impl assignable_sentinel_bidirectional_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_5(it) }),
        };
        this
    }
    pub unsafe fn to_bidirectional_iterator_int_ptr_(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ self.base_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_operation_counting_iterator_bidirectional_iterator_int_ptr___ {
    base_: bidirectional_iterator_int_ptr_,
}
impl assignable_sentinel_operation_counting_iterator_bidirectional_iterator_int_ptr___ {
    pub unsafe fn new_1(
        it: *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_13(it) }),
        };
        this
    }
    pub unsafe fn to_operation_counting_iterator_bidirectional_iterator_int_ptr__(
        &self,
    ) -> operation_counting_iterator_bidirectional_iterator_int_ptr__ {
        return operation_counting_iterator_bidirectional_iterator_int_ptr__::new_1(
            { &self.base_ },
            None,
        );
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_random_access_iterator_int_ptr__ {
    base_: *mut (),
}
impl assignable_sentinel_random_access_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_7(it) }),
        };
        this
    }
    pub unsafe fn to_random_access_iterator_int_ptr_(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_1({ self.base_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_operation_counting_iterator_random_access_iterator_int_ptr___ {
    base_: random_access_iterator_int_ptr_,
}
impl assignable_sentinel_operation_counting_iterator_random_access_iterator_int_ptr___ {
    pub unsafe fn new_1(
        it: *const operation_counting_iterator_random_access_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_14(it) }),
        };
        this
    }
    pub unsafe fn to_operation_counting_iterator_random_access_iterator_int_ptr__(
        &self,
    ) -> operation_counting_iterator_random_access_iterator_int_ptr__ {
        return operation_counting_iterator_random_access_iterator_int_ptr__::new_1(
            { &self.base_ },
            None,
        );
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_contiguous_iterator_int_ptr__ {
    base_: *mut (),
}
impl assignable_sentinel_contiguous_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_9(it) }),
        };
        this
    }
    pub unsafe fn to_contiguous_iterator_int_ptr_(&self) -> contiguous_iterator_int_ptr_ {
        return contiguous_iterator_int_ptr_::new_1({ self.base_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_operation_counting_iterator_contiguous_iterator_int_ptr___ {
    base_: contiguous_iterator_int_ptr_,
}
impl assignable_sentinel_operation_counting_iterator_contiguous_iterator_int_ptr___ {
    pub unsafe fn new_1(
        it: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_15(it) }),
        };
        this
    }
    pub unsafe fn to_operation_counting_iterator_contiguous_iterator_int_ptr__(
        &self,
    ) -> operation_counting_iterator_contiguous_iterator_int_ptr__ {
        return operation_counting_iterator_contiguous_iterator_int_ptr__::new_1(
            { &self.base_ },
            None,
        );
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_int_ptr_ {
    base_: *mut (),
}
impl assignable_sentinel_int_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_10((*it)) }),
        };
        this
    }
    pub unsafe fn to_pmut(&self) -> *mut () {
        return self.base_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct assignable_sentinel_operation_counting_iterator_int_ptr__ {
    base_: *mut (),
}
impl assignable_sentinel_operation_counting_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const operation_counting_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_16(it) }),
        };
        this
    }
    pub unsafe fn to_operation_counting_iterator_int_ptr_(
        &self,
    ) -> operation_counting_iterator_int_ptr_ {
        return operation_counting_iterator_int_ptr_::new_1({ &self.base_ }, None);
    }
}
pub unsafe fn check_assignable_37(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    {
        let mut it: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ first });
        let mut sent: assignable_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_0: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ last });
            assignable_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_0 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {}
}
pub unsafe fn check_assignable_39(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    {
        let mut it: cpp20_input_iterator_int_ptr_ = cpp20_input_iterator_int_ptr_::new({ first });
        let mut sent: assignable_sentinel_cpp20_input_iterator_int_ptr__ = {
            let mut __tmp_1: cpp20_input_iterator_int_ptr_ =
                cpp20_input_iterator_int_ptr_::new({ last });
            assignable_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_1 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {}
}
pub unsafe fn check_assignable_40(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    {
        let mut it: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({ first });
        let mut sent: assignable_sentinel_forward_iterator_int_ptr__ = {
            let mut __tmp_2: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({ last });
            assignable_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_2 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_forward_iterator_int_ptr__ = {
            let mut __tmp_3: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_forward_iterator_int_ptr__::new_1({ &mut __tmp_3 }, {
                Some((&mut ops as *mut IteratorOpCounts))
            })
        };
        let mut sent: assignable_sentinel_operation_counting_iterator_forward_iterator_int_ptr___ = {
            let mut __tmp_4: operation_counting_iterator_forward_iterator_int_ptr__ = {
                let mut __tmp_5: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ last });
                operation_counting_iterator_forward_iterator_int_ptr__::new_1({ &mut __tmp_5 }, {
                    Some((&mut ops as *mut IteratorOpCounts))
                })
            };
            assignable_sentinel_operation_counting_iterator_forward_iterator_int_ptr___::new_1({
                &mut __tmp_4
            })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_assignable_41(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    {
        let mut it: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ first });
        let mut sent: assignable_sentinel_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_6: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ last });
            assignable_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_6 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_7: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_7 }, {
                Some((&mut ops as *mut IteratorOpCounts))
            })
        };
        let mut sent : assignable_sentinel_operation_counting_iterator_bidirectional_iterator_int_ptr___ = { let mut __tmp_8 : operation_counting_iterator_bidirectional_iterator_int_ptr__  = { let mut __tmp_9 : bidirectional_iterator_int_ptr_  = bidirectional_iterator_int_ptr_ :: new_1 ( { last } , )  ; operation_counting_iterator_bidirectional_iterator_int_ptr__ :: new_1 ( { & mut __tmp_9 } , { Some ( ( & mut ops as *mut IteratorOpCounts ) ) } , )  } ; assignable_sentinel_operation_counting_iterator_bidirectional_iterator_int_ptr___ :: new_1 ( { & mut __tmp_8 } , )  } ;
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_assignable_42(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    {
        let mut it: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ first });
        let mut sent: assignable_sentinel_random_access_iterator_int_ptr__ = {
            let mut __tmp_10: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ last });
            assignable_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_10 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_random_access_iterator_int_ptr__ = {
            let mut __tmp_11: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_random_access_iterator_int_ptr__::new_1(
                { &mut __tmp_11 },
                { Some((&mut ops as *mut IteratorOpCounts)) },
            )
        };
        let mut sent : assignable_sentinel_operation_counting_iterator_random_access_iterator_int_ptr___ = { let mut __tmp_12 : operation_counting_iterator_random_access_iterator_int_ptr__  = { let mut __tmp_13 : random_access_iterator_int_ptr_  = random_access_iterator_int_ptr_ :: new_1 ( { last } , )  ; operation_counting_iterator_random_access_iterator_int_ptr__ :: new_1 ( { & mut __tmp_13 } , { Some ( ( & mut ops as *mut IteratorOpCounts ) ) } , )  } ; assignable_sentinel_operation_counting_iterator_random_access_iterator_int_ptr___ :: new_1 ( { & mut __tmp_12 } , )  } ;
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_assignable_43(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    {
        let mut it: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({ first });
        let mut sent: assignable_sentinel_contiguous_iterator_int_ptr__ = {
            let mut __tmp_14: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ last });
            assignable_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_14 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_contiguous_iterator_int_ptr__ = {
            let mut __tmp_15: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_15 }, {
                Some((&mut ops as *mut IteratorOpCounts))
            })
        };
        let mut sent : assignable_sentinel_operation_counting_iterator_contiguous_iterator_int_ptr___ = { let mut __tmp_16 : operation_counting_iterator_contiguous_iterator_int_ptr__  = { let mut __tmp_17 : contiguous_iterator_int_ptr_  = contiguous_iterator_int_ptr_ :: new_1 ( { last } , )  ; operation_counting_iterator_contiguous_iterator_int_ptr__ :: new_1 ( { & mut __tmp_17 } , { Some ( ( & mut ops as *mut IteratorOpCounts ) ) } , )  } ; assignable_sentinel_operation_counting_iterator_contiguous_iterator_int_ptr___ :: new_1 ( { & mut __tmp_16 } , )  } ;
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_assignable_44(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    {
        let mut it: *mut () = first;
        let mut sent: assignable_sentinel_int_ptr_ = {
            let mut __tmp_18: *mut () = last;
            assignable_sentinel_int_ptr_::new_1({ &mut __tmp_18 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_int_ptr_ = {
            let mut __tmp_19: *mut () = first;
            operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_19 }, {
                Some((&mut ops as *mut IteratorOpCounts))
            })
        };
        let mut sent: assignable_sentinel_operation_counting_iterator_int_ptr__ = {
            let mut __tmp_20: operation_counting_iterator_int_ptr_ = {
                let mut __tmp_21: *mut () = last;
                operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_21 }, {
                    Some((&mut ops as *mut IteratorOpCounts))
                })
            };
            assignable_sentinel_operation_counting_iterator_int_ptr__::new_1({ &mut __tmp_20 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_sized_sentinel_45(
    mut first: *mut (),
    mut last: *mut (),
    mut expected: *mut (),
) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ first });
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {}
}
pub unsafe fn check_sized_sentinel_46(
    mut first: *mut (),
    mut last: *mut (),
    mut expected: *mut (),
) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: cpp20_input_iterator_int_ptr_ = cpp20_input_iterator_int_ptr_::new({ first });
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {}
}
pub unsafe fn check_sized_sentinel_47(
    mut first: *mut (),
    mut last: *mut (),
    mut expected: *mut (),
) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({ first });
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_forward_iterator_int_ptr__ = {
            let mut __tmp_22: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_forward_iterator_int_ptr__::new_1({ &mut __tmp_22 }, {
                Some((&mut ops as *mut IteratorOpCounts))
            })
        };
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        {
            let big: () = (*unimplemented!());
            let small: () = (*unimplemented!());
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_sized_sentinel_48(
    mut first: *mut (),
    mut last: *mut (),
    mut expected: *mut (),
) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ first });
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_23: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_bidirectional_iterator_int_ptr__::new_1(
                { &mut __tmp_23 },
                { Some((&mut ops as *mut IteratorOpCounts)) },
            )
        };
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        {
            let big: () = (*unimplemented!());
            let small: () = (*unimplemented!());
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_sized_sentinel_49(
    mut first: *mut (),
    mut last: *mut (),
    mut expected: *mut (),
) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ first });
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_random_access_iterator_int_ptr__ = {
            let mut __tmp_24: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_random_access_iterator_int_ptr__::new_1(
                { &mut __tmp_24 },
                { Some((&mut ops as *mut IteratorOpCounts)) },
            )
        };
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        {
            unimplemented!();
        }
    }
}
pub unsafe fn check_sized_sentinel_50(
    mut first: *mut (),
    mut last: *mut (),
    mut expected: *mut (),
) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({ first });
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_contiguous_iterator_int_ptr__ = {
            let mut __tmp_25: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_25 }, {
                Some((&mut ops as *mut IteratorOpCounts))
            })
        };
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        {
            unimplemented!();
        }
    }
}
pub unsafe fn check_sized_sentinel_51(
    mut first: *mut (),
    mut last: *mut (),
    mut expected: *mut (),
) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: *mut () = first;
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_int_ptr_ = {
            let mut __tmp_26: *mut () = first;
            operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_26 }, {
                Some((&mut ops as *mut IteratorOpCounts))
            })
        };
        let mut sent: distance_apriori_sentinel =
            distance_apriori_sentinel::new_1({ (size as ()) });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        {
            unimplemented!();
        }
    }
}
pub unsafe fn check_sentinel_52(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ first });
        let mut sent: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_27: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ last });
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_27 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {}
}
pub unsafe fn check_sentinel_53(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: cpp20_input_iterator_int_ptr_ = cpp20_input_iterator_int_ptr_::new({ first });
        let mut sent: sentinel_wrapper_cpp20_input_iterator_int_ptr__ = {
            let mut __tmp_28: cpp20_input_iterator_int_ptr_ =
                cpp20_input_iterator_int_ptr_::new({ last });
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_28 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {}
}
pub unsafe fn check_sentinel_54(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({ first });
        let mut sent: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_29: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ last });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_29 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_forward_iterator_int_ptr__ = {
            let mut __tmp_30: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_forward_iterator_int_ptr__::new_1({ &mut __tmp_30 }, {
                Some((&mut ops as *mut IteratorOpCounts))
            })
        };
        let mut sent: sentinel_wrapper_operation_counting_iterator_forward_iterator_int_ptr___ = {
            let mut __tmp_31: operation_counting_iterator_forward_iterator_int_ptr__ = {
                let mut __tmp_32: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ last });
                operation_counting_iterator_forward_iterator_int_ptr__::new_1({ &mut __tmp_32 }, {
                    Some((&mut ops as *mut IteratorOpCounts))
                })
            };
            sentinel_wrapper_operation_counting_iterator_forward_iterator_int_ptr___::new_1({
                &mut __tmp_31
            })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        let big: () = (*unimplemented!());
        let small: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_sentinel_55(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ first });
        let mut sent: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_33: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ last });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_33 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_34: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_bidirectional_iterator_int_ptr__::new_1(
                { &mut __tmp_34 },
                { Some((&mut ops as *mut IteratorOpCounts)) },
            )
        };
        let mut sent : sentinel_wrapper_operation_counting_iterator_bidirectional_iterator_int_ptr___ = { let mut __tmp_35 : operation_counting_iterator_bidirectional_iterator_int_ptr__  = { let mut __tmp_36 : bidirectional_iterator_int_ptr_  = bidirectional_iterator_int_ptr_ :: new_1 ( { last } , )  ; operation_counting_iterator_bidirectional_iterator_int_ptr__ :: new_1 ( { & mut __tmp_36 } , { Some ( ( & mut ops as *mut IteratorOpCounts ) ) } , )  } ; sentinel_wrapper_operation_counting_iterator_bidirectional_iterator_int_ptr___ :: new_1 ( { & mut __tmp_35 } , )  } ;
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        let big: () = (*unimplemented!());
        let small: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_sentinel_56(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ first });
        let mut sent: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_37: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ last });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_37 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_random_access_iterator_int_ptr__ = {
            let mut __tmp_38: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_random_access_iterator_int_ptr__::new_1(
                { &mut __tmp_38 },
                { Some((&mut ops as *mut IteratorOpCounts)) },
            )
        };
        let mut sent : sentinel_wrapper_operation_counting_iterator_random_access_iterator_int_ptr___ = { let mut __tmp_39 : operation_counting_iterator_random_access_iterator_int_ptr__  = { let mut __tmp_40 : random_access_iterator_int_ptr_  = random_access_iterator_int_ptr_ :: new_1 ( { last } , )  ; operation_counting_iterator_random_access_iterator_int_ptr__ :: new_1 ( { & mut __tmp_40 } , { Some ( ( & mut ops as *mut IteratorOpCounts ) ) } , )  } ; sentinel_wrapper_operation_counting_iterator_random_access_iterator_int_ptr___ :: new_1 ( { & mut __tmp_39 } , )  } ;
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        let big: () = (*unimplemented!());
        let small: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_sentinel_57(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({ first });
        let mut sent: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_41: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ last });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_41 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_contiguous_iterator_int_ptr__ = {
            let mut __tmp_42: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_42 }, {
                Some((&mut ops as *mut IteratorOpCounts))
            })
        };
        let mut sent: sentinel_wrapper_operation_counting_iterator_contiguous_iterator_int_ptr___ = {
            let mut __tmp_43: operation_counting_iterator_contiguous_iterator_int_ptr__ = {
                let mut __tmp_44: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ last });
                operation_counting_iterator_contiguous_iterator_int_ptr__::new_1(
                    { &mut __tmp_44 },
                    { Some((&mut ops as *mut IteratorOpCounts)) },
                )
            };
            sentinel_wrapper_operation_counting_iterator_contiguous_iterator_int_ptr___::new_1({
                &mut __tmp_43
            })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        let big: () = (*unimplemented!());
        let small: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_sentinel_58(mut first: *mut (), mut last: *mut (), mut expected: *mut ()) {
    let mut size: () = (((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ());
    {
        let mut it: *mut () = first;
        let mut sent: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_45: *mut () = last;
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_45 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_int_ptr_ = {
            let mut __tmp_46: *mut () = first;
            operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_46 }, {
                Some((&mut ops as *mut IteratorOpCounts))
            })
        };
        let mut sent: sentinel_wrapper_operation_counting_iterator_int_ptr__ = {
            let mut __tmp_47: operation_counting_iterator_int_ptr_ = {
                let mut __tmp_48: *mut () = last;
                operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_48 }, {
                    Some((&mut ops as *mut IteratorOpCounts))
                })
            };
            sentinel_wrapper_operation_counting_iterator_int_ptr__::new_1({ &mut __tmp_47 })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_38))(&mut it, sent) });
        let big: () = (*unimplemented!());
        let small: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_59() -> () {
    let mut range: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
    let mut n: () = 0_();
    'loop_: while ((n) != (10_())) {
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_assignable_37(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_assignable_39(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_assignable_40(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_assignable_41(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_assignable_42(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_assignable_43(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_assignable_44(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sized_sentinel_45(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sized_sentinel_46(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sized_sentinel_47(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sized_sentinel_48(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sized_sentinel_49(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sized_sentinel_50(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sized_sentinel_51(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sentinel_52(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sentinel_53(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sentinel_54(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sentinel_55(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sentinel_56(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sentinel_57(_first, _last, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _last: *mut () = range.as_mut_ptr().offset((n) as isize);
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_sentinel_58(_first, _last, _expected)
        });
        n.prefix_inc();
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
    unimplemented!();
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
