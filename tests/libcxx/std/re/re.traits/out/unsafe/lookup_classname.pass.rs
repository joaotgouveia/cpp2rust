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
pub unsafe fn operator_ne_0(
    x: *const forward_iterator_const_char_ptr_,
    y: *const forward_iterator_const_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_char_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_char_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_const_char_ptr_,
    ) -> *mut forward_iterator_const_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_const_char_ptr_));
    }
}
pub unsafe fn operator_ne_1(
    x: *const forward_iterator_const_wchar_t_ptr_,
    y: *const forward_iterator_const_wchar_t_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_wchar_t_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_wchar_t_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_wchar_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_wchar_t_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_wchar_t_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_const_wchar_t_ptr_,
    ) -> *mut forward_iterator_const_wchar_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_const_wchar_t_ptr_));
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
pub unsafe fn operator_eq_2(
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
            operator_eq_2(
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
pub unsafe fn operator_add_3(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_4(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_3(_i, n)
    });
}
pub unsafe fn iter_swap_5(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_6(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_7(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_8(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_8(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_7(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_9(mut A: *const (), mut expected: (), mut icase: Option<()>) {
    let mut icase: () = icase.unwrap_or_else(|| unsafe { false });
    let mut t: () = unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_10(mut A: *const (), mut expected: (), mut icase: Option<()>) {
    let mut icase: () = icase.unwrap_or_else(|| unsafe { false });
    let mut t: () = unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_w_11(mut A: *const (), mut expected: (), mut icase: Option<()>) {
    let mut icase: () = icase.unwrap_or_else(|| unsafe { false });
    let mut t: () = unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    let matches_underscore: () = unimplemented!();
    if !(unimplemented!()) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
pub unsafe fn test_w_12(mut A: *const (), mut expected: (), mut icase: Option<()>) {
    let mut icase: () = icase.unwrap_or_else(|| unsafe { false });
    let mut t: () = unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    let matches_underscore: () = unimplemented!();
    if !(unimplemented!()) {
        unimplemented!();
    } else {
        unimplemented!();
    }
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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    (unsafe { test_9(c"d".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"D".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"d".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"D".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_w_11(c"w".as_ptr(), unimplemented!(), None) });
    (unsafe { test_w_11(c"W".as_ptr(), unimplemented!(), None) });
    (unsafe { test_w_11(c"w".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_w_11(c"W".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"s".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"S".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"s".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"S".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"alnum".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"AlNum".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"alnum".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"AlNum".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"alpha".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"Alpha".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"alpha".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Alpha".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"blank".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"Blank".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"blank".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Blank".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"cntrl".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"Cntrl".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"cntrl".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Cntrl".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"digit".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"Digit".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"digit".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Digit".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"digit".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"DIGIT".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"digit".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Digit".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"graph".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"GRAPH".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"graph".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Graph".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"lower".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"LOWER".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"lower".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Lower".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"print".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"PRINT".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"print".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Print".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"punct".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"PUNCT".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"punct".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Punct".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"space".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"SPACE".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"space".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Space".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"upper".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"UPPER".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"upper".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Upper".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"xdigit".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"XDIGIT".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"xdigit".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"Xdigit".as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_9(c"dig".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"".as_ptr(), unimplemented!(), None) });
    (unsafe { test_9(c"digits".as_ptr(), unimplemented!(), None) });
    (unsafe { test_10([100 as (), 0 as ()].as_ptr(), unimplemented!(), None) });
    (unsafe { test_10([68 as (), 0 as ()].as_ptr(), unimplemented!(), None) });
    (unsafe { test_10([100 as (), 0 as ()].as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_10([68 as (), 0 as ()].as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_w_12([119 as (), 0 as ()].as_ptr(), unimplemented!(), None) });
    (unsafe { test_w_12([87 as (), 0 as ()].as_ptr(), unimplemented!(), None) });
    (unsafe { test_w_12([119 as (), 0 as ()].as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_w_12([87 as (), 0 as ()].as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_10([115 as (), 0 as ()].as_ptr(), unimplemented!(), None) });
    (unsafe { test_10([83 as (), 0 as ()].as_ptr(), unimplemented!(), None) });
    (unsafe { test_10([115 as (), 0 as ()].as_ptr(), unimplemented!(), Some(true)) });
    (unsafe { test_10([83 as (), 0 as ()].as_ptr(), unimplemented!(), Some(true)) });
    (unsafe {
        test_10(
            [
                97 as (), 108 as (), 110 as (), 117 as (), 109 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [65 as (), 108 as (), 78 as (), 117 as (), 109 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                97 as (), 108 as (), 110 as (), 117 as (), 109 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [65 as (), 108 as (), 78 as (), 117 as (), 109 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [97 as (), 108 as (), 112 as (), 104 as (), 97 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [65 as (), 108 as (), 112 as (), 104 as (), 97 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [97 as (), 108 as (), 112 as (), 104 as (), 97 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [65 as (), 108 as (), 112 as (), 104 as (), 97 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [98 as (), 108 as (), 97 as (), 110 as (), 107 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [66 as (), 108 as (), 97 as (), 110 as (), 107 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [98 as (), 108 as (), 97 as (), 110 as (), 107 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [66 as (), 108 as (), 97 as (), 110 as (), 107 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                99 as (), 110 as (), 116 as (), 114 as (), 108 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                67 as (), 110 as (), 116 as (), 114 as (), 108 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                99 as (), 110 as (), 116 as (), 114 as (), 108 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                67 as (), 110 as (), 116 as (), 114 as (), 108 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                100 as (), 105 as (), 103 as (), 105 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                68 as (), 105 as (), 103 as (), 105 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                100 as (), 105 as (), 103 as (), 105 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                68 as (), 105 as (), 103 as (), 105 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                100 as (), 105 as (), 103 as (), 105 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [68 as (), 73 as (), 71 as (), 73 as (), 84 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                100 as (), 105 as (), 103 as (), 105 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                68 as (), 105 as (), 103 as (), 105 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                103 as (), 114 as (), 97 as (), 112 as (), 104 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [71 as (), 82 as (), 65 as (), 80 as (), 72 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                103 as (), 114 as (), 97 as (), 112 as (), 104 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [71 as (), 114 as (), 97 as (), 112 as (), 104 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                108 as (), 111 as (), 119 as (), 101 as (), 114 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [76 as (), 79 as (), 87 as (), 69 as (), 82 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                108 as (), 111 as (), 119 as (), 101 as (), 114 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                76 as (), 111 as (), 119 as (), 101 as (), 114 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                112 as (), 114 as (), 105 as (), 110 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [80 as (), 82 as (), 73 as (), 78 as (), 84 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                112 as (), 114 as (), 105 as (), 110 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                80 as (), 114 as (), 105 as (), 110 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                112 as (), 117 as (), 110 as (), 99 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [80 as (), 85 as (), 78 as (), 67 as (), 84 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                112 as (), 117 as (), 110 as (), 99 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [80 as (), 117 as (), 110 as (), 99 as (), 116 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [115 as (), 112 as (), 97 as (), 99 as (), 101 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [83 as (), 80 as (), 65 as (), 67 as (), 69 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [115 as (), 112 as (), 97 as (), 99 as (), 101 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [83 as (), 112 as (), 97 as (), 99 as (), 101 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                117 as (), 112 as (), 112 as (), 101 as (), 114 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [85 as (), 80 as (), 80 as (), 69 as (), 82 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                117 as (), 112 as (), 112 as (), 101 as (), 114 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                85 as (), 112 as (), 112 as (), 101 as (), 114 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                120 as (), 100 as (), 105 as (), 103 as (), 105 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                88 as (), 68 as (), 73 as (), 71 as (), 73 as (), 84 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe {
        test_10(
            [
                120 as (), 100 as (), 105 as (), 103 as (), 105 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [
                88 as (), 100 as (), 105 as (), 103 as (), 105 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            Some(true),
        )
    });
    (unsafe {
        test_10(
            [100 as (), 105 as (), 103 as (), 0 as ()].as_ptr(),
            unimplemented!(),
            None,
        )
    });
    (unsafe { test_10([0 as ()].as_ptr(), unimplemented!(), None) });
    (unsafe {
        test_10(
            [
                100 as (), 105 as (), 103 as (), 105 as (), 116 as (), 115 as (), 0 as (),
            ]
            .as_ptr(),
            unimplemented!(),
            None,
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
