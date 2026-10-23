extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct support_double_move_tracker {
    #[offset(0)]
    moved_from_: (),
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        unimplemented!();
        field!(other, moved_from_).write(true);
        __this
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        { support_double_move_tracker::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_std_vector_char__std_allocator_char____std_deque_char__std_allocator_char____std_list_char__std_allocator_char____std_forward_list_char__std_allocator_char___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_std_stack_double__std_deque_double__std_allocator_double_____std_queue_double__std_deque_double__std_allocator_double_____std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int_____types_type_list_std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int_____types_type_list_std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int_____types_type_list_std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double_____types_type_list_std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double_____types_type_list_std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double_____types_type_list_std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int______types_type_list_std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int______types_type_list_std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int______types_type_list_std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int______types_type_list_std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int______types_type_list_std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double______types_type_list_std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double______types_type_list_std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double______types_type_list_std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double______types_type_list_std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double______types_type_list_std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____
{}
pub fn for_each_0(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub fn for_each_2(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub fn for_each_3(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub fn for_each_4(
    mut _a0 : types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub fn for_each_5(
    mut _a0 : types_type_list_std_vector_char__std_allocator_char____std_deque_char__std_allocator_char____std_list_char__std_allocator_char____std_forward_list_char__std_allocator_char___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub fn for_each_6(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            ({ (*f.borrow()).spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            ({ (*f.borrow()).spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            ({ (*f.borrow()).spec::<fn()>(7).call() });
            0_()
        };
        swallow_7(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7)
    });
}
pub fn for_each_8(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            ({ (*f.borrow()).spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            ({ (*f.borrow()).spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            ({ (*f.borrow()).spec::<fn()>(7).call() });
            0_()
        };
        swallow_7(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7)
    });
}
pub fn for_each_9(
    mut _a0 : types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            ({ (*f.borrow()).spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            ({ (*f.borrow()).spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            ({ (*f.borrow()).spec::<fn()>(7).call() });
            0_()
        };
        swallow_7(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7)
    });
}
pub fn for_each_10(
    mut _a0 : types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            ({ (*f.borrow()).spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            ({ (*f.borrow()).spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            ({ (*f.borrow()).spec::<fn()>(7).call() });
            0_()
        };
        swallow_7(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7)
    });
}
pub fn for_each_11(
    mut _a0 : types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            ({ (*f.borrow()).spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            ({ (*f.borrow()).spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            ({ (*f.borrow()).spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            ({ (*f.borrow()).spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            ({ (*f.borrow()).spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            ({ (*f.borrow()).spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            ({ (*f.borrow()).spec::<fn()>(11).call() });
            0_()
        };
        swallow_12(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
        )
    });
}
pub fn for_each_13(
    mut _a0 : types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            ({ (*f.borrow()).spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            ({ (*f.borrow()).spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            ({ (*f.borrow()).spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            ({ (*f.borrow()).spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            ({ (*f.borrow()).spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            ({ (*f.borrow()).spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            ({ (*f.borrow()).spec::<fn()>(11).call() });
            0_()
        };
        swallow_12(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
        )
    });
}
pub fn for_each_14(
    mut _a0 : types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            ({ (*f.borrow()).spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            ({ (*f.borrow()).spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            ({ (*f.borrow()).spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            ({ (*f.borrow()).spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            ({ (*f.borrow()).spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            ({ (*f.borrow()).spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            ({ (*f.borrow()).spec::<fn()>(11).call() });
            0_()
        };
        swallow_12(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
        )
    });
}
pub fn for_each_15(
    mut _a0 : types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            ({ (*f.borrow()).spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            ({ (*f.borrow()).spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            ({ (*f.borrow()).spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            ({ (*f.borrow()).spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            ({ (*f.borrow()).spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            ({ (*f.borrow()).spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            ({ (*f.borrow()).spec::<fn()>(11).call() });
            0_()
        };
        swallow_12(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
        )
    });
}
pub fn for_each_16(
    mut _a0 : types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        swallow_17(_arg0, _arg1, _arg2)
    });
}
pub fn for_each_18(
    mut _a0 : types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        swallow_17(_arg0, _arg1, _arg2)
    });
}
pub fn for_each_19(
    mut _a0 : types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        swallow_17(_arg0, _arg1, _arg2)
    });
}
pub fn for_each_20(
    mut _a0 : types_type_list_std_stack_double__std_deque_double__std_allocator_double_____std_queue_double__std_deque_double__std_allocator_double_____std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        swallow_17(_arg0, _arg1, _arg2)
    });
}
pub fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: ()) {}
pub fn swallow_7(
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
pub fn swallow_12(
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
pub fn swallow_17(mut _a0: (), mut _a1: (), mut _a2: ()) {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_input_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_output_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct IteratorOpCounts {
    #[offset(0)]
    pub increments: (),
    #[offset(8)]
    pub decrements: (),
    #[offset(16)]
    pub zero_moves: (),
    #[offset(24)]
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
pub fn operator_eq_21(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> () {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<()>,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_21(
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<()>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_22(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_23(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_22(_i, n)
    });
}
pub fn iter_swap_24(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_25(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) }).reinterpret_cast::<()>();
}
pub fn operator_eq_26(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_27(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return std::cmp::Ord::cmp(
        &(lhs.with(|__s| __s.ptr_.clone())),
        &(rhs.with(|__s| __s.ptr_.clone())),
    );
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct adl_Iterator {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<()>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<()>, mut iter_moves: Ptr<()>, mut iter_swaps: Ptr<()>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<()>, iter_moves: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<()>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<()>::null() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<()>, iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_27(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_26(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<()>::null(),
                iter_moves_: Ptr::<()>::null(),
                iter_swaps_: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
}
impl rvalue_iterator_int_ptr_ {}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
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
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct BorrowedRange_int_ {}
const _: () = assert!(!(false), "!std::ranges::view<BorrowedRange<>>");
thread_local!(
    pub static enable_borrowed_range_28: Value<()> = Rc::new(RefCell::new(true));
);
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedRange<>>");
const _: () = assert!(true, "std::ranges::view<BorrowedView>");
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedView>");
const _: () = assert!(true, "std::ranges::view<NonBorrowedView>");
const _: () = assert!(!(false), "!std::ranges::borrowed_range<NonBorrowedView>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ {}
impl UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____ {
    pub fn new_1(adaptor: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ {}
impl UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____ {
    pub fn new_1(adaptor: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___
{}
impl UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ {
    pub fn new_1(adaptor: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____ {}
impl UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____ {
    pub fn new_1(adaptor: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____ {}
impl UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____ {
    pub fn new_1(adaptor: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___
{}
impl UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ { pub fn new_1 ( adaptor : Ptr< ()  > , ) -> Self { Self { }  } }
thread_local!(
    pub static ints_29: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static chars_30: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static pairs_31: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn test_is_equal_32(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_33(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_34(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_35(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_36(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_37(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_38(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_39(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_40(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_41(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_42(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_43(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_44(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_45(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_46(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_47(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_48(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_49(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_50(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_51(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_52(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_53(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_54(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_55(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_56(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_57(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_58(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_59(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_60(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_61(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_62(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_63(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_64(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () = std_vector_char__std_allocator_char__::new_1({ unimplemented!() }, {
            unimplemented!()
        });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_65(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                None,
            );
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_66(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_char__std_allocator_char__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_67(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                None,
            );
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_68(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_char__std_allocator_char__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_69(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                None,
            );
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_70(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_char__std_allocator_char__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_71(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                None,
            );
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_72(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_73(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_74(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_75(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_76(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_77(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_78(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_79(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_80(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_81(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_82(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_83(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_84(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_85(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_86(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_87(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_88(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_89(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_90(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_91(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_92(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_93(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_94(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_95(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_96(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_97(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_98(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_99(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_100(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_101(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_102(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_103(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_104(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_105(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_106(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_107(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_108(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_109(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_110(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_111(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_112(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_113(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_114(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_115(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_116(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_117(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_118(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_119(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_120(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_121(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_122(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_123(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_124(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_125(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_126(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_127(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_128(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_129(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_130(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_131(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_132(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_133(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_134(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_135(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_136(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_137(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_138(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_139(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_140(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_141(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_142(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_143(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let mut from: () =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_144(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_145(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_146(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_147(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_148(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_149(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_150(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_151(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_152(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_153(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_154(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_155(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_156(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_157(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_158(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_159(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
            { unimplemented!() },
            { unimplemented!() },
        )));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_160(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_161(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_162(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_163(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_164(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_165(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_166(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_167(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_168(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_169(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_170(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_171(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_172(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_173(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_174(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_175(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_176(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_177(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_178(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_179(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_180(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_181(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_182(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_183(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_184(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_185(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_186(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_187(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_188(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_189(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_190(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_191(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_192(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_193(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_194(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_195(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_196(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_197(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_198(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_199(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_200(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_201(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_202(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_203(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_204(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_205(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_206(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_207(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_208(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_209(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_210(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_211(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_212(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_213(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_214(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_215(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_216(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_217(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_218(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_219(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_220(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_221(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_222(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_223(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_224(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_225(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_226(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_227(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_228(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_229(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_230(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_231(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_232(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_233(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_234(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_235(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_236(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_237(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_238(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_239(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_240(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_241(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_242(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_243(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_244(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_245(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_246(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_247(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_248(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_249(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_250(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_251(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_252(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_253(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_254(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_255(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_256(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_257(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_258(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_259(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_260(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_261(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_262(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_263(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_264(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_265(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_266(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_267(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_268(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_269(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_270(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_271(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_272(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_273(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_274(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_275(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_276(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_277(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_278(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_279(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_280(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_281(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_282(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_283(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_284(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_285(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_286(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_287(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_288(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_289(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_290(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_291(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_292(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_293(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_294(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_295(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_296(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_297(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_298(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_299(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_300(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_301(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_302(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_303(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_304(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_305(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_306(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_307(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_308(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_309(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_310(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_311(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_312(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_313(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_314(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_315(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_316(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_317(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_318(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_319(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_320(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_321(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_322(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_323(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_324(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_325(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_326(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_327(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_328(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_329(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_330(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_331(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_332(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_333(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_334(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_335(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_336(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_337(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_338(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_339(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_340(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_341(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_342(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_343(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_344(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_345(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_346(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_347(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_348(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_349(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_350(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_351(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_352(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_353(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_354(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_355(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_356(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_357(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_358(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_359(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_360(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_361(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_362(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_363(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_364(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_365(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_366(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_367(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_368(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_369(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_370(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_371(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_372(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_373(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_374(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_375(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_376(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_377(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_378(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_379(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___::new_1(
                { unimplemented!() },
                { unimplemented!() },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_380(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_381(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_382(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_383(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_384(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_385(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_386(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_387(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_388(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_389(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_390(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_391(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from : Value<() > = Rc::new(RefCell::new(std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int___ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_392(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_393(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_394(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_395(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_396(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_397(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_398(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_399(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_400(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_401(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_402(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_403(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_404(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_405(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_406(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_407(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_408(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_409(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_410(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_411(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_412(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_413(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_414(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_415(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_416(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_417(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_418(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_419(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_420(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_421(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_422(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_423(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_424(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_425(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_426(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_427(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_428(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_429(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_430(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_431(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_432(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_433(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_434(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_435(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_436(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_437(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_438(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_439(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_440(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_441(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_442(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_443(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_444(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_445(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_446(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_447(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_448(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_449(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_450(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_451(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_452(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_453(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_454(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_455(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_456(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_457(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_458(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_459(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_460(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_461(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_462(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_463(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_464(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_465(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_466(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_467(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_468(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_469(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_470(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_471(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_472(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_473(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_474(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_475(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_476(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_477(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_478(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_479(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_480(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_481(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_482(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_483(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_484(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_485(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_486(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_permutation_487(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_488(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_stack_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to: Value<UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                    (result.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_489(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_stack_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to: Value<UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                    (result.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_490(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_stack_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to : Value<UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { (result .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_491(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_queue_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to: Value<UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                    (result.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_492(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_queue_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to: Value<UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                    (result.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_493(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_queue_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to : Value<UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { (result .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_494(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                None,
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from : Value<UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { (from .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        let unwrap_to: Value<UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                    (result.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_495(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                None,
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from : Value<UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { (from .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        let unwrap_to: Value<UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                    (result.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_496(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                None,
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from : Value<UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { (from .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        let unwrap_to : Value<UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { (result .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_497(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_stack_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to: Value<
            UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____,
        > = Rc::new(RefCell::new(
            UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____::new_1({
                (result.as_pointer()).reinterpret_cast::<()>()
            }),
        ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_498(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_stack_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to: Value<
            UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____,
        > = Rc::new(RefCell::new(
            UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____::new_1({
                (result.as_pointer()).reinterpret_cast::<()>()
            }),
        ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_499(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_stack_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to : Value<UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ :: new_1 ( { (result .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_500(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_queue_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to: Value<
            UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____,
        > = Rc::new(RefCell::new(
            UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____::new_1({
                (result.as_pointer()).reinterpret_cast::<()>()
            }),
        ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_501(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_queue_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to: Value<
            UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____,
        > = Rc::new(RefCell::new(
            UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____::new_1({
                (result.as_pointer()).reinterpret_cast::<()>()
            }),
        ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_502(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_queue_int__std_deque_int__std_allocator_int___::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from: Value<UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____> =
            Rc::new(RefCell::new(
                UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____::new_1({
                    (from.as_pointer()).reinterpret_cast::<()>()
                }),
            ));
        let unwrap_to : Value<UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ :: new_1 ( { (result .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_503(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                None,
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from : Value<UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { (from .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        let unwrap_to: Value<
            UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____,
        > = Rc::new(RefCell::new(
            UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____::new_1({
                (result.as_pointer()).reinterpret_cast::<()>()
            }),
        ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_504(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                None,
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from : Value<UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { (from .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        let unwrap_to: Value<
            UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____,
        > = Rc::new(RefCell::new(
            UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____::new_1({
                (result.as_pointer()).reinterpret_cast::<()>()
            }),
        ));
        unimplemented!();
    }
}
pub fn test_is_equal_for_adaptors_505(inputs: ()) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<()> = unimplemented!();
        let from: Value<()> = Rc::new(RefCell::new(
            std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                None,
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let unwrap_from : Value<UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___ :: new_1 ( { (from .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        let unwrap_to : Value<UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ > = Rc::new(RefCell::new(UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___ :: new_1 ( { (result .as_pointer() ).reinterpret_cast::<()  >()  } , )  )) ;
        unimplemented!();
    }
}
pub fn test_506() {
    {
        {
            ({
                for_each_0 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { for_each_2 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_32 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_33 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_34 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_35 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_507 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_36 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_37 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_38 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_39 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_508 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_40 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_41 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_42 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_43 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_509 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_44 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_45 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_46 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_47 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } )  , )
            });
            ({
                for_each_3 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { for_each_4 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_48 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_49 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_50 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_51 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_510 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_52 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_53 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_54 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_55 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_511 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_56 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_57 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_58 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_59 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_512 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_60 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_61 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_62 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_63 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } )  , )
            });
        }
        {
            ({
                for_each_5 ( <types_type_list_std_vector_char__std_allocator_char____std_deque_char__std_allocator_char____std_list_char__std_allocator_char____std_forward_list_char__std_allocator_char___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_64 ( unimplemented!()  , ) } )  ;
 ( { test_is_equal_65 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_66 ( unimplemented!()  , ) } )  ;
 ( { test_is_equal_67 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_68 ( unimplemented!()  , ) } )  ;
 ( { test_is_equal_69 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_70 ( unimplemented!()  , ) } )  ;
 ( { test_is_equal_71 ( unimplemented!()  , ) } )  ;
 } )  , )
            });
        }
    }
    {
        ({
            for_each_6 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { for_each_8 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_72 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_73 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_74 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_75 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_76 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_77 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_78 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_79 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_513 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_80 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_81 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_82 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_83 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_84 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_85 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_86 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_87 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_514 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_88 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_89 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_90 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_91 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_92 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_93 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_94 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_95 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_515 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_96 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_97 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_98 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_99 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_100 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_101 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_102 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_103 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_516 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_104 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_105 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_106 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_107 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_108 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_109 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_110 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_111 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_517 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_112 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_113 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_114 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_115 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_116 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_117 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_118 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_119 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_518 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_120 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_121 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_122 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_123 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_124 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_125 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_126 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_127 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_519 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_128 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_129 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_130 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_131 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_132 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_133 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_134 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_135 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } )  , )
        });
        ({
            for_each_9 ( <types_type_list_std_vector_int__std_allocator_int____std_deque_int__std_allocator_int____std_list_int__std_allocator_int____std_forward_list_int__std_allocator_int____std_set_int__std_less_int___std_allocator_int____std_multiset_int__std_less_int___std_allocator_int____std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int____std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int___  >::default()  , lambda! ( Generic , { } , | | { ( { for_each_10 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_136 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_137 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_138 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_139 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_140 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_141 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_142 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_143 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_520 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_144 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_145 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_146 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_147 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_148 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_149 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_150 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_151 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_521 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_152 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_153 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_154 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_155 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_156 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_157 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_158 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_159 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_522 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_160 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_161 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_162 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_163 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_164 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_165 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_166 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_167 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_523 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_168 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_169 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_170 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_171 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_172 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_173 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_174 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_175 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_524 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_176 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_177 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_178 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_179 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_180 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_181 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_182 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_183 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_525 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_184 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_185 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_186 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_187 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_188 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_189 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_190 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_191 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_526 ( <types_type_list_std_vector_double__std_allocator_double____std_deque_double__std_allocator_double____std_list_double__std_allocator_double____std_forward_list_double__std_allocator_double____std_set_double__std_less_double___std_allocator_double____std_multiset_double__std_less_double___std_allocator_double____std_unordered_set_double__std_hash_double___std_equal_to_double___std_allocator_double____std_unordered_multiset_double__std_hash_double___std_equal_to_double___std_allocator_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_192 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_193 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_194 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_195 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_196 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_197 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_198 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_199 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } )  , )
        });
    }
    {
        ({
            for_each_11 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { for_each_13 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_200 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_201 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_202 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_203 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_204 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_205 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_206 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_207 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_208 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_209 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_210 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_211 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_527 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_212 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_213 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_214 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_215 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_216 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_217 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_218 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_219 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_220 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_221 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_222 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_223 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_528 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_224 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_225 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_226 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_227 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_228 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_229 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_230 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_231 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_232 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_233 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_234 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_235 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_529 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_236 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_237 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_238 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_239 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_240 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_241 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_242 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_243 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_244 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_245 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_246 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_247 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_530 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_248 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_249 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_250 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_251 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_252 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_253 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_254 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_255 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_256 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_257 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_258 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_259 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_531 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_260 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_261 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_262 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_263 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_264 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_265 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_266 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_267 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_268 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_269 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_270 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_271 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_532 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_272 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_273 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_274 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_275 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_276 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_277 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_278 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_279 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_280 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_281 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_282 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_283 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_533 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_284 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_285 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_286 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_287 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_288 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_289 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_290 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_291 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_292 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_293 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_294 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_295 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_534 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_296 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_297 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_298 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_299 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_300 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_301 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_302 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_303 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_304 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_305 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_306 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_307 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_535 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_308 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_309 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_310 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_311 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_312 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_313 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_314 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_315 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_316 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_317 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_318 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_319 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_536 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_320 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_321 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_322 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_323 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_324 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_325 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_326 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_327 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_328 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_329 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_330 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_331 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_537 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_332 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_333 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_334 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_335 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_336 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_337 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_338 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_339 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_340 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_341 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_342 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_343 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } )  , )
        });
        ({
            for_each_14 ( <types_type_list_std_vector_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_deque_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_forward_list_std_pair_const_int__int___std_allocator_std_pair_const_int__int_____std_set_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_multiset_std_pair_const_int__int___std_less_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_multimap_int__int__std_less_int___std_allocator_std_pair_const_int__int_____std_unordered_set_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_multiset_std_pair_const_int__int___std_hash_std_pair_const_int__int____std_equal_to_std_pair_const_int__int____std_allocator_std_pair_const_int__int_____std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int_____std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int____  >::default()  , lambda! ( Generic , { } , | | { ( { for_each_15 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_344 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_345 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_346 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_347 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_348 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_349 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_350 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_351 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_352 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_353 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_354 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_355 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_538 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_356 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_357 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_358 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_359 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_360 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_361 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_362 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_363 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_364 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_365 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_366 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_367 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_539 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_368 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_369 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_370 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_371 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_372 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_373 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_374 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_375 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_376 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_377 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_378 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_379 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_540 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_380 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_381 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_382 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_383 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_384 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_385 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_386 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_387 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_388 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_389 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_390 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_391 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_541 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_392 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_393 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_394 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_395 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_396 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_397 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_398 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_399 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_400 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_401 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_402 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_403 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_542 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_404 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_405 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_406 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_407 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_408 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_409 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_410 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_411 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_412 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_413 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_414 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_415 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_543 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_416 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_417 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_418 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_419 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_420 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_421 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_422 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_423 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_424 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_425 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_426 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_427 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_544 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_428 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_429 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_430 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_431 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_432 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_433 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_434 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_435 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_436 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_437 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_438 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_439 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_545 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_440 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_441 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_442 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_443 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_444 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_445 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_446 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_447 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_448 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_449 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_450 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_451 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_546 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_452 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_453 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_454 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_455 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_456 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_457 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_458 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_459 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_460 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_461 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_462 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_463 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_547 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_464 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_465 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_466 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_467 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_468 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_469 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_470 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_471 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_472 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_473 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_474 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_475 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_548 ( <types_type_list_std_vector_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_deque_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_forward_list_std_pair_const_long__double___std_allocator_std_pair_const_long__double_____std_set_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_multiset_std_pair_const_long__double___std_less_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_map_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_multimap_long__double__std_less_long___std_allocator_std_pair_const_long__double_____std_unordered_set_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_multiset_std_pair_const_long__double___std_hash_std_pair_const_long__double____std_equal_to_std_pair_const_long__double____std_allocator_std_pair_const_long__double_____std_unordered_map_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double_____std_unordered_multimap_long__double__std_hash_long___std_equal_to_long___std_allocator_std_pair_const_long__double____  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_permutation_476 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_477 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_478 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_479 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_480 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_481 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_482 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_483 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_484 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_485 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_486 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_permutation_487 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } )  , )
        });
    }
    {
        ({
            for_each_16 ( <types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___  >::default()  , lambda! ( Generic , { } , | | { ( { for_each_18 ( <types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_for_adaptors_488 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_489 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_490 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_549 ( <types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_for_adaptors_491 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_492 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_493 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_550 ( <types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_for_adaptors_494 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_495 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_496 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } )  , )
        });
        ({
            for_each_19 ( <types_type_list_std_stack_int__std_deque_int__std_allocator_int_____std_queue_int__std_deque_int__std_allocator_int_____std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___  >::default()  , lambda! ( Generic , { } , | | { ( { for_each_20 ( <types_type_list_std_stack_double__std_deque_double__std_allocator_double_____std_queue_double__std_deque_double__std_allocator_double_____std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_for_adaptors_497 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_498 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_499 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_551 ( <types_type_list_std_stack_double__std_deque_double__std_allocator_double_____std_queue_double__std_deque_double__std_allocator_double_____std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_for_adaptors_500 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_501 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_502 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_552 ( <types_type_list_std_stack_double__std_deque_double__std_allocator_double_____std_queue_double__std_deque_double__std_allocator_double_____std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___  >::default()  , lambda! ( Generic , { } , | | { ( { test_is_equal_for_adaptors_503 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_504 ( unimplemented!()  , ) } )  ;
 } , | | { ( { test_is_equal_for_adaptors_505 ( unimplemented!()  , ) } )  ;
 } )  , ) } )  ;
 } )  , )
        });
    }
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_506() });
    return 0_();
}
pub trait BorrowedRange_int_Impl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl BorrowedRange_int_Impl for Ptr<BorrowedRange_int_> {}
pub trait UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___Impl
{
    fn get_container(&self) -> Ptr<()>;
}
impl UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___Impl for Ptr<UnwrapAdaptor_std_priority_queue_double__std_vector_double__std_allocator_double____std_less_double___> {
fn get_container ( &self , ) -> Ptr< ()  > { return ( field_ptr!(( (* self ) as *mut () )  , c) )   ;
 } }
pub trait UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___Impl
{
    fn get_container(&self) -> Ptr<()>;
}
impl UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___Impl
    for Ptr<
        UnwrapAdaptor_std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int___,
    >
{
    fn get_container(&self) -> Ptr<()> {
        return (field_ptr!(((*self) as *mut ()), c));
    }
}
pub trait UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____Impl {
    fn get_container(&self) -> Ptr<()>;
}
impl UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____Impl
    for Ptr<UnwrapAdaptor_std_queue_double__std_deque_double__std_allocator_double____>
{
    fn get_container(&self) -> Ptr<()> {
        return (field_ptr!(((*self) as *mut ()), c));
    }
}
pub trait UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____Impl {
    fn get_container(&self) -> Ptr<()>;
}
impl UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____Impl
    for Ptr<UnwrapAdaptor_std_queue_int__std_deque_int__std_allocator_int____>
{
    fn get_container(&self) -> Ptr<()> {
        return (field_ptr!(((*self) as *mut ()), c));
    }
}
pub trait UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____Impl {
    fn get_container(&self) -> Ptr<()>;
}
impl UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____Impl
    for Ptr<UnwrapAdaptor_std_stack_double__std_deque_double__std_allocator_double____>
{
    fn get_container(&self) -> Ptr<()> {
        return (field_ptr!(((*self) as *mut ()), c));
    }
}
pub trait UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____Impl {
    fn get_container(&self) -> Ptr<()>;
}
impl UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____Impl
    for Ptr<UnwrapAdaptor_std_stack_int__std_deque_int__std_allocator_int____>
{
    fn get_container(&self) -> Ptr<()> {
        return (field_ptr!(((*self) as *mut ()), c));
    }
}
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> ();
    fn iter_swaps(&self) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_sub_10(&self, n: ()) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> ();
    fn operator_add_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: ()) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: ()) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as () as ());
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_inc_14(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return prev;
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) {
        unimplemented!()
    }
}
impl cpp20_input_iterator_int_ptr_Impl for Ptr<cpp20_input_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp20_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp20_output_iterator_int_ptr_Impl for Ptr<cpp20_output_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
}
pub trait forward_sized_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait rvalue_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.it_.clone()) } == { _a0.with(|__s| __s.it_.clone()) });
    }
}
pub trait support_double_move_trackerImpl {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker>;
}
impl support_double_move_trackerImpl for Ptr<support_double_move_tracker> {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker> {
        unimplemented!();
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = enable_borrowed_range_28.with(|_| ());
    let _ = ints_29.with(|_| ());
    let _ = chars_30.with(|_| ());
    let _ = pairs_31.with(|_| ());
}
