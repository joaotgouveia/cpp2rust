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
pub unsafe fn base_0(i: *const cpp17_input_iterator_int_ptr__int_ptr_) -> *mut () {
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
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_1(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn base_2(i: *const forward_iterator_int_ptr_) -> *mut () {
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
impl std::cmp::PartialEq for forward_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_1(
                self as *const forward_iterator_int_ptr_,
                other as *const forward_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_int_ptr_ {}
pub unsafe fn operator_eq_3(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_4(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_5(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
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
impl std::cmp::PartialEq for bidirectional_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_3(
                self as *const bidirectional_iterator_int_ptr_,
                other as *const bidirectional_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
pub unsafe fn operator_sub_6(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_7(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn base_8(i: *const random_access_iterator_int_ptr_) -> *mut () {
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
            operator_eq_7(
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
pub unsafe fn operator_sub_9(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_10(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn base_11(i: *const contiguous_iterator_int_ptr_) -> *mut () {
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
            operator_eq_10(
                self as *const contiguous_iterator_int_ptr_,
                other as *const contiguous_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_int_ptr_ {}
pub unsafe fn base_12(mut i: *mut ()) -> *mut () {
    return i;
}
pub unsafe fn base_13(i: *const cpp20_input_iterator_int_ptr_) -> *mut () {
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
pub struct iter_value_or_void_cpp17_input_iterator_int_ptr__int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct iter_value_or_void_cpp20_input_iterator_int_ptr__ {}
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
pub unsafe fn base_14(
    it: *const operation_counting_iterator_forward_iterator_int_ptr__,
) -> forward_iterator_int_ptr_ {
    return forward_iterator_int_ptr_::new_1({ (*it).base_ });
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
            base_: (unsafe { base_2(it) }),
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
            base_2(&(*(unsafe { forward_iterator_int_ptr_::operator_inc_6(&mut tmp) })))
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
pub unsafe fn base_15(
    it: *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (*it).base_ });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
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
            base_: (unsafe { base_5(it) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn copy_from(
        o: *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            base_: std::ptr::null_mut(),
            counts_: std::ptr::null_mut(),
        };
        (unsafe {
            let _o: *const operation_counting_iterator_bidirectional_iterator_int_ptr__ = o;
            operation_counting_iterator_bidirectional_iterator_int_ptr__::copy_assign(
                &mut (*&raw mut this),
                _o,
            )
        });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        o: *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
    ) -> *mut operation_counting_iterator_bidirectional_iterator_int_ptr__ {
        self.base_ = (*o).base_;
        self.counts_ = (*o).counts_;
        return &mut (*(self as *mut operation_counting_iterator_bidirectional_iterator_int_ptr__));
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
    pub unsafe fn operator_eq(
        &self,
        other: *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
    ) -> () {
        (unsafe {
            operation_counting_iterator_bidirectional_iterator_int_ptr__::record_equality_comparison(
                self,
            )
        });
        return (unsafe {
            let mut _x: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ self.base_ });
            let mut _y: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (*other).base_ });
            operator_eq_3(&mut _x, &mut _y)
        });
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
impl std::cmp::PartialEq for operation_counting_iterator_bidirectional_iterator_int_ptr__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operation_counting_iterator_bidirectional_iterator_int_ptr__::operator_eq(
                self,
                other as *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
            )
        }
    }
}
impl std::cmp::Eq for operation_counting_iterator_bidirectional_iterator_int_ptr__ {}
impl Clone for operation_counting_iterator_bidirectional_iterator_int_ptr__ {
    fn clone(&self) -> Self {
        unsafe {
            operation_counting_iterator_bidirectional_iterator_int_ptr__::copy_from(
                self as *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
            )
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
pub unsafe fn base_16(
    it: *const operation_counting_iterator_random_access_iterator_int_ptr__,
) -> random_access_iterator_int_ptr_ {
    return random_access_iterator_int_ptr_::new_1({ (*it).base_ });
}
pub unsafe fn operator_sub_17(
    x: *const operation_counting_iterator_random_access_iterator_int_ptr__,
    y: *const operation_counting_iterator_random_access_iterator_int_ptr__,
) -> () {
    return (unsafe {
        let _x: random_access_iterator_int_ptr_ = (unsafe { base_16(x) });
        let _y: random_access_iterator_int_ptr_ = (unsafe { base_16(y) });
        operator_sub_6(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe)]
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
            base_: (unsafe { base_8(it) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn copy_from(
        o: *const operation_counting_iterator_random_access_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            base_: std::ptr::null_mut(),
            counts_: std::ptr::null_mut(),
        };
        (unsafe {
            let _o: *const operation_counting_iterator_random_access_iterator_int_ptr__ = o;
            operation_counting_iterator_random_access_iterator_int_ptr__::copy_assign(
                &mut (*&raw mut this),
                _o,
            )
        });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        o: *const operation_counting_iterator_random_access_iterator_int_ptr__,
    ) -> *mut operation_counting_iterator_random_access_iterator_int_ptr__ {
        self.base_ = (*o).base_;
        self.counts_ = (*o).counts_;
        return &mut (*(self as *mut operation_counting_iterator_random_access_iterator_int_ptr__));
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut operation_counting_iterator_random_access_iterator_int_ptr__,
    ) -> *mut operation_counting_iterator_random_access_iterator_int_ptr__ {
        return &mut (*(unsafe {
            let _o: *const operation_counting_iterator_random_access_iterator_int_ptr__ = o;
            operation_counting_iterator_random_access_iterator_int_ptr__::copy_assign(
                &mut (*(self as *mut operation_counting_iterator_random_access_iterator_int_ptr__)),
                _o,
            )
        }));
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
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut operation_counting_iterator_random_access_iterator_int_ptr__ {
        let mut tmp: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_8(&(*(unsafe { random_access_iterator_int_ptr_::operator_inc_7(&mut tmp) })))
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
            base_8(
                &(*(unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut tmp, n) })),
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
impl Clone for operation_counting_iterator_random_access_iterator_int_ptr__ {
    fn clone(&self) -> Self {
        unsafe {
            operation_counting_iterator_random_access_iterator_int_ptr__::copy_from(
                self as *const operation_counting_iterator_random_access_iterator_int_ptr__,
            )
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
pub unsafe fn base_18(
    it: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
) -> contiguous_iterator_int_ptr_ {
    return contiguous_iterator_int_ptr_::new_1({ (*it).base_ });
}
pub unsafe fn operator_sub_19(
    x: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
    y: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
) -> () {
    return (unsafe {
        let _x: contiguous_iterator_int_ptr_ = (unsafe { base_18(x) });
        let _y: contiguous_iterator_int_ptr_ = (unsafe { base_18(y) });
        operator_sub_9(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe)]
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
            base_: (unsafe { base_11(it) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn copy_from(
        o: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            base_: std::ptr::null_mut(),
            counts_: std::ptr::null_mut(),
        };
        (unsafe {
            let _o: *const operation_counting_iterator_contiguous_iterator_int_ptr__ = o;
            operation_counting_iterator_contiguous_iterator_int_ptr__::copy_assign(
                &mut (*&raw mut this),
                _o,
            )
        });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        o: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
    ) -> *mut operation_counting_iterator_contiguous_iterator_int_ptr__ {
        self.base_ = (*o).base_;
        self.counts_ = (*o).counts_;
        return &mut (*(self as *mut operation_counting_iterator_contiguous_iterator_int_ptr__));
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut operation_counting_iterator_contiguous_iterator_int_ptr__,
    ) -> *mut operation_counting_iterator_contiguous_iterator_int_ptr__ {
        return &mut (*(unsafe {
            let _o: *const operation_counting_iterator_contiguous_iterator_int_ptr__ = o;
            operation_counting_iterator_contiguous_iterator_int_ptr__::copy_assign(
                &mut (*(self as *mut operation_counting_iterator_contiguous_iterator_int_ptr__)),
                _o,
            )
        }));
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
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut operation_counting_iterator_contiguous_iterator_int_ptr__ {
        let mut tmp: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ self.base_ });
        self.base_ = (unsafe {
            base_11(&(*(unsafe { contiguous_iterator_int_ptr_::operator_inc_9(&mut tmp) })))
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
            base_11(&(*(unsafe { contiguous_iterator_int_ptr_::operator_add_assign(&mut tmp, n) })))
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
impl Clone for operation_counting_iterator_contiguous_iterator_int_ptr__ {
    fn clone(&self) -> Self {
        unsafe {
            operation_counting_iterator_contiguous_iterator_int_ptr__::copy_from(
                self as *const operation_counting_iterator_contiguous_iterator_int_ptr__,
            )
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
pub unsafe fn base_20(it: *const operation_counting_iterator_int_ptr_) -> *mut () {
    return (*it).base_;
}
pub unsafe fn operator_sub_21(
    x: *const operation_counting_iterator_int_ptr_,
    y: *const operation_counting_iterator_int_ptr_,
) -> () {
    return ((((unsafe { base_20(x) }) as usize - (unsafe { base_20(y) }) as usize)
        / ::std::mem::size_of::<()>()) as () as ());
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct operation_counting_iterator_int_ptr_ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_int_ptr_ {
    pub unsafe fn new_1(it: *const *mut (), mut counts: *mut IteratorOpCounts) -> Self {
        let mut this = Self {
            base_: (unsafe { base_12((*it)) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn copy_from(o: *const operation_counting_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: std::ptr::null_mut(),
            counts_: std::ptr::null_mut(),
        };
        (unsafe {
            let _o: *const operation_counting_iterator_int_ptr_ = o;
            operation_counting_iterator_int_ptr_::copy_assign(&mut (*&raw mut this), _o)
        });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        o: *const operation_counting_iterator_int_ptr_,
    ) -> *mut operation_counting_iterator_int_ptr_ {
        self.base_ = (*o).base_;
        self.counts_ = (*o).counts_;
        return &mut (*(self as *mut operation_counting_iterator_int_ptr_));
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut operation_counting_iterator_int_ptr_,
    ) -> *mut operation_counting_iterator_int_ptr_ {
        return &mut (*(unsafe {
            let _o: *const operation_counting_iterator_int_ptr_ = o;
            operation_counting_iterator_int_ptr_::copy_assign(
                &mut (*(self as *mut operation_counting_iterator_int_ptr_)),
                _o,
            )
        }));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.base_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.base_.offset((n) as isize));
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut operation_counting_iterator_int_ptr_ {
        let mut tmp: *mut () = self.base_;
        self.base_ = (unsafe { base_12(tmp.prefix_inc()) });
        (unsafe { operation_counting_iterator_int_ptr_::moved_by(self, 1_()) });
        return &mut (*(self as *mut operation_counting_iterator_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        n: (),
    ) -> *mut operation_counting_iterator_int_ptr_ {
        let mut tmp: *mut () = self.base_;
        self.base_ = (unsafe {
            base_12({
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
impl Clone for operation_counting_iterator_int_ptr_ {
    fn clone(&self) -> Self {
        unsafe {
            operation_counting_iterator_int_ptr_::copy_from(
                self as *const operation_counting_iterator_int_ptr_,
            )
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
pub struct operation_counting_iterator_cpp17_input_iterator_int_ptr__int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_cpp17_input_iterator_int_ptr__int_ptr__ {
    pub unsafe fn new_1(
        it: *const cpp17_input_iterator_int_ptr__int_ptr_,
        mut counts: *mut IteratorOpCounts,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_0(it) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*(unsafe {
            cpp17_input_iterator_int_ptr__int_ptr_::operator_deref(
                &cpp17_input_iterator_int_ptr__int_ptr_::new({ self.base_ }),
            )
        }));
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut operation_counting_iterator_cpp17_input_iterator_int_ptr__int_ptr__ {
        let mut tmp: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ self.base_ });
        self.base_ = (unsafe {
            base_0(
                &(*(unsafe { cpp17_input_iterator_int_ptr__int_ptr_::operator_inc_5(&mut tmp) })),
            )
        });
        (unsafe {
            operation_counting_iterator_cpp17_input_iterator_int_ptr__int_ptr__::moved_by(
                self,
                1_(),
            )
        });
        return &mut (*(self
            as *mut operation_counting_iterator_cpp17_input_iterator_int_ptr__int_ptr__));
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
impl Default for operation_counting_iterator_cpp17_input_iterator_int_ptr__int_ptr__ {
    fn default() -> Self {
        unsafe {
            operation_counting_iterator_cpp17_input_iterator_int_ptr__int_ptr__ {
                base_: std::ptr::null_mut(),
                counts_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct operation_counting_iterator_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
    counts_: *mut IteratorOpCounts,
}
impl operation_counting_iterator_cpp20_input_iterator_int_ptr__ {
    pub unsafe fn new_1(
        it: *const cpp20_input_iterator_int_ptr_,
        mut counts: *mut IteratorOpCounts,
    ) -> Self {
        let mut this = Self {
            base_: (unsafe { base_13(it) }),
            counts_: counts,
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*(unsafe {
            cpp20_input_iterator_int_ptr_::operator_deref(&cpp20_input_iterator_int_ptr_::new({
                self.base_
            }))
        }));
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut operation_counting_iterator_cpp20_input_iterator_int_ptr__ {
        let mut tmp: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ self.base_ });
        self.base_ = (unsafe {
            base_13(&(*(unsafe { cpp20_input_iterator_int_ptr_::operator_inc_5(&mut tmp) })))
        });
        (unsafe {
            operation_counting_iterator_cpp20_input_iterator_int_ptr__::moved_by(self, 1_())
        });
        return &mut (*(self as *mut operation_counting_iterator_cpp20_input_iterator_int_ptr__));
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
impl Default for operation_counting_iterator_cpp20_input_iterator_int_ptr__ {
    fn default() -> Self {
        unsafe {
            operation_counting_iterator_cpp20_input_iterator_int_ptr__ {
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
            base_: (unsafe { base_0(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp17_input_iterator_int_ptr__int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_0(other) }));
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
            base_: (unsafe { base_13(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_13(other) }));
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
            base_: (unsafe { base_2(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_2(other) }));
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
            base_: (unsafe { base_14(it) }),
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
            let mut _y: forward_iterator_int_ptr_ = (unsafe { base_14(other) });
            operator_eq_1(_x, &mut _y)
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
            base_: (unsafe { base_15(it) }),
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
            let mut _y: bidirectional_iterator_int_ptr_ = (unsafe { base_15(other) });
            operator_eq_3(_x, &mut _y)
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
            base_: (unsafe { base_8(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_8(other) }));
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
            base_: (unsafe { base_16(it) }),
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
            let mut _y: random_access_iterator_int_ptr_ = (unsafe { base_16(other) });
            operator_eq_7(_x, &mut _y)
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
            base_: (unsafe { base_11(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_11(other) }));
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
            base_: (unsafe { base_18(it) }),
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
            let mut _y: contiguous_iterator_int_ptr_ = (unsafe { base_18(other) });
            operator_eq_10(_x, &mut _y)
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
            base_: (unsafe { base_12((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_12((*other)) }));
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
            base_: (unsafe { base_20(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const operation_counting_iterator_int_ptr_) -> () {
        {
            (unsafe {
                operation_counting_iterator_int_ptr_::record_equality_comparison(&(*other))
            });
        }
        return ((self.base_) == (unsafe { base_20(other) }));
    }
}
pub unsafe fn operator_eq_22(
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
            operator_eq_22(
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
pub unsafe fn operator_add_23(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_24(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_23(_i, n)
    });
}
pub unsafe fn iter_swap_25(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_26(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_27(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_28(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_28(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_27(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn operator_sub_29(
    x: distance_apriori_sentinel,
    _a1: *const cpp17_input_iterator_int_ptr__int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_30(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_cpp17_input_iterator_int_ptr__int_ptr__,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_31(
    x: distance_apriori_sentinel,
    _a1: *const cpp20_input_iterator_int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_32(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_cpp20_input_iterator_int_ptr__,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_33(
    x: distance_apriori_sentinel,
    _a1: *const forward_iterator_int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_34(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_forward_iterator_int_ptr__,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_35(
    x: distance_apriori_sentinel,
    _a1: *const bidirectional_iterator_int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_36(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_bidirectional_iterator_int_ptr__,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_37(
    x: distance_apriori_sentinel,
    _a1: *const random_access_iterator_int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_38(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_random_access_iterator_int_ptr__,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_39(
    x: distance_apriori_sentinel,
    _a1: *const contiguous_iterator_int_ptr_,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_40(
    x: distance_apriori_sentinel,
    _a1: *const operation_counting_iterator_contiguous_iterator_int_ptr__,
) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_41(x: distance_apriori_sentinel, _a1: *const *mut ()) -> () {
    return x.count_;
}
pub unsafe fn operator_sub_42(
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
pub unsafe fn check_forward_43(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_equals_count: Option<()>,
) {
    let mut expected_equals_count: () = expected_equals_count.unwrap_or_else(|| unsafe { -1_() });
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    unimplemented!();
    {
        let mut it: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ first });
        let mut sent: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_0: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ last });
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_0 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn check_forward_45(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_equals_count: Option<()>,
) {
    let mut expected_equals_count: () = expected_equals_count.unwrap_or_else(|| unsafe { -1_() });
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    unimplemented!();
    {
        let mut it: cpp20_input_iterator_int_ptr_ = cpp20_input_iterator_int_ptr_::new({ first });
        let mut sent: sentinel_wrapper_cpp20_input_iterator_int_ptr__ = {
            let mut __tmp_1: cpp20_input_iterator_int_ptr_ =
                cpp20_input_iterator_int_ptr_::new({ last });
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_1 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn check_forward_46(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_equals_count: (),
) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    unimplemented!();
    {
        let mut it: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({ first });
        let mut sent: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_2: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({ last });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_2 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_forward_iterator_int_ptr__ = {
            let mut __tmp_3: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_forward_iterator_int_ptr__::new_1({ &mut __tmp_3 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        let mut sent: sentinel_wrapper_operation_counting_iterator_forward_iterator_int_ptr___ = {
            let mut __tmp_4: operation_counting_iterator_forward_iterator_int_ptr__ = {
                let mut __tmp_5: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ last });
                operation_counting_iterator_forward_iterator_int_ptr__::new_1({ &mut __tmp_5 }, {
                    (&mut ops as *mut IteratorOpCounts)
                })
            };
            sentinel_wrapper_operation_counting_iterator_forward_iterator_int_ptr___::new_1({
                &mut __tmp_4
            })
        };
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_forward_47(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_equals_count: (),
) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    unimplemented!();
    {
        let mut it: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ first });
        let mut sent: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_6: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ last });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_6 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_7: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_7 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        let mut sent : sentinel_wrapper_operation_counting_iterator_bidirectional_iterator_int_ptr___ = { let mut __tmp_8 : operation_counting_iterator_bidirectional_iterator_int_ptr__  = { let mut __tmp_9 : bidirectional_iterator_int_ptr_  = bidirectional_iterator_int_ptr_ :: new_1 ( { last } , )  ; operation_counting_iterator_bidirectional_iterator_int_ptr__ :: new_1 ( { & mut __tmp_9 } , { ( & mut ops as *mut IteratorOpCounts ) } , )  } ; sentinel_wrapper_operation_counting_iterator_bidirectional_iterator_int_ptr___ :: new_1 ( { & mut __tmp_8 } , )  } ;
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_forward_48(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_equals_count: (),
) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    unimplemented!();
    {
        let mut it: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ first });
        let mut sent: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_10: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ last });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_10 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_random_access_iterator_int_ptr__ = {
            let mut __tmp_11: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_random_access_iterator_int_ptr__::new_1(
                { &mut __tmp_11 },
                { (&mut ops as *mut IteratorOpCounts) },
            )
        };
        let mut sent : sentinel_wrapper_operation_counting_iterator_random_access_iterator_int_ptr___ = { let mut __tmp_12 : operation_counting_iterator_random_access_iterator_int_ptr__  = { let mut __tmp_13 : random_access_iterator_int_ptr_  = random_access_iterator_int_ptr_ :: new_1 ( { last } , )  ; operation_counting_iterator_random_access_iterator_int_ptr__ :: new_1 ( { & mut __tmp_13 } , { ( & mut ops as *mut IteratorOpCounts ) } , )  } ; sentinel_wrapper_operation_counting_iterator_random_access_iterator_int_ptr___ :: new_1 ( { & mut __tmp_12 } , )  } ;
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_forward_49(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_equals_count: (),
) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    unimplemented!();
    {
        let mut it: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({ first });
        let mut sent: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_14: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ last });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_14 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_contiguous_iterator_int_ptr__ = {
            let mut __tmp_15: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_15 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        let mut sent: sentinel_wrapper_operation_counting_iterator_contiguous_iterator_int_ptr___ = {
            let mut __tmp_16: operation_counting_iterator_contiguous_iterator_int_ptr__ = {
                let mut __tmp_17: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ last });
                operation_counting_iterator_contiguous_iterator_int_ptr__::new_1(
                    { &mut __tmp_17 },
                    { (&mut ops as *mut IteratorOpCounts) },
                )
            };
            sentinel_wrapper_operation_counting_iterator_contiguous_iterator_int_ptr___::new_1({
                &mut __tmp_16
            })
        };
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_forward_50(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_equals_count: (),
) {
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    unimplemented!();
    {
        let mut it: *mut () = first;
        let mut sent: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_18: *mut () = last;
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_18 })
        };
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_int_ptr_ = {
            let mut __tmp_19: *mut () = first;
            operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_19 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        let mut sent: sentinel_wrapper_operation_counting_iterator_int_ptr__ = {
            let mut __tmp_20: operation_counting_iterator_int_ptr_ = {
                let mut __tmp_21: *mut () = last;
                operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_21 }, {
                    (&mut ops as *mut IteratorOpCounts)
                })
            };
            sentinel_wrapper_operation_counting_iterator_int_ptr__::new_1({ &mut __tmp_20 })
        };
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_forward_sized_sentinel_51(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
) {
    let size: () = ((((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ first });
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_22: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ first });
            operation_counting_iterator_cpp17_input_iterator_int_ptr__int_ptr__::new_1(
                { &mut __tmp_22 },
                { (&mut ops as *mut IteratorOpCounts) },
            )
        };
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_forward_sized_sentinel_52(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
) {
    let size: () = ((((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: cpp20_input_iterator_int_ptr_ = cpp20_input_iterator_int_ptr_::new({ first });
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_cpp20_input_iterator_int_ptr__ = {
            let mut __tmp_23: cpp20_input_iterator_int_ptr_ =
                cpp20_input_iterator_int_ptr_::new({ first });
            operation_counting_iterator_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_23 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_forward_sized_sentinel_53(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
) {
    let size: () = ((((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({ first });
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_forward_iterator_int_ptr__ = {
            let mut __tmp_24: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_forward_iterator_int_ptr__::new_1({ &mut __tmp_24 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_forward_sized_sentinel_54(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
) {
    let size: () = ((((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ first });
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_25: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_bidirectional_iterator_int_ptr__::new_1(
                { &mut __tmp_25 },
                { (&mut ops as *mut IteratorOpCounts) },
            )
        };
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_forward_sized_sentinel_55(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
) {
    let size: () = ((((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ first });
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_random_access_iterator_int_ptr__ = {
            let mut __tmp_26: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_random_access_iterator_int_ptr__::new_1(
                { &mut __tmp_26 },
                { (&mut ops as *mut IteratorOpCounts) },
            )
        };
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_forward_sized_sentinel_56(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
) {
    let size: () = ((((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({ first });
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_contiguous_iterator_int_ptr__ = {
            let mut __tmp_27: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_27 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_forward_sized_sentinel_57(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
) {
    let size: () = ((((last as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    let M: () =
        ((((expected as usize - first as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: *mut () = first;
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_int_ptr_ = {
            let mut __tmp_28: *mut () = first;
            operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_28 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        let mut sent: distance_apriori_sentinel = distance_apriori_sentinel::new_1({ size });
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_backward_58(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_counts: IteratorOpCounts,
) {
    unimplemented!();
    const _: () = assert!(true, "std::bidirectional_iterator<It>");;
    let M: () = ((((expected as usize - last as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ last });
        let mut sent: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ first });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_29: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ last });
            operation_counting_iterator_bidirectional_iterator_int_ptr__::new_1(
                { &mut __tmp_29 },
                { (&mut ops as *mut IteratorOpCounts) },
            )
        };
        let mut sent: operation_counting_iterator_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_30: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_bidirectional_iterator_int_ptr__::new_1(
                { &mut __tmp_30 },
                { (&mut ops as *mut IteratorOpCounts) },
            )
        };
        const _: () = assert!(
            true,
            "std::bidirectional_iterator<operation_counting_iterator<It>>"
        );;
        const _: () = assert!(true, "Count == !std::sized_sentinel_for<It, It>");;
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(
                &mut it,
                n,
                operation_counting_iterator_bidirectional_iterator_int_ptr__::copy_from({ &sent }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_backward_59(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_counts: IteratorOpCounts,
) {
    unimplemented!();
    const _: () = assert!(true, "std::bidirectional_iterator<It>");;
    let M: () = ((((expected as usize - last as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ last });
        let mut sent: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ first });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_random_access_iterator_int_ptr__ = {
            let mut __tmp_31: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ last });
            operation_counting_iterator_random_access_iterator_int_ptr__::new_1(
                { &mut __tmp_31 },
                { (&mut ops as *mut IteratorOpCounts) },
            )
        };
        let mut sent: operation_counting_iterator_random_access_iterator_int_ptr__ = {
            let mut __tmp_32: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_random_access_iterator_int_ptr__::new_1(
                { &mut __tmp_32 },
                { (&mut ops as *mut IteratorOpCounts) },
            )
        };
        const _: () = assert!(
            true,
            "std::bidirectional_iterator<operation_counting_iterator<It>>"
        );;
        const _: () = assert!(true, "Count == !std::sized_sentinel_for<It, It>");;
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(
                &mut it,
                n,
                operation_counting_iterator_random_access_iterator_int_ptr__::copy_from({ &sent }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_backward_60(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_counts: IteratorOpCounts,
) {
    unimplemented!();
    const _: () = assert!(true, "std::bidirectional_iterator<It>");;
    let M: () = ((((expected as usize - last as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({ last });
        let mut sent: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({ first });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_contiguous_iterator_int_ptr__ = {
            let mut __tmp_33: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ last });
            operation_counting_iterator_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_33 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        let mut sent: operation_counting_iterator_contiguous_iterator_int_ptr__ = {
            let mut __tmp_34: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ first });
            operation_counting_iterator_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_34 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        const _: () = assert!(
            true,
            "std::bidirectional_iterator<operation_counting_iterator<It>>"
        );;
        const _: () = assert!(true, "Count == !std::sized_sentinel_for<It, It>");;
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(
                &mut it,
                n,
                operation_counting_iterator_contiguous_iterator_int_ptr__::copy_from({ &sent }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_backward_61(
    mut first: *mut (),
    mut last: *mut (),
    mut n: (),
    mut expected: *mut (),
    mut expected_counts: IteratorOpCounts,
) {
    unimplemented!();
    const _: () = assert!(true, "std::bidirectional_iterator<It>");;
    let M: () = ((((expected as usize - last as usize) / ::std::mem::size_of::<()>()) as ()) as ());
    {
        let mut it: *mut () = last;
        let mut sent: *mut () = first;
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(&mut it, n, sent)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ops: IteratorOpCounts = <IteratorOpCounts>::default();
        let mut it: operation_counting_iterator_int_ptr_ = {
            let mut __tmp_35: *mut () = last;
            operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_35 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        let mut sent: operation_counting_iterator_int_ptr_ = {
            let mut __tmp_36: *mut () = first;
            operation_counting_iterator_int_ptr_::new_1({ &mut __tmp_36 }, {
                (&mut ops as *mut IteratorOpCounts)
            })
        };
        const _: () = assert!(
            true,
            "std::bidirectional_iterator<operation_counting_iterator<It>>"
        );;
        const _: () = assert!(true, "Count == !std::sized_sentinel_for<It, It>");;
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_44))(
                &mut it,
                n,
                operation_counting_iterator_int_ptr_::copy_from({ &sent }),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct iota_iterator {
    pub x: (),
}
impl iota_iterator {
    pub unsafe fn operator_deref(&self) -> () {
        return self.x;
    }
    pub unsafe fn operator_inc_2(&mut self) -> *mut iota_iterator {
        self.x.prefix_inc();
        return &mut (*(self as *mut iota_iterator));
    }
    pub unsafe fn operator_post_inc_3(&mut self, mut _a0: ()) -> iota_iterator {
        self.x.prefix_inc();
        return iota_iterator {
            x: ((self.x) - (1_())),
        };
    }
    pub unsafe fn operator_eq(&self, _a0: *const iota_iterator) -> () {
        return (((*(self as *const iota_iterator)).x) == ((*_a0).x));
    }
    pub unsafe fn operator_sub(&self, that: *const iota_iterator) -> () {
        return ((self.x) - ((*that).x));
    }
    pub unsafe fn operator_dec_6(&mut self) -> *mut iota_iterator {
        self.x.prefix_dec();
        return &mut (*(self as *mut iota_iterator));
    }
    pub unsafe fn operator_post_dec_7(&mut self, mut _a0: ()) -> iota_iterator {
        self.x.prefix_dec();
        return iota_iterator {
            x: ((self.x) + (1_())),
        };
    }
}
impl std::cmp::PartialEq for iota_iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { iota_iterator::operator_eq(self, other as *const iota_iterator) }
    }
}
impl std::cmp::Eq for iota_iterator {}
const _: () = assert!(true, "std::bidirectional_iterator<iota_iterator>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<iota_iterator, iota_iterator>"
);
pub unsafe fn test_62() -> () {
    let mut range: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
    {
        let mut p: *mut () = std::ptr::null_mut();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: *mut () = std::ptr::null_mut();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
    }
    {
        let mut pc: *const () = std::ptr::null();
        pc = (range.as_mut_ptr().offset((5_()) as isize)).cast_const();
        unimplemented!();
        unimplemented!();
        pc = (range.as_mut_ptr().offset((5_()) as isize)).cast_const();
        unimplemented!();
        unimplemented!();
        pc = (range.as_mut_ptr().offset((5_()) as isize)).cast_const();
        unimplemented!();
        unimplemented!();
        pc = (range.as_mut_ptr().offset((5_()) as isize)).cast_const();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: *mut () = std::ptr::null_mut();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: *mut () = std::ptr::null_mut();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
        p = range.as_mut_ptr().offset((5_()) as isize);
        unimplemented!();
        unimplemented!();
    }
    let mut size: () = 0_();
    'loop_: while ((size) != (10_())) {
        let mut n: () = 0_();
        'loop_: while ((n) != (20_())) {
            {
                let mut expected: *mut () = if ((n) > (size)) {
                    range.as_mut_ptr().offset((size) as isize)
                } else {
                    range.as_mut_ptr().offset((n) as isize)
                };
                let mut equals_count: () = if ((n) > (size)) { ((size) + (1_())) } else { n };
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    let _expected: *mut () = expected;
                    let _expected_equals_count: () = -1_();
                    check_forward_43(_first, _last, n, _expected, None)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    let _expected: *mut () = expected;
                    let _expected_equals_count: () = -1_();
                    check_forward_45(_first, _last, n, _expected, None)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    let _expected: *mut () = expected;
                    let _expected_equals_count: () = equals_count;
                    check_forward_46(_first, _last, n, _expected, _expected_equals_count)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    let _expected: *mut () = expected;
                    let _expected_equals_count: () = equals_count;
                    check_forward_47(_first, _last, n, _expected, _expected_equals_count)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    let _expected: *mut () = expected;
                    let _expected_equals_count: () = equals_count;
                    check_forward_48(_first, _last, n, _expected, _expected_equals_count)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    let _expected: *mut () = expected;
                    let _expected_equals_count: () = equals_count;
                    check_forward_49(_first, _last, n, _expected, _expected_equals_count)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    let _expected: *mut () = expected;
                    let _expected_equals_count: () = equals_count;
                    check_forward_50(_first, _last, n, _expected, _expected_equals_count)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    check_forward_sized_sentinel_51(_first, _last, n, expected)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    check_forward_sized_sentinel_52(_first, _last, n, expected)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    check_forward_sized_sentinel_53(_first, _last, n, expected)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    check_forward_sized_sentinel_54(_first, _last, n, expected)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    check_forward_sized_sentinel_55(_first, _last, n, expected)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    check_forward_sized_sentinel_56(_first, _last, n, expected)
                });
                (unsafe {
                    let _first: *mut () = range.as_mut_ptr();
                    let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                    check_forward_sized_sentinel_57(_first, _last, n, expected)
                });
            }
            {
                let mut expected: *mut () = if ((n) > (size)) {
                    range.as_mut_ptr()
                } else {
                    range
                        .as_mut_ptr()
                        .offset((size) as isize)
                        .offset(-((n) as isize))
                };
                {
                    let mut expected_counts: IteratorOpCounts = IteratorOpCounts {
                        increments: 0_(),
                        decrements: ((range.as_mut_ptr().offset((size) as isize) as usize
                            - expected as usize)
                            / ::std::mem::size_of::<()>())
                            as (),
                        zero_moves: 0_(),
                        equal_cmps: if ((n) > (size)) { ((size) + (1_())) } else { n },
                    };
                    (unsafe {
                        let _first: *mut () = range.as_mut_ptr();
                        let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                        check_backward_58(_first, _last, -n, expected, expected_counts)
                    });
                }
                {
                    let mut expected_counts: IteratorOpCounts = IteratorOpCounts {
                        increments: 0_(),
                        decrements: if (((n) == (0_())) || ((n) >= (size))) {
                            0_()
                        } else {
                            1_()
                        },
                        zero_moves: if ((n) == (0_())) && ((size) != (0_())) {
                            1_()
                        } else {
                            0_()
                        },
                        equal_cmps: 0_(),
                    };
                    (unsafe {
                        let _first: *mut () = range.as_mut_ptr();
                        let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                        check_backward_59(_first, _last, -n, expected, expected_counts)
                    });
                    (unsafe {
                        let _first: *mut () = range.as_mut_ptr();
                        let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                        check_backward_60(_first, _last, -n, expected, expected_counts)
                    });
                    (unsafe {
                        let _first: *mut () = range.as_mut_ptr();
                        let _last: *mut () = range.as_mut_ptr().offset((size) as isize);
                        check_backward_61(_first, _last, -n, expected, expected_counts)
                    });
                }
            }
            n.prefix_inc();
        }
        size.prefix_inc();
    }
    {
        let mut i: iota_iterator = iota_iterator { x: 1_() };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
