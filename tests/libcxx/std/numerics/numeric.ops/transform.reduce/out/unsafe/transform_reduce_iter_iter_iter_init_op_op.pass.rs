extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_1(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_3(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_4(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_5(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_6(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
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
        unsafe { operator_cmp_6(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_7(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_13(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
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
            operator_cmp_13(
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
            operator_eq_7(
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
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_output_iterator_int_ptr_,
    ) -> *mut cpp17_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_output_iterator_int_ptr_));
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
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
pub unsafe fn operator_ne_14(
    x: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    y: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    ) -> *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_));
    }
}
pub unsafe fn operator_ne_15(
    x: *const cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    y: *const cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self
            as *mut cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    ) -> *mut cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self
            as *mut cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_ne_16(
    x: *const forward_iterator_const_unsigned_int_ptr_,
    y: *const forward_iterator_const_unsigned_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_unsigned_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_unsigned_int_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_unsigned_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_unsigned_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_unsigned_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_const_unsigned_int_ptr_,
    ) -> *mut forward_iterator_const_unsigned_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_const_unsigned_int_ptr_));
    }
}
pub unsafe fn operator_ne_17(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
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
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_const_int_ptr_,
    ) -> *mut forward_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_const_int_ptr_));
    }
}
pub unsafe fn operator_ne_18(
    x: *const bidirectional_iterator_const_unsigned_int_ptr_,
    y: *const bidirectional_iterator_const_unsigned_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_const_unsigned_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_unsigned_int_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_const_unsigned_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_const_unsigned_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_const_unsigned_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_const_unsigned_int_ptr_,
    ) -> *mut bidirectional_iterator_const_unsigned_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_const_unsigned_int_ptr_));
    }
}
pub unsafe fn operator_ne_19(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
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
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_const_int_ptr_,
    ) -> *mut bidirectional_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_const_int_ptr_));
    }
}
pub unsafe fn operator_sub_20(
    mut x: random_access_iterator_const_unsigned_int_ptr_,
    mut y: random_access_iterator_const_unsigned_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_ne_21(
    x: *const random_access_iterator_const_unsigned_int_ptr_,
    y: *const random_access_iterator_const_unsigned_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_const_unsigned_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_unsigned_int_ptr_ {
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
    pub unsafe fn operator_index(&self, mut n: ()) -> () {
        return &(*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_unsigned_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_unsigned_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_const_unsigned_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_unsigned_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_unsigned_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_const_unsigned_int_ptr_,
    ) -> *mut random_access_iterator_const_unsigned_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_const_unsigned_int_ptr_));
    }
}
pub unsafe fn operator_sub_22(
    mut x: random_access_iterator_const_int_ptr_,
    mut y: random_access_iterator_const_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_ne_23(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
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
    pub unsafe fn operator_index(&self, mut n: ()) -> () {
        return &(*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_const_int_ptr_,
    ) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
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
    pub unsafe fn move_from(_a0: *mut cpp20_random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_random_access_iterator_int_ptr_,
    ) -> *mut cpp20_random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_random_access_iterator_int_ptr_));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
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
pub unsafe fn operator_eq_24(
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
            operator_eq_24(
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
pub unsafe fn operator_add_25(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_26(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_25(_i, n)
    });
}
pub unsafe fn iter_swap_27(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_28(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_29(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_30(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_30(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_29(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_31(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_32(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_33(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_34(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_35(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_36(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_37(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_38(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_39(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_40(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_41(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_42(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_43(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_44(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_45(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_46(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_47(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_48(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_49(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_50(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_51(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_52(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_53(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_54(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_55(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_56(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_57(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_58(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_59(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_60(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_61(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_62(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_63(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_64(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_65(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_66(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_67(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_68(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_69(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_70(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_71(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_72(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_73(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_74(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_75(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_76(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_77(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_78(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_79(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_80(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_81(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_82(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_83(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_84(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_85(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_86(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_87(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_88(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_89(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_90(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_91(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_92(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_93(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_94(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_95(
    mut first1: *const (),
    mut last1: *const (),
    mut first2: *const (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_96(
    mut first1: *const (),
    mut last1: *const (),
    mut first2: *const (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_97(
    mut first1: *const (),
    mut last1: *const (),
    mut first2: *const (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_98(
    mut first1: *const (),
    mut last1: *const (),
    mut first2: *const (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_99(
    mut first1: *const (),
    mut last1: *const (),
    mut first2: *mut (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_100(
    mut first1: *mut (),
    mut last1: *mut (),
    mut first2: *const (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_101(
    mut first1: *const (),
    mut last1: *const (),
    mut first2: *mut (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_102(
    mut first1: *mut (),
    mut last1: *mut (),
    mut first2: *const (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_103(
    mut first1: *mut (),
    mut last1: *mut (),
    mut first2: *const (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_104(
    mut first1: *const (),
    mut last1: *const (),
    mut first2: *mut (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_105(
    mut first1: *mut (),
    mut last1: *mut (),
    mut first2: *const (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_106(
    mut first1: *const (),
    mut last1: *const (),
    mut first2: *mut (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_107(
    mut first1: *mut (),
    mut last1: *mut (),
    mut first2: *mut (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_108(
    mut first1: *mut (),
    mut last1: *mut (),
    mut first2: *mut (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_109(
    mut first1: *mut (),
    mut last1: *mut (),
    mut first2: *mut (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_110(
    mut first1: *mut (),
    mut last1: *mut (),
    mut first2: *mut (),
    mut init: (),
    mut op1: (),
    mut op2: (),
    mut x: (),
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub unsafe fn test_111() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        test_31(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        test_32(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_33(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_34(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_31(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_32(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_33(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_34(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_112() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        test_35(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        test_36(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_37(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_38(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_35(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_36(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_37(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_38(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_113() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        test_39(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        test_40(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_41(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_42(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_39(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_40(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_41(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_42(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_114() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        test_43(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        test_44(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_45(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_46(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_43(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_44(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_45(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_46(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_115() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_47(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        test_48(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
        });
        test_49(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_50(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
        });
        test_47(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_48(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        test_49(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_50(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_116() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_51(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        test_52(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
        });
        test_53(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_54(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
        });
        test_51(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_52(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        test_53(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_54(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_117() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_55(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        test_56(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
        });
        test_57(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_58(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
        });
        test_55(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_56(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        test_57(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_58(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_118() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_59(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        test_60(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
        });
        test_61(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_62(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
        });
        test_59(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_60(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        test_61(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_62(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_119() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_63(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        test_64(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_65(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_66(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_63(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_64(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_65(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_66(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_120() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_67(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        test_68(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_69(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_70(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_67(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_68(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_69(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_70(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_121() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_71(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        test_72(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_73(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_74(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_71(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_72(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_73(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_74(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_122() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_75(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        test_76(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_77(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_78(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_75(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_76(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_77(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_78(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_123() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_79(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        test_80(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_81(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_82(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_79(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_80(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_81(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_82(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_124() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_83(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        test_84(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_85(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_86(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_83(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_84(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_85(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_mut_ptr()).cast_const() });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_86(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_125() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_87(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        test_88(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_89(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_90(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_87(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_88(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_89(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_90(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_126() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        test_91(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        test_92(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_93(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        test_94(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_91(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        test_92(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_93(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr()).cast_const()
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        test_94(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_127() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: *const () = ia.as_mut_ptr();
        let _last1: *const () = ia.as_mut_ptr();
        test_95(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: *const () = ua.as_mut_ptr();
        let _last1: *const () = ua.as_mut_ptr();
        test_96(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: *const () = ia.as_mut_ptr();
        let _last1: *const () = ia.as_mut_ptr().offset((1_()) as isize);
        test_97(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: *const () = ua.as_mut_ptr();
        let _last1: *const () = ua.as_mut_ptr().offset((1_()) as isize);
        test_98(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: *const () = ia.as_mut_ptr();
        let _last1: *const () = ia.as_mut_ptr().offset((2_()) as isize);
        test_95(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: *const () = ua.as_mut_ptr();
        let _last1: *const () = ua.as_mut_ptr().offset((2_()) as isize);
        test_96(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: *const () = ia.as_mut_ptr();
        let _last1: *const () = ia.as_mut_ptr().offset((sa) as isize);
        test_97(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: *const () = ua.as_mut_ptr();
        let _last1: *const () = ua.as_mut_ptr().offset((sa) as isize);
        test_98(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_128() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: *const () = ia.as_mut_ptr();
        let _last1: *const () = ia.as_mut_ptr();
        test_99(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ua.as_mut_ptr();
        let _last1: *mut () = ua.as_mut_ptr();
        test_100(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: *const () = ia.as_mut_ptr();
        let _last1: *const () = ia.as_mut_ptr().offset((1_()) as isize);
        test_101(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ua.as_mut_ptr();
        let _last1: *mut () = ua.as_mut_ptr().offset((1_()) as isize);
        test_102(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: *const () = ia.as_mut_ptr();
        let _last1: *const () = ia.as_mut_ptr().offset((2_()) as isize);
        test_99(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ua.as_mut_ptr();
        let _last1: *mut () = ua.as_mut_ptr().offset((2_()) as isize);
        test_100(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: *const () = ia.as_mut_ptr();
        let _last1: *const () = ia.as_mut_ptr().offset((sa) as isize);
        test_101(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ua.as_mut_ptr();
        let _last1: *mut () = ua.as_mut_ptr().offset((sa) as isize);
        test_102(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_129() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: *mut () = ia.as_mut_ptr();
        let _last1: *mut () = ia.as_mut_ptr();
        test_103(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: *const () = ua.as_mut_ptr();
        let _last1: *const () = ua.as_mut_ptr();
        test_104(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ia.as_mut_ptr();
        let _last1: *mut () = ia.as_mut_ptr().offset((1_()) as isize);
        test_105(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: *const () = ua.as_mut_ptr();
        let _last1: *const () = ua.as_mut_ptr().offset((1_()) as isize);
        test_106(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ia.as_mut_ptr();
        let _last1: *mut () = ia.as_mut_ptr().offset((2_()) as isize);
        test_103(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: *const () = ua.as_mut_ptr();
        let _last1: *const () = ua.as_mut_ptr().offset((2_()) as isize);
        test_104(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ia.as_mut_ptr();
        let _last1: *mut () = ia.as_mut_ptr().offset((sa) as isize);
        test_105(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: *const () = ua.as_mut_ptr();
        let _last1: *const () = ua.as_mut_ptr().offset((sa) as isize);
        test_106(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_130() {
    let mut ia: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    let mut ua: [(); 6] = [2_(), 4_(), 6_(), 8_(), 10_(), 12_()];
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    (unsafe {
        let _first1: *mut () = ia.as_mut_ptr();
        let _last1: *mut () = ia.as_mut_ptr();
        test_107(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ua.as_mut_ptr();
        let _last1: *mut () = ua.as_mut_ptr();
        test_108(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ia.as_mut_ptr();
        let _last1: *mut () = ia.as_mut_ptr().offset((1_()) as isize);
        test_109(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ua.as_mut_ptr();
        let _last1: *mut () = ua.as_mut_ptr().offset((1_()) as isize);
        test_110(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ia.as_mut_ptr();
        let _last1: *mut () = ia.as_mut_ptr().offset((2_()) as isize);
        test_107(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ua.as_mut_ptr();
        let _last1: *mut () = ua.as_mut_ptr().offset((2_()) as isize);
        test_108(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ia.as_mut_ptr();
        let _last1: *mut () = ia.as_mut_ptr().offset((sa) as isize);
        test_109(
            _first1,
            _last1,
            ua.as_mut_ptr(),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    (unsafe {
        let _first1: *mut () = ua.as_mut_ptr();
        let _last1: *mut () = ua.as_mut_ptr().offset((sa) as isize);
        test_110(
            _first1,
            _last1,
            ia.as_mut_ptr(),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub unsafe fn test_return_type_131() {
    let mut p: *mut () = std::ptr::null_mut();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub unsafe fn test_return_type_132() {
    let mut p: *mut () = std::ptr::null_mut();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub unsafe fn test_return_type_133() {
    let mut p: *mut () = std::ptr::null_mut();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub unsafe fn test_return_type_134() {
    let mut p: *mut () = std::ptr::null_mut();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub unsafe fn test_return_type_135() {
    let mut p: *mut () = std::ptr::null_mut();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub unsafe fn test_return_type_136() {
    let mut p: *mut () = std::ptr::null_mut();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub unsafe fn test_return_type_137() {
    let mut p: *mut () = std::ptr::null_mut();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub unsafe fn test_move_only_types_138() {
    let mut ia: [MoveOnly; 3] = [
        MoveOnly::new({ Some(1_()) }),
        MoveOnly::new({ Some(2_()) }),
        MoveOnly::new({ Some(3_()) }),
    ];
    let mut ib: [MoveOnly; 3] = [
        MoveOnly::new({ Some(1_()) }),
        MoveOnly::new({ Some(2_()) }),
        MoveOnly::new({ Some(3_()) }),
    ];
    unimplemented!();
}
pub unsafe fn test_139() -> () {
    (unsafe { test_return_type_131() });
    (unsafe { test_return_type_132() });
    (unsafe { test_return_type_133() });
    (unsafe { test_return_type_134() });
    (unsafe { test_return_type_135() });
    (unsafe { test_return_type_136() });
    (unsafe { test_return_type_137() });
    (unsafe { test_111() });
    (unsafe { test_112() });
    (unsafe { test_113() });
    (unsafe { test_114() });
    (unsafe { test_115() });
    (unsafe { test_116() });
    (unsafe { test_117() });
    (unsafe { test_118() });
    (unsafe { test_119() });
    (unsafe { test_120() });
    (unsafe { test_121() });
    (unsafe { test_122() });
    (unsafe { test_123() });
    (unsafe { test_124() });
    (unsafe { test_125() });
    (unsafe { test_126() });
    (unsafe { test_127() });
    (unsafe { test_128() });
    (unsafe { test_129() });
    (unsafe { test_130() });
    (unsafe { test_move_only_types_138() });
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
    (unsafe { test_139() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
