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
pub unsafe fn base_0(mut i: *mut ()) -> *mut () {
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
            base_: (unsafe { base_0((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_0((*other)) }));
    }
}
pub unsafe fn operator_eq_1(
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
            operator_eq_1(
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
pub unsafe fn operator_add_2(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_3(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_2(_i, n)
    });
}
pub unsafe fn iter_swap_4(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_5(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*unimplemented!())).cast::<()>();
}
pub unsafe fn operator_eq_6(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_7(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_7(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self as *const adl_Iterator, other as *const adl_Iterator) }
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
impl NonBorrowedRange {
    pub unsafe fn new(arr: *mut ()) -> Self {
        let mut this = Self {
            data_: unimplemented!(),
            size_: unimplemented!(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonBorrowedRange {
    pub data_: *mut (),
    pub size_: (),
}
impl NonBorrowedRange {
    pub unsafe fn begin(&self) -> *mut () {
        return self.data_;
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_int_ptr_ {
        return {
            let mut __tmp_0: *mut () = self.data_.offset((self.size_) as isize);
            sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_0 })
        };
    }
}
pub unsafe fn rand_gen_8() -> () {
    return unimplemented!();
}
pub unsafe fn test_all_9() -> () {
    let mut unary_pred: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
        unsafe {
            return ((i) > (0_()));
        }
    });
    let mut binary_pred: FnPtr<fn((), ()) -> ()> =
        FnPtr::<fn((), ()) -> ()>::new(|mut i: (), mut j: ()| -> () {
            unsafe {
                return ((i) < (j));
            }
        });
    let mut gen_: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
        unsafe {
            return 42_();
        }
    });
    let mut in_: () = unimplemented!();
    let mut in2: () = unimplemented!();
    let mut mid: *mut () = unimplemented!().offset((1_()) as isize);
    let mut output: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut out2: () = unimplemented!().offset((1_()) as isize);
    let mut x: () = 2_();
    let mut count: () = 1_();
    (unsafe {
        dangling_1st_10(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut find_11)),
            &mut in_,
            &mut x,
        )
    });
    (unsafe {
        dangling_1st_12(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut find_if_13)),
            &mut in_,
            &mut unary_pred,
        )
    });
    (unsafe {
        dangling_1st_14(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut find_if_not_15)),
            &mut in_,
            &mut unary_pred,
        )
    });
    (unsafe {
        dangling_1st_16(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut find_last_17)),
            &mut in_,
            &mut x,
        )
    });
    (unsafe {
        dangling_1st_18(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut find_last_if_19)),
            &mut in_,
            &mut unary_pred,
        )
    });
    (unsafe {
        dangling_1st_20(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut find_last_if_not_21)),
            &mut in_,
            &mut unary_pred,
        )
    });
    (unsafe {
        dangling_1st_22(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut find_first_of_23)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_1st_24(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut adjacent_find_25)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_27)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_2nd_28(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_27)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_both_29(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_27)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_1st_30(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut partition_point_31)),
            &mut in_,
            &mut unary_pred,
        )
    });
    (unsafe {
        dangling_1st_32(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut lower_bound_33)),
            &mut in_,
            &mut x,
        )
    });
    (unsafe {
        dangling_1st_34(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut upper_bound_35)),
            &mut in_,
            &mut x,
        )
    });
    (unsafe {
        dangling_1st_36(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut equal_range_37)),
            &mut in_,
            &mut x,
        )
    });
    (unsafe {
        dangling_1st_38(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_39)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_40(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut max_element_41)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_42(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut minmax_element_43)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_44(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut search_45)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_1st_46(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut search_n_47)),
            &mut in_,
            &mut count,
            &mut x,
        )
    });
    (unsafe {
        dangling_1st_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut find_end_49)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_1st_50(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut is_sorted_until_51)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_52(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut is_heap_until_53)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_54(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut for_each_55)),
            &mut in_,
            &mut unary_pred,
        )
    });
    (unsafe {
        dangling_1st_56(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut copy_57)),
            &mut in_,
            &mut out,
        )
    });
    (unsafe {
        let mut _args: *mut () = unimplemented!();
        dangling_1st_58(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut copy_backward_59)),
            &mut in_,
            &mut _args,
        )
    });
    (unsafe {
        dangling_1st_60(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut copy_if_61)),
            &mut in_,
            &mut out,
            &mut unary_pred,
        )
    });
    (unsafe {
        dangling_1st_62(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut move_63)),
            &mut in_,
            &mut out,
        )
    });
    (unsafe {
        let mut _args: *mut () = unimplemented!();
        dangling_1st_64(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut move_backward_65)),
            &mut in_,
            &mut _args,
        )
    });
    (unsafe {
        dangling_1st_66(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut fill_67)),
            &mut in_,
            &mut x,
        )
    });
    {
        let mut out_transform: () = unimplemented!();
        (unsafe {
            let mut _args_2: *mut () = unimplemented!();
            dangling_1st_68(
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut transform_69)),
                &mut in_,
                &mut _args_2,
                &mut unary_pred,
            )
        });
        (unsafe {
            let mut _args_3: *mut () = unimplemented!();
            dangling_1st_70(
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut transform_69)),
                &mut in_,
                &mut in2,
                &mut _args_3,
                &mut binary_pred,
            )
        });
        (unsafe {
            let mut _args_3: *mut () = unimplemented!();
            dangling_2nd_71(
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut transform_69)),
                &mut in_,
                &mut in2,
                &mut _args_3,
                &mut binary_pred,
            )
        });
        (unsafe {
            let mut _args_3: *mut () = unimplemented!();
            dangling_both_72(
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut transform_69)),
                &mut in_,
                &mut in2,
                &mut _args_3,
                &mut binary_pred,
            )
        });
    }
    (unsafe {
        dangling_1st_73(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut generate_74)),
            &mut in_,
            &mut gen_,
        )
    });
    (unsafe {
        let _args_2: *mut *mut () = &mut out;
        let _args_3: *mut () = &mut x;
        dangling_1st_75(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut remove_copy_76)),
            &mut in_,
            _args_2,
            _args_3,
        )
    });
    (unsafe {
        dangling_1st_77(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut remove_copy_if_78)),
            &mut in_,
            &mut out,
            &mut unary_pred,
        )
    });
    (unsafe {
        let _args_2: *mut () = &mut x;
        let _args_3: *mut () = &mut x;
        dangling_1st_79(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut replace_80)),
            &mut in_,
            _args_2,
            _args_3,
        )
    });
    (unsafe {
        let mut _args_2: () = unimplemented!();
        dangling_1st_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut replace_if_82)),
            &mut in_,
            &mut _args_2,
            &mut x,
        )
    });
    (unsafe {
        let _args_2: *mut *mut () = &mut out;
        let _args_3: *mut () = &mut x;
        let _args_4: *mut () = &mut x;
        dangling_1st_83(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut replace_copy_84)),
            &mut in_,
            _args_2,
            _args_3,
            _args_4,
        )
    });
    (unsafe {
        let _args_2: *mut *mut () = &mut out;
        let _args_4: *mut () = &mut x;
        dangling_1st_85(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut replace_copy_if_86)),
            &mut in_,
            _args_2,
            &mut unary_pred,
            _args_4,
        )
    });
    (unsafe {
        dangling_1st_87(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut swap_ranges_88)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_2nd_89(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut swap_ranges_88)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_both_90(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut swap_ranges_88)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_1st_91(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_copy_92)),
            &mut in_,
            &mut out,
        )
    });
    (unsafe {
        dangling_1st_93(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_copy_94)),
            &mut in_,
            &mut mid,
            &mut out,
        )
    });
    (unsafe {
        dangling_1st_95(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut unique_copy_96)),
            &mut in_,
            &mut out,
        )
    });
    (unsafe {
        dangling_1st_97(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut partition_copy_98)),
            &mut in_,
            &mut out,
            &mut out2,
            &mut unary_pred,
        )
    });
    (unsafe {
        dangling_1st_99(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_100)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_2nd_101(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_100)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_both_102(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_100)),
            &mut in_,
            &mut in2,
        )
    });
    (unsafe {
        dangling_1st_103(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut merge_104)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_2nd_105(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut merge_104)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_both_106(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut merge_104)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_1st_107(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_difference_108)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_1st_109(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_intersection_110)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_2nd_111(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_intersection_110)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_both_112(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_intersection_110)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_1st_113(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_symmetric_difference_114)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_2nd_115(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_symmetric_difference_114)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_both_116(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_symmetric_difference_114)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_1st_117(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_union_118)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_2nd_119(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_union_118)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_both_120(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut set_union_118)),
            &mut in_,
            &mut in2,
            &mut out,
        )
    });
    (unsafe {
        dangling_1st_121(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut remove_122)),
            &mut in_,
            &mut x,
        )
    });
    (unsafe {
        dangling_1st_123(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut remove_if_124)),
            &mut in_,
            &mut unary_pred,
        )
    });
    (unsafe {
        dangling_1st_125(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_126)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_127(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut rotate_128)),
            &mut in_,
            &mut mid,
        )
    });
    {
        (unsafe {
            let mut _args: () = (unsafe { rand_gen_8() });
            dangling_1st_129(
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut shuffle_130)),
                &mut in_,
                &mut _args,
            )
        });
    }
    (unsafe {
        dangling_1st_131(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut unique_132)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_133(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut partition_134)),
            &mut in_,
            &mut unary_pred,
        )
    });
    {
        (unsafe {
            dangling_1st_135(
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut stable_partition_136)),
                &mut in_,
                &mut unary_pred,
            )
        });
    }
    (unsafe {
        dangling_1st_137(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut sort_138)),
            &mut in_,
        )
    });
    {
        (unsafe {
            dangling_1st_139(
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut stable_sort_140)),
                &mut in_,
            )
        });
    }
    (unsafe {
        dangling_1st_141(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_142)),
            &mut in_,
            &mut mid,
        )
    });
    (unsafe {
        dangling_1st_143(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut nth_element_144)),
            &mut in_,
            &mut mid,
        )
    });
    {
        (unsafe {
            dangling_1st_145(
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut inplace_merge_146)),
                &mut in_,
                &mut mid,
            )
        });
    }
    (unsafe {
        dangling_1st_147(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut make_heap_148)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_149(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut push_heap_150)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_151(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut pop_heap_152)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_147(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut make_heap_148)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_153(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut sort_heap_154)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_155(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut prev_permutation_156)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_157(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut next_permutation_158)),
            &mut in_,
        )
    });
    (unsafe {
        dangling_1st_159(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut iota_160)),
            &mut in_,
            &mut x,
        )
    });
    (unsafe {
        dangling_1st_161(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut shift_left_162)),
            &mut in_,
            &mut x,
        )
    });
    (unsafe {
        dangling_1st_163(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut shift_right_164)),
            &mut in_,
            &mut x,
        )
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
    (unsafe { test_all_9() });
    const _: () = assert!(true, "test_all()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
