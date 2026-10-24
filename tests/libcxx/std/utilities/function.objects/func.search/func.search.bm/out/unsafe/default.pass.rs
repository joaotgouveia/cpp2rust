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
    mut x: random_access_iterator_const_int_ptr_,
    mut n: (),
) -> random_access_iterator_const_int_ptr_ {
    (unsafe { random_access_iterator_const_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_const_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_1(
    mut x: random_access_iterator_const_int_ptr_,
    mut n: (),
) -> random_access_iterator_const_int_ptr_ {
    (unsafe { random_access_iterator_const_int_ptr_::operator_sub_assign(&mut x, n) });
    return random_access_iterator_const_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_2(
    mut x: random_access_iterator_const_int_ptr_,
    mut y: random_access_iterator_const_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_3(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_4(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_le_5(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) <= ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn operator_index(&self, mut n: ()) -> () {
        return &(*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_3(
                self as *const random_access_iterator_const_int_ptr_,
                other as *const random_access_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
pub unsafe fn operator_add_6(
    mut x: random_access_iterator_const_char_ptr_,
    mut n: (),
) -> random_access_iterator_const_char_ptr_ {
    (unsafe { random_access_iterator_const_char_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_const_char_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_7(
    mut x: random_access_iterator_const_char_ptr_,
    mut n: (),
) -> random_access_iterator_const_char_ptr_ {
    (unsafe { random_access_iterator_const_char_ptr_::operator_sub_assign(&mut x, n) });
    return random_access_iterator_const_char_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_8(
    mut x: random_access_iterator_const_char_ptr_,
    mut y: random_access_iterator_const_char_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_9(
    x: *const random_access_iterator_const_char_ptr_,
    y: *const random_access_iterator_const_char_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_10(
    x: *const random_access_iterator_const_char_ptr_,
    y: *const random_access_iterator_const_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn operator_le_11(
    x: *const random_access_iterator_const_char_ptr_,
    y: *const random_access_iterator_const_char_ptr_,
) -> () {
    return (((*x).it_) <= ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct random_access_iterator_const_char_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_char_ptr_ {
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
    pub unsafe fn operator_index(&self, mut n: ()) -> () {
        return &(*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_char_ptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_const_char_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_const_char_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_const_char_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_char_ptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_const_char_ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                self as *const random_access_iterator_const_char_ptr_,
                other as *const random_access_iterator_const_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_char_ptr_ {}
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
pub unsafe fn operator_eq_12(
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
            operator_eq_12(
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
pub unsafe fn operator_add_13(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_14(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_13(_i, n)
    });
}
pub unsafe fn iter_swap_15(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_16(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*unimplemented!());
}
pub unsafe fn operator_eq_17(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_18(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_18(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn do_search_19(
    mut b1: random_access_iterator_const_int_ptr_,
    mut e1: random_access_iterator_const_int_ptr_,
    mut b2: random_access_iterator_const_int_ptr_,
    mut e2: random_access_iterator_const_int_ptr_,
    mut result: random_access_iterator_const_int_ptr_,
) {
    let mut s: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn do_search_20(
    mut b1: random_access_iterator_const_char_ptr_,
    mut e1: random_access_iterator_const_char_ptr_,
    mut b2: random_access_iterator_const_char_ptr_,
    mut e2: random_access_iterator_const_char_ptr_,
    mut result: random_access_iterator_const_char_ptr_,
) {
    let mut s: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_21() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize)))
                .cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize)))
                .cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3_()) as isize)))
                .cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3_()) as isize)))
                .cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize)))
                .cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize)))
                .cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ic.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ic.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    let mut id: [(); 2] = [1_(), 2_()];
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (id.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (id.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ie.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ie.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((4_()) as isize)).cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ig.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ig.as_mut_ptr().offset((4_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((8_()) as isize)).cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ih.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ih.as_mut_ptr().offset((sh) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ii.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ii.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ih.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ij.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ij.as_mut_ptr().offset((sj) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ik.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ik.as_mut_ptr().offset((sk) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ij.as_mut_ptr().offset((6_()) as isize)).cast_const()
            });
        do_search_19(_b1, _e1, _b2, _e2, _result)
    });
}
pub unsafe fn test2_22() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize)))
                .cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize)))
                .cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3_()) as isize)))
                .cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3_()) as isize)))
                .cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize)))
                .cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize)))
                .cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ic.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ic.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    let mut id: [(); 2] = [1_(), 2_()];
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (id.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (id.as_mut_ptr().offset((2_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((1_()) as isize)).cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ie.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ie.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((4_()) as isize)).cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ig.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ig.as_mut_ptr().offset((4_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((8_()) as isize)).cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ih.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ih.as_mut_ptr().offset((sh) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ii.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ii.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ih.as_mut_ptr().offset((3_()) as isize)).cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ij.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ij.as_mut_ptr().offset((sj) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ik.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ik.as_mut_ptr().offset((sk) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ij.as_mut_ptr().offset((6_()) as isize)).cast_const()
            });
        do_search_20(_b1, _e1, _b2, _e2, _result)
    });
}
pub unsafe fn test_custom_pred_23() {
    let mut long_string: () = unimplemented!();
    let mut searcher: () = unimplemented!();
    let str: [(); 5] = std::mem::transmute(*b"1234\0");
    let mut ret: () = unimplemented!();
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
    (unsafe { test_21() });
    (unsafe { test2_22() });
    (unsafe { test_custom_pred_23() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
