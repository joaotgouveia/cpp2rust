extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Buffer_Counted__1_ {
    pub buffer: [(); 8],
}
impl Buffer_Counted__1_ {
    pub unsafe fn begin(&mut self) -> *mut Counted {
        return (self.buffer.as_mut_ptr() as *mut Counted);
    }
}
impl Default for Buffer_Counted__1_ {
    fn default() -> Self {
        unsafe {
            Buffer_Counted__1_ {
                buffer: [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Buffer_Counted__5_ {
    pub buffer: [(); 40],
}
impl Buffer_Counted__5_ {
    pub unsafe fn begin(&mut self) -> *mut Counted {
        return (self.buffer.as_mut_ptr() as *mut Counted);
    }
    pub unsafe fn end(&mut self) -> *mut Counted {
        return (unsafe { Buffer_Counted__5_::begin(self) }).offset((5_()) as isize);
    }
}
impl Default for Buffer_Counted__5_ {
    fn default() -> Self {
        unsafe {
            Buffer_Counted__5_ {
                buffer: [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ],
            }
        }
    }
}
pub unsafe fn operator_eq_5(l: *const Counted, r: *const Counted) -> () {
    return (((*l).value) == ((*r).value));
}
pub unsafe fn operator_ne_6(l: *const Counted, r: *const Counted) -> () {
    return !(unsafe {
        let _l: *const Counted = l;
        let _r: *const Counted = r;
        operator_eq_5(_l, _r)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct Counted {
    pub value: (),
    pub moved_from: (),
}
impl Counted {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            value: unimplemented!(),
            moved_from: false,
        };
        (unsafe { Counted::check_throw(&mut this) });
        (unsafe { Counted::increase_counters(&mut this) });
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Self {
            value: v,
            moved_from: false,
        };
        (unsafe { Counted::check_throw(&mut this) });
        (unsafe { Counted::increase_counters(&mut this) });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut current_objects_0)).prefix_dec();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut current_objects_0)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut total_objects_1)) = 0_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut total_objects_1))
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_copies_2)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut total_moves_3)) = 0_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut total_moves_3))
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on_4)) = -1_();
    }
    pub unsafe fn copy_from(rhs: *const Counted) -> Self {
        let mut this = Self {
            value: (*rhs).value,
            moved_from: false,
        };
        (unsafe { Counted::check_throw(&mut this) });
        (unsafe { Counted::increase_counters(&mut this) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_copies_2)).prefix_inc();
        this
    }
    pub unsafe fn move_from(rhs: *mut Counted) -> Self {
        let mut this = Self {
            value: (*rhs).value,
            moved_from: false,
        };
        (unsafe { Counted::check_throw(&mut this) });
        (unsafe { Counted::increase_counters(&mut this) });
        (*rhs).moved_from = true;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_moves_3)).prefix_inc();
        this
    }
    unsafe fn check_throw(&mut self) {
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on_4))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut total_objects_1)))
        {
            unimplemented!();
        }
    }
    unsafe fn increase_counters(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut current_objects_0)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_objects_1)).prefix_inc();
    }
}
impl std::cmp::PartialEq for Counted {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self as *const Counted, other as *const Counted) }
    }
}
impl std::cmp::Eq for Counted {}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::copy_from(self as *const Counted) }
    }
}
impl Default for Counted {
    fn default() -> Self {
        unsafe { Counted::new_1() }
    }
}
pub static mut current_objects_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut total_objects_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut total_copies_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut total_moves_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut throw_on_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { -1_() });
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
pub unsafe fn operator_ne_7(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
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
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
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
pub unsafe fn operator_ne_8(
    x: *const forward_iterator_IntWrapper_ptr_,
    y: *const forward_iterator_IntWrapper_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct forward_iterator_IntWrapper_ptr_ {
    it_: *mut IntWrapper_9,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_IntWrapper_ptr_ {
    pub unsafe fn new_1(mut it: *mut IntWrapper_9) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_IntWrapper_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_IntWrapper_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_IntWrapper_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn operator_ne_10(
    x: *const forward_iterator_Counted_ptr_,
    y: *const forward_iterator_Counted_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_11(i: *const forward_iterator_Counted_ptr_) -> *mut Counted {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct forward_iterator_Counted_ptr_ {
    it_: *mut Counted,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_Counted_ptr_ {
    pub unsafe fn new_1(mut it: *mut Counted) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_Counted_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_Counted_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_Counted_ptr_) -> Self {
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
pub struct sentinel_wrapper_forward_iterator_Counted_ptr__ {
    base_: *mut Counted,
}
impl sentinel_wrapper_forward_iterator_Counted_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_Counted_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_11(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_Counted_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_11(other) }));
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
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
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
const _: () = assert!(
    true,
    "std::is_class_v<decltype(std::ranges::uninitialized_default_construct)>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotDefaultCtrable {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<decltype(std::ranges::uninitialized_default_construct),\n    NotDefaultCtrable*, NotDefaultCtrable*>"
);
pub unsafe fn test_19() -> () {
    {
        let n: () = 3_();
        let mut alloc: () = unimplemented!();
        let mut data: *mut () = unimplemented!();
        let mut last: *mut () = data.offset((n) as isize);
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                data, last,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) != (n)) {
            (*data.offset((i) as isize)) = -i;
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut destroy_21))(data, last) });
        unimplemented!();
    }
    {
        let n: () = 3_();
        let mut alloc: () = unimplemented!();
        let mut data: *mut () = unimplemented!();
        let mut r: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                &mut r,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) != (n)) {
            (*unimplemented!()) = -i;
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut destroy_21))(&mut r) });
        unimplemented!();
    }
    {
        let n: () = 3_();
        let mut alloc: () = unimplemented!();
        let mut data: *mut IntWrapper_9 = unimplemented!();
        let mut last: *mut IntWrapper_9 = data.offset((n) as isize);
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                data, last,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) != (n)) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut destroy_21))(data, last) });
        unimplemented!();
    }
    {
        let n: () = 3_();
        let mut alloc: () = unimplemented!();
        let mut data: *mut IntWrapper_9 = unimplemented!();
        let mut r: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                &mut r,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) != (n)) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut destroy_21))(&mut r) });
        unimplemented!();
    }
    {
        let n: () = 5_();
        let mut pool: [(); 5] = [-1_(), -1_(), -1_(), -1_(), -1_()];
        let mut p: *mut () = pool.as_mut_ptr();
        let mut pend: *mut () = p.offset((n) as isize);
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                forward_iterator_int_ptr_::new_1({ p }),
                forward_iterator_int_ptr_::new_1({ pend }),
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) != (n)) {
            pool[(i) as usize] = ((i) + (17_()));
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let n: () = 5_();
        let mut pool: [(); 5] = [-1_(), -1_(), -1_(), -1_(), -1_()];
        let mut p: *mut () = pool.as_mut_ptr();
        let mut pend: *mut () = p.offset((n) as isize);
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                &mut ___range,
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) != (n)) {
            pool[(i) as usize] = ((i) + (17_()));
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let n: () = 5_();
        let mut pool: [IntWrapper_9; 5] = [
            IntWrapper_9 { n_: -31_() },
            IntWrapper_9 { n_: -41_() },
            IntWrapper_9 { n_: -59_() },
            IntWrapper_9 { n_: -26_() },
            IntWrapper_9 { n_: -53_() },
        ];
        let mut p: *mut IntWrapper_9 = pool.as_mut_ptr();
        (unsafe {
            let ___first: forward_iterator_IntWrapper_ptr_ =
                forward_iterator_IntWrapper_ptr_::new_1({ p });
            let ___last: forward_iterator_IntWrapper_ptr_ =
                forward_iterator_IntWrapper_ptr_::new_1({ p.offset((1_()) as isize) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                ___first, ___last,
            )
        });
        unimplemented!();
        unimplemented!();
        (unsafe {
            let ___first: forward_iterator_IntWrapper_ptr_ =
                forward_iterator_IntWrapper_ptr_::new_1({ p.offset((1_()) as isize) });
            let ___last: forward_iterator_IntWrapper_ptr_ =
                forward_iterator_IntWrapper_ptr_::new_1({ p.offset((n) as isize) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                ___first, ___last,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let n: () = 5_();
        let mut pool: [IntWrapper_9; 5] = [
            IntWrapper_9 { n_: -31_() },
            IntWrapper_9 { n_: -41_() },
            IntWrapper_9 { n_: -59_() },
            IntWrapper_9 { n_: -26_() },
            IntWrapper_9 { n_: -53_() },
        ];
        let mut p: *mut IntWrapper_9 = pool.as_mut_ptr();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                &mut ___range,
            )
        });
        unimplemented!();
        unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                &mut ___range,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntWrapper_9 {
    pub n_: (),
}
impl Default for IntWrapper_9 {
    fn default() -> Self {
        unsafe { IntWrapper_9 { n_: 42_() } }
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
    {
        let mut buf: Buffer_Counted__1_ = <Buffer_Counted__1_>::default();
        (unsafe {
            let ___first: *mut Counted = (unsafe { Buffer_Counted__1_::begin(&mut buf) });
            let ___last: *mut Counted = (unsafe { Buffer_Counted__1_::begin(&mut buf) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                ___first, ___last,
            )
        });
        unimplemented!();
        unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                &mut ___range,
            )
        });
        unimplemented!();
        unimplemented!();
        let mut it: forward_iterator_Counted_ptr_ = forward_iterator_Counted_ptr_::new_1({
            (unsafe { Buffer_Counted__1_::begin(&mut buf) })
        });
        let mut range: () = unimplemented!();
        (unsafe {
            let ___first: forward_iterator_Counted_ptr_ = unimplemented!();
            let ___last: sentinel_wrapper_forward_iterator_Counted_ptr__ = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                ___first, ___last,
            )
        });
        unimplemented!();
        unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                &mut range,
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let N: () = 5_();
        let mut buf: Buffer_Counted__5_ = <Buffer_Counted__5_>::default();
        (unsafe {
            let ___first: *mut Counted = (unsafe { Buffer_Counted__5_::begin(&mut buf) });
            let ___last: *mut Counted = (unsafe { Buffer_Counted__5_::end(&mut buf) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                ___first, ___last,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { Counted::reset() });
    }
    {
        let N: () = 5_();
        let mut buf: Buffer_Counted__5_ = <Buffer_Counted__5_>::default();
        let mut range: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                &mut range,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { Counted::reset() });
    }
    {
        let N: () = 3_();
        let mut buf: Buffer_Counted__5_ = <Buffer_Counted__5_>::default();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                unimplemented!(),
                unimplemented!(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { Counted::reset() });
    }
    {
        let N: () = 3_();
        let mut buf: Buffer_Counted__5_ = <Buffer_Counted__5_>::default();
        (unsafe {
            let mut ___range: () = (unsafe {
                let mut ___it: *mut Counted = (unsafe { Buffer_Counted__5_::begin(&mut buf) });
                (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_22))(&mut ___it, &N)
            });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                &mut ___range,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { Counted::reset() });
    }
    {
        let N: () = 3_();
        let mut buf: Buffer_Counted__5_ = <Buffer_Counted__5_>::default();
        let mut range: () = unimplemented!();
        (unsafe {
            let mut ___range: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_default_construct_20))(
                &mut ___range,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { Counted::reset() });
    }
    (unsafe { test_19() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const current_objects_0);
    std::cell::LazyCell::force(&*&raw const total_objects_1);
    std::cell::LazyCell::force(&*&raw const total_copies_2);
    std::cell::LazyCell::force(&*&raw const total_moves_3);
    std::cell::LazyCell::force(&*&raw const throw_on_4);
}
