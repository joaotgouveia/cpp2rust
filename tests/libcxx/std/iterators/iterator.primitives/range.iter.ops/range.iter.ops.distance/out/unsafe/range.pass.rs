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
            it_: (*unimplemented!()),
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
pub unsafe fn base_3(i: *const forward_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn base_4(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn base_5(i: *const random_access_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn base_6(i: *const contiguous_iterator_int_ptr_) -> *mut () {
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
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_8(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_8(other) }));
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
pub struct sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_4(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_4(other) }));
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
            base_: (unsafe { base_5(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_5(other) }));
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
            base_: (unsafe { base_6(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_6(other) }));
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
            base_: (unsafe { base_7((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_7((*other)) }));
    }
}
pub unsafe fn operator_sub_9(
    s: *const sized_sentinel_cpp20_input_iterator_int_ptr__,
    i: *const cpp20_input_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_8(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_10(
    i: *const cpp20_input_iterator_int_ptr_,
    s: *const sized_sentinel_cpp20_input_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_8(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
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
            base_: (unsafe { base_8(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_11(
    s: *const sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__,
    i: *const cpp17_input_iterator_int_ptr__int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_2(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_12(
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
pub unsafe fn operator_sub_13(
    s: *const sized_sentinel_cpp17_output_iterator_int_ptr__,
    i: *const cpp17_output_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_1(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_14(
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
pub unsafe fn operator_sub_15(
    s: *const sized_sentinel_forward_iterator_int_ptr__,
    i: *const forward_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_3(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_16(
    i: *const forward_iterator_int_ptr_,
    s: *const sized_sentinel_forward_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_3(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
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
            base_: (unsafe { base_3(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_17(
    s: *const sized_sentinel_bidirectional_iterator_int_ptr__,
    i: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_4(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_18(
    i: *const bidirectional_iterator_int_ptr_,
    s: *const sized_sentinel_bidirectional_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_4(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
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
            base_: (unsafe { base_4(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_19(
    s: *const sized_sentinel_random_access_iterator_int_ptr__,
    i: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_5(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_20(
    i: *const random_access_iterator_int_ptr_,
    s: *const sized_sentinel_random_access_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_5(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
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
            base_: (unsafe { base_5(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_21(
    s: *const sized_sentinel_contiguous_iterator_int_ptr__,
    i: *const contiguous_iterator_int_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_6(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_22(
    i: *const contiguous_iterator_int_ptr_,
    s: *const sized_sentinel_contiguous_iterator_int_ptr__,
) -> () {
    return (((unsafe { base_6(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
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
            base_: (unsafe { base_6(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_23(s: *const sized_sentinel_int_ptr_, i: *const *mut ()) -> () {
    return (((*s).base_ as usize - (unsafe { base_7((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_24(i: *const *mut (), s: *const sized_sentinel_int_ptr_) -> () {
    return (((unsafe { base_7((*i)) }) as usize - (*s).base_ as usize)
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
            base_: (unsafe { base_7((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_eq_25(
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
            operator_eq_25(
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
pub unsafe fn operator_add_26(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_27(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_26(_i, n)
    });
}
pub unsafe fn iter_swap_28(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_29(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*unimplemented!());
}
pub unsafe fn operator_eq_30(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_31(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_31(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_30(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_ordinary_32() {
    let mut r: R_33 = <R_33>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_34() {
    let mut r: R_35 = <R_35>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_36() {
    let mut r: R_37 = <R_37>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_38() {
    let mut r: R_39 = <R_39>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_40() {
    let mut r: R_41 = <R_41>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_42() {
    let mut r: R_43 = <R_43>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_44() {
    let mut r: R_45 = <R_45>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_46() {
    let mut r: R_47 = <R_47>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_48() {
    let mut r: R_49 = <R_49>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_50() {
    let mut r: R_51 = <R_51>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_52() {
    let mut r: R_53 = <R_53>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_54() {
    let mut r: R_55 = <R_55>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_56() {
    let mut r: R_57 = <R_57>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_58() {
    let mut r: R_59 = <R_59>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_60() {
    let mut r: R_61 = <R_61>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_62() {
    let mut r: R_63 = <R_63>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_ordinary_64() {
    let mut r: R_65 = <R_65>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_33 {
    pub a: [(); 3],
}
impl R_33 {
    pub unsafe fn begin(&self) -> cpp20_input_iterator_int_ptr_ {
        return cpp20_input_iterator_int_ptr_::new({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sized_sentinel_cpp20_input_iterator_int_ptr__ {
        return {
            let mut __tmp_0: cpp20_input_iterator_int_ptr_ = cpp20_input_iterator_int_ptr_::new({
                (*(&raw const self.a).cast_mut())
                    .as_mut_ptr()
                    .offset((3_()) as isize)
            });
            sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_0 })
        };
    }
}
impl Default for R_33 {
    fn default() -> Self {
        unsafe {
            R_33 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_35 {
    pub a: [(); 3],
}
impl R_35 {
    pub unsafe fn begin(&self) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        return cpp17_input_iterator_int_ptr__int_ptr_::new({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
        return {
            let mut __tmp_1: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({
                    (*(&raw const self.a).cast_mut())
                        .as_mut_ptr()
                        .offset((3_()) as isize)
                });
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_1 })
        };
    }
}
impl Default for R_35 {
    fn default() -> Self {
        unsafe {
            R_35 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_37 {
    pub a: [(); 3],
}
impl R_37 {
    pub unsafe fn begin(&self) -> cpp20_input_iterator_int_ptr_ {
        return cpp20_input_iterator_int_ptr_::new({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
        return {
            let mut __tmp_2: cpp20_input_iterator_int_ptr_ = cpp20_input_iterator_int_ptr_::new({
                (*(&raw const self.a).cast_mut())
                    .as_mut_ptr()
                    .offset((3_()) as isize)
            });
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_2 })
        };
    }
}
impl Default for R_37 {
    fn default() -> Self {
        unsafe {
            R_37 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_39 {
    pub a: [(); 3],
}
impl R_39 {
    pub unsafe fn begin(&self) -> cpp17_output_iterator_int_ptr_ {
        return cpp17_output_iterator_int_ptr_::new({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_cpp17_output_iterator_int_ptr__ {
        return {
            let mut __tmp_3: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({
                    (*(&raw const self.a).cast_mut())
                        .as_mut_ptr()
                        .offset((3_()) as isize)
                });
            sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_3 })
        };
    }
}
impl Default for R_39 {
    fn default() -> Self {
        unsafe {
            R_39 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_41 {
    pub a: [(); 3],
}
impl R_41 {
    pub unsafe fn begin(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_forward_iterator_int_ptr__ {
        return {
            let mut __tmp_4: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                (*(&raw const self.a).cast_mut())
                    .as_mut_ptr()
                    .offset((3_()) as isize)
            });
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &mut __tmp_4 })
        };
    }
}
impl Default for R_41 {
    fn default() -> Self {
        unsafe {
            R_41 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_43 {
    pub a: [(); 3],
}
impl R_43 {
    pub unsafe fn begin(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_bidirectional_iterator_int_ptr__ {
        return {
            let mut __tmp_5: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    (*(&raw const self.a).cast_mut())
                        .as_mut_ptr()
                        .offset((3_()) as isize)
                });
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_5 })
        };
    }
}
impl Default for R_43 {
    fn default() -> Self {
        unsafe {
            R_43 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_45 {
    pub a: [(); 3],
}
impl R_45 {
    pub unsafe fn begin(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_1({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_random_access_iterator_int_ptr__ {
        return {
            let mut __tmp_6: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    (*(&raw const self.a).cast_mut())
                        .as_mut_ptr()
                        .offset((3_()) as isize)
                });
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_6 })
        };
    }
}
impl Default for R_45 {
    fn default() -> Self {
        unsafe {
            R_45 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_47 {
    pub a: [(); 3],
}
impl R_47 {
    pub unsafe fn begin(&self) -> contiguous_iterator_int_ptr_ {
        return contiguous_iterator_int_ptr_::new_1({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_contiguous_iterator_int_ptr__ {
        return {
            let mut __tmp_7: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                (*(&raw const self.a).cast_mut())
                    .as_mut_ptr()
                    .offset((3_()) as isize)
            });
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_7 })
        };
    }
}
impl Default for R_47 {
    fn default() -> Self {
        unsafe {
            R_47 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_49 {
    pub a: [(); 3],
}
impl R_49 {
    pub unsafe fn begin(&self) -> *mut () {
        return (*(&raw const self.a).cast_mut()).as_mut_ptr();
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_int_ptr_ {
        return {
            let mut __tmp_8: *mut () = (*(&raw const self.a).cast_mut())
                .as_mut_ptr()
                .offset((3_()) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_8 })
        };
    }
}
impl Default for R_49 {
    fn default() -> Self {
        unsafe {
            R_49 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_51 {
    pub a: [(); 3],
}
impl R_51 {
    pub unsafe fn begin(&self) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        return cpp17_input_iterator_int_ptr__int_ptr_::new({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ {
        return {
            let mut __tmp_9: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({
                    (*(&raw const self.a).cast_mut())
                        .as_mut_ptr()
                        .offset((3_()) as isize)
                });
            sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &mut __tmp_9 })
        };
    }
}
impl Default for R_51 {
    fn default() -> Self {
        unsafe {
            R_51 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_53 {
    pub a: [(); 3],
}
impl R_53 {
    pub unsafe fn begin(&self) -> cpp17_output_iterator_int_ptr_ {
        return cpp17_output_iterator_int_ptr_::new({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sized_sentinel_cpp17_output_iterator_int_ptr__ {
        return {
            let mut __tmp_10: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({
                    (*(&raw const self.a).cast_mut())
                        .as_mut_ptr()
                        .offset((3_()) as isize)
                });
            sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ &mut __tmp_10 })
        };
    }
}
impl Default for R_53 {
    fn default() -> Self {
        unsafe {
            R_53 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_55 {
    pub a: [(); 3],
}
impl R_55 {
    pub unsafe fn begin(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sized_sentinel_forward_iterator_int_ptr__ {
        return {
            let mut __tmp_11: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                (*(&raw const self.a).cast_mut())
                    .as_mut_ptr()
                    .offset((3_()) as isize)
            });
            sized_sentinel_forward_iterator_int_ptr__::new_1({ &mut __tmp_11 })
        };
    }
}
impl Default for R_55 {
    fn default() -> Self {
        unsafe {
            R_55 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_57 {
    pub a: [(); 3],
}
impl R_57 {
    pub unsafe fn begin(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sized_sentinel_bidirectional_iterator_int_ptr__ {
        return {
            let mut __tmp_12: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    (*(&raw const self.a).cast_mut())
                        .as_mut_ptr()
                        .offset((3_()) as isize)
                });
            sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ &mut __tmp_12 })
        };
    }
}
impl Default for R_57 {
    fn default() -> Self {
        unsafe {
            R_57 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_59 {
    pub a: [(); 3],
}
impl R_59 {
    pub unsafe fn begin(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_1({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sized_sentinel_random_access_iterator_int_ptr__ {
        return {
            let mut __tmp_13: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    (*(&raw const self.a).cast_mut())
                        .as_mut_ptr()
                        .offset((3_()) as isize)
                });
            sized_sentinel_random_access_iterator_int_ptr__::new_1({ &mut __tmp_13 })
        };
    }
}
impl Default for R_59 {
    fn default() -> Self {
        unsafe {
            R_59 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_61 {
    pub a: [(); 3],
}
impl R_61 {
    pub unsafe fn begin(&self) -> contiguous_iterator_int_ptr_ {
        return contiguous_iterator_int_ptr_::new_1({
            (*(&raw const self.a).cast_mut()).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> sized_sentinel_contiguous_iterator_int_ptr__ {
        return {
            let mut __tmp_14: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                (*(&raw const self.a).cast_mut())
                    .as_mut_ptr()
                    .offset((3_()) as isize)
            });
            sized_sentinel_contiguous_iterator_int_ptr__::new_1({ &mut __tmp_14 })
        };
    }
}
impl Default for R_61 {
    fn default() -> Self {
        unsafe {
            R_61 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_63 {
    pub a: [(); 3],
}
impl R_63 {
    pub unsafe fn begin(&self) -> *mut () {
        return (*(&raw const self.a).cast_mut()).as_mut_ptr();
    }
    pub unsafe fn end(&self) -> sized_sentinel_int_ptr_ {
        return {
            let mut __tmp_15: *mut () = (*(&raw const self.a).cast_mut())
                .as_mut_ptr()
                .offset((3_()) as isize);
            sized_sentinel_int_ptr_::new_1({ &mut __tmp_15 })
        };
    }
}
impl Default for R_63 {
    fn default() -> Self {
        unsafe {
            R_63 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct R_65 {
    pub a: [(); 3],
}
impl R_65 {
    pub unsafe fn begin(&self) -> *mut () {
        return (*(&raw const self.a).cast_mut()).as_mut_ptr();
    }
    pub unsafe fn end(&self) -> *mut () {
        return (*(&raw const self.a).cast_mut())
            .as_mut_ptr()
            .offset((3_()) as isize);
    }
}
impl Default for R_65 {
    fn default() -> Self {
        unsafe {
            R_65 {
                a: [1_(), 2_(), 3_()],
            }
        }
    }
}
pub unsafe fn test_66() -> () {
    {
        let mut a: [(); 3] = [1_(), 2_(), 3_()];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let mut a: [(); 3] = [1_(), 2_(), 3_()];
        {
            let mut r : () = std_ranges_subrange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_int_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_16 : cpp20_input_iterator_int_ptr_  = cpp20_input_iterator_int_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 3_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_int_ptr__ :: new_1 ( { & mut __tmp_16 } , )  } } , )  ;
            unimplemented!();
        }
        {
            let mut r : () = std_ranges_subrange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_int_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_17 : cpp20_input_iterator_int_ptr_  = cpp20_input_iterator_int_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 3_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_int_ptr__ :: new_1 ( { & mut __tmp_17 } , )  } } , )  ;
            unimplemented!();
        }
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), const R&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), const R&&>"
        );;
    }
    {
        let mut a: [(); 3] = [1_(), 2_(), 3_()];
        {
            let mut r : () = std_ranges_subrange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_int_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_18 : cpp20_input_iterator_int_ptr_  = cpp20_input_iterator_int_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 3_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_int_ptr__ :: new_1 ( { & mut __tmp_18 } , )  } } , { 3_() } , )  ;
            unimplemented!();
        }
        {
            let mut r : () = std_ranges_subrange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_int_ptr_ :: new ( { a .as_mut_ptr() } , )  } , { { let mut __tmp_19 : cpp20_input_iterator_int_ptr_  = cpp20_input_iterator_int_ptr_ :: new ( { a .as_mut_ptr() . offset ( ( 3_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_int_ptr__ :: new_1 ( { & mut __tmp_19 } , )  } } , { 3_() } , )  ;
            unimplemented!();
        }
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), const R&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), const R&&>"
        );;
    }
    {
        (unsafe { test_ordinary_32() });
    }
    (unsafe { test_ordinary_34() });
    (unsafe { test_ordinary_36() });
    (unsafe { test_ordinary_38() });
    (unsafe { test_ordinary_40() });
    (unsafe { test_ordinary_42() });
    (unsafe { test_ordinary_44() });
    (unsafe { test_ordinary_46() });
    (unsafe { test_ordinary_48() });
    (unsafe { test_ordinary_50() });
    (unsafe { test_ordinary_32() });
    (unsafe { test_ordinary_52() });
    (unsafe { test_ordinary_54() });
    (unsafe { test_ordinary_56() });
    (unsafe { test_ordinary_58() });
    (unsafe { test_ordinary_60() });
    (unsafe { test_ordinary_62() });
    (unsafe { test_ordinary_64() });
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int>"
    );;
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int*>"
    );;
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
    (unsafe { test_66() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
