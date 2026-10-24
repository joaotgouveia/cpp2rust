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
pub struct types_type_list_char__long_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr___bidirectional_iterator_char_ptr___forward_iterator_char_ptr___cpp20_input_iterator_char_ptr___cpp17_input_iterator_char_ptr__char_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_long_long_ptr__contiguous_iterator_long_long_ptr___random_access_iterator_long_long_ptr___bidirectional_iterator_long_long_ptr___forward_iterator_long_long_ptr___cpp20_input_iterator_long_long_ptr___cpp17_input_iterator_long_long_ptr__long_long_ptr__
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
pub struct types_concatenate_types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr____types_type_list_bidirectional_iterator_char_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr___bidirectional_iterator_char_ptr____types_type_list_forward_iterator_char_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr___bidirectional_iterator_char_ptr___forward_iterator_char_ptr____types_type_list_cpp20_input_iterator_char_ptr___cpp17_input_iterator_char_ptr__char_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_long_long_ptr__contiguous_iterator_long_long_ptr___random_access_iterator_long_long_ptr____types_type_list_bidirectional_iterator_long_long_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_long_long_ptr__contiguous_iterator_long_long_ptr___random_access_iterator_long_long_ptr___bidirectional_iterator_long_long_ptr____types_type_list_forward_iterator_long_long_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_long_long_ptr__contiguous_iterator_long_long_ptr___random_access_iterator_long_long_ptr___bidirectional_iterator_long_long_ptr___forward_iterator_long_long_ptr____types_type_list_cpp20_input_iterator_long_long_ptr___cpp17_input_iterator_long_long_ptr__long_long_ptr___
{}
pub unsafe fn for_each_0(mut _a0: types_type_list_char__long_long_, mut f: FnPtr<Generic>) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        swallow_1(_arg0, _arg1)
    });
}
pub unsafe fn for_each_2(
    mut _a0 : types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr___bidirectional_iterator_char_ptr___forward_iterator_char_ptr___cpp20_input_iterator_char_ptr___cpp17_input_iterator_char_ptr__char_ptr__,
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
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        swallow_3(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6)
    });
}
pub unsafe fn for_each_4(
    mut _a0 : types_type_list_long_long_ptr__contiguous_iterator_long_long_ptr___random_access_iterator_long_long_ptr___bidirectional_iterator_long_long_ptr___forward_iterator_long_long_ptr___cpp20_input_iterator_long_long_ptr___cpp17_input_iterator_long_long_ptr__long_long_ptr__,
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
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        swallow_3(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6)
    });
}
pub unsafe fn swallow_3(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
    mut _a6: (),
) {
}
pub unsafe fn swallow_1(mut _a0: (), mut _a1: ()) {}
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
pub unsafe fn base_5(i: *const cpp17_input_iterator_char_ptr__char_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_char_ptr__char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_char_ptr__char_ptr_ {
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
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_input_iterator_char_ptr__char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_char_ptr__char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_char_ptr__char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn base_6(i: *const cpp17_input_iterator_long_long_ptr__long_long_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_long_long_ptr__long_long_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_long_long_ptr__long_long_ptr_ {
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
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_long_long_ptr__long_long_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_long_long_ptr__long_long_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_long_long_ptr__long_long_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_7(
    x: *const forward_iterator_char_ptr_,
    y: *const forward_iterator_char_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_8(
    x: *const forward_iterator_char_ptr_,
    y: *const forward_iterator_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_9(i: *const forward_iterator_char_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct forward_iterator_char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_char_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for forward_iterator_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const forward_iterator_char_ptr_,
                other as *const forward_iterator_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_char_ptr_ {}
pub unsafe fn operator_eq_10(
    x: *const forward_iterator_long_long_ptr_,
    y: *const forward_iterator_long_long_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_11(
    x: *const forward_iterator_long_long_ptr_,
    y: *const forward_iterator_long_long_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_12(i: *const forward_iterator_long_long_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct forward_iterator_long_long_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_long_long_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_long_long_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_long_long_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for forward_iterator_long_long_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const forward_iterator_long_long_ptr_,
                other as *const forward_iterator_long_long_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_long_long_ptr_ {}
pub unsafe fn operator_eq_13(
    x: *const forward_iterator_std_move_iterator_std_string_ptr__,
    y: *const forward_iterator_std_move_iterator_std_string_ptr__,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ne_14(
    x: *const forward_iterator_std_move_iterator_std_string_ptr__,
    y: *const forward_iterator_std_move_iterator_std_string_ptr__,
) -> () {
    return !(unimplemented!());
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct forward_iterator_std_move_iterator_std_string_ptr__ {
    it_: (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std_move_iterator_std_string_ptr__ {
    pub unsafe fn new_1(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*unimplemented!());
    }
    pub unsafe fn operator_inc_6(
        &mut self,
    ) -> *mut forward_iterator_std_move_iterator_std_string_ptr__ {
        unimplemented!();
        return &mut (*(self as *mut forward_iterator_std_move_iterator_std_string_ptr__));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_std_move_iterator_std_string_ptr__) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for forward_iterator_std_move_iterator_std_string_ptr__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_13(
                self as *const forward_iterator_std_move_iterator_std_string_ptr__,
                other as *const forward_iterator_std_move_iterator_std_string_ptr__,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_std_move_iterator_std_string_ptr__ {}
pub unsafe fn operator_eq_15(
    x: *const bidirectional_iterator_char_ptr_,
    y: *const bidirectional_iterator_char_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_16(
    x: *const bidirectional_iterator_char_ptr_,
    y: *const bidirectional_iterator_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_17(i: *const bidirectional_iterator_char_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct bidirectional_iterator_char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_char_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_15(
                self as *const bidirectional_iterator_char_ptr_,
                other as *const bidirectional_iterator_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_char_ptr_ {}
pub unsafe fn operator_eq_18(
    x: *const bidirectional_iterator_long_long_ptr_,
    y: *const bidirectional_iterator_long_long_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_19(
    x: *const bidirectional_iterator_long_long_ptr_,
    y: *const bidirectional_iterator_long_long_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_20(i: *const bidirectional_iterator_long_long_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct bidirectional_iterator_long_long_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_long_long_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_long_long_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_long_long_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_long_long_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_18(
                self as *const bidirectional_iterator_long_long_ptr_,
                other as *const bidirectional_iterator_long_long_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_long_long_ptr_ {}
pub unsafe fn operator_add_21(
    mut x: random_access_iterator_char_ptr_,
    mut n: (),
) -> random_access_iterator_char_ptr_ {
    (unsafe { random_access_iterator_char_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_char_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_eq_22(
    x: *const random_access_iterator_char_ptr_,
    y: *const random_access_iterator_char_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_23(
    x: *const random_access_iterator_char_ptr_,
    y: *const random_access_iterator_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_24(i: *const random_access_iterator_char_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct random_access_iterator_char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_char_ptr_ {
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
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_char_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_char_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for random_access_iterator_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_22(
                self as *const random_access_iterator_char_ptr_,
                other as *const random_access_iterator_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_char_ptr_ {}
pub unsafe fn operator_add_25(
    mut x: random_access_iterator_long_long_ptr_,
    mut n: (),
) -> random_access_iterator_long_long_ptr_ {
    (unsafe { random_access_iterator_long_long_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_long_long_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_eq_26(
    x: *const random_access_iterator_long_long_ptr_,
    y: *const random_access_iterator_long_long_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_27(
    x: *const random_access_iterator_long_long_ptr_,
    y: *const random_access_iterator_long_long_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_28(i: *const random_access_iterator_long_long_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct random_access_iterator_long_long_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_long_long_ptr_ {
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
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_long_long_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_long_long_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_long_long_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_long_long_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for random_access_iterator_long_long_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_26(
                self as *const random_access_iterator_long_long_ptr_,
                other as *const random_access_iterator_long_long_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_long_long_ptr_ {}
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
pub unsafe fn operator_add_29(
    mut x: contiguous_iterator_char_ptr_,
    mut n: (),
) -> contiguous_iterator_char_ptr_ {
    (unsafe { contiguous_iterator_char_ptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_char_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_ne_30(
    x: *const contiguous_iterator_char_ptr_,
    y: *const contiguous_iterator_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_31(i: *const contiguous_iterator_char_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct contiguous_iterator_char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_char_ptr_ {
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_char_ptr_));
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut contiguous_iterator_char_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn operator_add_32(
    mut x: contiguous_iterator_long_long_ptr_,
    mut n: (),
) -> contiguous_iterator_long_long_ptr_ {
    (unsafe { contiguous_iterator_long_long_ptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_long_long_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_ne_33(
    x: *const contiguous_iterator_long_long_ptr_,
    y: *const contiguous_iterator_long_long_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_34(i: *const contiguous_iterator_long_long_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct contiguous_iterator_long_long_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_long_long_ptr_ {
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_long_long_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_long_long_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut contiguous_iterator_long_long_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_long_long_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn base_35(mut i: *mut ()) -> *mut () {
    return i;
}
pub unsafe fn base_36(mut i: *mut ()) -> *mut () {
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
pub unsafe fn base_37(i: *const cpp20_input_iterator_char_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_char_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_char_ptr_,
    ) -> *mut cpp20_input_iterator_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_char_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp20_input_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp20_input_iterator_char_ptr_));
    }
}
pub unsafe fn base_38(i: *const cpp20_input_iterator_long_long_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_long_long_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_long_long_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_long_long_ptr_,
    ) -> *mut cpp20_input_iterator_long_long_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_long_long_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp20_input_iterator_long_long_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp20_input_iterator_long_long_ptr_));
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
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_char_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_char_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_35((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_35((*other)) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_contiguous_iterator_char_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_contiguous_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_31(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_char_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_31(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_random_access_iterator_char_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_random_access_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const random_access_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_24(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_char_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_24(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_bidirectional_iterator_char_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_bidirectional_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_17(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_char_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_17(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_char_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_forward_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_9(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_char_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_9(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_char_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_37(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_char_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_37(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_input_iterator_char_ptr__char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_5(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp17_input_iterator_char_ptr__char_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_5(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_long_long_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_long_long_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_36((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_36((*other)) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_contiguous_iterator_long_long_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_contiguous_iterator_long_long_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_34(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_long_long_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_34(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_random_access_iterator_long_long_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_random_access_iterator_long_long_ptr__ {
    pub unsafe fn new_1(it: *const random_access_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_28(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_long_long_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_28(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_bidirectional_iterator_long_long_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_bidirectional_iterator_long_long_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_20(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_long_long_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_20(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_long_long_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_forward_iterator_long_long_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_12(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_long_long_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_12(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_38(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_long_long_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_38(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_input_iterator_long_long_ptr__long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_6(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const cpp17_input_iterator_long_long_ptr__long_long_ptr_,
    ) -> () {
        {}
        return ((self.base_) == (unsafe { base_6(other) }));
    }
}
pub unsafe fn operator_sub_39(s: *const sized_sentinel_char_ptr_, i: *const *mut ()) -> () {
    return (((*s).base_ as usize - (unsafe { base_35((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_40(i: *const *mut (), s: *const sized_sentinel_char_ptr_) -> () {
    return (((unsafe { base_35((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<()>()) as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_char_ptr_ {
    base_: *mut (),
}
impl sized_sentinel_char_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_35((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        return ((self.base_) == (unsafe { base_35((*other)) }));
    }
}
pub unsafe fn operator_sub_41(
    s: *const sized_sentinel_contiguous_iterator_char_ptr__,
    i: *const contiguous_iterator_char_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_31(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_42(
    i: *const contiguous_iterator_char_ptr_,
    s: *const sized_sentinel_contiguous_iterator_char_ptr__,
) -> () {
    return (((unsafe { base_31(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_char_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_contiguous_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_31(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_char_ptr_) -> () {
        return ((self.base_) == (unsafe { base_31(other) }));
    }
}
pub unsafe fn operator_sub_43(
    s: *const sized_sentinel_random_access_iterator_char_ptr__,
    i: *const random_access_iterator_char_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_24(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_44(
    i: *const random_access_iterator_char_ptr_,
    s: *const sized_sentinel_random_access_iterator_char_ptr__,
) -> () {
    return (((unsafe { base_24(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_random_access_iterator_char_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_random_access_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const random_access_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_24(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_char_ptr_) -> () {
        return ((self.base_) == (unsafe { base_24(other) }));
    }
}
pub unsafe fn operator_sub_45(
    s: *const sized_sentinel_bidirectional_iterator_char_ptr__,
    i: *const bidirectional_iterator_char_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_17(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_46(
    i: *const bidirectional_iterator_char_ptr_,
    s: *const sized_sentinel_bidirectional_iterator_char_ptr__,
) -> () {
    return (((unsafe { base_17(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_bidirectional_iterator_char_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_bidirectional_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_17(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_char_ptr_) -> () {
        return ((self.base_) == (unsafe { base_17(other) }));
    }
}
pub unsafe fn operator_sub_47(
    s: *const sized_sentinel_forward_iterator_char_ptr__,
    i: *const forward_iterator_char_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_9(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_48(
    i: *const forward_iterator_char_ptr_,
    s: *const sized_sentinel_forward_iterator_char_ptr__,
) -> () {
    return (((unsafe { base_9(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_forward_iterator_char_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_forward_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_9(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_char_ptr_) -> () {
        return ((self.base_) == (unsafe { base_9(other) }));
    }
}
pub unsafe fn operator_sub_49(
    s: *const sized_sentinel_cpp20_input_iterator_char_ptr__,
    i: *const cpp20_input_iterator_char_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_37(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_50(
    i: *const cpp20_input_iterator_char_ptr_,
    s: *const sized_sentinel_cpp20_input_iterator_char_ptr__,
) -> () {
    return (((unsafe { base_37(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_cpp20_input_iterator_char_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_cpp20_input_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_37(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_char_ptr_) -> () {
        return ((self.base_) == (unsafe { base_37(other) }));
    }
}
pub unsafe fn operator_sub_51(
    s: *const sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__,
    i: *const cpp17_input_iterator_char_ptr__char_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_5(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_52(
    i: *const cpp17_input_iterator_char_ptr__char_ptr_,
    s: *const sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__,
) -> () {
    return (((unsafe { base_5(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_input_iterator_char_ptr__char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_5(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp17_input_iterator_char_ptr__char_ptr_) -> () {
        return ((self.base_) == (unsafe { base_5(other) }));
    }
}
pub unsafe fn operator_sub_53(s: *const sized_sentinel_long_long_ptr_, i: *const *mut ()) -> () {
    return (((*s).base_ as usize - (unsafe { base_36((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_54(i: *const *mut (), s: *const sized_sentinel_long_long_ptr_) -> () {
    return (((unsafe { base_36((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<()>()) as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_long_long_ptr_ {
    base_: *mut (),
}
impl sized_sentinel_long_long_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_36((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        return ((self.base_) == (unsafe { base_36((*other)) }));
    }
}
pub unsafe fn operator_sub_55(
    s: *const sized_sentinel_contiguous_iterator_long_long_ptr__,
    i: *const contiguous_iterator_long_long_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_34(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_56(
    i: *const contiguous_iterator_long_long_ptr_,
    s: *const sized_sentinel_contiguous_iterator_long_long_ptr__,
) -> () {
    return (((unsafe { base_34(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_long_long_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_contiguous_iterator_long_long_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_34(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_long_long_ptr_) -> () {
        return ((self.base_) == (unsafe { base_34(other) }));
    }
}
pub unsafe fn operator_sub_57(
    s: *const sized_sentinel_random_access_iterator_long_long_ptr__,
    i: *const random_access_iterator_long_long_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_28(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_58(
    i: *const random_access_iterator_long_long_ptr_,
    s: *const sized_sentinel_random_access_iterator_long_long_ptr__,
) -> () {
    return (((unsafe { base_28(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_random_access_iterator_long_long_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_random_access_iterator_long_long_ptr__ {
    pub unsafe fn new_1(it: *const random_access_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_28(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_long_long_ptr_) -> () {
        return ((self.base_) == (unsafe { base_28(other) }));
    }
}
pub unsafe fn operator_sub_59(
    s: *const sized_sentinel_bidirectional_iterator_long_long_ptr__,
    i: *const bidirectional_iterator_long_long_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_20(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_60(
    i: *const bidirectional_iterator_long_long_ptr_,
    s: *const sized_sentinel_bidirectional_iterator_long_long_ptr__,
) -> () {
    return (((unsafe { base_20(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_bidirectional_iterator_long_long_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_bidirectional_iterator_long_long_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_20(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_long_long_ptr_) -> () {
        return ((self.base_) == (unsafe { base_20(other) }));
    }
}
pub unsafe fn operator_sub_61(
    s: *const sized_sentinel_forward_iterator_long_long_ptr__,
    i: *const forward_iterator_long_long_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_12(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_62(
    i: *const forward_iterator_long_long_ptr_,
    s: *const sized_sentinel_forward_iterator_long_long_ptr__,
) -> () {
    return (((unsafe { base_12(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_forward_iterator_long_long_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_forward_iterator_long_long_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_12(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_long_long_ptr_) -> () {
        return ((self.base_) == (unsafe { base_12(other) }));
    }
}
pub unsafe fn operator_sub_63(
    s: *const sized_sentinel_cpp20_input_iterator_long_long_ptr__,
    i: *const cpp20_input_iterator_long_long_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_38(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_64(
    i: *const cpp20_input_iterator_long_long_ptr_,
    s: *const sized_sentinel_cpp20_input_iterator_long_long_ptr__,
) -> () {
    return (((unsafe { base_38(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_cpp20_input_iterator_long_long_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_cpp20_input_iterator_long_long_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_38(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_long_long_ptr_) -> () {
        return ((self.base_) == (unsafe { base_38(other) }));
    }
}
pub unsafe fn operator_sub_65(
    s: *const sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__,
    i: *const cpp17_input_iterator_long_long_ptr__long_long_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_6(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_66(
    i: *const cpp17_input_iterator_long_long_ptr__long_long_ptr_,
    s: *const sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__,
) -> () {
    return (((unsafe { base_6(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_input_iterator_long_long_ptr__long_long_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_6(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const cpp17_input_iterator_long_long_ptr__long_long_ptr_,
    ) -> () {
        return ((self.base_) == (unsafe { base_6(other) }));
    }
}
pub unsafe fn operator_eq_67(
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
            operator_eq_67(
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
pub unsafe fn operator_add_68(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_69(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_68(_i, n)
    });
}
pub unsafe fn iter_swap_70(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_71(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*unimplemented!());
}
pub unsafe fn operator_eq_72(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_73(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_73(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_72(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub struct UncheckedRange_cpp20_input_iterator_int_ptr___SentinelForNotSemiregular_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_cpp20_input_iterator_int_ptr___SentinelForNotWeaklyEqualityComparableWith_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__
{}
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
pub unsafe fn operator_eq_74(
    lhs: *const BooleanTestable,
    rhs: *const BooleanTestable,
) -> BooleanTestable {
    return BooleanTestable::new({ (((*lhs).value_) == ((*rhs).value_)) });
}
pub unsafe fn operator_ne_75(
    lhs: *const BooleanTestable,
    rhs: *const BooleanTestable,
) -> BooleanTestable {
    return BooleanTestable::new({ (((*lhs).value_) != ((*rhs).value_)) });
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct BooleanTestable {
    value_: (),
}
impl BooleanTestable {
    pub unsafe fn to_(&self) -> () {
        return self.value_;
    }
    pub unsafe fn operator_not(&mut self) -> *mut BooleanTestable {
        self.value_ = !(self.value_);
        return &mut (*unimplemented!());
    }
    pub unsafe fn new(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::PartialEq for BooleanTestable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            {
                let __cmp = &operator_eq_74(
                    self as *const BooleanTestable,
                    other as *const BooleanTestable,
                );
                BooleanTestable::to_(__cmp)
            }
        }
    }
}
impl std::cmp::Eq for BooleanTestable {}
pub static mut yes_76: std::cell::LazyCell<BooleanTestable> =
    std::cell::LazyCell::new(|| unsafe { BooleanTestable::new({ true }) });
pub static mut no_77: std::cell::LazyCell<BooleanTestable> =
    std::cell::LazyCell::new(|| unsafe { BooleanTestable::new({ false }) });
pub static mut StrictUnaryPredicate_78: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe { lambda_unsafe!(Generic, {}) });
pub static mut StrictBinaryPredicate_79: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe { lambda_unsafe!(Generic, {}) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StrictBooleanIterator_int_ptr_ {
    iter_: *mut (),
}
impl StrictBooleanIterator_int_ptr_ {}
const _: () = assert!(true, "std::forward_iterator<StrictBooleanIterator<int*>>");
const _: () = assert!(
    true,
    "std::sentinel_for<StrictBooleanIterator<int*>, StrictBooleanIterator<int*>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotEqualityComparable {}
const _: () = assert!(true, "HasContainsIt<int*>");
const _: () = assert!(true, "HasContainsIt<int*, int*>");
const _: () = assert!(!(false), "!HasContainsIt<NotEqualityComparable*>");
const _: () = assert!(!(false), "!HasContainsIt<InputIteratorNotDerivedFrom>");
const _: () = assert!(
    !(false),
    "!HasContainsIt<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!HasContainsIt<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!HasContainsIt<cpp20_input_iterator<int*>, SentinelForNotSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasContainsIt<cpp20_input_iterator<int*>, InputRangeNotSentinelEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasContainsIt<cpp20_input_iterator<int*>, sentinel_wrapper<cpp20_input_iterator<int*>>>"
);
const _: () = assert!(!(false), "!HasContainsIt<int*, int>");
const _: () = assert!(!(false), "!HasContainsIt<int, int*>");
const _: () = assert!(!(false), "!HasContainsR<int, int>");
const _: () = assert!(true, "HasContainsR<int[1], int>");
const _: () = assert!(
    !(false),
    "!HasContainsR<NotEqualityComparable[1], NotEqualityComparable>"
);
const _: () = assert!(!(false), "!HasContainsR<InputRangeNotDerivedFrom, int>");
const _: () = assert!(
    !(false),
    "!HasContainsR<InputRangeNotIndirectlyReadable, int>"
);
const _: () = assert!(
    !(false),
    "!HasContainsR<InputRangeNotInputOrOutputIterator, int>"
);
const _: () = assert!(
    !(false),
    "!HasContainsR<InputRangeNotSentinelSemiregular, int>"
);
const _: () = assert!(
    !(false),
    "!HasContainsR<InputRangeNotSentinelEqualityComparableWith, int>"
);
pub unsafe fn test_iterators_80() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((6_()) as isize);
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((1_()) as isize);
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = unimplemented!();
                let ___last: *mut () = unimplemented!();
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((10_()) as isize);
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((9_()) as isize);
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_82() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sentinel_wrapper_char_ptr_ = {
                    let mut __tmp_0: *mut () = a.as_mut_ptr().offset((6_()) as isize);
                    sentinel_wrapper_char_ptr_::new_1({ &mut __tmp_0 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sentinel_wrapper_char_ptr_ = {
                    let mut __tmp_1: *mut () = a.as_mut_ptr().offset((1_()) as isize);
                    sentinel_wrapper_char_ptr_::new_1({ &mut __tmp_1 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = unimplemented!();
                let ___last: sentinel_wrapper_char_ptr_ = {
                    let mut __tmp_2: *mut () = (unimplemented!() as *mut ());
                    sentinel_wrapper_char_ptr_::new_1({ &mut __tmp_2 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sentinel_wrapper_char_ptr_ = {
                    let mut __tmp_3: *mut () = a.as_mut_ptr().offset((10_()) as isize);
                    sentinel_wrapper_char_ptr_::new_1({ &mut __tmp_3 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sentinel_wrapper_char_ptr_ = {
                    let mut __tmp_4: *mut () = a.as_mut_ptr().offset((9_()) as isize);
                    sentinel_wrapper_char_ptr_::new_1({ &mut __tmp_4 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sentinel_wrapper_char_ptr_ = {
                    let mut __tmp_5: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                    sentinel_wrapper_char_ptr_::new_1({ &mut __tmp_5 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_83() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sized_sentinel_char_ptr_ = {
                    let mut __tmp_6: *mut () = a.as_mut_ptr().offset((6_()) as isize);
                    sized_sentinel_char_ptr_::new_1({ &mut __tmp_6 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sized_sentinel_char_ptr_ = {
                    let mut __tmp_7: *mut () = a.as_mut_ptr().offset((1_()) as isize);
                    sized_sentinel_char_ptr_::new_1({ &mut __tmp_7 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = unimplemented!();
                let ___last: sized_sentinel_char_ptr_ = {
                    let mut __tmp_8: *mut () = (unimplemented!() as *mut ());
                    sized_sentinel_char_ptr_::new_1({ &mut __tmp_8 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sized_sentinel_char_ptr_ = {
                    let mut __tmp_9: *mut () = a.as_mut_ptr().offset((10_()) as isize);
                    sized_sentinel_char_ptr_::new_1({ &mut __tmp_9 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sized_sentinel_char_ptr_ = {
                    let mut __tmp_10: *mut () = a.as_mut_ptr().offset((9_()) as isize);
                    sized_sentinel_char_ptr_::new_1({ &mut __tmp_10 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sized_sentinel_char_ptr_ = {
                    let mut __tmp_11: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                    sized_sentinel_char_ptr_::new_1({ &mut __tmp_11 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_84() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((6_()) as isize)
                    });
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((1_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((10_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((9_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((5_()) as isize)
                    });
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_85() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_12: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_12 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_13: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_13 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: sentinel_wrapper_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_14: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({ unimplemented!() });
                    sentinel_wrapper_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_14 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_15: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_15 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_16: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_16 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_17: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_17 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_86() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_18: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_18 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_19: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_19 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: sized_sentinel_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_20: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({ unimplemented!() });
                    sized_sentinel_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_20 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_21: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_21 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_22: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_22 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_char_ptr_ =
                    contiguous_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_contiguous_iterator_char_ptr__ = {
                    let mut __tmp_23: contiguous_iterator_char_ptr_ =
                        contiguous_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_contiguous_iterator_char_ptr__::new_1({ &mut __tmp_23 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_87() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((6_()) as isize)
                    });
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((1_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((10_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((9_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((5_()) as isize)
                    });
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_88() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_random_access_iterator_char_ptr__ = {
                    let mut __tmp_24: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_char_ptr__::new_1({ &mut __tmp_24 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_random_access_iterator_char_ptr__ = {
                    let mut __tmp_25: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_char_ptr__::new_1({ &mut __tmp_25 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: sentinel_wrapper_random_access_iterator_char_ptr__ = {
                    let mut __tmp_26: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({ unimplemented!() });
                    sentinel_wrapper_random_access_iterator_char_ptr__::new_1({ &mut __tmp_26 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_random_access_iterator_char_ptr__ = {
                    let mut __tmp_27: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_char_ptr__::new_1({ &mut __tmp_27 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_random_access_iterator_char_ptr__ = {
                    let mut __tmp_28: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_char_ptr__::new_1({ &mut __tmp_28 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_random_access_iterator_char_ptr__ = {
                    let mut __tmp_29: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_char_ptr__::new_1({ &mut __tmp_29 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_89() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_random_access_iterator_char_ptr__ = {
                    let mut __tmp_30: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_random_access_iterator_char_ptr__::new_1({ &mut __tmp_30 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_random_access_iterator_char_ptr__ = {
                    let mut __tmp_31: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_random_access_iterator_char_ptr__::new_1({ &mut __tmp_31 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: sized_sentinel_random_access_iterator_char_ptr__ = {
                    let mut __tmp_32: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({ unimplemented!() });
                    sized_sentinel_random_access_iterator_char_ptr__::new_1({ &mut __tmp_32 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_random_access_iterator_char_ptr__ = {
                    let mut __tmp_33: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_random_access_iterator_char_ptr__::new_1({ &mut __tmp_33 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_random_access_iterator_char_ptr__ = {
                    let mut __tmp_34: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_random_access_iterator_char_ptr__::new_1({ &mut __tmp_34 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_char_ptr_ =
                    random_access_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_random_access_iterator_char_ptr__ = {
                    let mut __tmp_35: random_access_iterator_char_ptr_ =
                        random_access_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_random_access_iterator_char_ptr__::new_1({ &mut __tmp_35 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_90() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((6_()) as isize)
                    });
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((1_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((10_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((9_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({
                        a.as_mut_ptr().offset((5_()) as isize)
                    });
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_91() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_36: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_36 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_37: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_37 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: sentinel_wrapper_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_38: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({ unimplemented!() });
                    sentinel_wrapper_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_38 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_39: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_39 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_40: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_40 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_41: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_41 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_92() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_42: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_42 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_43: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_43 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: sized_sentinel_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_44: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({ unimplemented!() });
                    sized_sentinel_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_44 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_45: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_45 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_46: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_46 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_char_ptr_ =
                    bidirectional_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_bidirectional_iterator_char_ptr__ = {
                    let mut __tmp_47: bidirectional_iterator_char_ptr_ =
                        bidirectional_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_char_ptr__::new_1({ &mut __tmp_47 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_93() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr().offset((6_()) as isize) });
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr().offset((1_()) as isize) });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr().offset((10_()) as isize) });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr().offset((9_()) as isize) });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_94() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_forward_iterator_char_ptr__ = {
                    let mut __tmp_48: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_char_ptr__::new_1({ &mut __tmp_48 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_forward_iterator_char_ptr__ = {
                    let mut __tmp_49: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_char_ptr__::new_1({ &mut __tmp_49 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: sentinel_wrapper_forward_iterator_char_ptr__ = {
                    let mut __tmp_50: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({ unimplemented!() });
                    sentinel_wrapper_forward_iterator_char_ptr__::new_1({ &mut __tmp_50 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_forward_iterator_char_ptr__ = {
                    let mut __tmp_51: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_char_ptr__::new_1({ &mut __tmp_51 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_forward_iterator_char_ptr__ = {
                    let mut __tmp_52: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_char_ptr__::new_1({ &mut __tmp_52 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_forward_iterator_char_ptr__ = {
                    let mut __tmp_53: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_char_ptr__::new_1({ &mut __tmp_53 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_95() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_forward_iterator_char_ptr__ = {
                    let mut __tmp_54: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_forward_iterator_char_ptr__::new_1({ &mut __tmp_54 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_forward_iterator_char_ptr__ = {
                    let mut __tmp_55: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_forward_iterator_char_ptr__::new_1({ &mut __tmp_55 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ unimplemented!() });
                let ___last: sized_sentinel_forward_iterator_char_ptr__ = {
                    let mut __tmp_56: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({ unimplemented!() });
                    sized_sentinel_forward_iterator_char_ptr__::new_1({ &mut __tmp_56 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_forward_iterator_char_ptr__ = {
                    let mut __tmp_57: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_forward_iterator_char_ptr__::new_1({ &mut __tmp_57 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_forward_iterator_char_ptr__ = {
                    let mut __tmp_58: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_forward_iterator_char_ptr__::new_1({ &mut __tmp_58 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_char_ptr_ =
                    forward_iterator_char_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_forward_iterator_char_ptr__ = {
                    let mut __tmp_59: forward_iterator_char_ptr_ =
                        forward_iterator_char_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_forward_iterator_char_ptr__::new_1({ &mut __tmp_59 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_96() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_60: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_60 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sentinel_wrapper_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_61 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 6_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_61 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_62: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_62 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sentinel_wrapper_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_63 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 1_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_63 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ unimplemented!() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_64: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({ unimplemented!() });
                    sentinel_wrapper_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_64 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sentinel_wrapper_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { unimplemented!()  } , )  } , { { let mut __tmp_65 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { unimplemented!()  } , )  ; sentinel_wrapper_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_65 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_66: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_66 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sentinel_wrapper_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_67 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 10_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_67 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_68: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_68 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sentinel_wrapper_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_69 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 9_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_69 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_70: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_70 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sentinel_wrapper_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_71 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 5_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_71 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_97() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_72: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_72 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sized_sentinel_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_73 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 6_() ) as isize ) } , )  ; sized_sentinel_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_73 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_74: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_74 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sized_sentinel_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_75 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 1_() ) as isize ) } , )  ; sized_sentinel_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_75 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ unimplemented!() });
                let ___last: sized_sentinel_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_76: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({ unimplemented!() });
                    sized_sentinel_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_76 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sized_sentinel_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { unimplemented!()  } , )  } , { { let mut __tmp_77 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { unimplemented!()  } , )  ; sized_sentinel_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_77 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_78: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_78 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sized_sentinel_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_79 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 10_() ) as isize ) } , )  ; sized_sentinel_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_79 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_80: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_80 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sized_sentinel_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_81 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 9_() ) as isize ) } , )  ; sized_sentinel_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_81 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_char_ptr_ =
                    cpp20_input_iterator_char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp20_input_iterator_char_ptr__ = {
                    let mut __tmp_82: cpp20_input_iterator_char_ptr_ =
                        cpp20_input_iterator_char_ptr_::new({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_82 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_char_ptr___sized_sentinel_cpp20_input_iterator_char_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_83 : cpp20_input_iterator_char_ptr_  = cpp20_input_iterator_char_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 5_() ) as isize ) } , )  ; sized_sentinel_cpp20_input_iterator_char_ptr__ :: new_1 ( { & mut __tmp_83 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_98() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_84: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_84
                    })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_85: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_85
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ unimplemented!() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_86: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({ unimplemented!() });
                    sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_86
                    })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_87: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_87
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_88: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_88
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_89: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_89
                    })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_99() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_90: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_90
                    })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_91: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_91
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ unimplemented!() });
                let ___last: sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_92: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({ unimplemented!() });
                    sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_92
                    })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_93: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_93
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_94: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_94
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_char_ptr__char_ptr_ =
                    cpp17_input_iterator_char_ptr__char_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__ = {
                    let mut __tmp_95: cpp17_input_iterator_char_ptr__char_ptr_ =
                        cpp17_input_iterator_char_ptr__char_ptr_::new({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                        &mut __tmp_95
                    })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_100() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((6_()) as isize);
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((1_()) as isize);
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = unimplemented!();
                let ___last: *mut () = unimplemented!();
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((10_()) as isize);
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((9_()) as isize);
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_101() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sentinel_wrapper_long_long_ptr_ = {
                    let mut __tmp_96: *mut () = a.as_mut_ptr().offset((6_()) as isize);
                    sentinel_wrapper_long_long_ptr_::new_1({ &mut __tmp_96 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sentinel_wrapper_long_long_ptr_ = {
                    let mut __tmp_97: *mut () = a.as_mut_ptr().offset((1_()) as isize);
                    sentinel_wrapper_long_long_ptr_::new_1({ &mut __tmp_97 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = unimplemented!();
                let ___last: sentinel_wrapper_long_long_ptr_ = {
                    let mut __tmp_98: *mut () = (unimplemented!() as *mut ());
                    sentinel_wrapper_long_long_ptr_::new_1({ &mut __tmp_98 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sentinel_wrapper_long_long_ptr_ = {
                    let mut __tmp_99: *mut () = a.as_mut_ptr().offset((10_()) as isize);
                    sentinel_wrapper_long_long_ptr_::new_1({ &mut __tmp_99 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sentinel_wrapper_long_long_ptr_ = {
                    let mut __tmp_100: *mut () = a.as_mut_ptr().offset((9_()) as isize);
                    sentinel_wrapper_long_long_ptr_::new_1({ &mut __tmp_100 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sentinel_wrapper_long_long_ptr_ = {
                    let mut __tmp_101: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                    sentinel_wrapper_long_long_ptr_::new_1({ &mut __tmp_101 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_102() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sized_sentinel_long_long_ptr_ = {
                    let mut __tmp_102: *mut () = a.as_mut_ptr().offset((6_()) as isize);
                    sized_sentinel_long_long_ptr_::new_1({ &mut __tmp_102 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sized_sentinel_long_long_ptr_ = {
                    let mut __tmp_103: *mut () = a.as_mut_ptr().offset((1_()) as isize);
                    sized_sentinel_long_long_ptr_::new_1({ &mut __tmp_103 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = unimplemented!();
                let ___last: sized_sentinel_long_long_ptr_ = {
                    let mut __tmp_104: *mut () = (unimplemented!() as *mut ());
                    sized_sentinel_long_long_ptr_::new_1({ &mut __tmp_104 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sized_sentinel_long_long_ptr_ = {
                    let mut __tmp_105: *mut () = a.as_mut_ptr().offset((10_()) as isize);
                    sized_sentinel_long_long_ptr_::new_1({ &mut __tmp_105 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sized_sentinel_long_long_ptr_ = {
                    let mut __tmp_106: *mut () = a.as_mut_ptr().offset((9_()) as isize);
                    sized_sentinel_long_long_ptr_::new_1({ &mut __tmp_106 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: sized_sentinel_long_long_ptr_ = {
                    let mut __tmp_107: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                    sized_sentinel_long_long_ptr_::new_1({ &mut __tmp_107 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_103() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((6_()) as isize)
                    });
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((1_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((10_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((9_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((5_()) as isize)
                    });
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_104() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_108: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_108 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_109: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_109 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: sentinel_wrapper_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_110: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({ unimplemented!() });
                    sentinel_wrapper_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_110 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_111: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_111 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_112: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_112 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_113: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_113 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_105() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_114: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_114 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_115: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_115 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: sized_sentinel_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_116: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({ unimplemented!() });
                    sized_sentinel_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_116 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_117: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_117 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_118: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_118 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: contiguous_iterator_long_long_ptr_ =
                    contiguous_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_contiguous_iterator_long_long_ptr__ = {
                    let mut __tmp_119: contiguous_iterator_long_long_ptr_ =
                        contiguous_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_contiguous_iterator_long_long_ptr__::new_1({ &mut __tmp_119 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_106() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((6_()) as isize)
                    });
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((1_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((10_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((9_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((5_()) as isize)
                    });
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_107() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_120: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_long_long_ptr__::new_1({
                        &mut __tmp_120
                    })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_121: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_long_long_ptr__::new_1({
                        &mut __tmp_121
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: sentinel_wrapper_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_122: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({ unimplemented!() });
                    sentinel_wrapper_random_access_iterator_long_long_ptr__::new_1({
                        &mut __tmp_122
                    })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_123: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_long_long_ptr__::new_1({
                        &mut __tmp_123
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_124: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_long_long_ptr__::new_1({
                        &mut __tmp_124
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_125: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_long_long_ptr__::new_1({
                        &mut __tmp_125
                    })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_108() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_126: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_random_access_iterator_long_long_ptr__::new_1({ &mut __tmp_126 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_127: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_random_access_iterator_long_long_ptr__::new_1({ &mut __tmp_127 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: sized_sentinel_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_128: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({ unimplemented!() });
                    sized_sentinel_random_access_iterator_long_long_ptr__::new_1({ &mut __tmp_128 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_129: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_random_access_iterator_long_long_ptr__::new_1({ &mut __tmp_129 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_130: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_random_access_iterator_long_long_ptr__::new_1({ &mut __tmp_130 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: random_access_iterator_long_long_ptr_ =
                    random_access_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_random_access_iterator_long_long_ptr__ = {
                    let mut __tmp_131: random_access_iterator_long_long_ptr_ =
                        random_access_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_random_access_iterator_long_long_ptr__::new_1({ &mut __tmp_131 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_109() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((6_()) as isize)
                    });
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((1_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((10_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((9_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((5_()) as isize)
                    });
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_110() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_132: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_long_long_ptr__::new_1({
                        &mut __tmp_132
                    })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_133: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_long_long_ptr__::new_1({
                        &mut __tmp_133
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: sentinel_wrapper_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_134: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({ unimplemented!() });
                    sentinel_wrapper_bidirectional_iterator_long_long_ptr__::new_1({
                        &mut __tmp_134
                    })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_135: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_long_long_ptr__::new_1({
                        &mut __tmp_135
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_136: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_long_long_ptr__::new_1({
                        &mut __tmp_136
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_137: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_long_long_ptr__::new_1({
                        &mut __tmp_137
                    })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_111() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_138: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_long_long_ptr__::new_1({ &mut __tmp_138 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_139: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_long_long_ptr__::new_1({ &mut __tmp_139 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: sized_sentinel_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_140: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({ unimplemented!() });
                    sized_sentinel_bidirectional_iterator_long_long_ptr__::new_1({ &mut __tmp_140 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_141: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_long_long_ptr__::new_1({ &mut __tmp_141 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_142: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_long_long_ptr__::new_1({ &mut __tmp_142 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: bidirectional_iterator_long_long_ptr_ =
                    bidirectional_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_bidirectional_iterator_long_long_ptr__ = {
                    let mut __tmp_143: bidirectional_iterator_long_long_ptr_ =
                        bidirectional_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_bidirectional_iterator_long_long_ptr__::new_1({ &mut __tmp_143 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_112() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((6_()) as isize)
                    });
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((1_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((10_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((9_()) as isize)
                    });
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({
                        a.as_mut_ptr().offset((5_()) as isize)
                    });
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_113() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_144: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_144 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_145: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_145 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: sentinel_wrapper_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_146: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({ unimplemented!() });
                    sentinel_wrapper_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_146 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_147: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_147 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_148: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_148 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_149: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_149 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_114() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_150: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_150 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_151: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_151 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ unimplemented!() });
                let ___last: sized_sentinel_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_152: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({ unimplemented!() });
                    sized_sentinel_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_152 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_153: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_153 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_154: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_154 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_long_long_ptr_ =
                    forward_iterator_long_long_ptr_::new_1({ a.as_mut_ptr() });
                let ___last: sized_sentinel_forward_iterator_long_long_ptr__ = {
                    let mut __tmp_155: forward_iterator_long_long_ptr_ =
                        forward_iterator_long_long_ptr_::new_1({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_forward_iterator_long_long_ptr__::new_1({ &mut __tmp_155 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_115() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_156: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_156 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sentinel_wrapper_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_157 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 6_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_157 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_158: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_158 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sentinel_wrapper_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_159 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 1_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_159 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ unimplemented!() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_160: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({ unimplemented!() });
                    sentinel_wrapper_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_160 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sentinel_wrapper_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { unimplemented!()  } , )  } , { { let mut __tmp_161 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { unimplemented!()  } , )  ; sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_161 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_162: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_162 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sentinel_wrapper_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_163 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 10_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_163 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_164: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_164 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sentinel_wrapper_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_165 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 9_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_165 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_166: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_166 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sentinel_wrapper_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_167 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 5_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_167 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_116() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_168: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_168 })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sized_sentinel_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_169 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 6_() ) as isize ) } , )  ; sized_sentinel_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_169 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_170: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_170 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sized_sentinel_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_171 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 1_() ) as isize ) } , )  ; sized_sentinel_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_171 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ unimplemented!() });
                let ___last: sized_sentinel_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_172: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({ unimplemented!() });
                    sized_sentinel_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_172 })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sized_sentinel_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { unimplemented!()  } , )  } , { { let mut __tmp_173 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { unimplemented!()  } , )  ; sized_sentinel_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_173 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_174: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_174 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sized_sentinel_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_175 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 10_() ) as isize ) } , )  ; sized_sentinel_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_175 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_176: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_176 })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sized_sentinel_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_177 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 9_() ) as isize ) } , )  ; sized_sentinel_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_177 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp20_input_iterator_long_long_ptr_ =
                    cpp20_input_iterator_long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp20_input_iterator_long_long_ptr__ = {
                    let mut __tmp_178: cpp20_input_iterator_long_long_ptr_ =
                        cpp20_input_iterator_long_long_ptr_::new({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_cpp20_input_iterator_long_long_ptr__::new_1({ &mut __tmp_178 })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole : () = std_ranges_subrange_cpp20_input_iterator_long_long_ptr___sized_sentinel_cpp20_input_iterator_long_long_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_179 : cpp20_input_iterator_long_long_ptr_  = cpp20_input_iterator_long_long_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 5_() ) as isize ) } , )  ; sized_sentinel_cpp20_input_iterator_long_long_ptr__ :: new_1 ( { & mut __tmp_179 } , )  } } , )  ;
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_117() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_180: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_180
                    })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_181: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_181
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ unimplemented!() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_182: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            unimplemented!()
                        });
                    sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_182
                    })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_183: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_183
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_184: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_184
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_185: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_185
                    })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_iterators_118() {
    {
        let mut a: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_186: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            a.as_mut_ptr().offset((6_()) as isize)
                        });
                    sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_186
                    })
                };
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 3_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 1] = [32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_187: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            a.as_mut_ptr().offset((1_()) as isize)
                        });
                    sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_187
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ unimplemented!() });
                let ___last: sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_188: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            unimplemented!()
                        });
                    sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_188
                    })
                };
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 1_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 10] = [
            32_(),
            3_(),
            2_(),
            1_(),
            0_(),
            23_(),
            21_(),
            9_(),
            40_(),
            100_(),
        ];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_189: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            a.as_mut_ptr().offset((10_()) as isize)
                        });
                    sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_189
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 9] = [3_(), 22_(), 1_(), 43_(), 99_(), 0_(), 56_(), 100_(), 32_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_190: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            a.as_mut_ptr().offset((9_()) as isize)
                        });
                    sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_190
                    })
                };
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 32_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [13_(), 1_(), 21_(), 4_(), 5_()];
        {
            let mut ret: () = (unsafe {
                let ___first: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                    cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({ a.as_mut_ptr() });
                let ___last: sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__ = {
                    let mut __tmp_191: cpp17_input_iterator_long_long_ptr__long_long_ptr_ =
                        cpp17_input_iterator_long_long_ptr__long_long_ptr_::new({
                            a.as_mut_ptr().offset((5_()) as isize)
                        });
                    sized_sentinel_cpp17_input_iterator_long_long_ptr__long_long_ptr__::new_1({
                        &mut __tmp_191
                    })
                };
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = 10_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
        {
            let mut range: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: () = -13_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut range,
                    &mut ___value,
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (-1_()));
                        }
                    }),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_119() -> () {
    (unsafe {
        for_each_0(
            <types_type_list_char__long_long_>::default(),
            lambda_unsafe!(
                Generic,
                {},
                || {
                    (unsafe {
                        for_each_2 ( <types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr___bidirectional_iterator_char_ptr___forward_iterator_char_ptr___cpp20_input_iterator_char_ptr___cpp17_input_iterator_char_ptr__char_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { { ( unsafe { test_iterators_80 ( ) } )  ;
 } ( unsafe { test_iterators_82 ( ) } )  ;
 ( unsafe { test_iterators_83 ( ) } )  ;
 } , | | { { ( unsafe { test_iterators_84 ( ) } )  ;
 } ( unsafe { test_iterators_85 ( ) } )  ;
 ( unsafe { test_iterators_86 ( ) } )  ;
 } , | | { { ( unsafe { test_iterators_87 ( ) } )  ;
 } ( unsafe { test_iterators_88 ( ) } )  ;
 ( unsafe { test_iterators_89 ( ) } )  ;
 } , | | { { ( unsafe { test_iterators_90 ( ) } )  ;
 } ( unsafe { test_iterators_91 ( ) } )  ;
 ( unsafe { test_iterators_92 ( ) } )  ;
 } , | | { { ( unsafe { test_iterators_93 ( ) } )  ;
 } ( unsafe { test_iterators_94 ( ) } )  ;
 ( unsafe { test_iterators_95 ( ) } )  ;
 } , | | { { } ( unsafe { test_iterators_96 ( ) } )  ;
 ( unsafe { test_iterators_97 ( ) } )  ;
 } , | | { { } ( unsafe { test_iterators_98 ( ) } )  ;
 ( unsafe { test_iterators_99 ( ) } )  ;
 } ) , )
                    });
                },
                || {
                    (unsafe {
                        for_each_4 ( <types_type_list_long_long_ptr__contiguous_iterator_long_long_ptr___random_access_iterator_long_long_ptr___bidirectional_iterator_long_long_ptr___forward_iterator_long_long_ptr___cpp20_input_iterator_long_long_ptr___cpp17_input_iterator_long_long_ptr__long_long_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { { ( unsafe { test_iterators_100 ( ) } )  ;
 } ( unsafe { test_iterators_101 ( ) } )  ;
 ( unsafe { test_iterators_102 ( ) } )  ;
 } , | | { { ( unsafe { test_iterators_103 ( ) } )  ;
 } ( unsafe { test_iterators_104 ( ) } )  ;
 ( unsafe { test_iterators_105 ( ) } )  ;
 } , | | { { ( unsafe { test_iterators_106 ( ) } )  ;
 } ( unsafe { test_iterators_107 ( ) } )  ;
 ( unsafe { test_iterators_108 ( ) } )  ;
 } , | | { { ( unsafe { test_iterators_109 ( ) } )  ;
 } ( unsafe { test_iterators_110 ( ) } )  ;
 ( unsafe { test_iterators_111 ( ) } )  ;
 } , | | { { ( unsafe { test_iterators_112 ( ) } )  ;
 } ( unsafe { test_iterators_113 ( ) } )  ;
 ( unsafe { test_iterators_114 ( ) } )  ;
 } , | | { { } ( unsafe { test_iterators_115 ( ) } )  ;
 ( unsafe { test_iterators_116 ( ) } )  ;
 } , | | { { } ( unsafe { test_iterators_117 ( ) } )  ;
 ( unsafe { test_iterators_118 ( ) } )  ;
 } ) , )
                    });
                }
            ),
        )
    });
    {
        let mut a: [(); 5] = [1_(), 9_(), 0_(), 13_(), 25_()];
        let mut projection_count: () = 0_();
        {
            let mut ret: () = (unsafe {
                let ___first: *mut () = a.as_mut_ptr();
                let ___last: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                let mut ___value: () = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    lambda_unsafe!(
                        {
                            let projection_count: *mut () = &mut projection_count;
                        },
                        |mut i: ()| -> () {
                            (*projection_count).prefix_inc();
                            return i;
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
            projection_count = 0_();
        }
        {
            let mut ret: () = (unsafe {
                let mut ___value: () = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut a,
                    &mut ___value,
                    lambda_unsafe!(
                        {
                            let projection_count: *mut () = &mut projection_count;
                        },
                        |mut i: ()| -> () {
                            (*projection_count).prefix_inc();
                            return i;
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let str: () = unimplemented!();
        let str1: () = unimplemented!();
        let mut projection_count: () = 0_();
        {
            let mut a: [(); 5] = [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let ___first: forward_iterator_std_move_iterator_std_string_ptr__ =
                    unimplemented!();
                let ___last: forward_iterator_std_move_iterator_std_string_ptr__ = unimplemented!();
                let mut ___value: *const () = c"hello world".as_ptr();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    lambda_unsafe!(
                        {
                            let projection_count: *mut () = &mut projection_count;
                        },
                        |i: ()| -> () {
                            (*projection_count).prefix_inc();
                            return unimplemented!();
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
            projection_count = 0_();
        }
        {
            let mut a: [(); 5] = [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let mut whole: () = unimplemented!();
            let mut ret: () = (unsafe {
                let mut ___value: *const () = c"hello world".as_ptr();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    lambda_unsafe!(
                        {
                            let projection_count: *mut () = &mut projection_count;
                        },
                        |i: ()| -> () {
                            (*projection_count).prefix_inc();
                            return unimplemented!();
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut whole: () = unimplemented!();
        let mut projection_count: () = 0_();
        {
            let mut ret: () = (unsafe {
                let ___first: () = unimplemented!();
                let ___last: () = unimplemented!();
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    lambda_unsafe!(
                        {
                            let projection_count: *mut () = &mut projection_count;
                        },
                        |mut b: ()| -> () {
                            (*projection_count).prefix_inc();
                            return b;
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
            projection_count = 0_();
        }
        {
            let mut ret: () = (unsafe {
                let mut ___value: () = true;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut whole,
                    &mut ___value,
                    lambda_unsafe!(
                        {
                            let projection_count: *mut () = &mut projection_count;
                        },
                        |mut b: ()| -> () {
                            (*projection_count).prefix_inc();
                            return b;
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let mut projection_count: () = 0_();
        let mut square_number: () = unimplemented!();
        {
            let mut ret: () = (unsafe {
                let ___first: () = unimplemented!();
                let ___last: () = unimplemented!();
                let mut ___value: () = 16_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    ___first,
                    ___last,
                    &mut ___value,
                    lambda_unsafe!(
                        {
                            let projection_count: *mut () = &mut projection_count;
                        },
                        |mut i: ()| -> () {
                            (*projection_count).prefix_inc();
                            return i;
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
            projection_count = 0_();
        }
        {
            let mut ret: () = (unsafe {
                let mut ___value: () = 16_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                    &mut square_number,
                    &mut ___value,
                    lambda_unsafe!(
                        {
                            let projection_count: *mut () = &mut projection_count;
                        },
                        |mut i: ()| -> () {
                            (*projection_count).prefix_inc();
                            return i;
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
        }
    }
    return true;
}
pub unsafe fn test_nonconstexpr_120() -> () {
    let mut a: () = unimplemented!();
    let mut projection_count: () = 0_();
    {
        let mut ret: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            let mut ___value: () = 0_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                ___first,
                ___last,
                &mut ___value,
                lambda_unsafe!(
                    {
                        let projection_count: *mut () = &mut projection_count;
                    },
                    |mut i: ()| -> () {
                        (*projection_count).prefix_inc();
                        return i;
                    }
                ),
            )
        });
        unimplemented!();
        unimplemented!();
        projection_count = 0_();
    }
    {
        let mut ret: () = (unsafe {
            let mut ___value: () = 0_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut contains_81))(
                &mut a,
                &mut ___value,
                lambda_unsafe!(
                    {
                        let projection_count: *mut () = &mut projection_count;
                    },
                    |mut i: ()| -> () {
                        (*projection_count).prefix_inc();
                        return i;
                    }
                ),
            )
        });
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
    (unsafe { test_119() });
    const _: () = assert!(true, "test()");;
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const yes_76);
    std::cell::LazyCell::force(&*&raw const no_77);
    std::cell::LazyCell::force(&*&raw const StrictUnaryPredicate_78);
    std::cell::LazyCell::force(&*&raw const StrictBinaryPredicate_79);
}
