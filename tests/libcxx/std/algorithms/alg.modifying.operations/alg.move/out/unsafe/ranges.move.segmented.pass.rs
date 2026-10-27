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
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut ()) -> Self {
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
impl Default for random_access_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { random_access_iterator_int_ptr_::new_1() }
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
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct contiguous_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut ()) -> Self {
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
impl Default for contiguous_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { contiguous_iterator_int_ptr_::new_1() }
    }
}
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
pub unsafe fn operator_sub_19(s: *const sized_sentinel_int_ptr_, i: *const *mut ()) -> () {
    return (((*s).base_ as usize - (unsafe { base_18((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_20(i: *const *mut (), s: *const sized_sentinel_int_ptr_) -> () {
    return (((unsafe { base_18((*i)) }) as usize - (*s).base_ as usize)
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
            base_: (unsafe { base_18((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        return ((self.base_) == (unsafe { base_18((*other)) }));
    }
}
pub unsafe fn operator_sub_21(
    s: *const sized_sentinel_contiguous_iterator_int_ptr__,
    i: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_17(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_22(
    i: *const contiguous_iterator_int_ptr_,
    s: *const sized_sentinel_contiguous_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_17(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
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
            base_: (unsafe { base_17(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        return ((self.base_) == (unsafe { base_17(other) }));
    }
}
pub unsafe fn operator_sub_23(
    s: *const sized_sentinel_random_access_iterator_int_ptr__,
    i: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_12(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_24(
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
pub unsafe fn operator_sub_25(
    s: *const sized_sentinel_bidirectional_iterator_int_ptr__,
    i: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_7(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_26(
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
pub unsafe fn operator_sub_27(
    s: *const sized_sentinel_forward_iterator_int_ptr__,
    i: *const forward_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_4(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_28(
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
pub unsafe fn operator_eq_29(
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
            operator_eq_29(
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
pub unsafe fn operator_add_30(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_31(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_30(_i, n)
    });
}
pub unsafe fn iter_swap_32(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_33(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*unimplemented!());
}
pub unsafe fn operator_eq_34(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_35(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_35(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_34(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_containers_36() {
    {
        let mut in_: () = unimplemented!();
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(___first, ___last, ___result)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = unimplemented!();
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut in_, unimplemented!())
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_containers_38() {
    {
        let mut in_: () = unimplemented!();
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(___first, ___last, ___result)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = unimplemented!();
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut in_, unimplemented!())
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_containers_39() {
    {
        let mut in_: () = unimplemented!();
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                ___first,
                ___last,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = unimplemented!();
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut in_, unimplemented!())
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_containers_40() {
    {
        let mut in_: () = unimplemented!();
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                ___first,
                ___last,
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: () = unimplemented!();
        let mut out: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut in_, unimplemented!())
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_join_view_41() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_43() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_44() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_45() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_46() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_47() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_48() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_49() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_50() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_51() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_52() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_53() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_54() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_55() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_join_view_56() {
    let mut to_subranges : std_ranges___pipeable_std___bind_back_t_std_ranges_views___transform___fn__std_tuple__lambda_at_ranges_move_segmented_pass_cpp_49_45____ = ( unsafe { let mut ___f : FnPtr<Generic>  = lambda_unsafe! ( Generic , { } , | vec_ : *mut () , | -> () { return unimplemented!()  ;
 } ) ; (*std::cell::LazyCell::force_mut(&mut *&raw mut transform_42)) ( & mut ___f , ) } ) ;
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(
                &mut ___range,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut arr: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut arr, unimplemented!())
        });
        unimplemented!();
    }
    {
        let mut vectors: () = unimplemented!();
        let mut range1: () = unimplemented!();
        let mut subrange_vector: () = unimplemented!();
        let mut to_vectors: () = unimplemented!();
        let mut range2: () = unimplemented!();
        let mut to_subrange_vector: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            let ___result: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut move_37))(&mut ___range, ___result)
        });
        unimplemented!();
    }
}
pub unsafe fn test_constexpr_57() -> () {
    (unsafe { test_containers_36() });
    (unsafe {
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_join_view_41 ( ) } )  ;
 ( unsafe { test_join_view_43 ( ) } )  ;
 ( unsafe { test_join_view_44 ( ) } )  ;
 } , | | { ( unsafe { test_join_view_45 ( ) } )  ;
 ( unsafe { test_join_view_46 ( ) } )  ;
 ( unsafe { test_join_view_47 ( ) } )  ;
 } , | | { ( unsafe { test_join_view_48 ( ) } )  ;
 ( unsafe { test_join_view_49 ( ) } )  ;
 ( unsafe { test_join_view_50 ( ) } )  ;
 } , | | { ( unsafe { test_join_view_51 ( ) } )  ;
 ( unsafe { test_join_view_52 ( ) } )  ;
 ( unsafe { test_join_view_53 ( ) } )  ;
 } , | | { ( unsafe { test_join_view_54 ( ) } )  ;
 ( unsafe { test_join_view_55 ( ) } )  ;
 ( unsafe { test_join_view_56 ( ) } )  ;
 } ) , )
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
    (unsafe { test_containers_38() });
    (unsafe { test_containers_39() });
    (unsafe { test_containers_40() });
    (unsafe { test_constexpr_57() });
    const _: () = assert!(true, "test_constexpr()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
