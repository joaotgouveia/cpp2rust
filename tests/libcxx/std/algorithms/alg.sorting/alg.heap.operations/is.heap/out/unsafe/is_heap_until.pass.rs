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
pub unsafe fn operator_add_0(
    mut x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_1(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_2(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
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
pub unsafe fn operator_eq_3(
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
pub unsafe fn operator_add_4(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_5(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_4(_i, n)
    });
}
pub unsafe fn iter_swap_6(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_7(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*unimplemented!());
}
pub unsafe fn operator_eq_8(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_9(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
pub unsafe fn test_constexpr_10() -> () {
    let mut ia: [(); 6] = [0_(), 0_(), 0_(), 0_(), 1_(), 0_()];
    let mut ib: [(); 6] = [0_(), 0_(), 0_(), 1_(), 1_(), 1_()];
    return ((unimplemented!()) == (ia.as_mut_ptr().offset((4_()) as isize)))
        && ((unimplemented!()) == (ib.as_mut_ptr().offset((3_()) as isize)));
}
pub unsafe fn test_11() {
    let mut i1: [(); 2] = [0_(), 0_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i2: [(); 2] = [0_(), 1_()];
    let mut i3: [(); 2] = [1_(), 0_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i4: [(); 3] = [0_(), 0_(), 0_()];
    let mut i5: [(); 3] = [0_(), 0_(), 1_()];
    let mut i6: [(); 3] = [0_(), 1_(), 0_()];
    let mut i7: [(); 3] = [0_(), 1_(), 1_()];
    let mut i8: [(); 3] = [1_(), 0_(), 0_()];
    let mut i9: [(); 3] = [1_(), 0_(), 1_()];
    let mut i10: [(); 3] = [1_(), 1_(), 0_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i11: [(); 4] = [0_(), 0_(), 0_(), 0_()];
    let mut i12: [(); 4] = [0_(), 0_(), 0_(), 1_()];
    let mut i13: [(); 4] = [0_(), 0_(), 1_(), 0_()];
    let mut i14: [(); 4] = [0_(), 0_(), 1_(), 1_()];
    let mut i15: [(); 4] = [0_(), 1_(), 0_(), 0_()];
    let mut i16: [(); 4] = [0_(), 1_(), 0_(), 1_()];
    let mut i17: [(); 4] = [0_(), 1_(), 1_(), 0_()];
    let mut i18: [(); 4] = [0_(), 1_(), 1_(), 1_()];
    let mut i19: [(); 4] = [1_(), 0_(), 0_(), 0_()];
    let mut i20: [(); 4] = [1_(), 0_(), 0_(), 1_()];
    let mut i21: [(); 4] = [1_(), 0_(), 1_(), 0_()];
    let mut i22: [(); 4] = [1_(), 0_(), 1_(), 1_()];
    let mut i23: [(); 4] = [1_(), 1_(), 0_(), 0_()];
    let mut i24: [(); 4] = [1_(), 1_(), 0_(), 1_()];
    let mut i25: [(); 4] = [1_(), 1_(), 1_(), 0_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i26: [(); 5] = [0_(), 0_(), 0_(), 0_(), 0_()];
    let mut i27: [(); 5] = [0_(), 0_(), 0_(), 0_(), 1_()];
    let mut i28: [(); 5] = [0_(), 0_(), 0_(), 1_(), 0_()];
    let mut i29: [(); 5] = [0_(), 0_(), 0_(), 1_(), 1_()];
    let mut i30: [(); 5] = [0_(), 0_(), 1_(), 0_(), 0_()];
    let mut i31: [(); 5] = [0_(), 0_(), 1_(), 0_(), 1_()];
    let mut i32: [(); 5] = [0_(), 0_(), 1_(), 1_(), 0_()];
    let mut i33: [(); 5] = [0_(), 0_(), 1_(), 1_(), 1_()];
    let mut i34: [(); 5] = [0_(), 1_(), 0_(), 0_(), 0_()];
    let mut i35: [(); 5] = [0_(), 1_(), 0_(), 0_(), 1_()];
    let mut i36: [(); 5] = [0_(), 1_(), 0_(), 1_(), 0_()];
    let mut i37: [(); 5] = [0_(), 1_(), 0_(), 1_(), 1_()];
    let mut i38: [(); 5] = [0_(), 1_(), 1_(), 0_(), 0_()];
    let mut i39: [(); 5] = [0_(), 1_(), 1_(), 0_(), 1_()];
    let mut i40: [(); 5] = [0_(), 1_(), 1_(), 1_(), 0_()];
    let mut i41: [(); 5] = [0_(), 1_(), 1_(), 1_(), 1_()];
    let mut i42: [(); 5] = [1_(), 0_(), 0_(), 0_(), 0_()];
    let mut i43: [(); 5] = [1_(), 0_(), 0_(), 0_(), 1_()];
    let mut i44: [(); 5] = [1_(), 0_(), 0_(), 1_(), 0_()];
    let mut i45: [(); 5] = [1_(), 0_(), 0_(), 1_(), 1_()];
    let mut i46: [(); 5] = [1_(), 0_(), 1_(), 0_(), 0_()];
    let mut i47: [(); 5] = [1_(), 0_(), 1_(), 0_(), 1_()];
    let mut i48: [(); 5] = [1_(), 0_(), 1_(), 1_(), 0_()];
    let mut i49: [(); 5] = [1_(), 0_(), 1_(), 1_(), 1_()];
    let mut i50: [(); 5] = [1_(), 1_(), 0_(), 0_(), 0_()];
    let mut i51: [(); 5] = [1_(), 1_(), 0_(), 0_(), 1_()];
    let mut i52: [(); 5] = [1_(), 1_(), 0_(), 1_(), 0_()];
    let mut i53: [(); 5] = [1_(), 1_(), 0_(), 1_(), 1_()];
    let mut i54: [(); 5] = [1_(), 1_(), 1_(), 0_(), 0_()];
    let mut i55: [(); 5] = [1_(), 1_(), 1_(), 0_(), 1_()];
    let mut i56: [(); 5] = [1_(), 1_(), 1_(), 1_(), 0_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i57: [(); 6] = [0_(), 0_(), 0_(), 0_(), 0_(), 0_()];
    let mut i58: [(); 6] = [0_(), 0_(), 0_(), 0_(), 0_(), 1_()];
    let mut i59: [(); 6] = [0_(), 0_(), 0_(), 0_(), 1_(), 0_()];
    let mut i60: [(); 6] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_()];
    let mut i61: [(); 6] = [0_(), 0_(), 0_(), 1_(), 0_(), 0_()];
    let mut i62: [(); 6] = [0_(), 0_(), 0_(), 1_(), 0_(), 1_()];
    let mut i63: [(); 6] = [0_(), 0_(), 0_(), 1_(), 1_(), 0_()];
    let mut i64: [(); 6] = [0_(), 0_(), 0_(), 1_(), 1_(), 1_()];
    let mut i65: [(); 6] = [0_(), 0_(), 1_(), 0_(), 0_(), 0_()];
    let mut i66: [(); 6] = [0_(), 0_(), 1_(), 0_(), 0_(), 1_()];
    let mut i67: [(); 6] = [0_(), 0_(), 1_(), 0_(), 1_(), 0_()];
    let mut i68: [(); 6] = [0_(), 0_(), 1_(), 0_(), 1_(), 1_()];
    let mut i69: [(); 6] = [0_(), 0_(), 1_(), 1_(), 0_(), 0_()];
    let mut i70: [(); 6] = [0_(), 0_(), 1_(), 1_(), 0_(), 1_()];
    let mut i71: [(); 6] = [0_(), 0_(), 1_(), 1_(), 1_(), 0_()];
    let mut i72: [(); 6] = [0_(), 0_(), 1_(), 1_(), 1_(), 1_()];
    let mut i73: [(); 6] = [0_(), 1_(), 0_(), 0_(), 0_(), 0_()];
    let mut i74: [(); 6] = [0_(), 1_(), 0_(), 0_(), 0_(), 1_()];
    let mut i75: [(); 6] = [0_(), 1_(), 0_(), 0_(), 1_(), 0_()];
    let mut i76: [(); 6] = [0_(), 1_(), 0_(), 0_(), 1_(), 1_()];
    let mut i77: [(); 6] = [0_(), 1_(), 0_(), 1_(), 0_(), 0_()];
    let mut i78: [(); 6] = [0_(), 1_(), 0_(), 1_(), 0_(), 1_()];
    let mut i79: [(); 6] = [0_(), 1_(), 0_(), 1_(), 1_(), 0_()];
    let mut i80: [(); 6] = [0_(), 1_(), 0_(), 1_(), 1_(), 1_()];
    let mut i81: [(); 6] = [0_(), 1_(), 1_(), 0_(), 0_(), 0_()];
    let mut i82: [(); 6] = [0_(), 1_(), 1_(), 0_(), 0_(), 1_()];
    let mut i83: [(); 6] = [0_(), 1_(), 1_(), 0_(), 1_(), 0_()];
    let mut i84: [(); 6] = [0_(), 1_(), 1_(), 0_(), 1_(), 1_()];
    let mut i85: [(); 6] = [0_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let mut i86: [(); 6] = [0_(), 1_(), 1_(), 1_(), 0_(), 1_()];
    let mut i87: [(); 6] = [0_(), 1_(), 1_(), 1_(), 1_(), 0_()];
    let mut i88: [(); 6] = [0_(), 1_(), 1_(), 1_(), 1_(), 1_()];
    let mut i89: [(); 6] = [1_(), 0_(), 0_(), 0_(), 0_(), 0_()];
    let mut i90: [(); 6] = [1_(), 0_(), 0_(), 0_(), 0_(), 1_()];
    let mut i91: [(); 6] = [1_(), 0_(), 0_(), 0_(), 1_(), 0_()];
    let mut i92: [(); 6] = [1_(), 0_(), 0_(), 0_(), 1_(), 1_()];
    let mut i93: [(); 6] = [1_(), 0_(), 0_(), 1_(), 0_(), 0_()];
    let mut i94: [(); 6] = [1_(), 0_(), 0_(), 1_(), 0_(), 1_()];
    let mut i95: [(); 6] = [1_(), 0_(), 0_(), 1_(), 1_(), 0_()];
    let mut i96: [(); 6] = [1_(), 0_(), 0_(), 1_(), 1_(), 1_()];
    let mut i97: [(); 6] = [1_(), 0_(), 1_(), 0_(), 0_(), 0_()];
    let mut i98: [(); 6] = [1_(), 0_(), 1_(), 0_(), 0_(), 1_()];
    let mut i99: [(); 6] = [1_(), 0_(), 1_(), 0_(), 1_(), 0_()];
    let mut i100: [(); 6] = [1_(), 0_(), 1_(), 0_(), 1_(), 1_()];
    let mut i101: [(); 6] = [1_(), 0_(), 1_(), 1_(), 0_(), 0_()];
    let mut i102: [(); 6] = [1_(), 0_(), 1_(), 1_(), 0_(), 1_()];
    let mut i103: [(); 6] = [1_(), 0_(), 1_(), 1_(), 1_(), 0_()];
    let mut i104: [(); 6] = [1_(), 0_(), 1_(), 1_(), 1_(), 1_()];
    let mut i105: [(); 6] = [1_(), 1_(), 0_(), 0_(), 0_(), 0_()];
    let mut i106: [(); 6] = [1_(), 1_(), 0_(), 0_(), 0_(), 1_()];
    let mut i107: [(); 6] = [1_(), 1_(), 0_(), 0_(), 1_(), 0_()];
    let mut i108: [(); 6] = [1_(), 1_(), 0_(), 0_(), 1_(), 1_()];
    let mut i109: [(); 6] = [1_(), 1_(), 0_(), 1_(), 0_(), 0_()];
    let mut i110: [(); 6] = [1_(), 1_(), 0_(), 1_(), 0_(), 1_()];
    let mut i111: [(); 6] = [1_(), 1_(), 0_(), 1_(), 1_(), 0_()];
    let mut i112: [(); 6] = [1_(), 1_(), 0_(), 1_(), 1_(), 1_()];
    let mut i113: [(); 6] = [1_(), 1_(), 1_(), 0_(), 0_(), 0_()];
    let mut i114: [(); 6] = [1_(), 1_(), 1_(), 0_(), 0_(), 1_()];
    let mut i115: [(); 6] = [1_(), 1_(), 1_(), 0_(), 1_(), 0_()];
    let mut i116: [(); 6] = [1_(), 1_(), 1_(), 0_(), 1_(), 1_()];
    let mut i117: [(); 6] = [1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let mut i118: [(); 6] = [1_(), 1_(), 1_(), 1_(), 0_(), 1_()];
    let mut i119: [(); 6] = [1_(), 1_(), 1_(), 1_(), 1_(), 0_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i120: [(); 7] = [0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_()];
    let mut i121: [(); 7] = [0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 1_()];
    let mut i122: [(); 7] = [0_(), 0_(), 0_(), 0_(), 0_(), 1_(), 0_()];
    let mut i123: [(); 7] = [0_(), 0_(), 0_(), 0_(), 0_(), 1_(), 1_()];
    let mut i124: [(); 7] = [0_(), 0_(), 0_(), 0_(), 1_(), 0_(), 0_()];
    let mut i125: [(); 7] = [0_(), 0_(), 0_(), 0_(), 1_(), 0_(), 1_()];
    let mut i126: [(); 7] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 0_()];
    let mut i127: [(); 7] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_()];
    let mut i128: [(); 7] = [0_(), 0_(), 0_(), 1_(), 0_(), 0_(), 0_()];
    let mut i129: [(); 7] = [0_(), 0_(), 0_(), 1_(), 0_(), 0_(), 1_()];
    let mut i130: [(); 7] = [0_(), 0_(), 0_(), 1_(), 0_(), 1_(), 0_()];
    let mut i131: [(); 7] = [0_(), 0_(), 0_(), 1_(), 0_(), 1_(), 1_()];
    let mut i132: [(); 7] = [0_(), 0_(), 0_(), 1_(), 1_(), 0_(), 0_()];
    let mut i133: [(); 7] = [0_(), 0_(), 0_(), 1_(), 1_(), 0_(), 1_()];
    let mut i134: [(); 7] = [0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 0_()];
    let mut i135: [(); 7] = [0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_()];
    let mut i136: [(); 7] = [0_(), 0_(), 1_(), 0_(), 0_(), 0_(), 0_()];
    let mut i137: [(); 7] = [0_(), 0_(), 1_(), 0_(), 0_(), 0_(), 1_()];
    let mut i138: [(); 7] = [0_(), 0_(), 1_(), 0_(), 0_(), 1_(), 0_()];
    let mut i139: [(); 7] = [0_(), 0_(), 1_(), 0_(), 0_(), 1_(), 1_()];
    let mut i140: [(); 7] = [0_(), 0_(), 1_(), 0_(), 1_(), 0_(), 0_()];
    let mut i141: [(); 7] = [0_(), 0_(), 1_(), 0_(), 1_(), 0_(), 1_()];
    let mut i142: [(); 7] = [0_(), 0_(), 1_(), 0_(), 1_(), 1_(), 0_()];
    let mut i143: [(); 7] = [0_(), 0_(), 1_(), 0_(), 1_(), 1_(), 1_()];
    let mut i144: [(); 7] = [0_(), 0_(), 1_(), 1_(), 0_(), 0_(), 0_()];
    let mut i145: [(); 7] = [0_(), 0_(), 1_(), 1_(), 0_(), 0_(), 1_()];
    let mut i146: [(); 7] = [0_(), 0_(), 1_(), 1_(), 0_(), 1_(), 0_()];
    let mut i147: [(); 7] = [0_(), 0_(), 1_(), 1_(), 0_(), 1_(), 1_()];
    let mut i148: [(); 7] = [0_(), 0_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let mut i149: [(); 7] = [0_(), 0_(), 1_(), 1_(), 1_(), 0_(), 1_()];
    let mut i150: [(); 7] = [0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_()];
    let mut i151: [(); 7] = [0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 1_()];
    let mut i152: [(); 7] = [0_(), 1_(), 0_(), 0_(), 0_(), 0_(), 0_()];
    let mut i153: [(); 7] = [0_(), 1_(), 0_(), 0_(), 0_(), 0_(), 1_()];
    let mut i154: [(); 7] = [0_(), 1_(), 0_(), 0_(), 0_(), 1_(), 0_()];
    let mut i155: [(); 7] = [0_(), 1_(), 0_(), 0_(), 0_(), 1_(), 1_()];
    let mut i156: [(); 7] = [0_(), 1_(), 0_(), 0_(), 1_(), 0_(), 0_()];
    let mut i157: [(); 7] = [0_(), 1_(), 0_(), 0_(), 1_(), 0_(), 1_()];
    let mut i158: [(); 7] = [0_(), 1_(), 0_(), 0_(), 1_(), 1_(), 0_()];
    let mut i159: [(); 7] = [0_(), 1_(), 0_(), 0_(), 1_(), 1_(), 1_()];
    let mut i160: [(); 7] = [0_(), 1_(), 0_(), 1_(), 0_(), 0_(), 0_()];
    let mut i161: [(); 7] = [0_(), 1_(), 0_(), 1_(), 0_(), 0_(), 1_()];
    let mut i162: [(); 7] = [0_(), 1_(), 0_(), 1_(), 0_(), 1_(), 0_()];
    let mut i163: [(); 7] = [0_(), 1_(), 0_(), 1_(), 0_(), 1_(), 1_()];
    let mut i164: [(); 7] = [0_(), 1_(), 0_(), 1_(), 1_(), 0_(), 0_()];
    let mut i165: [(); 7] = [0_(), 1_(), 0_(), 1_(), 1_(), 0_(), 1_()];
    let mut i166: [(); 7] = [0_(), 1_(), 0_(), 1_(), 1_(), 1_(), 0_()];
    let mut i167: [(); 7] = [0_(), 1_(), 0_(), 1_(), 1_(), 1_(), 1_()];
    let mut i168: [(); 7] = [0_(), 1_(), 1_(), 0_(), 0_(), 0_(), 0_()];
    let mut i169: [(); 7] = [0_(), 1_(), 1_(), 0_(), 0_(), 0_(), 1_()];
    let mut i170: [(); 7] = [0_(), 1_(), 1_(), 0_(), 0_(), 1_(), 0_()];
    let mut i171: [(); 7] = [0_(), 1_(), 1_(), 0_(), 0_(), 1_(), 1_()];
    let mut i172: [(); 7] = [0_(), 1_(), 1_(), 0_(), 1_(), 0_(), 0_()];
    let mut i173: [(); 7] = [0_(), 1_(), 1_(), 0_(), 1_(), 0_(), 1_()];
    let mut i174: [(); 7] = [0_(), 1_(), 1_(), 0_(), 1_(), 1_(), 0_()];
    let mut i175: [(); 7] = [0_(), 1_(), 1_(), 0_(), 1_(), 1_(), 1_()];
    let mut i176: [(); 7] = [0_(), 1_(), 1_(), 1_(), 0_(), 0_(), 0_()];
    let mut i177: [(); 7] = [0_(), 1_(), 1_(), 1_(), 0_(), 0_(), 1_()];
    let mut i178: [(); 7] = [0_(), 1_(), 1_(), 1_(), 0_(), 1_(), 0_()];
    let mut i179: [(); 7] = [0_(), 1_(), 1_(), 1_(), 0_(), 1_(), 1_()];
    let mut i180: [(); 7] = [0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let mut i181: [(); 7] = [0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 1_()];
    let mut i182: [(); 7] = [0_(), 1_(), 1_(), 1_(), 1_(), 1_(), 0_()];
    let mut i183: [(); 7] = [0_(), 1_(), 1_(), 1_(), 1_(), 1_(), 1_()];
    let mut i184: [(); 7] = [1_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_()];
    let mut i185: [(); 7] = [1_(), 0_(), 0_(), 0_(), 0_(), 0_(), 1_()];
    let mut i186: [(); 7] = [1_(), 0_(), 0_(), 0_(), 0_(), 1_(), 0_()];
    let mut i187: [(); 7] = [1_(), 0_(), 0_(), 0_(), 0_(), 1_(), 1_()];
    let mut i188: [(); 7] = [1_(), 0_(), 0_(), 0_(), 1_(), 0_(), 0_()];
    let mut i189: [(); 7] = [1_(), 0_(), 0_(), 0_(), 1_(), 0_(), 1_()];
    let mut i190: [(); 7] = [1_(), 0_(), 0_(), 0_(), 1_(), 1_(), 0_()];
    let mut i191: [(); 7] = [1_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_()];
    let mut i192: [(); 7] = [1_(), 0_(), 0_(), 1_(), 0_(), 0_(), 0_()];
    let mut i193: [(); 7] = [1_(), 0_(), 0_(), 1_(), 0_(), 0_(), 1_()];
    let mut i194: [(); 7] = [1_(), 0_(), 0_(), 1_(), 0_(), 1_(), 0_()];
    let mut i195: [(); 7] = [1_(), 0_(), 0_(), 1_(), 0_(), 1_(), 1_()];
    let mut i196: [(); 7] = [1_(), 0_(), 0_(), 1_(), 1_(), 0_(), 0_()];
    let mut i197: [(); 7] = [1_(), 0_(), 0_(), 1_(), 1_(), 0_(), 1_()];
    let mut i198: [(); 7] = [1_(), 0_(), 0_(), 1_(), 1_(), 1_(), 0_()];
    let mut i199: [(); 7] = [1_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_()];
    let mut i200: [(); 7] = [1_(), 0_(), 1_(), 0_(), 0_(), 0_(), 0_()];
    let mut i201: [(); 7] = [1_(), 0_(), 1_(), 0_(), 0_(), 0_(), 1_()];
    let mut i202: [(); 7] = [1_(), 0_(), 1_(), 0_(), 0_(), 1_(), 0_()];
    let mut i203: [(); 7] = [1_(), 0_(), 1_(), 0_(), 0_(), 1_(), 1_()];
    let mut i204: [(); 7] = [1_(), 0_(), 1_(), 0_(), 1_(), 0_(), 0_()];
    let mut i205: [(); 7] = [1_(), 0_(), 1_(), 0_(), 1_(), 0_(), 1_()];
    let mut i206: [(); 7] = [1_(), 0_(), 1_(), 0_(), 1_(), 1_(), 0_()];
    let mut i207: [(); 7] = [1_(), 0_(), 1_(), 0_(), 1_(), 1_(), 1_()];
    let mut i208: [(); 7] = [1_(), 0_(), 1_(), 1_(), 0_(), 0_(), 0_()];
    let mut i209: [(); 7] = [1_(), 0_(), 1_(), 1_(), 0_(), 0_(), 1_()];
    let mut i210: [(); 7] = [1_(), 0_(), 1_(), 1_(), 0_(), 1_(), 0_()];
    let mut i211: [(); 7] = [1_(), 0_(), 1_(), 1_(), 0_(), 1_(), 1_()];
    let mut i212: [(); 7] = [1_(), 0_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let mut i213: [(); 7] = [1_(), 0_(), 1_(), 1_(), 1_(), 0_(), 1_()];
    let mut i214: [(); 7] = [1_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_()];
    let mut i215: [(); 7] = [1_(), 0_(), 1_(), 1_(), 1_(), 1_(), 1_()];
    let mut i216: [(); 7] = [1_(), 1_(), 0_(), 0_(), 0_(), 0_(), 0_()];
    let mut i217: [(); 7] = [1_(), 1_(), 0_(), 0_(), 0_(), 0_(), 1_()];
    let mut i218: [(); 7] = [1_(), 1_(), 0_(), 0_(), 0_(), 1_(), 0_()];
    let mut i219: [(); 7] = [1_(), 1_(), 0_(), 0_(), 0_(), 1_(), 1_()];
    let mut i220: [(); 7] = [1_(), 1_(), 0_(), 0_(), 1_(), 0_(), 0_()];
    let mut i221: [(); 7] = [1_(), 1_(), 0_(), 0_(), 1_(), 0_(), 1_()];
    let mut i222: [(); 7] = [1_(), 1_(), 0_(), 0_(), 1_(), 1_(), 0_()];
    let mut i223: [(); 7] = [1_(), 1_(), 0_(), 0_(), 1_(), 1_(), 1_()];
    let mut i224: [(); 7] = [1_(), 1_(), 0_(), 1_(), 0_(), 0_(), 0_()];
    let mut i225: [(); 7] = [1_(), 1_(), 0_(), 1_(), 0_(), 0_(), 1_()];
    let mut i226: [(); 7] = [1_(), 1_(), 0_(), 1_(), 0_(), 1_(), 0_()];
    let mut i227: [(); 7] = [1_(), 1_(), 0_(), 1_(), 0_(), 1_(), 1_()];
    let mut i228: [(); 7] = [1_(), 1_(), 0_(), 1_(), 1_(), 0_(), 0_()];
    let mut i229: [(); 7] = [1_(), 1_(), 0_(), 1_(), 1_(), 0_(), 1_()];
    let mut i230: [(); 7] = [1_(), 1_(), 0_(), 1_(), 1_(), 1_(), 0_()];
    let mut i231: [(); 7] = [1_(), 1_(), 0_(), 1_(), 1_(), 1_(), 1_()];
    let mut i232: [(); 7] = [1_(), 1_(), 1_(), 0_(), 0_(), 0_(), 0_()];
    let mut i233: [(); 7] = [1_(), 1_(), 1_(), 0_(), 0_(), 0_(), 1_()];
    let mut i234: [(); 7] = [1_(), 1_(), 1_(), 0_(), 0_(), 1_(), 0_()];
    let mut i235: [(); 7] = [1_(), 1_(), 1_(), 0_(), 0_(), 1_(), 1_()];
    let mut i236: [(); 7] = [1_(), 1_(), 1_(), 0_(), 1_(), 0_(), 0_()];
    let mut i237: [(); 7] = [1_(), 1_(), 1_(), 0_(), 1_(), 0_(), 1_()];
    let mut i238: [(); 7] = [1_(), 1_(), 1_(), 0_(), 1_(), 1_(), 0_()];
    let mut i239: [(); 7] = [1_(), 1_(), 1_(), 0_(), 1_(), 1_(), 1_()];
    let mut i240: [(); 7] = [1_(), 1_(), 1_(), 1_(), 0_(), 0_(), 0_()];
    let mut i241: [(); 7] = [1_(), 1_(), 1_(), 1_(), 0_(), 0_(), 1_()];
    let mut i242: [(); 7] = [1_(), 1_(), 1_(), 1_(), 0_(), 1_(), 0_()];
    let mut i243: [(); 7] = [1_(), 1_(), 1_(), 1_(), 0_(), 1_(), 1_()];
    let mut i244: [(); 7] = [1_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let mut i245: [(); 7] = [1_(), 1_(), 1_(), 1_(), 1_(), 0_(), 1_()];
    let mut i246: [(); 7] = [1_(), 1_(), 1_(), 1_(), 1_(), 1_(), 0_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test_11() });
    const _: () = assert!(true, "test_constexpr()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
