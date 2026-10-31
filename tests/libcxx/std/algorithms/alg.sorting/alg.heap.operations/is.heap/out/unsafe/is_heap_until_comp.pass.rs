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
pub unsafe fn operator_add_0(
    mut x: random_access_iterator_int_ptr_,
    mut n: isize,
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_1(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_2(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
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
            operator_eq_2(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
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
pub unsafe fn operator_eq_3(
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
            operator_eq_3(
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
pub unsafe fn operator_add_4(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_5(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_4(_i, n)
    });
}
pub unsafe fn iter_swap_6(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_7(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_8(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_9(
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
        unsafe { operator_cmp_9(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_constexpr_10() -> bool {
    let mut ia: [i32; 4] = [1, 0, 0, 0];
    let mut ib: [i32; 4] = [0, 1, 1, 0];
    return ((unimplemented!()) == (ia.as_mut_ptr().offset((1) as isize)))
        && ((unimplemented!()) == (ib.as_mut_ptr().offset((3) as isize)));
}
pub unsafe fn test_11() {
    let mut i1: [i32; 2] = [0, 0];
    assert!(((unimplemented!()) == (i1.as_mut_ptr())));
    assert!(((unimplemented!()) == (i1.as_mut_ptr().offset((1) as isize))));
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ i1.as_mut_ptr() });
            operator_eq_2(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ i1.as_mut_ptr().offset((1) as isize) });
            operator_eq_2(&mut _x, &mut _y)
        })
    );
    let mut i2: [i32; 2] = [0, 1];
    let mut i3: [i32; 2] = [1, 0];
    assert!(((unimplemented!()) == (i1.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i2.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i3.as_mut_ptr().offset((1) as isize))));
    let mut i4: [i32; 3] = [0, 0, 0];
    let mut i5: [i32; 3] = [0, 0, 1];
    let mut i6: [i32; 3] = [0, 1, 0];
    let mut i7: [i32; 3] = [0, 1, 1];
    let mut i8: [i32; 3] = [1, 0, 0];
    let mut i9: [i32; 3] = [1, 0, 1];
    let mut i10: [i32; 3] = [1, 1, 0];
    assert!(((unimplemented!()) == (i4.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i5.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i6.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i7.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i8.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i9.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i10.as_mut_ptr().offset((2) as isize))));
    let mut i11: [i32; 4] = [0, 0, 0, 0];
    let mut i12: [i32; 4] = [0, 0, 0, 1];
    let mut i13: [i32; 4] = [0, 0, 1, 0];
    let mut i14: [i32; 4] = [0, 0, 1, 1];
    let mut i15: [i32; 4] = [0, 1, 0, 0];
    let mut i16: [i32; 4] = [0, 1, 0, 1];
    let mut i17: [i32; 4] = [0, 1, 1, 0];
    let mut i18: [i32; 4] = [0, 1, 1, 1];
    let mut i19: [i32; 4] = [1, 0, 0, 0];
    let mut i20: [i32; 4] = [1, 0, 0, 1];
    let mut i21: [i32; 4] = [1, 0, 1, 0];
    let mut i22: [i32; 4] = [1, 0, 1, 1];
    let mut i23: [i32; 4] = [1, 1, 0, 0];
    let mut i24: [i32; 4] = [1, 1, 0, 1];
    let mut i25: [i32; 4] = [1, 1, 1, 0];
    assert!(((unimplemented!()) == (i11.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i12.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i13.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i14.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i15.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i16.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i17.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i18.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i19.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i20.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i21.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i22.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i23.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i24.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i25.as_mut_ptr().offset((3) as isize))));
    let mut i26: [i32; 5] = [0, 0, 0, 0, 0];
    let mut i27: [i32; 5] = [0, 0, 0, 0, 1];
    let mut i28: [i32; 5] = [0, 0, 0, 1, 0];
    let mut i29: [i32; 5] = [0, 0, 0, 1, 1];
    let mut i30: [i32; 5] = [0, 0, 1, 0, 0];
    let mut i31: [i32; 5] = [0, 0, 1, 0, 1];
    let mut i32: [i32; 5] = [0, 0, 1, 1, 0];
    let mut i33: [i32; 5] = [0, 0, 1, 1, 1];
    let mut i34: [i32; 5] = [0, 1, 0, 0, 0];
    let mut i35: [i32; 5] = [0, 1, 0, 0, 1];
    let mut i36: [i32; 5] = [0, 1, 0, 1, 0];
    let mut i37: [i32; 5] = [0, 1, 0, 1, 1];
    let mut i38: [i32; 5] = [0, 1, 1, 0, 0];
    let mut i39: [i32; 5] = [0, 1, 1, 0, 1];
    let mut i40: [i32; 5] = [0, 1, 1, 1, 0];
    let mut i41: [i32; 5] = [0, 1, 1, 1, 1];
    let mut i42: [i32; 5] = [1, 0, 0, 0, 0];
    let mut i43: [i32; 5] = [1, 0, 0, 0, 1];
    let mut i44: [i32; 5] = [1, 0, 0, 1, 0];
    let mut i45: [i32; 5] = [1, 0, 0, 1, 1];
    let mut i46: [i32; 5] = [1, 0, 1, 0, 0];
    let mut i47: [i32; 5] = [1, 0, 1, 0, 1];
    let mut i48: [i32; 5] = [1, 0, 1, 1, 0];
    let mut i49: [i32; 5] = [1, 0, 1, 1, 1];
    let mut i50: [i32; 5] = [1, 1, 0, 0, 0];
    let mut i51: [i32; 5] = [1, 1, 0, 0, 1];
    let mut i52: [i32; 5] = [1, 1, 0, 1, 0];
    let mut i53: [i32; 5] = [1, 1, 0, 1, 1];
    let mut i54: [i32; 5] = [1, 1, 1, 0, 0];
    let mut i55: [i32; 5] = [1, 1, 1, 0, 1];
    let mut i56: [i32; 5] = [1, 1, 1, 1, 0];
    assert!(((unimplemented!()) == (i26.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i27.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i28.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i29.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i30.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i31.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i32.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i33.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i34.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i35.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i36.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i37.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i38.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i39.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i40.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i41.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i42.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i43.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i44.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i45.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i46.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i47.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i48.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i49.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i50.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i51.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i52.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i53.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i54.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i55.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i56.as_mut_ptr().offset((4) as isize))));
    let mut i57: [i32; 6] = [0, 0, 0, 0, 0, 0];
    let mut i58: [i32; 6] = [0, 0, 0, 0, 0, 1];
    let mut i59: [i32; 6] = [0, 0, 0, 0, 1, 0];
    let mut i60: [i32; 6] = [0, 0, 0, 0, 1, 1];
    let mut i61: [i32; 6] = [0, 0, 0, 1, 0, 0];
    let mut i62: [i32; 6] = [0, 0, 0, 1, 0, 1];
    let mut i63: [i32; 6] = [0, 0, 0, 1, 1, 0];
    let mut i64: [i32; 6] = [0, 0, 0, 1, 1, 1];
    let mut i65: [i32; 6] = [0, 0, 1, 0, 0, 0];
    let mut i66: [i32; 6] = [0, 0, 1, 0, 0, 1];
    let mut i67: [i32; 6] = [0, 0, 1, 0, 1, 0];
    let mut i68: [i32; 6] = [0, 0, 1, 0, 1, 1];
    let mut i69: [i32; 6] = [0, 0, 1, 1, 0, 0];
    let mut i70: [i32; 6] = [0, 0, 1, 1, 0, 1];
    let mut i71: [i32; 6] = [0, 0, 1, 1, 1, 0];
    let mut i72: [i32; 6] = [0, 0, 1, 1, 1, 1];
    let mut i73: [i32; 6] = [0, 1, 0, 0, 0, 0];
    let mut i74: [i32; 6] = [0, 1, 0, 0, 0, 1];
    let mut i75: [i32; 6] = [0, 1, 0, 0, 1, 0];
    let mut i76: [i32; 6] = [0, 1, 0, 0, 1, 1];
    let mut i77: [i32; 6] = [0, 1, 0, 1, 0, 0];
    let mut i78: [i32; 6] = [0, 1, 0, 1, 0, 1];
    let mut i79: [i32; 6] = [0, 1, 0, 1, 1, 0];
    let mut i80: [i32; 6] = [0, 1, 0, 1, 1, 1];
    let mut i81: [i32; 6] = [0, 1, 1, 0, 0, 0];
    let mut i82: [i32; 6] = [0, 1, 1, 0, 0, 1];
    let mut i83: [i32; 6] = [0, 1, 1, 0, 1, 0];
    let mut i84: [i32; 6] = [0, 1, 1, 0, 1, 1];
    let mut i85: [i32; 6] = [0, 1, 1, 1, 0, 0];
    let mut i86: [i32; 6] = [0, 1, 1, 1, 0, 1];
    let mut i87: [i32; 6] = [0, 1, 1, 1, 1, 0];
    let mut i88: [i32; 6] = [0, 1, 1, 1, 1, 1];
    let mut i89: [i32; 6] = [1, 0, 0, 0, 0, 0];
    let mut i90: [i32; 6] = [1, 0, 0, 0, 0, 1];
    let mut i91: [i32; 6] = [1, 0, 0, 0, 1, 0];
    let mut i92: [i32; 6] = [1, 0, 0, 0, 1, 1];
    let mut i93: [i32; 6] = [1, 0, 0, 1, 0, 0];
    let mut i94: [i32; 6] = [1, 0, 0, 1, 0, 1];
    let mut i95: [i32; 6] = [1, 0, 0, 1, 1, 0];
    let mut i96: [i32; 6] = [1, 0, 0, 1, 1, 1];
    let mut i97: [i32; 6] = [1, 0, 1, 0, 0, 0];
    let mut i98: [i32; 6] = [1, 0, 1, 0, 0, 1];
    let mut i99: [i32; 6] = [1, 0, 1, 0, 1, 0];
    let mut i100: [i32; 6] = [1, 0, 1, 0, 1, 1];
    let mut i101: [i32; 6] = [1, 0, 1, 1, 0, 0];
    let mut i102: [i32; 6] = [1, 0, 1, 1, 0, 1];
    let mut i103: [i32; 6] = [1, 0, 1, 1, 1, 0];
    let mut i104: [i32; 6] = [1, 0, 1, 1, 1, 1];
    let mut i105: [i32; 6] = [1, 1, 0, 0, 0, 0];
    let mut i106: [i32; 6] = [1, 1, 0, 0, 0, 1];
    let mut i107: [i32; 6] = [1, 1, 0, 0, 1, 0];
    let mut i108: [i32; 6] = [1, 1, 0, 0, 1, 1];
    let mut i109: [i32; 6] = [1, 1, 0, 1, 0, 0];
    let mut i110: [i32; 6] = [1, 1, 0, 1, 0, 1];
    let mut i111: [i32; 6] = [1, 1, 0, 1, 1, 0];
    let mut i112: [i32; 6] = [1, 1, 0, 1, 1, 1];
    let mut i113: [i32; 6] = [1, 1, 1, 0, 0, 0];
    let mut i114: [i32; 6] = [1, 1, 1, 0, 0, 1];
    let mut i115: [i32; 6] = [1, 1, 1, 0, 1, 0];
    let mut i116: [i32; 6] = [1, 1, 1, 0, 1, 1];
    let mut i117: [i32; 6] = [1, 1, 1, 1, 0, 0];
    let mut i118: [i32; 6] = [1, 1, 1, 1, 0, 1];
    let mut i119: [i32; 6] = [1, 1, 1, 1, 1, 0];
    assert!(((unimplemented!()) == (i57.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i58.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i59.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i60.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i61.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i62.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i63.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i64.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i65.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i66.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i67.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i68.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i69.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i70.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i71.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i72.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i73.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i74.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i75.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i76.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i77.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i78.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i79.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i80.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i81.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i82.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i83.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i84.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i85.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i86.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i87.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i88.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i89.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i90.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i91.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i92.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i93.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i94.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i95.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i96.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i97.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i98.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i99.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i100.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i101.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i102.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i103.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i104.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i105.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i106.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i107.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i108.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i109.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i110.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i111.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i112.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i113.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i114.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i115.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i116.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i117.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i118.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i119.as_mut_ptr().offset((5) as isize))));
    let mut i120: [i32; 7] = [0, 0, 0, 0, 0, 0, 0];
    let mut i121: [i32; 7] = [0, 0, 0, 0, 0, 0, 1];
    let mut i122: [i32; 7] = [0, 0, 0, 0, 0, 1, 0];
    let mut i123: [i32; 7] = [0, 0, 0, 0, 0, 1, 1];
    let mut i124: [i32; 7] = [0, 0, 0, 0, 1, 0, 0];
    let mut i125: [i32; 7] = [0, 0, 0, 0, 1, 0, 1];
    let mut i126: [i32; 7] = [0, 0, 0, 0, 1, 1, 0];
    let mut i127: [i32; 7] = [0, 0, 0, 0, 1, 1, 1];
    let mut i128: [i32; 7] = [0, 0, 0, 1, 0, 0, 0];
    let mut i129: [i32; 7] = [0, 0, 0, 1, 0, 0, 1];
    let mut i130: [i32; 7] = [0, 0, 0, 1, 0, 1, 0];
    let mut i131: [i32; 7] = [0, 0, 0, 1, 0, 1, 1];
    let mut i132: [i32; 7] = [0, 0, 0, 1, 1, 0, 0];
    let mut i133: [i32; 7] = [0, 0, 0, 1, 1, 0, 1];
    let mut i134: [i32; 7] = [0, 0, 0, 1, 1, 1, 0];
    let mut i135: [i32; 7] = [0, 0, 0, 1, 1, 1, 1];
    let mut i136: [i32; 7] = [0, 0, 1, 0, 0, 0, 0];
    let mut i137: [i32; 7] = [0, 0, 1, 0, 0, 0, 1];
    let mut i138: [i32; 7] = [0, 0, 1, 0, 0, 1, 0];
    let mut i139: [i32; 7] = [0, 0, 1, 0, 0, 1, 1];
    let mut i140: [i32; 7] = [0, 0, 1, 0, 1, 0, 0];
    let mut i141: [i32; 7] = [0, 0, 1, 0, 1, 0, 1];
    let mut i142: [i32; 7] = [0, 0, 1, 0, 1, 1, 0];
    let mut i143: [i32; 7] = [0, 0, 1, 0, 1, 1, 1];
    let mut i144: [i32; 7] = [0, 0, 1, 1, 0, 0, 0];
    let mut i145: [i32; 7] = [0, 0, 1, 1, 0, 0, 1];
    let mut i146: [i32; 7] = [0, 0, 1, 1, 0, 1, 0];
    let mut i147: [i32; 7] = [0, 0, 1, 1, 0, 1, 1];
    let mut i148: [i32; 7] = [0, 0, 1, 1, 1, 0, 0];
    let mut i149: [i32; 7] = [0, 0, 1, 1, 1, 0, 1];
    let mut i150: [i32; 7] = [0, 0, 1, 1, 1, 1, 0];
    let mut i151: [i32; 7] = [0, 0, 1, 1, 1, 1, 1];
    let mut i152: [i32; 7] = [0, 1, 0, 0, 0, 0, 0];
    let mut i153: [i32; 7] = [0, 1, 0, 0, 0, 0, 1];
    let mut i154: [i32; 7] = [0, 1, 0, 0, 0, 1, 0];
    let mut i155: [i32; 7] = [0, 1, 0, 0, 0, 1, 1];
    let mut i156: [i32; 7] = [0, 1, 0, 0, 1, 0, 0];
    let mut i157: [i32; 7] = [0, 1, 0, 0, 1, 0, 1];
    let mut i158: [i32; 7] = [0, 1, 0, 0, 1, 1, 0];
    let mut i159: [i32; 7] = [0, 1, 0, 0, 1, 1, 1];
    let mut i160: [i32; 7] = [0, 1, 0, 1, 0, 0, 0];
    let mut i161: [i32; 7] = [0, 1, 0, 1, 0, 0, 1];
    let mut i162: [i32; 7] = [0, 1, 0, 1, 0, 1, 0];
    let mut i163: [i32; 7] = [0, 1, 0, 1, 0, 1, 1];
    let mut i164: [i32; 7] = [0, 1, 0, 1, 1, 0, 0];
    let mut i165: [i32; 7] = [0, 1, 0, 1, 1, 0, 1];
    let mut i166: [i32; 7] = [0, 1, 0, 1, 1, 1, 0];
    let mut i167: [i32; 7] = [0, 1, 0, 1, 1, 1, 1];
    let mut i168: [i32; 7] = [0, 1, 1, 0, 0, 0, 0];
    let mut i169: [i32; 7] = [0, 1, 1, 0, 0, 0, 1];
    let mut i170: [i32; 7] = [0, 1, 1, 0, 0, 1, 0];
    let mut i171: [i32; 7] = [0, 1, 1, 0, 0, 1, 1];
    let mut i172: [i32; 7] = [0, 1, 1, 0, 1, 0, 0];
    let mut i173: [i32; 7] = [0, 1, 1, 0, 1, 0, 1];
    let mut i174: [i32; 7] = [0, 1, 1, 0, 1, 1, 0];
    let mut i175: [i32; 7] = [0, 1, 1, 0, 1, 1, 1];
    let mut i176: [i32; 7] = [0, 1, 1, 1, 0, 0, 0];
    let mut i177: [i32; 7] = [0, 1, 1, 1, 0, 0, 1];
    let mut i178: [i32; 7] = [0, 1, 1, 1, 0, 1, 0];
    let mut i179: [i32; 7] = [0, 1, 1, 1, 0, 1, 1];
    let mut i180: [i32; 7] = [0, 1, 1, 1, 1, 0, 0];
    let mut i181: [i32; 7] = [0, 1, 1, 1, 1, 0, 1];
    let mut i182: [i32; 7] = [0, 1, 1, 1, 1, 1, 0];
    let mut i183: [i32; 7] = [0, 1, 1, 1, 1, 1, 1];
    let mut i184: [i32; 7] = [1, 0, 0, 0, 0, 0, 0];
    let mut i185: [i32; 7] = [1, 0, 0, 0, 0, 0, 1];
    let mut i186: [i32; 7] = [1, 0, 0, 0, 0, 1, 0];
    let mut i187: [i32; 7] = [1, 0, 0, 0, 0, 1, 1];
    let mut i188: [i32; 7] = [1, 0, 0, 0, 1, 0, 0];
    let mut i189: [i32; 7] = [1, 0, 0, 0, 1, 0, 1];
    let mut i190: [i32; 7] = [1, 0, 0, 0, 1, 1, 0];
    let mut i191: [i32; 7] = [1, 0, 0, 0, 1, 1, 1];
    let mut i192: [i32; 7] = [1, 0, 0, 1, 0, 0, 0];
    let mut i193: [i32; 7] = [1, 0, 0, 1, 0, 0, 1];
    let mut i194: [i32; 7] = [1, 0, 0, 1, 0, 1, 0];
    let mut i195: [i32; 7] = [1, 0, 0, 1, 0, 1, 1];
    let mut i196: [i32; 7] = [1, 0, 0, 1, 1, 0, 0];
    let mut i197: [i32; 7] = [1, 0, 0, 1, 1, 0, 1];
    let mut i198: [i32; 7] = [1, 0, 0, 1, 1, 1, 0];
    let mut i199: [i32; 7] = [1, 0, 0, 1, 1, 1, 1];
    let mut i200: [i32; 7] = [1, 0, 1, 0, 0, 0, 0];
    let mut i201: [i32; 7] = [1, 0, 1, 0, 0, 0, 1];
    let mut i202: [i32; 7] = [1, 0, 1, 0, 0, 1, 0];
    let mut i203: [i32; 7] = [1, 0, 1, 0, 0, 1, 1];
    let mut i204: [i32; 7] = [1, 0, 1, 0, 1, 0, 0];
    let mut i205: [i32; 7] = [1, 0, 1, 0, 1, 0, 1];
    let mut i206: [i32; 7] = [1, 0, 1, 0, 1, 1, 0];
    let mut i207: [i32; 7] = [1, 0, 1, 0, 1, 1, 1];
    let mut i208: [i32; 7] = [1, 0, 1, 1, 0, 0, 0];
    let mut i209: [i32; 7] = [1, 0, 1, 1, 0, 0, 1];
    let mut i210: [i32; 7] = [1, 0, 1, 1, 0, 1, 0];
    let mut i211: [i32; 7] = [1, 0, 1, 1, 0, 1, 1];
    let mut i212: [i32; 7] = [1, 0, 1, 1, 1, 0, 0];
    let mut i213: [i32; 7] = [1, 0, 1, 1, 1, 0, 1];
    let mut i214: [i32; 7] = [1, 0, 1, 1, 1, 1, 0];
    let mut i215: [i32; 7] = [1, 0, 1, 1, 1, 1, 1];
    let mut i216: [i32; 7] = [1, 1, 0, 0, 0, 0, 0];
    let mut i217: [i32; 7] = [1, 1, 0, 0, 0, 0, 1];
    let mut i218: [i32; 7] = [1, 1, 0, 0, 0, 1, 0];
    let mut i219: [i32; 7] = [1, 1, 0, 0, 0, 1, 1];
    let mut i220: [i32; 7] = [1, 1, 0, 0, 1, 0, 0];
    let mut i221: [i32; 7] = [1, 1, 0, 0, 1, 0, 1];
    let mut i222: [i32; 7] = [1, 1, 0, 0, 1, 1, 0];
    let mut i223: [i32; 7] = [1, 1, 0, 0, 1, 1, 1];
    let mut i224: [i32; 7] = [1, 1, 0, 1, 0, 0, 0];
    let mut i225: [i32; 7] = [1, 1, 0, 1, 0, 0, 1];
    let mut i226: [i32; 7] = [1, 1, 0, 1, 0, 1, 0];
    let mut i227: [i32; 7] = [1, 1, 0, 1, 0, 1, 1];
    let mut i228: [i32; 7] = [1, 1, 0, 1, 1, 0, 0];
    let mut i229: [i32; 7] = [1, 1, 0, 1, 1, 0, 1];
    let mut i230: [i32; 7] = [1, 1, 0, 1, 1, 1, 0];
    let mut i231: [i32; 7] = [1, 1, 0, 1, 1, 1, 1];
    let mut i232: [i32; 7] = [1, 1, 1, 0, 0, 0, 0];
    let mut i233: [i32; 7] = [1, 1, 1, 0, 0, 0, 1];
    let mut i234: [i32; 7] = [1, 1, 1, 0, 0, 1, 0];
    let mut i235: [i32; 7] = [1, 1, 1, 0, 0, 1, 1];
    let mut i236: [i32; 7] = [1, 1, 1, 0, 1, 0, 0];
    let mut i237: [i32; 7] = [1, 1, 1, 0, 1, 0, 1];
    let mut i238: [i32; 7] = [1, 1, 1, 0, 1, 1, 0];
    let mut i239: [i32; 7] = [1, 1, 1, 0, 1, 1, 1];
    let mut i240: [i32; 7] = [1, 1, 1, 1, 0, 0, 0];
    let mut i241: [i32; 7] = [1, 1, 1, 1, 0, 0, 1];
    let mut i242: [i32; 7] = [1, 1, 1, 1, 0, 1, 0];
    let mut i243: [i32; 7] = [1, 1, 1, 1, 0, 1, 1];
    let mut i244: [i32; 7] = [1, 1, 1, 1, 1, 0, 0];
    let mut i245: [i32; 7] = [1, 1, 1, 1, 1, 0, 1];
    let mut i246: [i32; 7] = [1, 1, 1, 1, 1, 1, 0];
    assert!(((unimplemented!()) == (i120.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i121.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i122.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i123.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i124.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i125.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i126.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i127.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i128.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i129.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i130.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i131.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i132.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i133.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i134.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i135.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i136.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i137.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i138.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i139.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i140.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i141.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i142.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i143.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i144.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i145.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i146.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i147.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i148.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i149.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i150.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i151.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i152.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i153.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i154.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i155.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i156.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i157.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i158.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i159.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i160.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i161.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i162.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i163.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i164.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i165.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i166.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i167.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i168.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i169.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i170.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i171.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i172.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i173.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i174.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i175.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i176.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i177.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i178.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i179.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i180.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i181.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i182.as_mut_ptr().offset((6) as isize))));
    assert!(((unimplemented!()) == (i183.as_mut_ptr().offset((7) as isize))));
    assert!(((unimplemented!()) == (i184.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i185.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i186.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i187.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i188.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i189.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i190.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i191.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i192.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i193.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i194.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i195.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i196.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i197.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i198.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i199.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i200.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i201.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i202.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i203.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i204.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i205.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i206.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i207.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i208.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i209.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i210.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i211.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i212.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i213.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i214.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i215.as_mut_ptr().offset((1) as isize))));
    assert!(((unimplemented!()) == (i216.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i217.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i218.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i219.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i220.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i221.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i222.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i223.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i224.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i225.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i226.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i227.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i228.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i229.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i230.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i231.as_mut_ptr().offset((2) as isize))));
    assert!(((unimplemented!()) == (i232.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i233.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i234.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i235.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i236.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i237.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i238.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i239.as_mut_ptr().offset((3) as isize))));
    assert!(((unimplemented!()) == (i240.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i241.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i242.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i243.as_mut_ptr().offset((4) as isize))));
    assert!(((unimplemented!()) == (i244.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i245.as_mut_ptr().offset((5) as isize))));
    assert!(((unimplemented!()) == (i246.as_mut_ptr().offset((6) as isize))));
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
    (unsafe { test_11() });
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
