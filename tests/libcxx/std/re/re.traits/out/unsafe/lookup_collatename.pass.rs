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
    moved_from_: bool,
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        assert!(!((*other).moved_from_));
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        assert!(!((*other).moved_from_));
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
    it_: *mut i32,
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
    it_: *mut i32,
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
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_char_ptr_ {
    it_: *const libc::c_char,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_char_ptr_ {
    pub unsafe fn new_1(mut it: *const libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const libc::c_char {
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
impl UnsafeCppAdvance for forward_iterator_const_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_ne_1(
    x: *const forward_iterator_const_wchar_t_ptr_,
    y: *const forward_iterator_const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_wchar_t_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_wchar_t_ptr_ {
    pub unsafe fn new_1(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
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
impl UnsafeCppAdvance for forward_iterator_const_wchar_t_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut i32 {
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
    it_: *mut i32,
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut i32,
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
    pub increments: usize,
    pub decrements: usize,
    pub zero_moves: usize,
    pub equal_cmps: usize,
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_usize,
                decrements: 0_usize,
                zero_moves: 0_usize,
                equal_cmps: 0_usize,
            }
        }
    }
}
pub unsafe fn operator_eq_2(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> bool {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut i32,
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
pub unsafe fn operator_add_3(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_4(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_3(_i, n)
    });
}
pub unsafe fn iter_swap_5(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_6(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_7(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_8(
    lhs: *const adl_Iterator,
    rhs: *const adl_Iterator,
) -> std::cmp::Ordering {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut i32,
    iter_moves_: *mut i32,
    iter_swaps_: *mut i32,
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut i32, mut iter_moves: *mut i32, mut iter_swaps: *mut i32) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut i32, iter_moves: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut i32, iter_swaps: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> i32 {
        assert!(!(self.iter_moves_).is_null());
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> i32 {
        assert!(!(self.iter_swaps_).is_null());
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut i32 {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: isize) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> isize {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64
            as isize);
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: isize) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: isize) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: i32) -> adl_Iterator {
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
impl UnsafeCppAdvance for adl_Iterator {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_14();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_16();
    }
}
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
    it_: *mut *mut i32,
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> bool {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*const libc::c_char , new, value)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const libc::c_char) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn test_9(mut A: *const libc::c_char, expected: *const Vec<libc::c_char>) {
    let mut t: () = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_10(mut A: *const i32, expected: *const Vec<i32>) {
    let mut t: () = unimplemented!();
    assert!(unimplemented!());
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe {
        let mut _expected: Vec<libc::c_char> = std::slice::from_raw_parts(
            (&[(0 as libc::c_char), (0 as libc::c_char)]).as_ptr(),
            1_usize as usize,
        )
        .to_vec()
        .iter()
        .copied()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
        test_9(c"NUL".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\x07".as_ptr(), libcc2rs::c_str_len(c"\x07".as_ptr()) + 1)
                .to_vec();
        test_9(c"alert".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\x08".as_ptr(), libcc2rs::c_str_len(c"\x08".as_ptr()) + 1)
                .to_vec();
        test_9(c"backspace".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\t".as_ptr(), libcc2rs::c_str_len(c"\t".as_ptr()) + 1)
                .to_vec();
        test_9(c"tab".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\r".as_ptr(), libcc2rs::c_str_len(c"\r".as_ptr()) + 1)
                .to_vec();
        test_9(c"carriage-return".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\n".as_ptr(), libcc2rs::c_str_len(c"\n".as_ptr()) + 1)
                .to_vec();
        test_9(c"newline".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\x0b".as_ptr(), libcc2rs::c_str_len(c"\x0b".as_ptr()) + 1)
                .to_vec();
        test_9(c"vertical-tab".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\x0c".as_ptr(), libcc2rs::c_str_len(c"\x0c".as_ptr()) + 1)
                .to_vec();
        test_9(c"form-feed".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c" ".as_ptr(), libcc2rs::c_str_len(c" ".as_ptr()) + 1)
                .to_vec();
        test_9(c"space".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"!".as_ptr(), libcc2rs::c_str_len(c"!".as_ptr()) + 1)
                .to_vec();
        test_9(c"exclamation-mark".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\"".as_ptr(), libcc2rs::c_str_len(c"\"".as_ptr()) + 1)
                .to_vec();
        test_9(c"quotation-mark".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"#".as_ptr(), libcc2rs::c_str_len(c"#".as_ptr()) + 1)
                .to_vec();
        test_9(c"number-sign".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"$".as_ptr(), libcc2rs::c_str_len(c"$".as_ptr()) + 1)
                .to_vec();
        test_9(c"dollar-sign".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"%".as_ptr(), libcc2rs::c_str_len(c"%".as_ptr()) + 1)
                .to_vec();
        test_9(c"percent-sign".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"&".as_ptr(), libcc2rs::c_str_len(c"&".as_ptr()) + 1)
                .to_vec();
        test_9(c"ampersand".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\'".as_ptr(), libcc2rs::c_str_len(c"\'".as_ptr()) + 1)
                .to_vec();
        test_9(c"apostrophe".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"(".as_ptr(), libcc2rs::c_str_len(c"(".as_ptr()) + 1)
                .to_vec();
        test_9(c"left-parenthesis".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c")".as_ptr(), libcc2rs::c_str_len(c")".as_ptr()) + 1)
                .to_vec();
        test_9(c"right-parenthesis".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"*".as_ptr(), libcc2rs::c_str_len(c"*".as_ptr()) + 1)
                .to_vec();
        test_9(c"asterisk".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"+".as_ptr(), libcc2rs::c_str_len(c"+".as_ptr()) + 1)
                .to_vec();
        test_9(c"plus-sign".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c",".as_ptr(), libcc2rs::c_str_len(c",".as_ptr()) + 1)
                .to_vec();
        test_9(c"comma".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"-".as_ptr(), libcc2rs::c_str_len(c"-".as_ptr()) + 1)
                .to_vec();
        test_9(c"hyphen-minus".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"-".as_ptr(), libcc2rs::c_str_len(c"-".as_ptr()) + 1)
                .to_vec();
        test_9(c"hyphen".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c".".as_ptr(), libcc2rs::c_str_len(c".".as_ptr()) + 1)
                .to_vec();
        test_9(c"full-stop".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c".".as_ptr(), libcc2rs::c_str_len(c".".as_ptr()) + 1)
                .to_vec();
        test_9(c"period".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"/".as_ptr(), libcc2rs::c_str_len(c"/".as_ptr()) + 1)
                .to_vec();
        test_9(c"slash".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"/".as_ptr(), libcc2rs::c_str_len(c"/".as_ptr()) + 1)
                .to_vec();
        test_9(c"solidus".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"0".as_ptr(), libcc2rs::c_str_len(c"0".as_ptr()) + 1)
                .to_vec();
        test_9(c"zero".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"1".as_ptr(), libcc2rs::c_str_len(c"1".as_ptr()) + 1)
                .to_vec();
        test_9(c"one".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"2".as_ptr(), libcc2rs::c_str_len(c"2".as_ptr()) + 1)
                .to_vec();
        test_9(c"two".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"3".as_ptr(), libcc2rs::c_str_len(c"3".as_ptr()) + 1)
                .to_vec();
        test_9(c"three".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"4".as_ptr(), libcc2rs::c_str_len(c"4".as_ptr()) + 1)
                .to_vec();
        test_9(c"four".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"5".as_ptr(), libcc2rs::c_str_len(c"5".as_ptr()) + 1)
                .to_vec();
        test_9(c"five".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"6".as_ptr(), libcc2rs::c_str_len(c"6".as_ptr()) + 1)
                .to_vec();
        test_9(c"six".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"7".as_ptr(), libcc2rs::c_str_len(c"7".as_ptr()) + 1)
                .to_vec();
        test_9(c"seven".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"8".as_ptr(), libcc2rs::c_str_len(c"8".as_ptr()) + 1)
                .to_vec();
        test_9(c"eight".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"9".as_ptr(), libcc2rs::c_str_len(c"9".as_ptr()) + 1)
                .to_vec();
        test_9(c"nine".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c":".as_ptr(), libcc2rs::c_str_len(c":".as_ptr()) + 1)
                .to_vec();
        test_9(c"colon".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c";".as_ptr(), libcc2rs::c_str_len(c";".as_ptr()) + 1)
                .to_vec();
        test_9(c"semicolon".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"<".as_ptr(), libcc2rs::c_str_len(c"<".as_ptr()) + 1)
                .to_vec();
        test_9(c"less-than-sign".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"=".as_ptr(), libcc2rs::c_str_len(c"=".as_ptr()) + 1)
                .to_vec();
        test_9(c"equals-sign".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c">".as_ptr(), libcc2rs::c_str_len(c">".as_ptr()) + 1)
                .to_vec();
        test_9(c"greater-than-sign".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"?".as_ptr(), libcc2rs::c_str_len(c"?".as_ptr()) + 1)
                .to_vec();
        test_9(c"question-mark".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"@".as_ptr(), libcc2rs::c_str_len(c"@".as_ptr()) + 1)
                .to_vec();
        test_9(c"commercial-at".as_ptr(), &mut _expected)
    });
    let mut c: libc::c_char = ('A' as libc::c_char);
    'loop_: while ((c as i32) <= (('Z' as libc::c_char) as i32)) {
        let a: [libc::c_char; 2] = [c, (0 as libc::c_char)];
        (unsafe {
            let _A: *const libc::c_char = a.as_ptr();
            let mut _expected: Vec<libc::c_char> =
                std::slice::from_raw_parts(a.as_ptr(), libcc2rs::c_str_len(a.as_ptr()) + 1)
                    .to_vec();
            test_9(_A, &mut _expected)
        });
        c.prefix_inc();
    }
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"[".as_ptr(), libcc2rs::c_str_len(c"[".as_ptr()) + 1)
                .to_vec();
        test_9(c"left-square-bracket".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\\".as_ptr(), libcc2rs::c_str_len(c"\\".as_ptr()) + 1)
                .to_vec();
        test_9(c"backslash".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"\\".as_ptr(), libcc2rs::c_str_len(c"\\".as_ptr()) + 1)
                .to_vec();
        test_9(c"reverse-solidus".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"]".as_ptr(), libcc2rs::c_str_len(c"]".as_ptr()) + 1)
                .to_vec();
        test_9(c"right-square-bracket".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"^".as_ptr(), libcc2rs::c_str_len(c"^".as_ptr()) + 1)
                .to_vec();
        test_9(c"circumflex-accent".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"^".as_ptr(), libcc2rs::c_str_len(c"^".as_ptr()) + 1)
                .to_vec();
        test_9(c"circumflex".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"_".as_ptr(), libcc2rs::c_str_len(c"_".as_ptr()) + 1)
                .to_vec();
        test_9(c"low-line".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"_".as_ptr(), libcc2rs::c_str_len(c"_".as_ptr()) + 1)
                .to_vec();
        test_9(c"underscore".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"`".as_ptr(), libcc2rs::c_str_len(c"`".as_ptr()) + 1)
                .to_vec();
        test_9(c"grave-accent".as_ptr(), &mut _expected)
    });
    let mut c: libc::c_char = ('a' as libc::c_char);
    'loop_: while ((c as i32) <= (('z' as libc::c_char) as i32)) {
        let a: [libc::c_char; 2] = [c, (0 as libc::c_char)];
        (unsafe {
            let _A: *const libc::c_char = a.as_ptr();
            let mut _expected: Vec<libc::c_char> =
                std::slice::from_raw_parts(a.as_ptr(), libcc2rs::c_str_len(a.as_ptr()) + 1)
                    .to_vec();
            test_9(_A, &mut _expected)
        });
        c.prefix_inc();
    }
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"{".as_ptr(), libcc2rs::c_str_len(c"{".as_ptr()) + 1)
                .to_vec();
        test_9(c"left-brace".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"{".as_ptr(), libcc2rs::c_str_len(c"{".as_ptr()) + 1)
                .to_vec();
        test_9(c"left-curly-bracket".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"|".as_ptr(), libcc2rs::c_str_len(c"|".as_ptr()) + 1)
                .to_vec();
        test_9(c"vertical-line".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"}".as_ptr(), libcc2rs::c_str_len(c"}".as_ptr()) + 1)
                .to_vec();
        test_9(c"right-brace".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"}".as_ptr(), libcc2rs::c_str_len(c"}".as_ptr()) + 1)
                .to_vec();
        test_9(c"right-curly-bracket".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"~".as_ptr(), libcc2rs::c_str_len(c"~".as_ptr()) + 1)
                .to_vec();
        test_9(c"tilde".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"".as_ptr(), libcc2rs::c_str_len(c"".as_ptr()) + 1)
                .to_vec();
        test_9(c"tild".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"".as_ptr(), libcc2rs::c_str_len(c"".as_ptr()) + 1)
                .to_vec();
        test_9(c"ch".as_ptr(), &mut _expected)
    });
    (unsafe {
        let mut _expected: Vec<i32> =
            std::slice::from_raw_parts([0 as i32, 0 as i32].as_ptr(), 1_usize as usize)
                .to_vec()
                .iter()
                .copied()
                .chain(std::iter::once(0_i32))
                .collect();
        test_10(
            [78 as i32, 85 as i32, 76 as i32, 0 as i32].as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [7 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([7 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                97 as i32, 108 as i32, 101 as i32, 114 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [8 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([8 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                98 as i32, 97 as i32, 99 as i32, 107 as i32, 115 as i32, 112 as i32, 97 as i32,
                99 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [9 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([9 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [116 as i32, 97 as i32, 98 as i32, 0 as i32].as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [13 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([13 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                99 as i32, 97 as i32, 114 as i32, 114 as i32, 105 as i32, 97 as i32, 103 as i32,
                101 as i32, 45 as i32, 114 as i32, 101 as i32, 116 as i32, 117 as i32, 114 as i32,
                110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [10 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([10 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                110 as i32, 101 as i32, 119 as i32, 108 as i32, 105 as i32, 110 as i32, 101 as i32,
                0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [11 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([11 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                118 as i32, 101 as i32, 114 as i32, 116 as i32, 105 as i32, 99 as i32, 97 as i32,
                108 as i32, 45 as i32, 116 as i32, 97 as i32, 98 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [12 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([12 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                102 as i32, 111 as i32, 114 as i32, 109 as i32, 45 as i32, 102 as i32, 101 as i32,
                101 as i32, 100 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [32 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([32 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                115 as i32, 112 as i32, 97 as i32, 99 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [33 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([33 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                101 as i32, 120 as i32, 99 as i32, 108 as i32, 97 as i32, 109 as i32, 97 as i32,
                116 as i32, 105 as i32, 111 as i32, 110 as i32, 45 as i32, 109 as i32, 97 as i32,
                114 as i32, 107 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [34 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([34 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                113 as i32, 117 as i32, 111 as i32, 116 as i32, 97 as i32, 116 as i32, 105 as i32,
                111 as i32, 110 as i32, 45 as i32, 109 as i32, 97 as i32, 114 as i32, 107 as i32,
                0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [35 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([35 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                110 as i32, 117 as i32, 109 as i32, 98 as i32, 101 as i32, 114 as i32, 45 as i32,
                115 as i32, 105 as i32, 103 as i32, 110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [36 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([36 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                100 as i32, 111 as i32, 108 as i32, 108 as i32, 97 as i32, 114 as i32, 45 as i32,
                115 as i32, 105 as i32, 103 as i32, 110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [37 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([37 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                112 as i32, 101 as i32, 114 as i32, 99 as i32, 101 as i32, 110 as i32, 116 as i32,
                45 as i32, 115 as i32, 105 as i32, 103 as i32, 110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [38 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([38 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                97 as i32, 109 as i32, 112 as i32, 101 as i32, 114 as i32, 115 as i32, 97 as i32,
                110 as i32, 100 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [39 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([39 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                97 as i32, 112 as i32, 111 as i32, 115 as i32, 116 as i32, 114 as i32, 111 as i32,
                112 as i32, 104 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [40 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([40 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                108 as i32, 101 as i32, 102 as i32, 116 as i32, 45 as i32, 112 as i32, 97 as i32,
                114 as i32, 101 as i32, 110 as i32, 116 as i32, 104 as i32, 101 as i32, 115 as i32,
                105 as i32, 115 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [41 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([41 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                114 as i32, 105 as i32, 103 as i32, 104 as i32, 116 as i32, 45 as i32, 112 as i32,
                97 as i32, 114 as i32, 101 as i32, 110 as i32, 116 as i32, 104 as i32, 101 as i32,
                115 as i32, 105 as i32, 115 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [42 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([42 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                97 as i32, 115 as i32, 116 as i32, 101 as i32, 114 as i32, 105 as i32, 115 as i32,
                107 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [43 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([43 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                112 as i32, 108 as i32, 117 as i32, 115 as i32, 45 as i32, 115 as i32, 105 as i32,
                103 as i32, 110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [44 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([44 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                99 as i32, 111 as i32, 109 as i32, 109 as i32, 97 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [45 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([45 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                104 as i32, 121 as i32, 112 as i32, 104 as i32, 101 as i32, 110 as i32, 45 as i32,
                109 as i32, 105 as i32, 110 as i32, 117 as i32, 115 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [45 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([45 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                104 as i32, 121 as i32, 112 as i32, 104 as i32, 101 as i32, 110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [46 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([46 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                102 as i32, 117 as i32, 108 as i32, 108 as i32, 45 as i32, 115 as i32, 116 as i32,
                111 as i32, 112 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [46 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([46 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                112 as i32, 101 as i32, 114 as i32, 105 as i32, 111 as i32, 100 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [47 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([47 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                115 as i32, 108 as i32, 97 as i32, 115 as i32, 104 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [47 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([47 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                115 as i32, 111 as i32, 108 as i32, 105 as i32, 100 as i32, 117 as i32, 115 as i32,
                0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [48 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([48 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [122 as i32, 101 as i32, 114 as i32, 111 as i32, 0 as i32].as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [49 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([49 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [111 as i32, 110 as i32, 101 as i32, 0 as i32].as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [50 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([50 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [116 as i32, 119 as i32, 111 as i32, 0 as i32].as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [51 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([51 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                116 as i32, 104 as i32, 114 as i32, 101 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [52 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([52 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [102 as i32, 111 as i32, 117 as i32, 114 as i32, 0 as i32].as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [53 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([53 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [102 as i32, 105 as i32, 118 as i32, 101 as i32, 0 as i32].as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [54 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([54 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [115 as i32, 105 as i32, 120 as i32, 0 as i32].as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [55 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([55 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                115 as i32, 101 as i32, 118 as i32, 101 as i32, 110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [56 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([56 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                101 as i32, 105 as i32, 103 as i32, 104 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [57 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([57 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [110 as i32, 105 as i32, 110 as i32, 101 as i32, 0 as i32].as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [58 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([58 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                99 as i32, 111 as i32, 108 as i32, 111 as i32, 110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [59 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([59 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                115 as i32, 101 as i32, 109 as i32, 105 as i32, 99 as i32, 111 as i32, 108 as i32,
                111 as i32, 110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [60 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([60 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                108 as i32, 101 as i32, 115 as i32, 115 as i32, 45 as i32, 116 as i32, 104 as i32,
                97 as i32, 110 as i32, 45 as i32, 115 as i32, 105 as i32, 103 as i32, 110 as i32,
                0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [61 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([61 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                101 as i32, 113 as i32, 117 as i32, 97 as i32, 108 as i32, 115 as i32, 45 as i32,
                115 as i32, 105 as i32, 103 as i32, 110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [62 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([62 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                103 as i32, 114 as i32, 101 as i32, 97 as i32, 116 as i32, 101 as i32, 114 as i32,
                45 as i32, 116 as i32, 104 as i32, 97 as i32, 110 as i32, 45 as i32, 115 as i32,
                105 as i32, 103 as i32, 110 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [63 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([63 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                113 as i32, 117 as i32, 101 as i32, 115 as i32, 116 as i32, 105 as i32, 111 as i32,
                110 as i32, 45 as i32, 109 as i32, 97 as i32, 114 as i32, 107 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [64 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([64 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                99 as i32, 111 as i32, 109 as i32, 109 as i32, 101 as i32, 114 as i32, 99 as i32,
                105 as i32, 97 as i32, 108 as i32, 45 as i32, 97 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    let mut c: i32 = (65 as i32);
    'loop_: while ((c) <= (90 as i32)) {
        let a: [i32; 2] = [c, 0_i32];
        (unsafe {
            let _A: *const i32 = a.as_ptr();
            let mut _expected: Vec<i32> =
                std::slice::from_raw_parts(a.as_ptr(), libcc2rs::c_str_len(a.as_ptr()) + 1)
                    .to_vec();
            test_10(_A, &mut _expected)
        });
        c.prefix_inc();
    }
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [91 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([91 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                108 as i32, 101 as i32, 102 as i32, 116 as i32, 45 as i32, 115 as i32, 113 as i32,
                117 as i32, 97 as i32, 114 as i32, 101 as i32, 45 as i32, 98 as i32, 114 as i32,
                97 as i32, 99 as i32, 107 as i32, 101 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [92 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([92 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                98 as i32, 97 as i32, 99 as i32, 107 as i32, 115 as i32, 108 as i32, 97 as i32,
                115 as i32, 104 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [92 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([92 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                114 as i32, 101 as i32, 118 as i32, 101 as i32, 114 as i32, 115 as i32, 101 as i32,
                45 as i32, 115 as i32, 111 as i32, 108 as i32, 105 as i32, 100 as i32, 117 as i32,
                115 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [93 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([93 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                114 as i32, 105 as i32, 103 as i32, 104 as i32, 116 as i32, 45 as i32, 115 as i32,
                113 as i32, 117 as i32, 97 as i32, 114 as i32, 101 as i32, 45 as i32, 98 as i32,
                114 as i32, 97 as i32, 99 as i32, 107 as i32, 101 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [94 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([94 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                99 as i32, 105 as i32, 114 as i32, 99 as i32, 117 as i32, 109 as i32, 102 as i32,
                108 as i32, 101 as i32, 120 as i32, 45 as i32, 97 as i32, 99 as i32, 99 as i32,
                101 as i32, 110 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [94 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([94 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                99 as i32, 105 as i32, 114 as i32, 99 as i32, 117 as i32, 109 as i32, 102 as i32,
                108 as i32, 101 as i32, 120 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [95 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([95 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                108 as i32, 111 as i32, 119 as i32, 45 as i32, 108 as i32, 105 as i32, 110 as i32,
                101 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [95 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([95 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                117 as i32, 110 as i32, 100 as i32, 101 as i32, 114 as i32, 115 as i32, 99 as i32,
                111 as i32, 114 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [96 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([96 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                103 as i32, 114 as i32, 97 as i32, 118 as i32, 101 as i32, 45 as i32, 97 as i32,
                99 as i32, 99 as i32, 101 as i32, 110 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    let mut c: i32 = (97 as i32);
    'loop_: while ((c) <= (122 as i32)) {
        let a: [i32; 2] = [c, 0_i32];
        (unsafe {
            let _A: *const i32 = a.as_ptr();
            let mut _expected: Vec<i32> =
                std::slice::from_raw_parts(a.as_ptr(), libcc2rs::c_str_len(a.as_ptr()) + 1)
                    .to_vec();
            test_10(_A, &mut _expected)
        });
        c.prefix_inc();
    }
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [123 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([123 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                108 as i32, 101 as i32, 102 as i32, 116 as i32, 45 as i32, 98 as i32, 114 as i32,
                97 as i32, 99 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [123 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([123 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                108 as i32, 101 as i32, 102 as i32, 116 as i32, 45 as i32, 99 as i32, 117 as i32,
                114 as i32, 108 as i32, 121 as i32, 45 as i32, 98 as i32, 114 as i32, 97 as i32,
                99 as i32, 107 as i32, 101 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [124 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([124 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                118 as i32, 101 as i32, 114 as i32, 116 as i32, 105 as i32, 99 as i32, 97 as i32,
                108 as i32, 45 as i32, 108 as i32, 105 as i32, 110 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [125 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([125 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                114 as i32, 105 as i32, 103 as i32, 104 as i32, 116 as i32, 45 as i32, 98 as i32,
                114 as i32, 97 as i32, 99 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [125 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([125 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                114 as i32, 105 as i32, 103 as i32, 104 as i32, 116 as i32, 45 as i32, 99 as i32,
                117 as i32, 114 as i32, 108 as i32, 121 as i32, 45 as i32, 98 as i32, 114 as i32,
                97 as i32, 99 as i32, 107 as i32, 101 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [126 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([126 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [
                116 as i32, 105 as i32, 108 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [0 as i32].as_ptr(),
            libcc2rs::c_str_len([0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10(
            [116 as i32, 105 as i32, 108 as i32, 100 as i32, 0 as i32].as_ptr(),
            &mut _expected,
        )
    });
    (unsafe {
        let mut _expected: Vec<i32> = std::slice::from_raw_parts(
            [0 as i32].as_ptr(),
            libcc2rs::c_str_len([0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_10([99 as i32, 104 as i32, 0 as i32].as_ptr(), &mut _expected)
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
