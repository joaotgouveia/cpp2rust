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
pub unsafe fn operator_eq_0(
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
            operator_eq_0(
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
pub unsafe fn operator_add_1(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_2(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_1(_i, n)
    });
}
pub unsafe fn iter_swap_3(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_4(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*unimplemented!())).cast::<()>();
}
pub unsafe fn operator_eq_5(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_6(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_6(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct BorrowedRange_int_ {}
const _: () = assert!(!(false), "!std::ranges::view<BorrowedRange<>>");
pub static mut enable_borrowed_range_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedRange<>>");
const _: () = assert!(true, "std::ranges::view<BorrowedView>");
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedView>");
const _: () = assert!(true, "std::ranges::view<NonBorrowedView>");
const _: () = assert!(!(false), "!std::ranges::borrowed_range<NonBorrowedView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct invocable_telemetry {
    pub invocations: (),
    pub moves: (),
    pub copies: (),
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct invocable_with_telemetry_std_plus_void__ {
    f_: (),
    telemetry_: *mut invocable_telemetry,
}
impl invocable_with_telemetry_std_plus_void__ {
    pub unsafe fn new(mut f: (), telemetry: *mut invocable_telemetry) -> Self {
        let mut this = Self {
            f_: unimplemented!(),
            telemetry_: (telemetry),
        };
        this
    }
    pub unsafe fn move_from(other: *mut invocable_with_telemetry_std_plus_void__) -> Self {
        let mut this = Self {
            f_: unimplemented!(),
            telemetry_: {
                unimplemented!();
                unimplemented!()
            },
        };
        (*this.telemetry_).moves.prefix_inc();
        this
    }
    pub unsafe fn copy_from(other: *const invocable_with_telemetry_std_plus_void__) -> Self {
        let mut this = Self {
            f_: unimplemented!(),
            telemetry_: {
                unimplemented!();
                (*other).telemetry_
            },
        };
        (*this.telemetry_).copies.prefix_inc();
        this
    }
    pub unsafe fn operator_call_6_int_int_ref(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_int_ref_int(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_double_double(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
}
impl Clone for invocable_with_telemetry_std_plus_void__ {
    fn clone(&self) -> Self {
        unsafe {
            invocable_with_telemetry_std_plus_void__::copy_from(
                self as *const invocable_with_telemetry_std_plus_void__,
            )
        }
    }
}
impl Default for invocable_with_telemetry_std_plus_void__ {
    fn default() -> Self {
        unsafe {
            invocable_with_telemetry_std_plus_void__ {
                f_: unimplemented!(),
                telemetry_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct invocable_with_telemetry_std_multiplies_void__ {
    f_: (),
    telemetry_: *mut invocable_telemetry,
}
impl invocable_with_telemetry_std_multiplies_void__ {
    pub unsafe fn new(mut f: (), telemetry: *mut invocable_telemetry) -> Self {
        let mut this = Self {
            f_: unimplemented!(),
            telemetry_: (telemetry),
        };
        this
    }
    pub unsafe fn move_from(other: *mut invocable_with_telemetry_std_multiplies_void__) -> Self {
        let mut this = Self {
            f_: unimplemented!(),
            telemetry_: {
                unimplemented!();
                unimplemented!()
            },
        };
        (*this.telemetry_).moves.prefix_inc();
        this
    }
    pub unsafe fn copy_from(other: *const invocable_with_telemetry_std_multiplies_void__) -> Self {
        let mut this = Self {
            f_: unimplemented!(),
            telemetry_: {
                unimplemented!();
                (*other).telemetry_
            },
        };
        (*this.telemetry_).copies.prefix_inc();
        this
    }
    pub unsafe fn operator_call_6_int_int_ref(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_int_ref_int(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
}
impl Clone for invocable_with_telemetry_std_multiplies_void__ {
    fn clone(&self) -> Self {
        unsafe {
            invocable_with_telemetry_std_multiplies_void__::copy_from(
                self as *const invocable_with_telemetry_std_multiplies_void__,
            )
        }
    }
}
impl Default for invocable_with_telemetry_std_multiplies_void__ {
    fn default() -> Self {
        unsafe {
            invocable_with_telemetry_std_multiplies_void__ {
                f_: unimplemented!(),
                telemetry_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ {
    f_: FnPtr<Generic>,
    telemetry_: *mut invocable_telemetry,
}
impl invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ {
    pub unsafe fn new(mut f: FnPtr<Generic>, telemetry: *mut invocable_telemetry) -> Self {
        let mut this = Self {
            f_: f.copy_from(),
            telemetry_: (telemetry),
        };
        this
    }
    pub unsafe fn move_from(
        other: *mut invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__,
    ) -> Self {
        let mut this = Self {
            f_: (*unimplemented!()).move_from(),
            telemetry_: {
                unimplemented!();
                unimplemented!()
            },
        };
        (*this.telemetry_).moves.prefix_inc();
        this
    }
    pub unsafe fn copy_from(
        other: *const invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__,
    ) -> Self {
        let mut this = Self {
            f_: (*other).f_.copy_from(),
            telemetry_: {
                unimplemented!();
                (*other).telemetry_
            },
        };
        (*this.telemetry_).copies.prefix_inc();
        this
    }
    pub unsafe fn operator_call_6_int_int_ref(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_int_ref_unsigned_long(
        &mut self,
        args_0: *mut (),
        args_1: *mut (),
    ) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_int_ref_int(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
}
impl Clone for invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ {
    fn clone(&self) -> Self {
        unsafe {
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__::copy_from(self as *const invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__)
        }
    }
}
impl Default for invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ {
    fn default() -> Self {
        unsafe {
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ {
                f_: <FnPtr<Generic>>::default(),
                telemetry_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ {
    f_: FnPtr<Generic>,
    telemetry_: *mut invocable_telemetry,
}
impl invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ {
    pub unsafe fn new(mut f: FnPtr<Generic>, telemetry: *mut invocable_telemetry) -> Self {
        let mut this = Self {
            f_: f.copy_from(),
            telemetry_: (telemetry),
        };
        this
    }
    pub unsafe fn move_from(
        other: *mut invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__,
    ) -> Self {
        let mut this = Self {
            f_: (*unimplemented!()).move_from(),
            telemetry_: {
                unimplemented!();
                unimplemented!()
            },
        };
        (*this.telemetry_).moves.prefix_inc();
        this
    }
    pub unsafe fn copy_from(
        other: *const invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__,
    ) -> Self {
        let mut this = Self {
            f_: (*other).f_.copy_from(),
            telemetry_: {
                unimplemented!();
                (*other).telemetry_
            },
        };
        (*this.telemetry_).copies.prefix_inc();
        this
    }
    pub unsafe fn operator_call_6_int_int_ref(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_int_ref_int(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
}
impl Clone for invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ {
    fn clone(&self) -> Self {
        unsafe {
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__::copy_from(self as *const invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__)
        }
    }
}
impl Default for invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ {
    fn default() -> Self {
        unsafe {
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ {
                f_: <FnPtr<Generic>>::default(),
                telemetry_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct invocable_with_telemetry_std_plus_long__ {
    f_: (),
    telemetry_: *mut invocable_telemetry,
}
impl invocable_with_telemetry_std_plus_long__ {
    pub unsafe fn new(mut f: (), telemetry: *mut invocable_telemetry) -> Self {
        let mut this = Self {
            f_: f.clone(),
            telemetry_: (telemetry),
        };
        this
    }
    pub unsafe fn move_from(other: *mut invocable_with_telemetry_std_plus_long__) -> Self {
        let mut this = Self {
            f_: (*unimplemented!()),
            telemetry_: {
                unimplemented!();
                unimplemented!()
            },
        };
        (*this.telemetry_).moves.prefix_inc();
        this
    }
    pub unsafe fn copy_from(other: *const invocable_with_telemetry_std_plus_long__) -> Self {
        let mut this = Self {
            f_: (*other).f_.clone(),
            telemetry_: {
                unimplemented!();
                (*other).telemetry_
            },
        };
        (*this.telemetry_).copies.prefix_inc();
        this
    }
    pub unsafe fn operator_call_6_int_int_ref(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_int_ref_long(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_int_ref_int(&mut self, args_0: *mut (), args_1: *mut ()) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_int_const_int_ref(
        &mut self,
        args_0: *mut (),
        args_1: *const (),
    ) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_const_int_ref_long(
        &mut self,
        args_0: *const (),
        args_1: *mut (),
    ) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_const_int_ref_int(
        &mut self,
        args_0: *const (),
        args_1: *mut (),
    ) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
}
impl Clone for invocable_with_telemetry_std_plus_long__ {
    fn clone(&self) -> Self {
        unsafe {
            invocable_with_telemetry_std_plus_long__::copy_from(
                self as *const invocable_with_telemetry_std_plus_long__,
            )
        }
    }
}
impl Default for invocable_with_telemetry_std_plus_long__ {
    fn default() -> Self {
        unsafe {
            invocable_with_telemetry_std_plus_long__ {
                f_: unimplemented!(),
                telemetry_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ {
    f_: FnPtr<fn((), ()) -> ()>,
    telemetry_: *mut invocable_telemetry,
}
impl invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ {
    pub unsafe fn new(mut f: FnPtr<fn((), ()) -> ()>, telemetry: *mut invocable_telemetry) -> Self {
        let mut this = Self {
            f_: f.copy_from(),
            telemetry_: (telemetry),
        };
        this
    }
    pub unsafe fn move_from(
        other: *mut invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__,
    ) -> Self {
        let mut this = Self {
            f_: (*unimplemented!()).move_from(),
            telemetry_: {
                unimplemented!();
                unimplemented!()
            },
        };
        (*this.telemetry_).moves.prefix_inc();
        this
    }
    pub unsafe fn copy_from(
        other: *const invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__,
    ) -> Self {
        let mut this = Self {
            f_: (*other).f_.copy_from(),
            telemetry_: {
                unimplemented!();
                (*other).telemetry_
            },
        };
        (*this.telemetry_).copies.prefix_inc();
        this
    }
    pub unsafe fn operator_call_6_double_double_ref(
        &mut self,
        args_0: *mut (),
        args_1: *mut (),
    ) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn operator_call_6_double_ref_double(
        &mut self,
        args_0: *mut (),
        args_1: *mut (),
    ) -> () {
        unimplemented!();
        (*self.telemetry_).invocations.prefix_inc();
        return unimplemented!();
    }
}
impl Clone for invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ {
    fn clone(&self) -> Self {
        unsafe {
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__::copy_from(self as *const invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__)
        }
    }
}
impl Default for invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ {
    fn default() -> Self {
        unsafe {
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ {
                f_: FnPtr::<fn((), ()) -> ()>::new(|x: (), y: ()| -> () {
                    unsafe {
                        return ((x) + (y));
                    }
                }),
                telemetry_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn triangular_sum_8(input: *const ()) -> () {
    unimplemented!();
    let mut __decomp_9: () = (unsafe {
        let ___r: *const () = input;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut minmax_element_10))(___r, None, None)
    });
    return (((unsafe {
        let ___r: *const () = input;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut distance_11))(___r)
    }) / (2_()))
        * ((*unimplemented!()) + (*unimplemented!())));
}
pub unsafe fn triangular_sum_12(input: *mut ()) -> () {
    unimplemented!();
    let mut __decomp_13: () = (unsafe {
        let ___r: *mut () = input;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut minmax_element_10))(___r, None, None)
    });
    return ((((unsafe {
        let ___r: *mut () = input;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut distance_11))(___r)
    }) / (2_()))
        * ((unimplemented!()) + (unimplemented!()))) as ());
}
pub unsafe fn triangular_sum_14(input: *const ()) -> () {
    unimplemented!();
    let mut __decomp_15: () = (unsafe {
        let ___r: *const () = input;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut minmax_element_10))(___r, None, None)
    });
    return (((unsafe {
        let ___r: *const () = input;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut distance_11))(___r)
    }) / (2_()))
        * ((*unimplemented!()) + (*unimplemented!())));
}
pub unsafe fn triangular_sum_16(input: *const ()) -> () {
    unimplemented!();
    let mut __decomp_17: () = (unsafe {
        let ___r: *const () = input;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut minmax_element_10))(___r, None, None)
    });
    return (((unsafe {
        let ___r: *const () = input;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut distance_11))(___r)
    }) / (2_()))
        * ((*unimplemented!()) + (*unimplemented!())));
}
pub unsafe fn factorial_18(n: ()) -> () {
    unimplemented!();
    let mut result: () = 1_();
    let mut i: () = 1_();
    'loop_: while ((i) <= (n)) {
        result *= i;
        i.prefix_inc();
    }
    return result;
}
pub unsafe fn factorial_19(n: ()) -> () {
    unimplemented!();
    let mut result: () = 1_();
    let mut i: () = 1_();
    'loop_: while ((i) <= (n)) {
        result = (result).wrapping_mul(i);
        i.prefix_inc();
    }
    return result;
}
const _: () = assert!(true, "factorial(0) == 1");
const _: () = assert!(true, "factorial(1) == 1");
const _: () = assert!(true, "factorial(2) == 2");
const _: () = assert!(true, "factorial(3) == 6");
const _: () = assert!(true, "factorial(4) == 24");
const _: () = assert!(true, "factorial(5) == 120");
pub unsafe fn fibonacci_20(n: ()) -> () {
    unimplemented!();
    let mut result: () = 0_();
    let mut prev: () = 1_();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        result += unimplemented!();
        i.prefix_inc();
    }
    return result;
}
const _: () = assert!(true, "fibonacci(0) == 0");
const _: () = assert!(true, "fibonacci(1) == 1");
const _: () = assert!(true, "fibonacci(2) == 1");
const _: () = assert!(true, "fibonacci(3) == 2");
const _: () = assert!(true, "fibonacci(4) == 3");
const _: () = assert!(true, "fibonacci(5) == 5");
const _: () = assert!(true, "fibonacci(6) == 8");
const _: () = assert!(true, "fibonacci(7) == 13");
const _: () = assert!(true, "fibonacci(8) == 21");
const _: () = assert!(true, "fibonacci(9) == 34");
pub unsafe fn check_iterator_21(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_void__ =
            invocable_with_telemetry_std_plus_void__::new({ unimplemented!() }, { &mut telemetry });
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                invocable_with_telemetry_std_plus_void__::copy_from({ &f2 }),
            )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_iterator_23(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_multiplies_void__ =
            invocable_with_telemetry_std_multiplies_void__::new({ unimplemented!() }, {
                &mut telemetry
            });
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                invocable_with_telemetry_std_multiplies_void__::copy_from({ &f2 }),
            )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_iterator_24(r: *mut (), mut f: FnPtr<Generic>, expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                f.copy_from(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ =
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__::new(
                { f.copy_from() },
                { &mut telemetry },
            );
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22)) ( ___first , ___last , invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ :: copy_from ( { & f2 } , )  , )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_iterator_25(r: *mut (), mut f: FnPtr<Generic>, expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                f.copy_from(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ =
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__::new(
                { f.copy_from() },
                { &mut telemetry },
            );
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22)) ( ___first , ___last , invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ :: copy_from ( { & f2 } , )  , )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_iterator_26(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_void__ =
            invocable_with_telemetry_std_plus_void__::new({ unimplemented!() }, { &mut telemetry });
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                invocable_with_telemetry_std_plus_void__::copy_from({ &f2 }),
            )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_iterator_27(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                f.clone(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_long__ =
            invocable_with_telemetry_std_plus_long__::new({ f.clone() }, { &mut telemetry });
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                invocable_with_telemetry_std_plus_long__::copy_from({ &f2 }),
            )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_iterator_28(r: *mut (), mut f: FnPtr<fn((), ()) -> ()>, expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                f.copy_from(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ =
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__::new(
                { f.copy_from() },
                { &mut telemetry },
            );
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22)) ( ___first , ___last , invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ :: copy_from ( { & f2 } , )  , )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_iterator_29(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                f.clone(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_long__ =
            invocable_with_telemetry_std_plus_long__::new({ f.clone() }, { &mut telemetry });
        let mut result: () = (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                ___first,
                ___last,
                invocable_with_telemetry_std_plus_long__::copy_from({ &f2 }),
            )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_lvalue_range_30(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_void__ =
            invocable_with_telemetry_std_plus_void__::new({ unimplemented!() }, { &mut telemetry });
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: invocable_with_telemetry_std_plus_void__ =
                invocable_with_telemetry_std_plus_void__::copy_from({ &f2 });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_lvalue_range_31(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_multiplies_void__ =
            invocable_with_telemetry_std_multiplies_void__::new({ unimplemented!() }, {
                &mut telemetry
            });
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: invocable_with_telemetry_std_multiplies_void__ =
                invocable_with_telemetry_std_multiplies_void__::copy_from({ &f2 });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_lvalue_range_32(r: *mut (), mut f: FnPtr<Generic>, expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: FnPtr<Generic> = f.copy_from();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ =
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__::new(
                { f.copy_from() },
                { &mut telemetry },
            );
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__  = invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ :: copy_from ( { & f2 } , )  ;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_lvalue_range_33(r: *mut (), mut f: FnPtr<Generic>, expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: FnPtr<Generic> = f.copy_from();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ =
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__::new(
                { f.copy_from() },
                { &mut telemetry },
            );
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__  = invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ :: copy_from ( { & f2 } , )  ;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_lvalue_range_34(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_void__ =
            invocable_with_telemetry_std_plus_void__::new({ unimplemented!() }, { &mut telemetry });
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: invocable_with_telemetry_std_plus_void__ =
                invocable_with_telemetry_std_plus_void__::copy_from({ &f2 });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_lvalue_range_35(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: () = f.clone();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_long__ =
            invocable_with_telemetry_std_plus_long__::new({ f.clone() }, { &mut telemetry });
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: invocable_with_telemetry_std_plus_long__ =
                invocable_with_telemetry_std_plus_long__::copy_from({ &f2 });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_lvalue_range_36(
    r: *mut (),
    mut f: FnPtr<fn((), ()) -> ()>,
    expected: *const (),
) {
    {
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: FnPtr<fn((), ()) -> ()> = f.copy_from();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ =
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__::new(
                { f.copy_from() },
                { &mut telemetry },
            );
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__  = invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ :: copy_from ( { & f2 } , )  ;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_lvalue_range_37(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: () = f.clone();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_long__ =
            invocable_with_telemetry_std_plus_long__::new({ f.clone() }, { &mut telemetry });
        let mut result: () = (unsafe {
            let ___range: *mut () = r;
            let ___func: invocable_with_telemetry_std_plus_long__ =
                invocable_with_telemetry_std_plus_long__::copy_from({ &f2 });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(___range, ___func)
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_rvalue_range_38(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_void__ =
            invocable_with_telemetry_std_plus_void__::new({ unimplemented!() }, { &mut telemetry });
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                invocable_with_telemetry_std_plus_void__::copy_from({ &f2 }),
            )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_rvalue_range_39(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_multiplies_void__ =
            invocable_with_telemetry_std_multiplies_void__::new({ unimplemented!() }, {
                &mut telemetry
            });
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                invocable_with_telemetry_std_multiplies_void__::copy_from({ &f2 }),
            )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_rvalue_range_40(r: *mut (), mut f: FnPtr<Generic>, expected: *const ()) {
    {
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                f.copy_from(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ =
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__::new(
                { f.copy_from() },
                { &mut telemetry },
            );
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22)) ( & mut *(&raw mut ( * unimplemented!()  )  ).cast::<()>() , invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_128_29__ :: copy_from ( { & f2 } , )  , )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_rvalue_range_41(r: *mut (), mut f: FnPtr<Generic>, expected: *const ()) {
    {
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                f.copy_from(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ =
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__::new(
                { f.copy_from() },
                { &mut telemetry },
            );
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22)) ( & mut *(&raw mut ( * unimplemented!()  )  ).cast::<()>() , invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_135_14__ :: copy_from ( { & f2 } , )  , )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_rvalue_range_42(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut r2: () = (*r).clone();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                unimplemented!(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_void__ =
            invocable_with_telemetry_std_plus_void__::new({ unimplemented!() }, { &mut telemetry });
        let mut r2: () = (*r).clone();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                invocable_with_telemetry_std_plus_void__::copy_from({ &f2 }),
            )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_rvalue_range_43(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                f.clone(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_long__ =
            invocable_with_telemetry_std_plus_long__::new({ f.clone() }, { &mut telemetry });
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                invocable_with_telemetry_std_plus_long__::copy_from({ &f2 }),
            )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_rvalue_range_44(
    r: *mut (),
    mut f: FnPtr<fn((), ()) -> ()>,
    expected: *const (),
) {
    {
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                f.copy_from(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ =
            invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__::new(
                { f.copy_from() },
                { &mut telemetry },
            );
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22)) ( & mut *(&raw mut ( * unimplemented!()  )  ).cast::<()>() , invocable_with_telemetry__lambda_at_ranges_fold_right_last_pass_cpp_183_27__ :: copy_from ( { & f2 } , )  , )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_rvalue_range_45(r: *mut (), mut f: (), expected: *const ()) {
    {
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                f.clone(),
            )
        });
        unimplemented!();
    }
    {
        let mut telemetry: invocable_telemetry = <invocable_telemetry>::default();
        let mut f2: invocable_with_telemetry_std_plus_long__ =
            invocable_with_telemetry_std_plus_long__::new({ f.clone() }, { &mut telemetry });
        let mut r2: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut fold_right_last_22))(
                &mut *(&raw mut (*unimplemented!())).cast::<()>(),
                invocable_with_telemetry_std_plus_long__::copy_from({ &f2 }),
            )
        });
        unimplemented!();
        if unimplemented!() {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn check_46(mut r: (), mut f: (), expected: *const ()) {
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = unimplemented!();
        let _expected: *const () = expected;
        check_iterator_21(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = unimplemented!();
        let _expected: *const () = expected;
        check_lvalue_range_30(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = unimplemented!();
        let _expected: *const () = expected;
        check_rvalue_range_38(_r, _f, _expected)
    });
}
pub unsafe fn check_47(mut r: (), mut f: (), expected: *const ()) {
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = unimplemented!();
        let _expected: *const () = expected;
        check_iterator_23(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = unimplemented!();
        let _expected: *const () = expected;
        check_lvalue_range_31(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = unimplemented!();
        let _expected: *const () = expected;
        check_rvalue_range_39(_r, _f, _expected)
    });
}
pub unsafe fn check_48(mut r: (), mut f: FnPtr<Generic>, expected: *const ()) {
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: FnPtr<Generic> = f.copy_from();
        let _expected: *const () = expected;
        check_iterator_24(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: FnPtr<Generic> = f.copy_from();
        let _expected: *const () = expected;
        check_lvalue_range_32(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: FnPtr<Generic> = f.copy_from();
        let _expected: *const () = expected;
        check_rvalue_range_40(_r, _f, _expected)
    });
}
pub unsafe fn check_49(mut r: (), mut f: FnPtr<Generic>, expected: *const ()) {
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: FnPtr<Generic> = f.copy_from();
        let _expected: *const () = expected;
        check_iterator_25(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: FnPtr<Generic> = f.copy_from();
        let _expected: *const () = expected;
        check_lvalue_range_33(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: FnPtr<Generic> = f.copy_from();
        let _expected: *const () = expected;
        check_rvalue_range_41(_r, _f, _expected)
    });
}
pub unsafe fn check_50(mut r: (), mut f: (), expected: *const ()) {
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = unimplemented!();
        let _expected: *const () = expected;
        check_iterator_26(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = unimplemented!();
        let _expected: *const () = expected;
        check_lvalue_range_34(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = unimplemented!();
        let _expected: *const () = expected;
        check_rvalue_range_42(_r, _f, _expected)
    });
}
pub unsafe fn check_51(mut r: (), mut f: (), expected: *const ()) {
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = f.clone();
        let _expected: *const () = expected;
        check_iterator_27(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = f.clone();
        let _expected: *const () = expected;
        check_lvalue_range_35(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = f.clone();
        let _expected: *const () = expected;
        check_rvalue_range_43(_r, _f, _expected)
    });
}
pub unsafe fn check_52(mut r: (), mut f: FnPtr<fn((), ()) -> ()>, expected: *const ()) {
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: FnPtr<fn((), ()) -> ()> = f.copy_from();
        let _expected: *const () = expected;
        check_iterator_28(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: FnPtr<fn((), ()) -> ()> = f.copy_from();
        let _expected: *const () = expected;
        check_lvalue_range_36(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: FnPtr<fn((), ()) -> ()> = f.copy_from();
        let _expected: *const () = expected;
        check_rvalue_range_44(_r, _f, _expected)
    });
}
pub unsafe fn check_53(mut r: (), mut f: (), expected: *const ()) {
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = f.clone();
        let _expected: *const () = expected;
        check_iterator_29(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = f.clone();
        let _expected: *const () = expected;
        check_lvalue_range_37(_r, _f, _expected)
    });
    (unsafe {
        let _r: *mut () = &mut r;
        let _f: () = f.clone();
        let _expected: *const () = expected;
        check_rvalue_range_45(_r, _f, _expected)
    });
}
pub unsafe fn empty_range_test_case_54() {
    let data: () = unimplemented!();
    (unsafe {
        let mut _expected: () = unimplemented!();
        check_46(unimplemented!(), unimplemented!(), &mut _expected)
    });
    (unsafe {
        let mut _expected: () = unimplemented!();
        check_47(unimplemented!(), unimplemented!(), &mut _expected)
    });
}
pub unsafe fn common_range_test_case_55() {
    let data: () = unimplemented!();
    (unsafe {
        let _r: () = unimplemented!();
        let mut _expected: () = unimplemented!();
        check_46(_r, unimplemented!(), &mut _expected)
    });
    (unsafe {
        let _r: () = unimplemented!();
        let mut _expected: () = unimplemented!();
        check_47(_r, unimplemented!(), &mut _expected)
    });
    let mut multiply_with_next: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let n: () = 1_();
        },
        |x: (), y: ()| -> () {
            let result: () = (((x) * (y)) * (n));
            n = x;
            return result;
        },
        |x: (), y: ()| -> () {
            let result: () = ((x).wrapping_mul(y)).wrapping_mul(n);
            n = x;
            return (result as ());
        }
    );
    (unsafe {
        let _r: () = unimplemented!();
        let mut _expected: () = unimplemented!();
        check_48(_r, multiply_with_next.copy_from(), &mut _expected)
    });
    let mut fib: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let n: () = 1_();
        },
        |mut x: (), mut _a1: ()| -> () {
            let mut old_x: () = x;
            x += n;
            n = old_x;
            return x;
        }
    );
    (unsafe {
        let _r: () = unimplemented!();
        let mut _expected: () = unimplemented!();
        check_49(_r, fib.copy_from(), &mut _expected)
    });
}
pub unsafe fn non_common_range_test_case_56() {
    let mut parse: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|s: ()| -> () {
        unsafe {
            return if unimplemented!() {
                0_f64
            } else {
                if unimplemented!() {
                    1_f64
                } else {
                    if unimplemented!() {
                        2_f64
                    } else {
                        if unimplemented!() {
                            3_f64
                        } else {
                            if unimplemented!() {
                                4_f64
                            } else {
                                if unimplemented!() {
                                    5_f64
                                } else {
                                    if unimplemented!() {
                                        6_f64
                                    } else {
                                        if unimplemented!() {
                                            7_f64
                                        } else {
                                            if unimplemented!() {
                                                8_f64
                                            } else {
                                                if unimplemented!() {
                                                    9_f64
                                                } else {
                                                    {
                                                        unimplemented!();
                                                        10_f64
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
        }
    });
    {
        let mut data: () = unimplemented!();
        let mut range : std_ranges_transform_view_std_ranges_ref_view_std_vector_std_string__std_allocator_std_string______lambda_at_ranges_fold_right_last_pass_cpp_145_16__ = unimplemented!()  ;
        (unsafe {
            let _r: () = range.clone();
            let mut _expected: () = unimplemented!();
            check_50(_r, unimplemented!(), &mut _expected)
        });
    }
}
pub unsafe fn test_case_57() -> () {
    (unsafe { empty_range_test_case_54() });
    (unsafe { common_range_test_case_55() });
    (unsafe { non_common_range_test_case_56() });
    return true;
}
pub unsafe fn runtime_only_test_case_58() {
    {
        let data: () = unimplemented!();
        let expected: () = (unsafe { triangular_sum_14(&data) });
        (unsafe {
            let mut _expected: () = unimplemented!();
            check_51(unimplemented!(), unimplemented!(), &mut _expected)
        });
    }
    {
        let data: () = unimplemented!();
        let mut plus: FnPtr<fn((), ()) -> ()> =
            FnPtr::<fn((), ()) -> ()>::new(|x: (), y: ()| -> () {
                unsafe {
                    return ((x) + (y));
                }
            });
        let expected: () = -21.1_f64;
        (unsafe {
            let mut _expected: () = unimplemented!();
            check_52(unimplemented!(), plus.copy_from(), &mut _expected)
        });
    }
    {
        let data: () = unimplemented!();
        let expected: () = (unsafe { triangular_sum_16(&data) });
        (unsafe {
            let mut _expected: () = unimplemented!();
            check_53(unimplemented!(), unimplemented!(), &mut _expected)
        });
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
    (unsafe { test_case_57() });
    const _: () = assert!(true, "test_case()");;
    (unsafe { runtime_only_test_case_58() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_7);
}
