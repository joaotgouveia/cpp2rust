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
pub struct types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_std_vector_char__std_allocator_char____std_deque_char__std_allocator_char____std_list_char__std_allocator_char____std_forward_list_char__std_allocator_char___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_std_stack_double__std_deque_double__std_allocator_double_____std_queue_double__std_deque_double__std_allocator_double_____std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___
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
pub struct types_concatenate_types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int_____types_type_list_std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int_____types_type_list_std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int_____types_type_list_std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double_____types_type_list_std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double_____types_type_list_std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double_____types_type_list_std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int______types_type_list_std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int______types_type_list_std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int______types_type_list_std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int______types_type_list_std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double______types_type_list_std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double______types_type_list_std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double______types_type_list_std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double______types_type_list_std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub unsafe fn for_each_2(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub unsafe fn for_each_3(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub unsafe fn for_each_4(
    mut _a0 : types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub unsafe fn for_each_5(
    mut _a0 : types_type_list_std_vector_char__std_allocator_char____std_deque_char__std_allocator_char____std_list_char__std_allocator_char____std_forward_list_char__std_allocator_char___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub unsafe fn for_each_6(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        swallow_7(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7)
    });
}
pub unsafe fn for_each_8(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        swallow_7(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7)
    });
}
pub unsafe fn for_each_9(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        swallow_7(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7)
    });
}
pub unsafe fn for_each_10(
    mut _a0 : types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        swallow_7(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7)
    });
}
pub unsafe fn for_each_11(
    mut _a0 : types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            (unsafe { f.spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            (unsafe { f.spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            (unsafe { f.spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            (unsafe { f.spec::<fn()>(11).call() });
            0_()
        };
        swallow_12(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
        )
    });
}
pub unsafe fn for_each_13(
    mut _a0 : types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            (unsafe { f.spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            (unsafe { f.spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            (unsafe { f.spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            (unsafe { f.spec::<fn()>(11).call() });
            0_()
        };
        swallow_12(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
        )
    });
}
pub unsafe fn for_each_14(
    mut _a0 : types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            (unsafe { f.spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            (unsafe { f.spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            (unsafe { f.spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            (unsafe { f.spec::<fn()>(11).call() });
            0_()
        };
        swallow_12(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
        )
    });
}
pub unsafe fn for_each_15(
    mut _a0 : types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            (unsafe { f.spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            (unsafe { f.spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            (unsafe { f.spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            (unsafe { f.spec::<fn()>(11).call() });
            0_()
        };
        swallow_12(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
        )
    });
}
pub unsafe fn for_each_16(
    mut _a0 : types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        swallow_17(_arg0, _arg1, _arg2)
    });
}
pub unsafe fn for_each_18(
    mut _a0 : types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        swallow_17(_arg0, _arg1, _arg2)
    });
}
pub unsafe fn for_each_19(
    mut _a0 : types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        swallow_17(_arg0, _arg1, _arg2)
    });
}
pub unsafe fn for_each_20(
    mut _a0 : types_type_list_std_stack_double__std_deque_double__std_allocator_double_____std_queue_double__std_deque_double__std_allocator_double_____std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        swallow_17(_arg0, _arg1, _arg2)
    });
}
pub unsafe fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: ()) {}
pub unsafe fn swallow_7(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
    mut _a6: (),
    mut _a7: (),
) {
}
pub unsafe fn swallow_12(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
    mut _a6: (),
    mut _a7: (),
    mut _a8: (),
    mut _a9: (),
    mut _a10: (),
    mut _a11: (),
) {
}
pub unsafe fn swallow_17(mut _a0: (), mut _a1: (), mut _a2: ()) {}
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
pub unsafe fn operator_eq_21(
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
            operator_eq_21(
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
pub unsafe fn operator_add_22(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_23(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_22(_i, n)
    });
}
pub unsafe fn iter_swap_24(mut a: adl_Iterator, mut b: adl_Iterator) {
    (unsafe {
        let ___x: *mut *mut () = &mut a.ptr_;
        let ___y: *mut *mut () = &mut b.ptr_;
        swap_25(___x, ___y)
    });
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_26(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
}
pub unsafe fn operator_eq_27(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_28(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_28(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_27(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub static mut enable_borrowed_range_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedRange<>>");
const _: () = assert!(true, "std::ranges::view<BorrowedView>");
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedView>");
const _: () = assert!(true, "std::ranges::view<NonBorrowedView>");
const _: () = assert!(!(false), "!std::ranges::borrowed_range<NonBorrowedView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ {}
impl UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ {
    pub unsafe fn new_1(adaptor: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn get_container(&mut self) -> *mut () {
        return &mut (self.c);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ {}
impl UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ {
    pub unsafe fn new_1(adaptor: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn get_container(&mut self) -> *mut () {
        return &mut (self.c);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___
{}
impl UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ {
    pub unsafe fn new_1(adaptor: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn get_container(&mut self) -> *mut () {
        return &mut (self.c);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____ {}
impl UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____ {
    pub unsafe fn new_1(adaptor: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn get_container(&mut self) -> *mut () {
        return &mut (self.c);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____ {}
impl UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____ {
    pub unsafe fn new_1(adaptor: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn get_container(&mut self) -> *mut () {
        return &mut (self.c);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___
{}
impl UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ { pub unsafe fn new_1 ( adaptor : *mut () , ) -> Self { let mut this = Self { } ;
 this } pub unsafe fn get_container ( &mut self , ) -> *mut () { return & mut ( self . c  )  ;
 } }
pub static mut ints_30: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut chars_31: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut pairs_32: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn test_is_equal_33(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_34(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_35(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_36(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_37(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_38(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_39(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_40(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_41(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_42(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_43(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_44(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_45(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_46(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_47(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_48(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_49(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_50(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_51(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_52(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_53(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_54(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_55(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_56(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_57(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_58(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_59(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_60(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_61(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_62(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_63(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_64(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_65(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_66(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_67(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_68(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_69(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_70(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_71(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_72(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_73(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_74(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_75(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_76(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_77(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_78(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_79(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_80(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_81(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_82(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_83(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_84(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_85(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_86(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_87(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_88(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_89(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_90(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_91(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_92(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_93(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_94(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_95(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_96(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_97(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_98(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_99(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_100(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_101(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_102(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_103(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_104(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_105(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_106(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_107(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_108(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_109(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_110(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_111(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_112(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_113(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_114(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_115(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_116(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_117(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_118(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_119(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_120(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_121(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_122(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_123(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_124(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_125(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_126(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_127(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_128(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_129(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_130(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_131(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_132(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_133(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_134(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_135(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_136(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_137(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_138(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_139(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_140(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_141(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_142(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_143(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_144(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_145(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_146(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_147(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_148(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_149(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_150(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_151(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_152(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_153(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_154(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_155(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_156(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_157(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_158(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_159(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_160(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_161(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_162(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_163(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_164(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_165(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_166(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_167(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_168(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_169(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_170(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_171(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_172(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_173(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_174(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_175(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_176(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_177(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_178(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_179(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_180(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_181(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_182(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_183(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_184(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_185(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_186(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_187(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_188(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_189(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_190(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_191(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_192(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_193(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_194(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_195(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_196(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_197(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_198(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_199(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_200(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_201(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_202(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_203(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_204(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_205(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_206(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_207(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_208(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_209(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_210(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_211(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_212(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_213(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_214(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_215(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_216(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_217(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_218(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_219(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_220(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_221(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_222(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_223(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_224(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_225(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_226(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_227(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_228(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_229(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_230(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_231(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_232(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_233(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_234(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_235(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_236(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_237(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_238(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_239(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_240(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_241(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_242(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_243(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_244(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_245(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_246(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_247(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_248(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_249(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_250(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_251(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_252(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_253(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_254(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_255(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_256(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_257(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_258(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_259(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_260(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_261(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_262(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_263(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_264(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_265(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_266(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_267(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_268(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_269(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_270(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_271(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_272(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_273(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_274(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_275(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_276(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_277(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_278(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_279(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_280(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_281(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_282(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_283(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_284(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_285(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_286(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_287(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_288(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_289(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_290(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_291(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_292(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_293(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_294(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_295(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_296(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_297(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_298(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_299(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_300(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_301(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_302(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_303(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_304(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_305(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_306(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_307(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_308(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_309(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_310(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_311(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_312(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_313(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_314(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_315(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_316(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_317(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_318(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_319(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_320(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_321(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_322(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_323(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_324(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_325(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_326(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_327(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_328(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_329(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_330(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_331(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_332(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_333(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_334(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_335(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_336(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_337(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_338(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_339(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_340(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_341(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_342(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_343(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_344(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_345(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_346(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_347(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_348(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_349(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_350(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_351(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_352(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_353(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_354(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_355(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_356(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_357(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_358(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_359(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_360(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_361(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_362(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_363(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_364(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_365(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_366(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_367(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_368(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_369(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_370(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_371(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_372(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_373(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_374(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_375(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_376(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_377(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_378(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_379(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_380(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_381(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_382(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_383(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_384(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_385(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_386(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_387(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_388(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_389(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_390(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_391(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_392(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_393(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_394(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_395(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_396(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_397(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_398(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_399(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_400(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_401(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_402(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_403(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_404(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_405(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_406(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_407(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_408(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_409(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_410(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_411(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_412(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_413(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_414(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_415(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_416(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_417(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_418(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_419(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_420(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_421(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_422(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_423(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_424(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_425(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_426(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_427(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_428(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_429(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_430(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_431(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_432(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_433(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_434(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_435(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_436(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_437(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_438(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_439(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_440(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_441(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_442(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_443(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_444(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_445(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_446(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_447(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_448(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_449(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_450(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_451(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_452(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_453(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_454(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_455(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_456(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_457(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_458(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_459(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_460(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_461(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_462(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_463(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_464(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_465(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_466(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_467(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_468(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_469(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_470(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_471(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_472(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_473(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_474(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_475(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_476(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_477(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_478(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_479(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_480(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_481(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_482(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_483(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_484(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_485(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_486(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_487(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_permutation_488(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_489(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to: UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut result).cast::<()>()
            });
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_490(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to: UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut result).cast::<()>()
            });
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_491(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to : UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ = UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_492(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to: UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut result).cast::<()>()
            });
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_493(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to: UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut result).cast::<()>()
            });
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_494(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to : UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ = UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_495(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from : UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ = UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { & mut *(&raw mut from ).cast::<()>() } , )  ;
        let mut unwrap_to: UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut result).cast::<()>()
            });
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_496(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from : UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ = UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { & mut *(&raw mut from ).cast::<()>() } , )  ;
        let mut unwrap_to: UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut result).cast::<()>()
            });
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_497(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from : UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ = UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { & mut *(&raw mut from ).cast::<()>() } , )  ;
        let mut unwrap_to : UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ = UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_498(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to : UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____ = UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_499(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to : UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____ = UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_500(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to : UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ = UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_501(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to : UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____ = UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_502(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to : UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____ = UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_503(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from: UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ =
            UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                &mut *(&raw mut from).cast::<()>()
            });
        let mut unwrap_to : UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ = UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_504(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from : UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ = UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { & mut *(&raw mut from ).cast::<()>() } , )  ;
        let mut unwrap_to : UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____ = UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_505(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from : UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ = UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { & mut *(&raw mut from ).cast::<()>() } , )  ;
        let mut unwrap_to : UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____ = UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_is_equal_for_adaptors_506(mut inputs: ()) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *const () = &(*unimplemented!());
        let mut from: () = unimplemented!();
        let mut result: () = unimplemented!();
        let mut unwrap_from : UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ = UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { & mut *(&raw mut from ).cast::<()>() } , )  ;
        let mut unwrap_to : UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ = UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ :: new_1 ( { & mut *(&raw mut result ).cast::<()>() } , )  ;
        unimplemented!();
    }
}
pub unsafe fn test_507() {
    {
        {
            (unsafe {
                for_each_0 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_2 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_33 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_34 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_35 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_36 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_508 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_37 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_38 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_39 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_40 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_509 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_41 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_42 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_43 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_44 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_510 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_45 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_46 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_47 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_48 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } ) , )
            });
            (unsafe {
                for_each_3 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_4 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_49 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_50 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_51 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_52 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_511 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_53 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_54 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_55 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_56 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_512 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_57 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_58 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_59 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_60 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_513 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_61 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_62 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_63 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_64 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } ) , )
            });
        }
        {
            (unsafe {
                for_each_5 ( <types_type_list_std_vector_char__std_allocator_char____std_deque_char__std_allocator_char____std_list_char__std_allocator_char____std_forward_list_char__std_allocator_char___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_65 ( unimplemented!() , ) } )  ;
 ( unsafe { test_is_equal_66 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_67 ( unimplemented!() , ) } )  ;
 ( unsafe { test_is_equal_68 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_69 ( unimplemented!() , ) } )  ;
 ( unsafe { test_is_equal_70 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_71 ( unimplemented!() , ) } )  ;
 ( unsafe { test_is_equal_72 ( unimplemented!() , ) } )  ;
 } ) , )
            });
        }
    }
    {
        (unsafe {
            for_each_6 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_8 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_73 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_74 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_75 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_76 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_77 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_78 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_79 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_80 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_514 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_81 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_82 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_83 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_84 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_85 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_86 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_87 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_88 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_515 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_89 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_90 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_91 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_92 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_93 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_94 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_95 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_96 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_516 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_97 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_98 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_99 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_100 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_101 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_102 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_103 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_104 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_517 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_105 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_106 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_107 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_108 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_109 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_110 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_111 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_112 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_518 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_113 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_114 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_115 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_116 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_117 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_118 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_119 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_120 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_519 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_121 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_122 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_123 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_124 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_125 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_126 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_127 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_128 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_520 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_129 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_130 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_131 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_132 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_133 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_134 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_135 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_136 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } ) , )
        });
        (unsafe {
            for_each_9 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_10 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_137 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_138 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_139 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_140 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_141 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_142 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_143 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_144 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_521 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_145 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_146 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_147 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_148 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_149 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_150 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_151 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_152 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_522 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_153 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_154 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_155 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_156 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_157 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_158 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_159 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_160 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_523 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_161 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_162 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_163 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_164 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_165 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_166 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_167 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_168 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_524 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_169 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_170 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_171 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_172 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_173 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_174 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_175 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_176 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_525 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_177 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_178 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_179 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_180 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_181 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_182 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_183 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_184 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_526 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_185 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_186 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_187 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_188 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_189 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_190 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_191 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_192 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_527 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_193 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_194 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_195 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_196 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_197 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_198 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_199 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_200 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } ) , )
        });
    }
    {
        (unsafe {
            for_each_11 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_13 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_201 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_202 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_203 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_204 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_205 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_206 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_207 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_208 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_209 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_210 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_211 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_212 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_528 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_213 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_214 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_215 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_216 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_217 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_218 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_219 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_220 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_221 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_222 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_223 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_224 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_529 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_225 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_226 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_227 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_228 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_229 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_230 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_231 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_232 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_233 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_234 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_235 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_236 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_530 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_237 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_238 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_239 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_240 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_241 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_242 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_243 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_244 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_245 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_246 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_247 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_248 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_531 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_249 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_250 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_251 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_252 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_253 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_254 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_255 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_256 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_257 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_258 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_259 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_260 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_532 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_261 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_262 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_263 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_264 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_265 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_266 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_267 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_268 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_269 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_270 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_271 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_272 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_533 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_273 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_274 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_275 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_276 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_277 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_278 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_279 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_280 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_281 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_282 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_283 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_284 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_534 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_285 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_286 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_287 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_288 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_289 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_290 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_291 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_292 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_293 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_294 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_295 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_296 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_535 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_297 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_298 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_299 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_300 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_301 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_302 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_303 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_304 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_305 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_306 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_307 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_308 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_536 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_309 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_310 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_311 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_312 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_313 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_314 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_315 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_316 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_317 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_318 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_319 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_320 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_537 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_321 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_322 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_323 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_324 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_325 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_326 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_327 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_328 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_329 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_330 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_331 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_332 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_538 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_333 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_334 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_335 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_336 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_337 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_338 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_339 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_340 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_341 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_342 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_343 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_344 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } ) , )
        });
        (unsafe {
            for_each_14 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_15 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_345 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_346 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_347 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_348 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_349 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_350 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_351 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_352 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_353 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_354 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_355 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_356 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_539 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_357 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_358 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_359 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_360 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_361 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_362 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_363 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_364 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_365 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_366 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_367 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_368 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_540 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_369 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_370 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_371 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_372 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_373 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_374 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_375 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_376 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_377 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_378 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_379 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_380 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_541 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_381 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_382 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_383 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_384 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_385 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_386 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_387 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_388 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_389 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_390 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_391 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_392 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_542 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_393 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_394 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_395 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_396 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_397 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_398 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_399 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_400 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_401 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_402 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_403 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_404 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_543 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_405 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_406 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_407 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_408 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_409 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_410 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_411 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_412 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_413 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_414 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_415 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_416 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_544 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_417 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_418 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_419 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_420 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_421 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_422 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_423 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_424 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_425 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_426 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_427 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_428 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_545 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_429 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_430 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_431 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_432 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_433 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_434 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_435 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_436 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_437 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_438 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_439 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_440 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_546 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_441 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_442 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_443 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_444 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_445 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_446 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_447 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_448 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_449 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_450 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_451 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_452 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_547 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_453 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_454 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_455 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_456 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_457 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_458 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_459 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_460 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_461 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_462 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_463 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_464 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_548 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_465 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_466 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_467 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_468 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_469 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_470 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_471 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_472 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_473 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_474 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_475 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_476 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_549 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_permutation_477 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_478 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_479 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_480 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_481 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_482 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_483 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_484 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_485 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_486 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_487 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_permutation_488 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } ) , )
        });
    }
    {
        (unsafe {
            for_each_16 ( <types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_18 ( <types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_for_adaptors_489 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_490 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_491 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_550 ( <types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_for_adaptors_492 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_493 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_494 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_551 ( <types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_for_adaptors_495 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_496 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_497 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } ) , )
        });
        (unsafe {
            for_each_19 ( <types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_20 ( <types_type_list_std_stack_double__std_deque_double__std_allocator_double_____std_queue_double__std_deque_double__std_allocator_double_____std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_for_adaptors_498 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_499 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_500 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_552 ( <types_type_list_std_stack_double__std_deque_double__std_allocator_double_____std_queue_double__std_deque_double__std_allocator_double_____std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_for_adaptors_501 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_502 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_503 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_553 ( <types_type_list_std_stack_double__std_deque_double__std_allocator_double_____std_queue_double__std_deque_double__std_allocator_double_____std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_is_equal_for_adaptors_504 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_505 ( unimplemented!() , ) } )  ;
 } , | | { ( unsafe { test_is_equal_for_adaptors_506 ( unimplemented!() , ) } )  ;
 } ) , ) } )  ;
 } ) , )
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
    (unsafe { test_507() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_29);
    std::cell::LazyCell::force(&*&raw const ints_30);
    std::cell::LazyCell::force(&*&raw const chars_31);
    std::cell::LazyCell::force(&*&raw const pairs_32);
}
