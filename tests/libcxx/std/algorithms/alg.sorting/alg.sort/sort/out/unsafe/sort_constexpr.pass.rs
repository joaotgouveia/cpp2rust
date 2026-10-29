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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_add_0(
    mut x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_1(
    mut x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_sub_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_2(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_3(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_4(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_5(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
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
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
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
impl std::cmp::Ord for random_access_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_5(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_5(
                other as *const random_access_iterator_int_ptr_,
                self as *const random_access_iterator_int_ptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for random_access_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_3(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
pub unsafe fn operator_add_6(
    mut x: random_access_iterator_MoveOnly_ptr_,
    mut n: (),
) -> random_access_iterator_MoveOnly_ptr_ {
    (unsafe { random_access_iterator_MoveOnly_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_MoveOnly_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_7(
    mut x: random_access_iterator_MoveOnly_ptr_,
    mut n: (),
) -> random_access_iterator_MoveOnly_ptr_ {
    (unsafe { random_access_iterator_MoveOnly_ptr_::operator_sub_assign(&mut x, n) });
    return random_access_iterator_MoveOnly_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_8(
    mut x: random_access_iterator_MoveOnly_ptr_,
    mut y: random_access_iterator_MoveOnly_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<MoveOnly>()) as () as ());
}
pub unsafe fn operator_eq_9(
    x: *const random_access_iterator_MoveOnly_ptr_,
    y: *const random_access_iterator_MoveOnly_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_10(
    x: *const random_access_iterator_MoveOnly_ptr_,
    y: *const random_access_iterator_MoveOnly_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_11(
    x: *const random_access_iterator_MoveOnly_ptr_,
    y: *const random_access_iterator_MoveOnly_ptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_MoveOnly_ptr_ {
    it_: *mut MoveOnly,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_MoveOnly_ptr_ {
    pub unsafe fn new_1(mut it: *mut MoveOnly) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_MoveOnly_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_MoveOnly_ptr_,
    ) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
}
impl std::cmp::Ord for random_access_iterator_MoveOnly_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_11(
                self as *const random_access_iterator_MoveOnly_ptr_,
                other as *const random_access_iterator_MoveOnly_ptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_11(
                other as *const random_access_iterator_MoveOnly_ptr_,
                self as *const random_access_iterator_MoveOnly_ptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for random_access_iterator_MoveOnly_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for random_access_iterator_MoveOnly_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                self as *const random_access_iterator_MoveOnly_ptr_,
                other as *const random_access_iterator_MoveOnly_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_MoveOnly_ptr_ {}
pub unsafe fn operator_add_12(
    mut x: random_access_iterator_char_ptrptr_,
    mut n: (),
) -> random_access_iterator_char_ptrptr_ {
    (unsafe { random_access_iterator_char_ptrptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_char_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_13(
    mut x: random_access_iterator_char_ptrptr_,
    mut n: (),
) -> random_access_iterator_char_ptrptr_ {
    (unsafe { random_access_iterator_char_ptrptr_::operator_sub_assign(&mut x, n) });
    return random_access_iterator_char_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_14(
    mut x: random_access_iterator_char_ptrptr_,
    mut y: random_access_iterator_char_ptrptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<*mut ()>()) as () as ());
}
pub unsafe fn operator_eq_15(
    x: *const random_access_iterator_char_ptrptr_,
    y: *const random_access_iterator_char_ptrptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_16(
    x: *const random_access_iterator_char_ptrptr_,
    y: *const random_access_iterator_char_ptrptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_17(
    x: *const random_access_iterator_char_ptrptr_,
    y: *const random_access_iterator_char_ptrptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_char_ptrptr_ {
    it_: *mut *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_char_ptrptr_ {
    pub unsafe fn new_1(mut it: *mut *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_char_ptrptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_char_ptrptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_char_ptrptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_char_ptrptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_char_ptrptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_char_ptrptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_char_ptrptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut random_access_iterator_char_ptrptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_char_ptrptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_char_ptrptr_,
    ) -> *mut random_access_iterator_char_ptrptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_char_ptrptr_));
    }
}
impl std::cmp::Ord for random_access_iterator_char_ptrptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_17(
                self as *const random_access_iterator_char_ptrptr_,
                other as *const random_access_iterator_char_ptrptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_17(
                other as *const random_access_iterator_char_ptrptr_,
                self as *const random_access_iterator_char_ptrptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for random_access_iterator_char_ptrptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for random_access_iterator_char_ptrptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_15(
                self as *const random_access_iterator_char_ptrptr_,
                other as *const random_access_iterator_char_ptrptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_char_ptrptr_ {}
pub unsafe fn operator_add_18(
    mut x: random_access_iterator_const_char_ptrptr_,
    mut n: (),
) -> random_access_iterator_const_char_ptrptr_ {
    (unsafe { random_access_iterator_const_char_ptrptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_const_char_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_19(
    mut x: random_access_iterator_const_char_ptrptr_,
    mut n: (),
) -> random_access_iterator_const_char_ptrptr_ {
    (unsafe { random_access_iterator_const_char_ptrptr_::operator_sub_assign(&mut x, n) });
    return random_access_iterator_const_char_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_20(
    mut x: random_access_iterator_const_char_ptrptr_,
    mut y: random_access_iterator_const_char_ptrptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<*const ()>()) as () as ());
}
pub unsafe fn operator_eq_21(
    x: *const random_access_iterator_const_char_ptrptr_,
    y: *const random_access_iterator_const_char_ptrptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_22(
    x: *const random_access_iterator_const_char_ptrptr_,
    y: *const random_access_iterator_const_char_ptrptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_23(
    x: *const random_access_iterator_const_char_ptrptr_,
    y: *const random_access_iterator_const_char_ptrptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_const_char_ptrptr_ {
    it_: *mut *const (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_char_ptrptr_ {
    pub unsafe fn new_1(mut it: *mut *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_char_ptrptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_char_ptrptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_const_char_ptrptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_const_char_ptrptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_const_char_ptrptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_char_ptrptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_const_char_ptrptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_char_ptrptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_char_ptrptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_const_char_ptrptr_,
    ) -> *mut random_access_iterator_const_char_ptrptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_const_char_ptrptr_));
    }
}
impl std::cmp::Ord for random_access_iterator_const_char_ptrptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_23(
                self as *const random_access_iterator_const_char_ptrptr_,
                other as *const random_access_iterator_const_char_ptrptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_23(
                other as *const random_access_iterator_const_char_ptrptr_,
                self as *const random_access_iterator_const_char_ptrptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for random_access_iterator_const_char_ptrptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_char_ptrptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_21(
                self as *const random_access_iterator_const_char_ptrptr_,
                other as *const random_access_iterator_const_char_ptrptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_char_ptrptr_ {}
pub unsafe fn operator_add_24(
    mut x: random_access_iterator_int_ptrptr_,
    mut n: (),
) -> random_access_iterator_int_ptrptr_ {
    (unsafe { random_access_iterator_int_ptrptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_25(
    mut x: random_access_iterator_int_ptrptr_,
    mut n: (),
) -> random_access_iterator_int_ptrptr_ {
    (unsafe { random_access_iterator_int_ptrptr_::operator_sub_assign(&mut x, n) });
    return random_access_iterator_int_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_26(
    mut x: random_access_iterator_int_ptrptr_,
    mut y: random_access_iterator_int_ptrptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<*mut ()>()) as () as ());
}
pub unsafe fn operator_eq_27(
    x: *const random_access_iterator_int_ptrptr_,
    y: *const random_access_iterator_int_ptrptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_28(
    x: *const random_access_iterator_int_ptrptr_,
    y: *const random_access_iterator_int_ptrptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_29(
    x: *const random_access_iterator_int_ptrptr_,
    y: *const random_access_iterator_int_ptrptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_int_ptrptr_ {
    it_: *mut *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptrptr_ {
    pub unsafe fn new_1(mut it: *mut *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_int_ptrptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_int_ptrptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_int_ptrptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_int_ptrptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_int_ptrptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_int_ptrptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_int_ptrptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut random_access_iterator_int_ptrptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_int_ptrptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_int_ptrptr_,
    ) -> *mut random_access_iterator_int_ptrptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_int_ptrptr_));
    }
}
impl std::cmp::Ord for random_access_iterator_int_ptrptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_29(
                self as *const random_access_iterator_int_ptrptr_,
                other as *const random_access_iterator_int_ptrptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_29(
                other as *const random_access_iterator_int_ptrptr_,
                self as *const random_access_iterator_int_ptrptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for random_access_iterator_int_ptrptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptrptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_27(
                self as *const random_access_iterator_int_ptrptr_,
                other as *const random_access_iterator_int_ptrptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptrptr_ {}
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
pub unsafe fn operator_add_30(
    mut x: contiguous_iterator_int_ptr_,
    mut n: (),
) -> contiguous_iterator_int_ptr_ {
    (unsafe { contiguous_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_31(
    mut x: contiguous_iterator_int_ptr_,
    mut n: (),
) -> contiguous_iterator_int_ptr_ {
    (unsafe { contiguous_iterator_int_ptr_::operator_sub_assign(&mut x, n) });
    return contiguous_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_32(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_33(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_34(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_35(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
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
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut contiguous_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
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
impl std::cmp::Ord for contiguous_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_35(
                self as *const contiguous_iterator_int_ptr_,
                other as *const contiguous_iterator_int_ptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_35(
                other as *const contiguous_iterator_int_ptr_,
                self as *const contiguous_iterator_int_ptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for contiguous_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for contiguous_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_33(
                self as *const contiguous_iterator_int_ptr_,
                other as *const contiguous_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_int_ptr_ {}
pub unsafe fn operator_add_36(
    mut x: contiguous_iterator_MoveOnly_ptr_,
    mut n: (),
) -> contiguous_iterator_MoveOnly_ptr_ {
    (unsafe { contiguous_iterator_MoveOnly_ptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_MoveOnly_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_37(
    mut x: contiguous_iterator_MoveOnly_ptr_,
    mut n: (),
) -> contiguous_iterator_MoveOnly_ptr_ {
    (unsafe { contiguous_iterator_MoveOnly_ptr_::operator_sub_assign(&mut x, n) });
    return contiguous_iterator_MoveOnly_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_38(
    mut x: contiguous_iterator_MoveOnly_ptr_,
    mut y: contiguous_iterator_MoveOnly_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<MoveOnly>()) as () as ());
}
pub unsafe fn operator_eq_39(
    x: *const contiguous_iterator_MoveOnly_ptr_,
    y: *const contiguous_iterator_MoveOnly_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_40(
    x: *const contiguous_iterator_MoveOnly_ptr_,
    y: *const contiguous_iterator_MoveOnly_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_41(
    x: *const contiguous_iterator_MoveOnly_ptr_,
    y: *const contiguous_iterator_MoveOnly_ptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_MoveOnly_ptr_ {
    it_: *mut MoveOnly,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_MoveOnly_ptr_ {
    pub unsafe fn new_1(mut it: *mut MoveOnly) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_MoveOnly_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn operator_dec_10(&mut self) -> *mut contiguous_iterator_MoveOnly_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut contiguous_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut contiguous_iterator_MoveOnly_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut contiguous_iterator_MoveOnly_ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut contiguous_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_MoveOnly_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_MoveOnly_ptr_,
    ) -> *mut contiguous_iterator_MoveOnly_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_MoveOnly_ptr_));
    }
}
impl std::cmp::Ord for contiguous_iterator_MoveOnly_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_41(
                self as *const contiguous_iterator_MoveOnly_ptr_,
                other as *const contiguous_iterator_MoveOnly_ptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_41(
                other as *const contiguous_iterator_MoveOnly_ptr_,
                self as *const contiguous_iterator_MoveOnly_ptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for contiguous_iterator_MoveOnly_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for contiguous_iterator_MoveOnly_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_39(
                self as *const contiguous_iterator_MoveOnly_ptr_,
                other as *const contiguous_iterator_MoveOnly_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_MoveOnly_ptr_ {}
pub unsafe fn operator_add_42(
    mut x: contiguous_iterator_char_ptrptr_,
    mut n: (),
) -> contiguous_iterator_char_ptrptr_ {
    (unsafe { contiguous_iterator_char_ptrptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_char_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_43(
    mut x: contiguous_iterator_char_ptrptr_,
    mut n: (),
) -> contiguous_iterator_char_ptrptr_ {
    (unsafe { contiguous_iterator_char_ptrptr_::operator_sub_assign(&mut x, n) });
    return contiguous_iterator_char_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_44(
    mut x: contiguous_iterator_char_ptrptr_,
    mut y: contiguous_iterator_char_ptrptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<*mut ()>()) as () as ());
}
pub unsafe fn operator_eq_45(
    x: *const contiguous_iterator_char_ptrptr_,
    y: *const contiguous_iterator_char_ptrptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_46(
    x: *const contiguous_iterator_char_ptrptr_,
    y: *const contiguous_iterator_char_ptrptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_47(
    x: *const contiguous_iterator_char_ptrptr_,
    y: *const contiguous_iterator_char_ptrptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_char_ptrptr_ {
    it_: *mut *mut (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_char_ptrptr_ {
    pub unsafe fn new_1(mut it: *mut *mut ()) -> Self {
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_char_ptrptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_char_ptrptr_));
    }
    pub unsafe fn operator_dec_10(&mut self) -> *mut contiguous_iterator_char_ptrptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut contiguous_iterator_char_ptrptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut contiguous_iterator_char_ptrptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_char_ptrptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut contiguous_iterator_char_ptrptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut contiguous_iterator_char_ptrptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_char_ptrptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_char_ptrptr_,
    ) -> *mut contiguous_iterator_char_ptrptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_char_ptrptr_));
    }
}
impl std::cmp::Ord for contiguous_iterator_char_ptrptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_47(
                self as *const contiguous_iterator_char_ptrptr_,
                other as *const contiguous_iterator_char_ptrptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_47(
                other as *const contiguous_iterator_char_ptrptr_,
                self as *const contiguous_iterator_char_ptrptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for contiguous_iterator_char_ptrptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for contiguous_iterator_char_ptrptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_45(
                self as *const contiguous_iterator_char_ptrptr_,
                other as *const contiguous_iterator_char_ptrptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_char_ptrptr_ {}
pub unsafe fn operator_add_48(
    mut x: contiguous_iterator_const_char_ptrptr_,
    mut n: (),
) -> contiguous_iterator_const_char_ptrptr_ {
    (unsafe { contiguous_iterator_const_char_ptrptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_const_char_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_49(
    mut x: contiguous_iterator_const_char_ptrptr_,
    mut n: (),
) -> contiguous_iterator_const_char_ptrptr_ {
    (unsafe { contiguous_iterator_const_char_ptrptr_::operator_sub_assign(&mut x, n) });
    return contiguous_iterator_const_char_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_50(
    mut x: contiguous_iterator_const_char_ptrptr_,
    mut y: contiguous_iterator_const_char_ptrptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<*const ()>()) as () as ());
}
pub unsafe fn operator_eq_51(
    x: *const contiguous_iterator_const_char_ptrptr_,
    y: *const contiguous_iterator_const_char_ptrptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_52(
    x: *const contiguous_iterator_const_char_ptrptr_,
    y: *const contiguous_iterator_const_char_ptrptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_53(
    x: *const contiguous_iterator_const_char_ptrptr_,
    y: *const contiguous_iterator_const_char_ptrptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_const_char_ptrptr_ {
    it_: *mut *const (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char_ptrptr_ {
    pub unsafe fn new_1(mut it: *mut *const ()) -> Self {
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_const_char_ptrptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_const_char_ptrptr_));
    }
    pub unsafe fn operator_dec_10(&mut self) -> *mut contiguous_iterator_const_char_ptrptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut contiguous_iterator_const_char_ptrptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut contiguous_iterator_const_char_ptrptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_const_char_ptrptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut contiguous_iterator_const_char_ptrptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut contiguous_iterator_const_char_ptrptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_char_ptrptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_const_char_ptrptr_,
    ) -> *mut contiguous_iterator_const_char_ptrptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_const_char_ptrptr_));
    }
}
impl std::cmp::Ord for contiguous_iterator_const_char_ptrptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_53(
                self as *const contiguous_iterator_const_char_ptrptr_,
                other as *const contiguous_iterator_const_char_ptrptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_53(
                other as *const contiguous_iterator_const_char_ptrptr_,
                self as *const contiguous_iterator_const_char_ptrptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for contiguous_iterator_const_char_ptrptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for contiguous_iterator_const_char_ptrptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_51(
                self as *const contiguous_iterator_const_char_ptrptr_,
                other as *const contiguous_iterator_const_char_ptrptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_const_char_ptrptr_ {}
pub unsafe fn operator_add_54(
    mut x: contiguous_iterator_int_ptrptr_,
    mut n: (),
) -> contiguous_iterator_int_ptrptr_ {
    (unsafe { contiguous_iterator_int_ptrptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_int_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_55(
    mut x: contiguous_iterator_int_ptrptr_,
    mut n: (),
) -> contiguous_iterator_int_ptrptr_ {
    (unsafe { contiguous_iterator_int_ptrptr_::operator_sub_assign(&mut x, n) });
    return contiguous_iterator_int_ptrptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_56(
    mut x: contiguous_iterator_int_ptrptr_,
    mut y: contiguous_iterator_int_ptrptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<*mut ()>()) as () as ());
}
pub unsafe fn operator_eq_57(
    x: *const contiguous_iterator_int_ptrptr_,
    y: *const contiguous_iterator_int_ptrptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_58(
    x: *const contiguous_iterator_int_ptrptr_,
    y: *const contiguous_iterator_int_ptrptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_lt_59(
    x: *const contiguous_iterator_int_ptrptr_,
    y: *const contiguous_iterator_int_ptrptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_int_ptrptr_ {
    it_: *mut *mut (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptrptr_ {
    pub unsafe fn new_1(mut it: *mut *mut ()) -> Self {
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_int_ptrptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_int_ptrptr_));
    }
    pub unsafe fn operator_dec_10(&mut self) -> *mut contiguous_iterator_int_ptrptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut contiguous_iterator_int_ptrptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut contiguous_iterator_int_ptrptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_int_ptrptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut contiguous_iterator_int_ptrptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut contiguous_iterator_int_ptrptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_int_ptrptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_int_ptrptr_,
    ) -> *mut contiguous_iterator_int_ptrptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_int_ptrptr_));
    }
}
impl std::cmp::Ord for contiguous_iterator_int_ptrptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_59(
                self as *const contiguous_iterator_int_ptrptr_,
                other as *const contiguous_iterator_int_ptrptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_59(
                other as *const contiguous_iterator_int_ptrptr_,
                self as *const contiguous_iterator_int_ptrptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for contiguous_iterator_int_ptrptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for contiguous_iterator_int_ptrptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_57(
                self as *const contiguous_iterator_int_ptrptr_,
                other as *const contiguous_iterator_int_ptrptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_int_ptrptr_ {}
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
pub unsafe fn operator_eq_60(
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
            operator_eq_60(
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
pub unsafe fn operator_add_61(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_62(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_61(_i, n)
    });
}
pub unsafe fn iter_swap_63(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_64(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_65(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_66(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_66(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_65(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn operator_eq_67(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_68(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_69(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_70(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_71(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_72(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_73(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
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
        unsafe { operator_cmp_73(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_67(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_74(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_75(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_76(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_77(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_78(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_79(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_80(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
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
            operator_cmp_80(
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
            operator_eq_74(
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
pub static mut LargeN_81: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 128_() });
pub unsafe fn test_82() -> () {
    let mut orig: [(); 7] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (7_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [(); 7] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_83() -> () {
    let mut orig: [(); 7] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (7_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [(); 7] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_84() -> () {
    let mut orig: [(); 128] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (128_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [(); 128] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_85() -> () {
    let mut orig: [(); 128] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (128_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [(); 128] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_86() -> () {
    let mut orig: [(); 7] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (7_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [MoveOnly; 7] = [
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_87() -> () {
    let mut orig: [(); 7] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (7_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [MoveOnly; 7] = [
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_88() -> () {
    let mut orig: [(); 128] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (128_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [MoveOnly; 128] = [
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_89() -> () {
    let mut orig: [(); 128] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (128_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [MoveOnly; 128] = [
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_90() -> () {
    let mut orig: [(); 7] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (7_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [(); 7] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_91() -> () {
    let mut orig: [(); 128] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (128_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [(); 128] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_92() -> () {
    let mut orig: [(); 7] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (7_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [MoveOnly; 7] = [
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_93() -> () {
    let mut orig: [(); 128] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (128_())) {
        x = ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_());
        orig[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let mut work: [MoveOnly; 128] = [
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_pointers_94() -> () {
    let mut data: [(); 17] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut orig: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (17_())) {
        orig[(i) as usize] = (&mut data[((x).wrapping_rem(258_())) as usize] as *mut ());
        i.prefix_inc();
    }
    let mut work: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_pointers_95() -> () {
    let mut data: [(); 17] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut orig: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (17_())) {
        orig[(i) as usize] = (&mut data[((x).wrapping_rem(258_())) as usize] as *mut ());
        i.prefix_inc();
    }
    let mut work: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_pointers_96() -> () {
    let data: [(); 17] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut orig: [*const (); 17] = [
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (17_())) {
        orig[(i) as usize] = (&data[((x).wrapping_rem(258_())) as usize] as *const ());
        i.prefix_inc();
    }
    let mut work: [*const (); 17] = [
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_pointers_97() -> () {
    let data: [(); 17] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut orig: [*const (); 17] = [
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (17_())) {
        orig[(i) as usize] = (&data[((x).wrapping_rem(258_())) as usize] as *const ());
        i.prefix_inc();
    }
    let mut work: [*const (); 17] = [
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_pointers_98() -> () {
    let mut data: [(); 17] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut orig: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (17_())) {
        orig[(i) as usize] = (&mut data[((x).wrapping_rem(258_())) as usize] as *mut ());
        i.prefix_inc();
    }
    let mut work: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_pointers_99() -> () {
    let mut data: [(); 17] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut orig: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (17_())) {
        orig[(i) as usize] = (&mut data[((x).wrapping_rem(258_())) as usize] as *mut ());
        i.prefix_inc();
    }
    let mut work: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_pointers_100() -> () {
    let mut data: [(); 17] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut orig: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (17_())) {
        orig[(i) as usize] = (&mut data[((x).wrapping_rem(258_())) as usize] as *mut ());
        i.prefix_inc();
    }
    let mut work: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_pointers_101() -> () {
    let data: [(); 17] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut orig: [*const (); 17] = [
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (17_())) {
        orig[(i) as usize] = (&data[((x).wrapping_rem(258_())) as usize] as *const ());
        i.prefix_inc();
    }
    let mut work: [*const (); 17] = [
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_pointers_102() -> () {
    let mut data: [(); 17] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut orig: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (17_())) {
        orig[(i) as usize] = (&mut data[((x).wrapping_rem(258_())) as usize] as *mut ());
        i.prefix_inc();
    }
    let mut work: [*mut (); 17] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test_82() });
    (unsafe { test_83() });
    (unsafe { test_84() });
    (unsafe { test_85() });
    (unsafe { test_86() });
    (unsafe { test_87() });
    (unsafe { test_88() });
    (unsafe { test_89() });
    (unsafe { test_pointers_94() });
    (unsafe { test_pointers_95() });
    (unsafe { test_pointers_96() });
    (unsafe { test_pointers_97() });
    (unsafe { test_pointers_98() });
    (unsafe { test_pointers_99() });
    (unsafe { test_90() });
    (unsafe { test_91() });
    (unsafe { test_92() });
    (unsafe { test_93() });
    (unsafe { test_pointers_100() });
    (unsafe { test_pointers_101() });
    (unsafe { test_pointers_102() });
    const _: () = assert!(true, "test<7, int, int*>()");;
    const _: () = assert!(true, "test<7, int, random_access_iterator<int*>>()");;
    const _: () = assert!(true, "test<7, int, contiguous_iterator<int*>>()");;
    const _: () = assert!(true, "test<LargeN, int, int*>()");;
    const _: () = assert!(true, "test<LargeN, int, random_access_iterator<int*>>()");;
    const _: () = assert!(true, "test<LargeN, int, contiguous_iterator<int*>>()");;
    const _: () = assert!(true, "test<7, MoveOnly, MoveOnly*>()");;
    const _: () = assert!(
        true,
        "test<7, MoveOnly, random_access_iterator<MoveOnly*>>()"
    );;
    const _: () = assert!(true, "test<7, MoveOnly, contiguous_iterator<MoveOnly*>>()");;
    const _: () = assert!(true, "test<LargeN, MoveOnly, MoveOnly*>()");;
    const _: () = assert!(
        true,
        "test<LargeN, MoveOnly, random_access_iterator<MoveOnly*>>()"
    );;
    const _: () = assert!(
        true,
        "test<LargeN, MoveOnly, contiguous_iterator<MoveOnly*>>()"
    );;
    const _: () = assert!(true, "test_pointers<17, char, char**>()");;
    const _: () = assert!(
        true,
        "test_pointers<17, char, random_access_iterator<char**>>()"
    );;
    const _: () = assert!(
        true,
        "test_pointers<17, char, contiguous_iterator<char**>>()"
    );;
    const _: () = assert!(true, "test_pointers<17, const char, const char**>()");;
    const _: () = assert!(
        true,
        "test_pointers<17, const char, random_access_iterator<const char**>>()"
    );;
    const _: () = assert!(
        true,
        "test_pointers<17, const char, contiguous_iterator<const char**>>()"
    );;
    const _: () = assert!(true, "test_pointers<17, int, int**>()");;
    const _: () = assert!(
        true,
        "test_pointers<17, int, random_access_iterator<int**>>()"
    );;
    const _: () = assert!(true, "test_pointers<17, int, contiguous_iterator<int**>>()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const LargeN_81);
}
