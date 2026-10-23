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
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_char__signed_char_ {
    pub unsafe fn new(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_char__signed_char_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    data_: (),
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n)
            > (unsafe {
                sized_allocator_unsigned_char__unsigned_char__signed_char_::max_size(self)
            }))
        {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_short__short_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_short__short_ {
    pub unsafe fn new(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_short__unsigned_short__short_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_short__short_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_short__unsigned_short__short_ {
    data_: (),
}
impl sized_allocator_unsigned_short__unsigned_short__short_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n)
            > (unsafe { sized_allocator_unsigned_short__unsigned_short__short_::max_size(self) }))
        {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_int__int_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_int__int_ {
    pub unsafe fn new(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_int__unsigned_int__int_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_int__int_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_int__unsigned_int__int_ {
    data_: (),
}
impl sized_allocator_unsigned_int__unsigned_int__int_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n) > (unsafe { sized_allocator_unsigned_int__unsigned_int__int_::max_size(self) })) {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_long__long_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_long__long_ {
    pub unsafe fn new(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_long__long_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_long__unsigned_long__long_ {
    data_: (),
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n) > (unsafe { sized_allocator_unsigned_long__unsigned_long__long_::max_size(self) }))
        {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
pub struct types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__
{}
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
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr____types_type_list_bidirectional_iterator_const_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr____types_type_list_forward_iterator_const_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr____types_type_list_cpp17_input_iterator_const_int_ptr__const_int_ptr___
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__,
    mut f: Test,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { Test::operator_call_1_const_int_ptr(&mut f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { Test::operator_call_1_class_contiguous_iterator_const_int_ptr_(&mut f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { Test::operator_call_1_class_random_access_iterator_const_int_ptr_(&mut f) });
            0_()
        };
        let _arg3: () = {
            (unsafe { Test::operator_call_1_class_bidirectional_iterator_const_int_ptr_(&mut f) });
            0_()
        };
        let _arg4: () = {
            (unsafe { Test::operator_call_1_class_forward_iterator_const_int_ptr_(&mut f) });
            0_()
        };
        let _arg5: () = {
            (unsafe { Test::operator_call_1_class_cpp17_input_iterator_const_int_ptr_(&mut f) });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn swallow_1(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
) {
}
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
pub unsafe fn operator_ne_2(
    x: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    y: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_ne_3(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct forward_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_int_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_int_ptr_));
    }
}
pub unsafe fn operator_ne_4(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bidirectional_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_const_int_ptr_));
    }
}
pub unsafe fn operator_ne_5(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
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
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
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
pub unsafe fn operator_ne_6(
    x: *const contiguous_iterator_const_int_ptr_,
    y: *const contiguous_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct contiguous_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_int_ptr_ {
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_const_int_ptr_));
    }
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
pub unsafe fn operator_eq_7(
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
            operator_eq_7(
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
pub unsafe fn operator_add_8(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_9(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_8(_i, n)
    });
}
pub unsafe fn iter_swap_10(mut a: adl_Iterator, mut b: adl_Iterator) {
    (unsafe {
        let ___x: *mut *mut () = &mut a.ptr_;
        let ___y: *mut *mut () = &mut b.ptr_;
        swap_11(___x, ___y)
    });
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_12(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
}
pub unsafe fn operator_eq_13(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_14(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_14(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_13(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub struct Test {}
impl Test {
    pub unsafe fn operator_call_1_const_int_ptr(&mut self) {
        let mut ia: [(); 8] = [0_(), 1_(), 2_(), 2_(), 0_(), 1_(), 2_(), 3_()];
        let sa: () = (::std::mem::size_of::<[(); 8]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&mut self) {
        let mut ia: [(); 8] = [0_(), 1_(), 2_(), 2_(), 0_(), 1_(), 2_(), 3_()];
        let sa: () = (::std::mem::size_of::<[(); 8]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_int_ptr_(&mut self) {
        let mut ia: [(); 8] = [0_(), 1_(), 2_(), 2_(), 0_(), 1_(), 2_(), 3_()];
        let sa: () = (::std::mem::size_of::<[(); 8]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&mut self) {
        let mut ia: [(); 8] = [0_(), 1_(), 2_(), 2_(), 0_(), 1_(), 2_(), 3_()];
        let sa: () = (::std::mem::size_of::<[(); 8]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_int_ptr_(&mut self) {
        let mut ia: [(); 8] = [0_(), 1_(), 2_(), 2_(), 0_(), 1_(), 2_(), 3_()];
        let sa: () = (::std::mem::size_of::<[(); 8]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_int_ptr_(&mut self) {
        let mut ia: [(); 8] = [0_(), 1_(), 2_(), 2_(), 0_(), 1_(), 2_(), 3_()];
        let sa: () = (::std::mem::size_of::<[(); 8]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_15() -> () {
    (unsafe {
        for_each_0 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__ >::default() , <Test >::default() , )
    });
    {
        {
            let mut vec_: () = unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) != (256_())) {
                let mut offset: () = 0_();
                'loop_: while ((offset) != (64_())) {
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    offset.prefix_inc();
                }
                i.prefix_inc();
            }
        }
        {
            let mut in_: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            unimplemented!();
        }
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
    (unsafe { test_15() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
