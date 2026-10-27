extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MultiStringType {
    pub w_: *const (),
    pub wn_: (),
    pub u8_: *const (),
    pub u8n_: (),
    pub u16_: *const (),
    pub u16n_: (),
    pub u32_: *const (),
    pub u32n_: (),
    pub s_: *const (),
    pub sn_: (),
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const (),
        mut wn: (),
        mut u8: *const (),
        mut u8n: (),
        mut u16: *const (),
        mut u16n: (),
        mut u32: *const (),
        mut u32n: (),
        mut s: *const (),
        mut sn: (),
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const ()) -> *const () {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const ()) -> *const () {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const ()) -> *const () {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const ()) -> *const () {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const ()) -> *const () {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const ()) -> () {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const ()) -> () {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const ()) -> () {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const ()) -> () {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const ()) -> () {
        return self.u32n_;
    }
    pub unsafe fn to_pconst_12(&self) -> *const () {
        return self.s_;
    }
    pub unsafe fn to_pconst_13(&self) -> *const () {
        return self.w_;
    }
    pub unsafe fn to_pconst_14(&self) -> *const () {
        return self.u8_;
    }
    pub unsafe fn to_pconst_15(&self) -> *const () {
        return self.u16_;
    }
    pub unsafe fn to_pconst_16(&self) -> *const () {
        return self.u32_;
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct random_access_iterator_char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_char_ptr_ {}
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
pub unsafe fn operator_sub_0(
    mut x: contiguous_iterator_const_char_ptr_,
    mut y: contiguous_iterator_const_char_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn base_1(i: *const contiguous_iterator_const_char_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct contiguous_iterator_const_char_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
}
pub unsafe fn operator_sub_2(
    mut x: contiguous_iterator_const_wchar_t_ptr_,
    mut y: contiguous_iterator_const_wchar_t_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn base_3(i: *const contiguous_iterator_const_wchar_t_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct contiguous_iterator_const_wchar_t_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_wchar_t_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
}
pub unsafe fn operator_sub_4(
    mut x: contiguous_iterator_const_char8_t_ptr_,
    mut y: contiguous_iterator_const_char8_t_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn base_5(i: *const contiguous_iterator_const_char8_t_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct contiguous_iterator_const_char8_t_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char8_t_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
}
pub unsafe fn operator_sub_6(
    mut x: contiguous_iterator_const_char16_t_ptr_,
    mut y: contiguous_iterator_const_char16_t_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn base_7(i: *const contiguous_iterator_const_char16_t_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct contiguous_iterator_const_char16_t_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char16_t_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
}
pub unsafe fn operator_sub_8(
    mut x: contiguous_iterator_const_char32_t_ptr_,
    mut y: contiguous_iterator_const_char32_t_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn base_9(i: *const contiguous_iterator_const_char32_t_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct contiguous_iterator_const_char32_t_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char32_t_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> () {
        return self.it_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct contiguous_iterator_char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_char_ptr_ {}
pub unsafe fn base_10(mut i: *const ()) -> *const () {
    return i;
}
pub unsafe fn base_11(mut i: *const ()) -> *const () {
    return i;
}
pub unsafe fn base_12(mut i: *const ()) -> *const () {
    return i;
}
pub unsafe fn base_13(mut i: *const ()) -> *const () {
    return i;
}
pub unsafe fn base_14(mut i: *const ()) -> *const () {
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
pub unsafe fn operator_sub_15(s: *const sized_sentinel_const_char_ptr_, i: *const *const ()) -> () {
    return (((*s).base_ as usize - (unsafe { base_10((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_16(i: *const *const (), s: *const sized_sentinel_const_char_ptr_) -> () {
    return (((unsafe { base_10((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<()>()) as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_char_ptr_ {
    base_: *const (),
}
impl sized_sentinel_const_char_ptr_ {
    pub unsafe fn new_1(it: *const *const ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_10((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_17(
    s: *const sized_sentinel_contiguous_iterator_const_char_ptr__,
    i: *const contiguous_iterator_const_char_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_1(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_18(
    i: *const contiguous_iterator_const_char_ptr_,
    s: *const sized_sentinel_contiguous_iterator_const_char_ptr__,
) -> () {
    return (((unsafe { base_1(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_const_char_ptr__ {
    base_: *const (),
}
impl sized_sentinel_contiguous_iterator_const_char_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_1(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_19(
    s: *const sized_sentinel_const_wchar_t_ptr_,
    i: *const *const (),
) -> () {
    return (((*s).base_ as usize - (unsafe { base_11((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_20(
    i: *const *const (),
    s: *const sized_sentinel_const_wchar_t_ptr_,
) -> () {
    return (((unsafe { base_11((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<()>()) as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_wchar_t_ptr_ {
    base_: *const (),
}
impl sized_sentinel_const_wchar_t_ptr_ {
    pub unsafe fn new_1(it: *const *const ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_11((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_21(
    s: *const sized_sentinel_contiguous_iterator_const_wchar_t_ptr__,
    i: *const contiguous_iterator_const_wchar_t_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_3(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_22(
    i: *const contiguous_iterator_const_wchar_t_ptr_,
    s: *const sized_sentinel_contiguous_iterator_const_wchar_t_ptr__,
) -> () {
    return (((unsafe { base_3(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_const_wchar_t_ptr__ {
    base_: *const (),
}
impl sized_sentinel_contiguous_iterator_const_wchar_t_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_wchar_t_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_3(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_23(
    s: *const sized_sentinel_const_char8_t_ptr_,
    i: *const *const (),
) -> () {
    return (((*s).base_ as usize - (unsafe { base_12((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_24(
    i: *const *const (),
    s: *const sized_sentinel_const_char8_t_ptr_,
) -> () {
    return (((unsafe { base_12((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<()>()) as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_char8_t_ptr_ {
    base_: *const (),
}
impl sized_sentinel_const_char8_t_ptr_ {
    pub unsafe fn new_1(it: *const *const ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_12((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_25(
    s: *const sized_sentinel_contiguous_iterator_const_char8_t_ptr__,
    i: *const contiguous_iterator_const_char8_t_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_5(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_26(
    i: *const contiguous_iterator_const_char8_t_ptr_,
    s: *const sized_sentinel_contiguous_iterator_const_char8_t_ptr__,
) -> () {
    return (((unsafe { base_5(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_const_char8_t_ptr__ {
    base_: *const (),
}
impl sized_sentinel_contiguous_iterator_const_char8_t_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_char8_t_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_5(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_27(
    s: *const sized_sentinel_const_char16_t_ptr_,
    i: *const *const (),
) -> () {
    return (((*s).base_ as usize - (unsafe { base_13((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_28(
    i: *const *const (),
    s: *const sized_sentinel_const_char16_t_ptr_,
) -> () {
    return (((unsafe { base_13((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<()>()) as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_char16_t_ptr_ {
    base_: *const (),
}
impl sized_sentinel_const_char16_t_ptr_ {
    pub unsafe fn new_1(it: *const *const ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_13((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_29(
    s: *const sized_sentinel_contiguous_iterator_const_char16_t_ptr__,
    i: *const contiguous_iterator_const_char16_t_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_7(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_30(
    i: *const contiguous_iterator_const_char16_t_ptr_,
    s: *const sized_sentinel_contiguous_iterator_const_char16_t_ptr__,
) -> () {
    return (((unsafe { base_7(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_const_char16_t_ptr__ {
    base_: *const (),
}
impl sized_sentinel_contiguous_iterator_const_char16_t_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_char16_t_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_7(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_31(
    s: *const sized_sentinel_const_char32_t_ptr_,
    i: *const *const (),
) -> () {
    return (((*s).base_ as usize - (unsafe { base_14((*i)) }) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_sub_32(
    i: *const *const (),
    s: *const sized_sentinel_const_char32_t_ptr_,
) -> () {
    return (((unsafe { base_14((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<()>()) as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_char32_t_ptr_ {
    base_: *const (),
}
impl sized_sentinel_const_char32_t_ptr_ {
    pub unsafe fn new_1(it: *const *const ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_14((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_33(
    s: *const sized_sentinel_contiguous_iterator_const_char32_t_ptr__,
    i: *const contiguous_iterator_const_char32_t_ptr_,
) -> () {
    return (((*s).base_ as usize - (unsafe { base_9(i) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_34(
    i: *const contiguous_iterator_const_char32_t_ptr_,
    s: *const sized_sentinel_contiguous_iterator_const_char32_t_ptr__,
) -> () {
    return (((unsafe { base_9(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<()>())
        as ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_const_char32_t_ptr__ {
    base_: *const (),
}
impl sized_sentinel_contiguous_iterator_const_char32_t_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_char32_t_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_9(it) }),
        };
        this
    }
}
pub unsafe fn operator_eq_35(
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
            operator_eq_35(
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
pub unsafe fn operator_add_36(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_37(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_36(_i, n)
    });
}
pub unsafe fn iter_swap_38(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_39(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_40(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_41(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_41(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_40(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_construction_42(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_43(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_44(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_45(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_46(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_47(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_48(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_49(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_50(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_51(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_52(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_53(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_54(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_55(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_56(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_57(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_58(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_59(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_60(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_61(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_62(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_63(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_64(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_65(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_66(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_67(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_68(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_69(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_70(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_construction_71(mut val: ()) {
    let mut sv: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_with_char_72() {
    let val: () = unimplemented!();
    (unsafe { test_construction_42(unimplemented!()) });
    (unsafe { test_construction_43(unimplemented!()) });
    (unsafe { test_construction_44(unimplemented!()) });
    (unsafe { test_construction_45(unimplemented!()) });
    (unsafe { test_construction_46(unimplemented!()) });
    (unsafe { test_construction_47(unimplemented!()) });
}
pub unsafe fn test_with_char_73() {
    let val: () = unimplemented!();
    (unsafe { test_construction_48(unimplemented!()) });
    (unsafe { test_construction_49(unimplemented!()) });
    (unsafe { test_construction_50(unimplemented!()) });
    (unsafe { test_construction_51(unimplemented!()) });
    (unsafe { test_construction_52(unimplemented!()) });
    (unsafe { test_construction_53(unimplemented!()) });
}
pub unsafe fn test_with_char_74() {
    let val: () = unimplemented!();
    (unsafe { test_construction_54(unimplemented!()) });
    (unsafe { test_construction_55(unimplemented!()) });
    (unsafe { test_construction_56(unimplemented!()) });
    (unsafe { test_construction_57(unimplemented!()) });
    (unsafe { test_construction_58(unimplemented!()) });
    (unsafe { test_construction_59(unimplemented!()) });
}
pub unsafe fn test_with_char_75() {
    let val: () = unimplemented!();
    (unsafe { test_construction_60(unimplemented!()) });
    (unsafe { test_construction_61(unimplemented!()) });
    (unsafe { test_construction_62(unimplemented!()) });
    (unsafe { test_construction_63(unimplemented!()) });
    (unsafe { test_construction_64(unimplemented!()) });
    (unsafe { test_construction_65(unimplemented!()) });
}
pub unsafe fn test_with_char_76() {
    let val: () = unimplemented!();
    (unsafe { test_construction_66(unimplemented!()) });
    (unsafe { test_construction_67(unimplemented!()) });
    (unsafe { test_construction_68(unimplemented!()) });
    (unsafe { test_construction_69(unimplemented!()) });
    (unsafe { test_construction_70(unimplemented!()) });
    (unsafe { test_construction_71(unimplemented!()) });
}
pub unsafe fn test_77() -> () {
    (unsafe { test_with_char_72() });
    (unsafe { test_with_char_73() });
    (unsafe { test_with_char_74() });
    (unsafe { test_with_char_75() });
    (unsafe { test_with_char_76() });
    return true;
}
const _: () = assert!(
    true,
    "std::is_constructible_v<std::string_view, const char*, char*>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::string_view, char*, const char*>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::string_view, char*, void*>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::string_view, signed char*, signed char*>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::string_view,\n                                       random_access_iterator<char*>,\n                                       random_access_iterator<char*>>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::string_view, contiguous_iterator<char*>, contiguous_iterator<char*>>"
);
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
    (unsafe { test_77() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
