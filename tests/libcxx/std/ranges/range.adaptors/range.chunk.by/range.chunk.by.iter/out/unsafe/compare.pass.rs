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
pub struct types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr__
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
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr____types_type_list_bidirectional_iterator_const_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr____types_type_list_forward_iterator_const_int_ptr___
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__,
    mut f: TestWithPair,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { TestWithPair::operator_call_1_int_ptr(&f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { TestWithPair::operator_call_1_class_contiguous_iterator_int_ptr_(&f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { TestWithPair::operator_call_1_class_random_access_iterator_int_ptr_(&f) });
            0_()
        };
        let _arg3: () = {
            (unsafe { TestWithPair::operator_call_1_class_bidirectional_iterator_int_ptr_(&f) });
            0_()
        };
        let _arg4: () = {
            (unsafe { TestWithPair::operator_call_1_class_forward_iterator_int_ptr_(&f) });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_2(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr__,
    mut f: TestWithPair,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { TestWithPair::operator_call_1_const_int_ptr(&f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { TestWithPair::operator_call_1_class_contiguous_iterator_const_int_ptr_(&f) });
            0_()
        };
        let _arg2: () = {
            (unsafe {
                TestWithPair::operator_call_1_class_random_access_iterator_const_int_ptr_(&f)
            });
            0_()
        };
        let _arg3: () = {
            (unsafe {
                TestWithPair::operator_call_1_class_bidirectional_iterator_const_int_ptr_(&f)
            });
            0_()
        };
        let _arg4: () = {
            (unsafe { TestWithPair::operator_call_1_class_forward_iterator_const_int_ptr_(&f) });
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
pub unsafe fn operator_eq_3(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_4(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_5(i: *const forward_iterator_int_ptr_) -> *mut () {
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
            operator_eq_3(
                self as *const forward_iterator_int_ptr_,
                other as *const forward_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_int_ptr_ {}
pub unsafe fn operator_eq_6(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_7(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_8(i: *const forward_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
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
impl std::cmp::PartialEq for forward_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const forward_iterator_const_int_ptr_,
                other as *const forward_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_int_ptr_ {}
pub unsafe fn operator_eq_9(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_10(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_11(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
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
            operator_eq_9(
                self as *const bidirectional_iterator_int_ptr_,
                other as *const bidirectional_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
pub unsafe fn operator_eq_12(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_13(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_14(i: *const bidirectional_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
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
    pub unsafe fn operator_dec_7(&mut self) -> *mut bidirectional_iterator_const_int_ptr_ {
        self.it_.prefix_dec();
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
impl std::cmp::PartialEq for bidirectional_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const bidirectional_iterator_const_int_ptr_,
                other as *const bidirectional_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
pub unsafe fn operator_sub_15(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_16(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_17(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_18(i: *const random_access_iterator_int_ptr_) -> *mut () {
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
            operator_eq_16(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
pub unsafe fn operator_sub_19(
    mut x: random_access_iterator_const_int_ptr_,
    mut y: random_access_iterator_const_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_20(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_21(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_22(i: *const random_access_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
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
impl std::cmp::PartialEq for random_access_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_20(
                self as *const random_access_iterator_const_int_ptr_,
                other as *const random_access_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
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
pub unsafe fn operator_sub_23(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_24(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_25(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_26(i: *const contiguous_iterator_int_ptr_) -> *mut () {
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
            operator_eq_24(
                self as *const contiguous_iterator_int_ptr_,
                other as *const contiguous_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_int_ptr_ {}
pub unsafe fn operator_sub_27(
    mut x: contiguous_iterator_const_int_ptr_,
    mut y: contiguous_iterator_const_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_28(
    x: *const contiguous_iterator_const_int_ptr_,
    y: *const contiguous_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_29(
    x: *const contiguous_iterator_const_int_ptr_,
    y: *const contiguous_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_30(i: *const contiguous_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
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
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_const_int_ptr_,
    ) -> *mut contiguous_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_const_int_ptr_));
    }
}
impl std::cmp::PartialEq for contiguous_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_28(
                self as *const contiguous_iterator_const_int_ptr_,
                other as *const contiguous_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_const_int_ptr_ {}
pub unsafe fn base_31(mut i: *mut ()) -> *mut () {
    return i;
}
pub unsafe fn base_32(mut i: *const ()) -> *const () {
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
pub struct sentinel_wrapper_int_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_int_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_31((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_31((*other)) }));
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
            base_: (unsafe { base_26(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_26(other) }));
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
            base_: (unsafe { base_18(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_18(other) }));
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
            base_: (unsafe { base_11(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_int_ptr_) -> () {
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
            base_: (unsafe { base_5(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_5(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_const_int_ptr_ {
    base_: *const (),
}
impl sentinel_wrapper_const_int_ptr_ {
    pub unsafe fn new_1(it: *const *const ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_32((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *const ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_32((*other)) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_contiguous_iterator_const_int_ptr__ {
    base_: *const (),
}
impl sentinel_wrapper_contiguous_iterator_const_int_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_30(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_const_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_30(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_random_access_iterator_const_int_ptr__ {
    base_: *const (),
}
impl sentinel_wrapper_random_access_iterator_const_int_ptr__ {
    pub unsafe fn new_1(it: *const random_access_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_22(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_const_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_22(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_bidirectional_iterator_const_int_ptr__ {
    base_: *const (),
}
impl sentinel_wrapper_bidirectional_iterator_const_int_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_14(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_const_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_14(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_const_int_ptr__ {
    base_: *const (),
}
impl sentinel_wrapper_forward_iterator_const_int_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_8(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_const_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_8(other) }));
    }
}
pub unsafe fn operator_eq_33(
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
            operator_eq_33(
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
pub unsafe fn operator_add_34(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_35(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_34(_i, n)
    });
}
pub unsafe fn iter_swap_36(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_37(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_38(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_39(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_39(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_38(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, Default)]
pub struct TrackInitialization {
    pub moved_: *mut (),
    pub copied_: *mut (),
}
impl TrackInitialization {
    pub unsafe fn new(mut moved: *mut (), mut copied: *mut ()) -> Self {
        let mut this = Self {
            moved_: moved,
            copied_: copied,
        };
        this
    }
    pub unsafe fn copy_from(other: *const TrackInitialization) -> Self {
        let mut this = Self {
            moved_: (*other).moved_,
            copied_: (*other).copied_,
        };
        (*this.copied_) = true;
        this
    }
    pub unsafe fn move_from(other: *mut TrackInitialization) -> Self {
        let mut this = Self {
            moved_: (*other).moved_,
            copied_: (*other).copied_,
        };
        (*this.moved_) = true;
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const TrackInitialization,
    ) -> *mut TrackInitialization {
        self.moved_ = (*_a0).moved_;
        self.copied_ = (*_a0).copied_;
        return &mut (*(self as *mut TrackInitialization));
    }
}
impl Clone for TrackInitialization {
    fn clone(&self) -> Self {
        unsafe { TrackInitialization::copy_from(self as *const TrackInitialization) }
    }
}
pub type IsConst = ();
pub const IsConst_no: IsConst = false;
pub const IsConst_yes: IsConst = true;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_int_ptr__int_ptr_ {
    begin_: *mut (),
    end_: *mut (),
}
impl View_int_ptr__int_ptr_ {
    pub unsafe fn new(mut b: *mut (), mut e: *mut ()) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> *mut () {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> *mut () {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_int_ptr__sentinel_wrapper_int_ptr__ {
    begin_: *mut (),
    end_: sentinel_wrapper_int_ptr_,
}
impl View_int_ptr__sentinel_wrapper_int_ptr__ {
    pub unsafe fn new(mut b: *mut (), mut e: sentinel_wrapper_int_ptr_) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> *mut () {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> sentinel_wrapper_int_ptr_ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr__ {
    begin_: contiguous_iterator_int_ptr_,
    end_: contiguous_iterator_int_ptr_,
}
impl View_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr__ {
    pub unsafe fn new(
        mut b: contiguous_iterator_int_ptr_,
        mut e: contiguous_iterator_int_ptr_,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> contiguous_iterator_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> contiguous_iterator_int_ptr_ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_contiguous_iterator_int_ptr___sentinel_wrapper_contiguous_iterator_int_ptr___ {
    begin_: contiguous_iterator_int_ptr_,
    end_: sentinel_wrapper_contiguous_iterator_int_ptr__,
}
impl View_contiguous_iterator_int_ptr___sentinel_wrapper_contiguous_iterator_int_ptr___ {
    pub unsafe fn new(
        mut b: contiguous_iterator_int_ptr_,
        mut e: sentinel_wrapper_contiguous_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> contiguous_iterator_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> sentinel_wrapper_contiguous_iterator_int_ptr__ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_random_access_iterator_int_ptr___random_access_iterator_int_ptr__ {
    begin_: random_access_iterator_int_ptr_,
    end_: random_access_iterator_int_ptr_,
}
impl View_random_access_iterator_int_ptr___random_access_iterator_int_ptr__ {
    pub unsafe fn new(
        mut b: random_access_iterator_int_ptr_,
        mut e: random_access_iterator_int_ptr_,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> random_access_iterator_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> random_access_iterator_int_ptr_ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_random_access_iterator_int_ptr___sentinel_wrapper_random_access_iterator_int_ptr___
{
    begin_: random_access_iterator_int_ptr_,
    end_: sentinel_wrapper_random_access_iterator_int_ptr__,
}
impl View_random_access_iterator_int_ptr___sentinel_wrapper_random_access_iterator_int_ptr___ {
    pub unsafe fn new(
        mut b: random_access_iterator_int_ptr_,
        mut e: sentinel_wrapper_random_access_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> random_access_iterator_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> sentinel_wrapper_random_access_iterator_int_ptr__ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr__ {
    begin_: bidirectional_iterator_int_ptr_,
    end_: bidirectional_iterator_int_ptr_,
}
impl View_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr__ {
    pub unsafe fn new(
        mut b: bidirectional_iterator_int_ptr_,
        mut e: bidirectional_iterator_int_ptr_,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> bidirectional_iterator_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> bidirectional_iterator_int_ptr_ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_bidirectional_iterator_int_ptr___sentinel_wrapper_bidirectional_iterator_int_ptr___
{
    begin_: bidirectional_iterator_int_ptr_,
    end_: sentinel_wrapper_bidirectional_iterator_int_ptr__,
}
impl View_bidirectional_iterator_int_ptr___sentinel_wrapper_bidirectional_iterator_int_ptr___ {
    pub unsafe fn new(
        mut b: bidirectional_iterator_int_ptr_,
        mut e: sentinel_wrapper_bidirectional_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> bidirectional_iterator_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> sentinel_wrapper_bidirectional_iterator_int_ptr__ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_forward_iterator_int_ptr___forward_iterator_int_ptr__ {
    begin_: forward_iterator_int_ptr_,
    end_: forward_iterator_int_ptr_,
}
impl View_forward_iterator_int_ptr___forward_iterator_int_ptr__ {
    pub unsafe fn new(mut b: forward_iterator_int_ptr_, mut e: forward_iterator_int_ptr_) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> forward_iterator_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> forward_iterator_int_ptr_ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_forward_iterator_int_ptr___sentinel_wrapper_forward_iterator_int_ptr___ {
    begin_: forward_iterator_int_ptr_,
    end_: sentinel_wrapper_forward_iterator_int_ptr__,
}
impl View_forward_iterator_int_ptr___sentinel_wrapper_forward_iterator_int_ptr___ {
    pub unsafe fn new(
        mut b: forward_iterator_int_ptr_,
        mut e: sentinel_wrapper_forward_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> forward_iterator_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> sentinel_wrapper_forward_iterator_int_ptr__ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_const_int_ptr__const_int_ptr_ {
    begin_: *const (),
    end_: *const (),
}
impl View_const_int_ptr__const_int_ptr_ {
    pub unsafe fn new(mut b: *const (), mut e: *const ()) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> *const () {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> *const () {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_const_int_ptr__sentinel_wrapper_const_int_ptr__ {
    begin_: *const (),
    end_: sentinel_wrapper_const_int_ptr_,
}
impl View_const_int_ptr__sentinel_wrapper_const_int_ptr__ {
    pub unsafe fn new(mut b: *const (), mut e: sentinel_wrapper_const_int_ptr_) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> *const () {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> sentinel_wrapper_const_int_ptr_ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_contiguous_iterator_const_int_ptr___contiguous_iterator_const_int_ptr__ {
    begin_: contiguous_iterator_const_int_ptr_,
    end_: contiguous_iterator_const_int_ptr_,
}
impl View_contiguous_iterator_const_int_ptr___contiguous_iterator_const_int_ptr__ {
    pub unsafe fn new(
        mut b: contiguous_iterator_const_int_ptr_,
        mut e: contiguous_iterator_const_int_ptr_,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> contiguous_iterator_const_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> contiguous_iterator_const_int_ptr_ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_contiguous_iterator_const_int_ptr___sentinel_wrapper_contiguous_iterator_const_int_ptr___
{
    begin_: contiguous_iterator_const_int_ptr_,
    end_: sentinel_wrapper_contiguous_iterator_const_int_ptr__,
}
impl
    View_contiguous_iterator_const_int_ptr___sentinel_wrapper_contiguous_iterator_const_int_ptr___
{
    pub unsafe fn new(
        mut b: contiguous_iterator_const_int_ptr_,
        mut e: sentinel_wrapper_contiguous_iterator_const_int_ptr__,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> contiguous_iterator_const_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> sentinel_wrapper_contiguous_iterator_const_int_ptr__ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_random_access_iterator_const_int_ptr___random_access_iterator_const_int_ptr__ {
    begin_: random_access_iterator_const_int_ptr_,
    end_: random_access_iterator_const_int_ptr_,
}
impl View_random_access_iterator_const_int_ptr___random_access_iterator_const_int_ptr__ {
    pub unsafe fn new(
        mut b: random_access_iterator_const_int_ptr_,
        mut e: random_access_iterator_const_int_ptr_,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> random_access_iterator_const_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> random_access_iterator_const_int_ptr_ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_random_access_iterator_const_int_ptr___sentinel_wrapper_random_access_iterator_const_int_ptr___
{
    begin_: random_access_iterator_const_int_ptr_,
    end_: sentinel_wrapper_random_access_iterator_const_int_ptr__,
}
impl View_random_access_iterator_const_int_ptr___sentinel_wrapper_random_access_iterator_const_int_ptr___ { pub unsafe fn new ( mut b : random_access_iterator_const_int_ptr_ , mut e : sentinel_wrapper_random_access_iterator_const_int_ptr__ , ) -> Self { let mut this = Self { begin_ : b , end_ : e , } ;
 this } pub unsafe fn begin ( &mut self , ) -> random_access_iterator_const_int_ptr_ { return self . begin_   ;
 } pub unsafe fn end ( &mut self , ) -> sentinel_wrapper_random_access_iterator_const_int_ptr__ { return self . end_   ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_bidirectional_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr__ {
    begin_: bidirectional_iterator_const_int_ptr_,
    end_: bidirectional_iterator_const_int_ptr_,
}
impl View_bidirectional_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr__ {
    pub unsafe fn new(
        mut b: bidirectional_iterator_const_int_ptr_,
        mut e: bidirectional_iterator_const_int_ptr_,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> bidirectional_iterator_const_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> bidirectional_iterator_const_int_ptr_ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_bidirectional_iterator_const_int_ptr___sentinel_wrapper_bidirectional_iterator_const_int_ptr___
{
    begin_: bidirectional_iterator_const_int_ptr_,
    end_: sentinel_wrapper_bidirectional_iterator_const_int_ptr__,
}
impl View_bidirectional_iterator_const_int_ptr___sentinel_wrapper_bidirectional_iterator_const_int_ptr___ { pub unsafe fn new ( mut b : bidirectional_iterator_const_int_ptr_ , mut e : sentinel_wrapper_bidirectional_iterator_const_int_ptr__ , ) -> Self { let mut this = Self { begin_ : b , end_ : e , } ;
 this } pub unsafe fn begin ( &mut self , ) -> bidirectional_iterator_const_int_ptr_ { return self . begin_   ;
 } pub unsafe fn end ( &mut self , ) -> sentinel_wrapper_bidirectional_iterator_const_int_ptr__ { return self . end_   ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_forward_iterator_const_int_ptr___forward_iterator_const_int_ptr__ {
    begin_: forward_iterator_const_int_ptr_,
    end_: forward_iterator_const_int_ptr_,
}
impl View_forward_iterator_const_int_ptr___forward_iterator_const_int_ptr__ {
    pub unsafe fn new(
        mut b: forward_iterator_const_int_ptr_,
        mut e: forward_iterator_const_int_ptr_,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> forward_iterator_const_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> forward_iterator_const_int_ptr_ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct View_forward_iterator_const_int_ptr___sentinel_wrapper_forward_iterator_const_int_ptr___
{
    begin_: forward_iterator_const_int_ptr_,
    end_: sentinel_wrapper_forward_iterator_const_int_ptr__,
}
impl View_forward_iterator_const_int_ptr___sentinel_wrapper_forward_iterator_const_int_ptr___ {
    pub unsafe fn new(
        mut b: forward_iterator_const_int_ptr_,
        mut e: sentinel_wrapper_forward_iterator_const_int_ptr__,
    ) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&mut self) -> forward_iterator_const_int_ptr_ {
        return self.begin_;
    }
    pub unsafe fn end(&mut self) -> sentinel_wrapper_forward_iterator_const_int_ptr__ {
        return self.end_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntWrapper {
    pub value_: (),
}
impl IntWrapper {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value_: v };
        this
    }
    pub unsafe fn lessEqual(&self, mut other: IntWrapper) -> () {
        return ((self.value_) <= (other.value_));
    }
}
pub unsafe fn test_40() {
    let mut make_chunk_by_view: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |arr: *mut ()| -> () {
            let mut view: View_int_ptr__int_ptr_ =
                View_int_ptr__int_ptr_::new({ unimplemented!() }, {
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            return unimplemented!();
        });
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_41() {
    let mut make_chunk_by_view: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |arr: *mut ()| -> () {
            let mut view: View_int_ptr__sentinel_wrapper_int_ptr__ =
                View_int_ptr__sentinel_wrapper_int_ptr__::new({ unimplemented!() }, {
                    {
                        let mut __tmp_0: *mut () =
                            unimplemented!().offset((unimplemented!()) as isize);
                        sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_0 })
                    }
                });
            return unimplemented!();
        });
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_42() {
    let mut make_chunk_by_view: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |arr: *mut ()| -> () {
            let mut view: View_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr__ =
                View_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr__::new(
                    { contiguous_iterator_int_ptr_::new_1({ unimplemented!() }) },
                    {
                        contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })
                    },
                );
            return unimplemented!();
        });
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_43() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_contiguous_iterator_int_ptr___sentinel_wrapper_contiguous_iterator_int_ptr___ = View_contiguous_iterator_int_ptr___sentinel_wrapper_contiguous_iterator_int_ptr___ :: new ( { contiguous_iterator_int_ptr_ :: new_1 ( { unimplemented!()  } , )  } , { { let mut __tmp_1 : contiguous_iterator_int_ptr_  = contiguous_iterator_int_ptr_ :: new_1 ( { unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) } , )  ; sentinel_wrapper_contiguous_iterator_int_ptr__ :: new_1 ( { & mut __tmp_1 } , )  } } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_44() {
    let mut make_chunk_by_view: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |arr: *mut ()| -> () {
            let mut view: View_random_access_iterator_int_ptr___random_access_iterator_int_ptr__ =
                View_random_access_iterator_int_ptr___random_access_iterator_int_ptr__::new(
                    { random_access_iterator_int_ptr_::new_1({ unimplemented!() }) },
                    {
                        random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })
                    },
                );
            return unimplemented!();
        });
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_45() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_random_access_iterator_int_ptr___sentinel_wrapper_random_access_iterator_int_ptr___ = View_random_access_iterator_int_ptr___sentinel_wrapper_random_access_iterator_int_ptr___ :: new ( { random_access_iterator_int_ptr_ :: new_1 ( { unimplemented!()  } , )  } , { { let mut __tmp_2 : random_access_iterator_int_ptr_  = random_access_iterator_int_ptr_ :: new_1 ( { unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) } , )  ; sentinel_wrapper_random_access_iterator_int_ptr__ :: new_1 ( { & mut __tmp_2 } , )  } } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_46() {
    let mut make_chunk_by_view: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |arr: *mut ()| -> () {
            let mut view: View_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr__ =
                View_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr__::new(
                    { bidirectional_iterator_int_ptr_::new_1({ unimplemented!() }) },
                    {
                        bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })
                    },
                );
            return unimplemented!();
        });
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_47() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_bidirectional_iterator_int_ptr___sentinel_wrapper_bidirectional_iterator_int_ptr___ = View_bidirectional_iterator_int_ptr___sentinel_wrapper_bidirectional_iterator_int_ptr___ :: new ( { bidirectional_iterator_int_ptr_ :: new_1 ( { unimplemented!()  } , )  } , { { let mut __tmp_3 : bidirectional_iterator_int_ptr_  = bidirectional_iterator_int_ptr_ :: new_1 ( { unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) } , )  ; sentinel_wrapper_bidirectional_iterator_int_ptr__ :: new_1 ( { & mut __tmp_3 } , )  } } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_48() {
    let mut make_chunk_by_view: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |arr: *mut ()| -> () {
            let mut view: View_forward_iterator_int_ptr___forward_iterator_int_ptr__ =
                View_forward_iterator_int_ptr___forward_iterator_int_ptr__::new(
                    { forward_iterator_int_ptr_::new_1({ unimplemented!() }) },
                    {
                        forward_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })
                    },
                );
            return unimplemented!();
        });
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_49() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_forward_iterator_int_ptr___sentinel_wrapper_forward_iterator_int_ptr___ = View_forward_iterator_int_ptr___sentinel_wrapper_forward_iterator_int_ptr___ :: new ( { forward_iterator_int_ptr_ :: new_1 ( { unimplemented!()  } , )  } , { { let mut __tmp_4 : forward_iterator_int_ptr_  = forward_iterator_int_ptr_ :: new_1 ( { unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) } , )  ; sentinel_wrapper_forward_iterator_int_ptr__ :: new_1 ( { & mut __tmp_4 } , )  } } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_50() {
    let mut make_chunk_by_view: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |arr: *mut ()| -> () {
            let mut view: View_const_int_ptr__const_int_ptr_ =
                View_const_int_ptr__const_int_ptr_::new({ unimplemented!() }, {
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            return unimplemented!();
        });
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_51() {
    let mut make_chunk_by_view: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |arr: *mut ()| -> () {
            let mut view: View_const_int_ptr__sentinel_wrapper_const_int_ptr__ =
                View_const_int_ptr__sentinel_wrapper_const_int_ptr__::new({ unimplemented!() }, {
                    {
                        let mut __tmp_5: *const () =
                            unimplemented!().offset((unimplemented!()) as isize);
                        sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_5 })
                    }
                });
            return unimplemented!();
        });
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_52() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_contiguous_iterator_const_int_ptr___contiguous_iterator_const_int_ptr__ = View_contiguous_iterator_const_int_ptr___contiguous_iterator_const_int_ptr__ :: new ( { contiguous_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  ) .cast_const() } , )  } , { contiguous_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) ) .cast_const() } , )  } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_53() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_contiguous_iterator_const_int_ptr___sentinel_wrapper_contiguous_iterator_const_int_ptr___ = View_contiguous_iterator_const_int_ptr___sentinel_wrapper_contiguous_iterator_const_int_ptr___ :: new ( { contiguous_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  ) .cast_const() } , )  } , { { let mut __tmp_6 : contiguous_iterator_const_int_ptr_  = contiguous_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) ) .cast_const() } , )  ; sentinel_wrapper_contiguous_iterator_const_int_ptr__ :: new_1 ( { & mut __tmp_6 } , )  } } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_54() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_random_access_iterator_const_int_ptr___random_access_iterator_const_int_ptr__ = View_random_access_iterator_const_int_ptr___random_access_iterator_const_int_ptr__ :: new ( { random_access_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  ) .cast_const() } , )  } , { random_access_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) ) .cast_const() } , )  } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_55() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_random_access_iterator_const_int_ptr___sentinel_wrapper_random_access_iterator_const_int_ptr___ = View_random_access_iterator_const_int_ptr___sentinel_wrapper_random_access_iterator_const_int_ptr___ :: new ( { random_access_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  ) .cast_const() } , )  } , { { let mut __tmp_7 : random_access_iterator_const_int_ptr_  = random_access_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) ) .cast_const() } , )  ; sentinel_wrapper_random_access_iterator_const_int_ptr__ :: new_1 ( { & mut __tmp_7 } , )  } } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_56() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_bidirectional_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr__ = View_bidirectional_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr__ :: new ( { bidirectional_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  ) .cast_const() } , )  } , { bidirectional_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) ) .cast_const() } , )  } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_57() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_bidirectional_iterator_const_int_ptr___sentinel_wrapper_bidirectional_iterator_const_int_ptr___ = View_bidirectional_iterator_const_int_ptr___sentinel_wrapper_bidirectional_iterator_const_int_ptr___ :: new ( { bidirectional_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  ) .cast_const() } , )  } , { { let mut __tmp_8 : bidirectional_iterator_const_int_ptr_  = bidirectional_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) ) .cast_const() } , )  ; sentinel_wrapper_bidirectional_iterator_const_int_ptr__ :: new_1 ( { & mut __tmp_8 } , )  } } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_58() {
    let mut make_chunk_by_view: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |arr: *mut ()| -> () {
            let mut view: View_forward_iterator_const_int_ptr___forward_iterator_const_int_ptr__ =
                View_forward_iterator_const_int_ptr___forward_iterator_const_int_ptr__::new(
                    { forward_iterator_const_int_ptr_::new_1({ (unimplemented!()).cast_const() }) },
                    {
                        forward_iterator_const_int_ptr_::new_1({
                            (unimplemented!().offset((unimplemented!()) as isize)).cast_const()
                        })
                    },
                );
            return unimplemented!();
        });
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_59() {
    let mut make_chunk_by_view: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |arr: *mut ()| -> () {
            let mut view : View_forward_iterator_const_int_ptr___sentinel_wrapper_forward_iterator_const_int_ptr___ = View_forward_iterator_const_int_ptr___sentinel_wrapper_forward_iterator_const_int_ptr___ :: new ( { forward_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  ) .cast_const() } , )  } , { { let mut __tmp_9 : forward_iterator_const_int_ptr_  = forward_iterator_const_int_ptr_ :: new_1 ( { ( unimplemented!()  . offset ( ( unimplemented!()  ) as isize ) ) .cast_const() } , )  ; sentinel_wrapper_forward_iterator_const_int_ptr__ :: new_1 ( { & mut __tmp_9 } , )  } } , )  ;
            return unimplemented!();
        }
    );
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut view: () = (unsafe {
            make_chunk_by_view
                .spec::<fn(*mut ()) -> ()>(0)
                .call(&mut array)
        });
        let mut i: () = unimplemented!();
        let mut result: () = (!(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestWithPair {}
impl TestWithPair {
    pub unsafe fn operator_call_1_int_ptr(&self) {
        (unsafe { test_40() });
        (unsafe { test_41() });
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_int_ptr_(&self) {
        (unsafe { test_42() });
        (unsafe { test_43() });
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_int_ptr_(&self) {
        (unsafe { test_44() });
        (unsafe { test_45() });
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self) {
        (unsafe { test_46() });
        (unsafe { test_47() });
    }
    pub unsafe fn operator_call_1_class_forward_iterator_int_ptr_(&self) {
        (unsafe { test_48() });
        (unsafe { test_49() });
    }
    pub unsafe fn operator_call_1_const_int_ptr(&self) {
        (unsafe { test_50() });
        (unsafe { test_51() });
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&self) {
        (unsafe { test_52() });
        (unsafe { test_53() });
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_int_ptr_(&self) {
        (unsafe { test_54() });
        (unsafe { test_55() });
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&self) {
        (unsafe { test_56() });
        (unsafe { test_57() });
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_int_ptr_(&self) {
        (unsafe { test_58() });
        (unsafe { test_59() });
    }
}
pub unsafe fn tests_60() -> () {
    let mut tester: TestWithPair = <TestWithPair>::default();
    (unsafe {
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , tester , )
    });
    (unsafe {
        for_each_2 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr__ >::default() , tester , )
    });
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
    (unsafe { tests_60() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
