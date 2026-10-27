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
pub unsafe fn base_1(i: *const cpp17_output_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
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
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
pub unsafe fn base_2(i: *const cpp17_input_iterator_int_ptr__int_ptr_) -> *mut () {
    return (*i).it_;
}
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
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn base_3(i: *const forward_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_int_ptr_));
    }
}
pub unsafe fn base_4(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
    pub unsafe fn operator_dec_7(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
}
pub unsafe fn base_5(i: *const random_access_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
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
    pub unsafe fn operator_index(&self, mut n: ()) -> () {
        return &mut (*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
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
pub unsafe fn base_6(i: *const contiguous_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
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
    pub unsafe fn operator_index(&self, mut n: ()) -> () {
        return &mut (*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut contiguous_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
}
pub unsafe fn base_7(mut i: *mut ()) -> *mut () {
    return i;
}
pub unsafe fn base_8(i: *const cpp20_input_iterator_int_ptr_) -> *mut () {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct iter_value_or_void_cpp17_output_iterator_int_ptr__ {}
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
#[derive(FnPtrArg)]
pub struct operation_counting_iterator_forward_iterator_int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_forward_iterator_int_ptr__ {
    pub unsafe fn new_1(
        it: *const forward_iterator_int_ptr_,
        mut counts: *mut IteratorOpCounts,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_3(it) }),
            counts_: counts,
        };
        this
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
#[repr(C)]
#[derive(FnPtrArg)]
pub struct operation_counting_iterator_bidirectional_iterator_int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_bidirectional_iterator_int_ptr__ {
    pub unsafe fn new_1(
        it: *const bidirectional_iterator_int_ptr_,
        mut counts: *mut IteratorOpCounts,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_4(it) }),
            counts_: counts,
        };
        this
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
            base_4(&(*(unsafe { bidirectional_iterator_int_ptr_::operator_inc_6(&mut tmp) })))
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
            base_4(&(*(unsafe { bidirectional_iterator_int_ptr_::operator_dec_7(&mut tmp) })))
        });
        (unsafe {
            operation_counting_iterator_bidirectional_iterator_int_ptr__::moved_by(self, -1_())
        });
        return &mut (*(self as *mut operation_counting_iterator_bidirectional_iterator_int_ptr__));
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
#[repr(C)]
#[derive(FnPtrArg)]
pub struct operation_counting_iterator_random_access_iterator_int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_random_access_iterator_int_ptr__ {
    pub unsafe fn new_1(
        it: *const random_access_iterator_int_ptr_,
        mut counts: *mut IteratorOpCounts,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_5(it) }),
            counts_: counts,
        };
        this
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
                n,
            )
        }));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        n: (),
    ) -> *mut operation_counting_iterator_random_access_iterator_int_ptr__ {
        let mut tmp: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_5(
                &(*(unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut tmp, n) })),
            )
        });
        (unsafe {
            operation_counting_iterator_random_access_iterator_int_ptr__::moved_by(self, n)
        });
        return &mut (*(self as *mut operation_counting_iterator_random_access_iterator_int_ptr__));
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
#[repr(C)]
#[derive(FnPtrArg)]
pub struct operation_counting_iterator_contiguous_iterator_int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_contiguous_iterator_int_ptr__ {
    pub unsafe fn new_1(
        it: *const contiguous_iterator_int_ptr_,
        mut counts: *mut IteratorOpCounts,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_6(it) }),
            counts_: counts,
        };
        this
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
                n,
            )
        }));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        n: (),
    ) -> *mut operation_counting_iterator_contiguous_iterator_int_ptr__ {
        let mut tmp: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_6(&(*(unsafe { contiguous_iterator_int_ptr_::operator_add_assign(&mut tmp, n) })))
        });
        (unsafe { operation_counting_iterator_contiguous_iterator_int_ptr__::moved_by(self, n) });
        return &mut (*(self as *mut operation_counting_iterator_contiguous_iterator_int_ptr__));
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
#[repr(C)]
#[derive(FnPtrArg)]
pub struct operation_counting_iterator_int_ptr_ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_int_ptr_ {
    pub unsafe fn new_1(it: *const *mut (), mut counts: *mut IteratorOpCounts) -> Self {
        let mut this = Self {
            base_: (unsafe { base_7((*it)) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.base_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.base_.offset((n) as isize));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        n: (),
    ) -> *mut operation_counting_iterator_int_ptr_ {
        let mut tmp: *mut () = self.base_;
        self.base_ = (unsafe {
            base_7({
                tmp = (tmp).wrapping_add(n as usize);
                tmp
            })
        });
        (unsafe { operation_counting_iterator_int_ptr_::moved_by(self, n) });
        return &mut (*(self as *mut operation_counting_iterator_int_ptr_));
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
#[derive(FnPtrArg)]
pub struct operation_counting_iterator_cpp17_output_iterator_int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_cpp17_output_iterator_int_ptr__ {
    pub unsafe fn new_1(
        it: *const cpp17_output_iterator_int_ptr_,
        mut counts: *mut IteratorOpCounts,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_1(it) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*(unsafe {
            cpp17_output_iterator_int_ptr_::operator_deref(&cpp17_output_iterator_int_ptr_::new({
                self.base_
            }))
        }));
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut operation_counting_iterator_cpp17_output_iterator_int_ptr__ {
        let mut tmp: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ self.base_ });
        self.base_ = (unsafe {
            base_1(&(*(unsafe { cpp17_output_iterator_int_ptr_::operator_inc_5(&mut tmp) })))
        });
        (unsafe {
            operation_counting_iterator_cpp17_output_iterator_int_ptr__::moved_by(self, 1_())
        });
        return &mut (*(self as *mut operation_counting_iterator_cpp17_output_iterator_int_ptr__));
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
impl Default for operation_counting_iterator_cpp17_output_iterator_int_ptr__ {
    fn default() -> Self {
        unsafe {
            operation_counting_iterator_cpp17_output_iterator_int_ptr__ {
                base_: std::ptr::null_mut(),
                counts_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_9(
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
            operator_eq_9(
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
pub unsafe fn operator_add_10(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_11(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_10(_i, n)
    });
}
pub unsafe fn iter_swap_12(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_13(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_14(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_15(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_15(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_14(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn check_16(mut first: *mut (), mut n: (), mut expected: *mut ()) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let mut abs: FnPtr<Generic> = lambda_unsafe!(Generic, {});
    {
        let mut it: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ first });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {}
}
pub unsafe fn check_18(mut first: *mut (), mut n: (), mut expected: *mut ()) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let mut abs: FnPtr<Generic> = lambda_unsafe!(Generic, {});
    {
        let mut it: cpp20_input_iterator_int_ptr_ = cpp20_input_iterator_int_ptr_::new({ first });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {}
}
pub unsafe fn check_19(mut first: *mut (), mut n: (), mut expected: *mut ()) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let mut abs: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: ()| -> () {
        return if ((x) < (0_())) { -x } else { x };
    });
    {
        let mut it: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({ first });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_forward_iterator_int_ptr__ = {
            let mut __tmp_0: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_forward_iterator_int_ptr__::new_1({ &mut __tmp_0 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        {
            let big: () = (*unimplemented!());
            let small: () = (*unimplemented!());
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_20(mut first: *mut (), mut n: (), mut expected: *mut ()) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let mut abs: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: ()| -> () {
        return if ((x) < (0_())) { -x } else { x };
    });
    {
        let mut it: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ first });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_1: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_1 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        {
            let big: () = (*unimplemented!());
            let small: () = (*unimplemented!());
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_21(mut first: *mut (), mut n: (), mut expected: *mut ()) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let mut abs: FnPtr<Generic> = lambda_unsafe!(Generic, {});
    {
        let mut it: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ first });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_random_access_iterator_int_ptr__ = {
            let mut __tmp_2: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_random_access_iterator_int_ptr__::new_1({ &mut __tmp_2 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        {
            unimplemented!();
        }
    }
}
pub unsafe fn check_22(mut first: *mut (), mut n: (), mut expected: *mut ()) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let mut abs: FnPtr<Generic> = lambda_unsafe!(Generic, {});
    {
        let mut it: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({ first });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_contiguous_iterator_int_ptr__ = {
            let mut __tmp_3: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_3 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        {
            unimplemented!();
        }
    }
}
pub unsafe fn check_23(mut first: *mut (), mut n: (), mut expected: *mut ()) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let mut abs: FnPtr<Generic> = lambda_unsafe!(Generic, {});
    {
        let mut it: *mut () = first;
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_int_ptr_ = {
            let mut __tmp_4: *mut () = first;
            operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_4 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        {
            unimplemented!();
        }
    }
}
pub unsafe fn check_24(mut first: *mut (), mut n: (), mut expected: *mut ()) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let mut abs: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: ()| -> () {
        return if ((x) < (0_())) { -x } else { x };
    });
    {
        let mut it: cpp17_output_iterator_int_ptr_ = cpp17_output_iterator_int_ptr_::new({ first });
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_cpp17_output_iterator_int_ptr__ = {
            let mut __tmp_5: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ first });
            operation_counting_iterator_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_5 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_17))(&mut it, n) });
        {
            let big: () = (*unimplemented!());
            let small: () = (*unimplemented!());
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_25() -> () {
    let mut range: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
    let mut n: () = 0_();
    'loop_: while ((n) != (10_())) {
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _n: () = n;
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_16(_first, _n, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _n: () = n;
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_18(_first, _n, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _n: () = n;
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_19(_first, _n, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _n: () = n;
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_20(_first, _n, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _n: () = n;
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_21(_first, _n, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _n: () = n;
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_22(_first, _n, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _n: () = n;
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_23(_first, _n, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr();
            let _n: () = n;
            let _expected: *mut () = range.as_mut_ptr().offset((n) as isize);
            check_24(_first, _n, _expected)
        });
        n.prefix_inc();
    }
    let mut n: () = 0_();
    'loop_: while ((n) != (10_())) {
        (unsafe {
            let _first: *mut () = range.as_mut_ptr().offset((9_()) as isize);
            let _n: () = -n;
            let _expected: *mut () = range
                .as_mut_ptr()
                .offset((9_()) as isize)
                .offset(-((n) as isize));
            check_20(_first, _n, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr().offset((9_()) as isize);
            let _n: () = -n;
            let _expected: *mut () = range
                .as_mut_ptr()
                .offset((9_()) as isize)
                .offset(-((n) as isize));
            check_21(_first, _n, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr().offset((9_()) as isize);
            let _n: () = -n;
            let _expected: *mut () = range
                .as_mut_ptr()
                .offset((9_()) as isize)
                .offset(-((n) as isize));
            check_22(_first, _n, _expected)
        });
        (unsafe {
            let _first: *mut () = range.as_mut_ptr().offset((9_()) as isize);
            let _n: () = -n;
            let _expected: *mut () = range
                .as_mut_ptr()
                .offset((9_()) as isize)
                .offset(-((n) as isize));
            check_23(_first, _n, _expected)
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
