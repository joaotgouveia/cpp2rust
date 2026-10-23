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
pub struct test_macros_detail_is_same_long__long_ {}
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
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_output_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_output_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
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
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
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
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
}
pub unsafe fn operator_ne_5(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_6(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
}
pub unsafe fn operator_sub_7(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn base_8(i: *const random_access_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
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
pub unsafe fn operator_sub_9(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn base_10(i: *const contiguous_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn base_11(mut i: *mut ()) -> *mut () {
    return i;
}
pub unsafe fn base_12(mut i: *const ()) -> *const () {
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
pub struct sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_2(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp17_input_iterator_int_ptr__int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_2(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp17_output_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp17_output_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_1(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp17_output_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_1(other) }));
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_6(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_6(other) }));
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
pub struct sentinel_wrapper_contiguous_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_10(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_10(other) }));
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
            base_: (unsafe { base_11((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_11((*other)) }));
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
            base_: (unsafe { base_12((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *const ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_12((*other)) }));
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
pub unsafe fn operator_sub_14(
    s: *const sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__,
    i: *const cpp17_input_iterator_int_ptr__int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_2(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_15(
    i: *const cpp17_input_iterator_int_ptr__int_ptr_,
    s: *const sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__,
) -> () {
    return (((unsafe { base_2(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_2(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_16(
    s: *const sized_sentinel_cpp20_input_iterator_int_ptr__,
    i: *const cpp20_input_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_13(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_17(
    i: *const cpp20_input_iterator_int_ptr_,
    s: *const sized_sentinel_cpp20_input_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_13(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_cpp20_input_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_13(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_18(
    s: *const sized_sentinel_cpp17_output_iterator_int_ptr__,
    i: *const cpp17_output_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_1(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_19(
    i: *const cpp17_output_iterator_int_ptr_,
    s: *const sized_sentinel_cpp17_output_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_1(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_cpp17_output_iterator_int_ptr__ {
    base_: *mut (),
}
impl sized_sentinel_cpp17_output_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_1(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_20(
    s: *const sized_sentinel_forward_iterator_int_ptr__,
    i: *const forward_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_4(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_21(
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
}
pub unsafe fn operator_sub_22(
    s: *const sized_sentinel_bidirectional_iterator_int_ptr__,
    i: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_6(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_23(
    i: *const bidirectional_iterator_int_ptr_,
    s: *const sized_sentinel_bidirectional_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_6(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
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
            base_: (unsafe { base_6(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_24(
    s: *const sized_sentinel_random_access_iterator_int_ptr__,
    i: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_8(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_25(
    i: *const random_access_iterator_int_ptr_,
    s: *const sized_sentinel_random_access_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_8(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
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
            base_: (unsafe { base_8(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_26(
    s: *const sized_sentinel_contiguous_iterator_int_ptr__,
    i: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_10(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_27(
    i: *const contiguous_iterator_int_ptr_,
    s: *const sized_sentinel_contiguous_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_10(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
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
            base_: (unsafe { base_10(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_28(s: *const sized_sentinel_int_ptr_, i: *const *mut ()) -> () {
    return (((*s).base_ as usize - (unsafe { base_11((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_29(i: *const *mut (), s: *const sized_sentinel_int_ptr_) -> () {
    return (((unsafe { base_11((*i)) }) as usize - (*s).base_ as usize)
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
            base_: (unsafe { base_11((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_30(s: *const sized_sentinel_const_int_ptr_, i: *const *const ()) -> () {
    return (((*s).base_ as usize - (unsafe { base_12((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_31(i: *const *const (), s: *const sized_sentinel_const_int_ptr_) -> () {
    return (((unsafe { base_12((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<()>()) as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_int_ptr_ {
    base_: *const (),
}
impl sized_sentinel_const_int_ptr_ {
    pub unsafe fn new_1(it: *const *const ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_12((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_eq_32(
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
            operator_eq_32(
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
pub unsafe fn operator_add_33(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_34(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_33(_i, n)
    });
}
pub unsafe fn iter_swap_35(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_36(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
}
pub unsafe fn operator_eq_37(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_38(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_38(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_37(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_unsized_39() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
        let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_0: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_0 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_1: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_1
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_2: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_2
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_3: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_3
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_4: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_4
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_5: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_5
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_6: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_6
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_7: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_7
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_8: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_8
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_9: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_9
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_10: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_10
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_11: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_11
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_12: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_12
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_13: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_13
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_14: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_14
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_15: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_15
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_16: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_16
                    })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
}
pub unsafe fn test_unsized_40() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut first: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
            let mut __tmp_17: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ a.as_mut_ptr() });
            sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_17 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_18: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_18 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_19: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_19 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_20: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_20 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_21: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_21 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_22: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_22 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_23: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_23 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_24: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_24 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_25: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_25 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_26: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_26 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_27: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_27 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_28: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_28 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_29: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_29 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_30: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_30 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_31: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_31 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_32: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_32 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_33: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_33 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
}
pub unsafe fn test_unsized_41() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut first: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let mut __tmp_34: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_34 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_35: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_35 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_36: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_36 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_37: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_37 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_38: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_38 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_39: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_39 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_40: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_40 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_41: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_41 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_42: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_42 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_43: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_43 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_44: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_44 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_45: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_45 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_46: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_46 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_47: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_47 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_48: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_48 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_49: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_49 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let mut __tmp_50: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_50 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
}
pub unsafe fn test_unsized_42() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut first: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_51: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_51 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_52: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_52 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_53: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_53 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_54: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_54 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_55: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_55 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_56: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_56 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_57: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_57 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_58: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_58 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_59: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_59 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_60: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_60 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_61: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_61 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_62: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_62 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_63: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_63 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_64: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_64 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_65: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_65 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_66: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_66 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_67: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_67 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
}
pub unsafe fn test_unsized_43() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut first: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let mut __tmp_68: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_68 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_69: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_69 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_70: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_70 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_71: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_71 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_72: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_72 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_73: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_73 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_74: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_74 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_75: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_75 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_76: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_76 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_77: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_77 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_78: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_78 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_79: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_79 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_80: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_80 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_81: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_81 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_82: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_82 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_83: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_83 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let mut __tmp_84: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_84 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
}
pub unsafe fn test_unsized_44() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut first: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let mut __tmp_85: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_85 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_86: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_86 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_87: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_87 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_88: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_88 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_89: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_89 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_90: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_90 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_91: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_91 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_92: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_92 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_93: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_93 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_94: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_94 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_95: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_95 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_96: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_96 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_97: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_97 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_98: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_98 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_99: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_99 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_100: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_100 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_101: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({
                            (*a).as_mut_ptr().offset((3_()) as isize)
                        });
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_101 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
}
pub unsafe fn test_unsized_45() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut first: *mut () = a.as_mut_ptr();
        let mut last: sentinel_wrapper_int_ptr_ = {
            let mut __tmp_102: *mut () = a.as_mut_ptr();
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_102 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_103: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_103 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_104: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_104 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_105: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_105 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_106: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_106 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_107: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_107 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_108: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_108 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_109: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_109 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_110: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_110 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_111: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_111 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_112: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_112 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_113: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_113 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_114: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_114 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_115: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_115 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_116: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_116 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_117: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_117 })
                };
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let mut __tmp_118: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_118 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
}
pub unsafe fn test_unsized_46() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut first: *const () = a.as_mut_ptr();
        let mut last: sentinel_wrapper_const_int_ptr_ = {
            let mut __tmp_119: *const () = a.as_mut_ptr();
            sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_119 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_120: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_120 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_121: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_121 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_122: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_122 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_123: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_123 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_124: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_124 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_125: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_125 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_126: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_126 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_127: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_127 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_128: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_128 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_129: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_129 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_130: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_130 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_131: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_131 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_132: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_132 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_133: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_133 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_134: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_134 })
                };
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let mut __tmp_135: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_135 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
}
pub unsafe fn test_unsized_47() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut first: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
}
pub unsafe fn test_unsized_48() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut first: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
}
pub unsafe fn test_sized_49() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_136: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_136
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_137: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_137
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_138: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_138
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_139: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_139
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_140: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_140
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_141: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_141
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_142: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_142
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_143: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_143
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_144: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_144
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_145: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_145
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_146: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_146
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_147: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_147
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_148: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_148
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_149: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_149
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_150: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_150
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let mut __tmp_151: cpp17_input_iterator_int_ptr__int_ptr_ =
                        cpp17_input_iterator_int_ptr__int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        &mut __tmp_151
                    })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
        let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_152: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
            sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_152 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
        let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let mut __tmp_153: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({
                    a.as_mut_ptr().offset((3_()) as isize)
                });
            sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_153 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_50() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_154: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_154 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_155: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_155 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_156: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_156 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_157: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_157 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_158: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_158 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_159: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_159 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_160: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_160 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_161: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_161 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_162: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_162 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_163: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_163 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_164: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_164 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_165: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_165 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_166: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_166 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_167: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_167 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_168: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_168 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let mut __tmp_169: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_169 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
            let mut __tmp_170: cpp20_input_iterator_int_ptr_ =
                cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr() });
            sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_170 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
            let mut __tmp_171: cpp20_input_iterator_int_ptr_ =
                cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr().offset((3_()) as isize) });
            sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_171 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_51() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_172: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_172 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_173: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_173 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_174: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_174 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_175: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_175 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_176: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_176 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_177: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_177 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_178: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_178 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_179: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_179 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_180: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_180 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_181: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_181 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_182: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_182 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_183: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_183 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_184: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_184 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_185: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_185 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_186: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_186 })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let mut __tmp_187: cpp17_output_iterator_int_ptr_ =
                        cpp17_output_iterator_int_ptr_::new({ (*a).as_mut_ptr() });
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_187 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
            let mut __tmp_188: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ a.as_mut_ptr() });
            sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_188 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
            let mut __tmp_189: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ a.as_mut_ptr().offset((3_()) as isize) });
            sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_189 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_52() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_190: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_190 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_191: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_191 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_192: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_192 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_193: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_193 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_194: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_194 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_195: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_195 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_196: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_196 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_197: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_197 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_198: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_198 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_199: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_199 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_200: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_200 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_201: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_201 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_202: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_202 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_203: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_203 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_204: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_204 })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr().offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let mut __tmp_205: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_205 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
            let mut __tmp_206: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
            sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_206 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
            let mut __tmp_207: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((3_()) as isize) });
            sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_207 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_53() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_208: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_208 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_209: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_209 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_210: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_210 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_211: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_211 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_212: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_212 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_213: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_213 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_214: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_214 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_215: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_215 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_216: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_216 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_217: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_217 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_218: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_218 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_219: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_219 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_220: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_220 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_221: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_221 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_222: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_222 })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let mut __tmp_223: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_223 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_224: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
            sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_224 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
            let mut __tmp_225: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((3_()) as isize) });
            sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_225 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_54() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_226: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_226 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_227: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_227 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_228: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_228 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_229: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_229 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_230: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_230 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_231: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_231 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_232: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_232 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_233: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_233 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_234: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_234 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_235: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_235 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_236: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_236 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_237: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_237 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_238: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_238 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_239: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_239 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_240: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_240 })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let mut __tmp_241: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_241 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
            let mut __tmp_242: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
            sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_242 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
            let mut __tmp_243: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((3_()) as isize) });
            sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_243 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_55() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_244: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_244 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_245: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_245 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_246: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_246 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_247: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_247 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_248: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_248 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_249: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_249 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_250: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_250 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_251: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_251 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_252: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_252 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_253: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_253 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_254: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_254 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_255: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_255 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_256: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_256 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_257: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_257 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_258: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_258 })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let mut __tmp_259: contiguous_iterator_int_ptr_ =
                        contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_259 })
                };
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
            let mut __tmp_260: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
            sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_260 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
            let mut __tmp_261: contiguous_iterator_int_ptr_ =
                contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((3_()) as isize) });
            sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_261 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_56() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ =
                    sized_sentinel_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: *mut () = a.as_mut_ptr();
        let mut last: sized_sentinel_int_ptr_ = {
            let mut __tmp_262: *mut () = a.as_mut_ptr();
            sized_sentinel_int_ptr_::new_1({ &mut __tmp_262 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: *mut () = a.as_mut_ptr();
        let mut last: sized_sentinel_int_ptr_ = {
            let mut __tmp_263: *mut () = a.as_mut_ptr().offset((3_()) as isize);
            sized_sentinel_int_ptr_::new_1({ &mut __tmp_263 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_57() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: *const () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ =
                    sized_sentinel_const_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: *const () = a.as_mut_ptr();
        let mut last: sized_sentinel_const_int_ptr_ = {
            let mut __tmp_264: *const () = a.as_mut_ptr();
            sized_sentinel_const_int_ptr_::new_1({ &mut __tmp_264 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: *const () = a.as_mut_ptr();
        let mut last: sized_sentinel_const_int_ptr_ = {
            let mut __tmp_265: *const () = a.as_mut_ptr().offset((3_()) as isize);
            sized_sentinel_const_int_ptr_::new_1({ &mut __tmp_265 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_58() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *mut () = (*a).as_mut_ptr();
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: *mut () = a.as_mut_ptr();
        let mut last: *mut () = a.as_mut_ptr();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: *mut () = a.as_mut_ptr();
        let mut last: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_59() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            },
            || {
                let mut first: *mut () = (*a).as_mut_ptr().offset((3_()) as isize);
                let mut last: *const () = (*a).as_mut_ptr();
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: *mut () = a.as_mut_ptr();
        let mut last: *const () = a.as_mut_ptr();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: *mut () = a.as_mut_ptr();
        let mut last: *const () = a.as_mut_ptr().offset((3_()) as isize);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_60() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((3_()) as isize) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sized_61() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {
                let a: *mut [(); 3] = &mut a;
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({
                        (*a).as_mut_ptr().offset((3_()) as isize)
                    });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (*a).as_mut_ptr() });
                unimplemented!();
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
        (unsafe { check.spec::<fn()>(2).call() });
        (unsafe { check.spec::<fn()>(3).call() });
        (unsafe { check.spec::<fn()>(4).call() });
        (unsafe { check.spec::<fn()>(5).call() });
        (unsafe { check.spec::<fn()>(6).call() });
        (unsafe { check.spec::<fn()>(7).call() });
        (unsafe { check.spec::<fn()>(8).call() });
        (unsafe { check.spec::<fn()>(9).call() });
        (unsafe { check.spec::<fn()>(10).call() });
        (unsafe { check.spec::<fn()>(11).call() });
        (unsafe { check.spec::<fn()>(12).call() });
        (unsafe { check.spec::<fn()>(13).call() });
        (unsafe { check.spec::<fn()>(14).call() });
        (unsafe { check.spec::<fn()>(15).call() });
    }
    {
        let mut first: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut first: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let mut last: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((3_()) as isize) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StrideCounter {
    pub it_: *mut (),
    pub inc_: *mut (),
}
impl StrideCounter {
    pub unsafe fn new_1(mut it: *mut (), mut inc: *mut ()) -> Self {
        let mut this = Self { it_: it, inc_: inc };
        this
    }
    pub unsafe fn operator_inc_3(&mut self) -> *mut StrideCounter {
        self.it_.prefix_inc();
        (*self.inc_) += 1_();
        return &mut (*(self as *mut StrideCounter));
    }
}
const _: () = assert!(true, "std::forward_iterator<StrideCounter>");
const _: () = assert!(
    !(false),
    "!std::sized_sentinel_for<StrideCounter, StrideCounter>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SizedStrideCounter {
    pub it_: *mut (),
    pub minus_: *mut (),
}
impl SizedStrideCounter {
    pub unsafe fn new_1(mut it: *mut (), mut minus: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            minus_: minus,
        };
        this
    }
    pub unsafe fn operator_sub(&self, mut rhs: SizedStrideCounter) -> () {
        (*self.minus_) += 1_();
        return ((self.it_ as usize - rhs.it_ as usize) / ::std::mem::size_of::<()>()) as ();
    }
}
const _: () = assert!(true, "std::forward_iterator<SizedStrideCounter>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<SizedStrideCounter, SizedStrideCounter>"
);
pub unsafe fn test_stride_counting_62() {
    {
        let mut a: [(); 3] = [1_(), 2_(), 3_()];
        let mut inc: () = 0_();
        let mut first: StrideCounter =
            StrideCounter::new_1({ a.as_mut_ptr() }, { (&mut inc as *mut ()) });
        let mut last: StrideCounter =
            StrideCounter::new_1({ a.as_mut_ptr().offset((3_()) as isize) }, {
                std::ptr::null_mut()
            });
        let mut result: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut distance_63))(first, last) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 3] = [1_(), 2_(), 3_()];
        let mut minus: () = 0_();
        let mut first: SizedStrideCounter =
            SizedStrideCounter::new_1({ a.as_mut_ptr() }, { (&mut minus as *mut ()) });
        let mut last: SizedStrideCounter =
            SizedStrideCounter::new_1({ a.as_mut_ptr().offset((3_()) as isize) }, {
                std::ptr::null_mut()
            });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut distance_63))(&mut first, last)
        });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_deque_64() {
    let mut c: () = unimplemented!();
    let mut view : std_ranges_join_view_std_ranges_ref_view_std_deque_std_deque_double__std_allocator_double____std_allocator_std_deque_double__std_allocator_double______ = unimplemented!()  ;
    let mut n: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        n = (n).wrapping_add(i);
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
}
pub unsafe fn test_65() -> () {
    {
        let mut a: [(); 3] = [1_(), 2_(), 3_()];
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    (unsafe { test_unsized_39() });
    (unsafe { test_unsized_40() });
    (unsafe { test_unsized_41() });
    (unsafe { test_unsized_42() });
    (unsafe { test_unsized_43() });
    (unsafe { test_unsized_44() });
    (unsafe { test_unsized_45() });
    (unsafe { test_unsized_46() });
    (unsafe { test_unsized_47() });
    (unsafe { test_unsized_48() });
    (unsafe { test_sized_49() });
    (unsafe { test_sized_50() });
    (unsafe { test_sized_51() });
    (unsafe { test_sized_52() });
    (unsafe { test_sized_53() });
    (unsafe { test_sized_54() });
    (unsafe { test_sized_55() });
    (unsafe { test_sized_56() });
    (unsafe { test_sized_57() });
    (unsafe { test_sized_58() });
    (unsafe { test_sized_59() });
    (unsafe { test_sized_60() });
    (unsafe { test_sized_61() });
    {
        const _: () = assert!(true, "!std::is_copy_constructible_v<It>");;
        const _: () = assert!(!(false), "!std::sentinel_for<It, It>");;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, It&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, It&&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&&, It&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&&, It&&>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
        );;
        let mut a: [(); 3] = [1_(), 2_(), 3_()];
        let mut last: sentinel_wrapper_cpp20_input_iterator_int_ptr__ = {
            let mut __tmp_266: cpp20_input_iterator_int_ptr_ =
                cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr().offset((3_()) as isize) });
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_266 })
        };
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, Sent&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, Sent&&>"
        );;
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(!(false), "!std::sentinel_for<It, It>");;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, It&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, It&&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&&, It&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&&, It&&>"
        );;
    }
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int, int>"
    );;
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int*, int>"
    );;
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int, int*>"
    );;
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int*, char*>"
    );;
    {
        let mut c: () = unimplemented!();
        let mut view : std_ranges_join_view_std_ranges_ref_view_std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int______ = unimplemented!()  ;
        let mut n: () = 0_();
        let mut i: () = 0_();
        'loop_: while ((i) < (10_())) {
            n = (n).wrapping_add(i);
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        let mut view: std_ranges_join_view_std_ranges_ref_view_std_array_std_array_char__3___10___ =
            unimplemented!();
        unimplemented!();
    }
    {
        (unsafe { test_deque_64() });
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
    (unsafe { test_65() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
