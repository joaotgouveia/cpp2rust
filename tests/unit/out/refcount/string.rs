extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_7 {}
impl Clone for allocator_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_7 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_7 {}
impl allocator_7 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_8 {}
impl Clone for allocator_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_8 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_8 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_8 {
    pub fn copy_from(__a: Ptr<allocator_8>) -> Self {
        unimplemented!()
    }
}
impl allocator_8 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_8>) -> Ptr<allocator_8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_9 {}
impl Clone for allocator_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_9 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_9 {
    pub fn copy_from(__a: Ptr<allocator_9>) -> Self {
        unimplemented!()
    }
}
impl allocator_9 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_9>) -> Ptr<allocator_9> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_10 {}
impl Clone for allocator_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_10 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_10 {
    pub fn copy_from(__a: Ptr<allocator_10>) -> Self {
        unimplemented!()
    }
}
impl allocator_10 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_10>) -> Ptr<allocator_10> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_11 {}
impl Clone for allocator_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_11 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_11 {
    pub fn copy_from(__a: Ptr<allocator_11>) -> Self {
        unimplemented!()
    }
}
impl allocator_11 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_11>) -> Ptr<allocator_11> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_12 {}
impl Clone for allocator_traits_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_12 {
    pub fn allocate_1(__a: Ptr<allocator_8>, mut __n: u64) -> Ptr<i8> {
        unimplemented!()
    }
}
impl allocator_traits_12 {
    pub fn deallocate(__a: Ptr<allocator_8>, mut __p: Ptr<i8>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_12 {
    pub fn max_size(__a: Ptr<allocator_8>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_12 {
    pub fn select_on_container_copy_construction(__rhs: Ptr<allocator_8>) -> allocator_8 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_13 {}
impl Clone for allocator_traits_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_13 {
    pub fn allocate_1(__a: Ptr<allocator_9>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_13 {
    pub fn deallocate(__a: Ptr<allocator_9>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_13 {
    pub fn max_size(__a: Ptr<allocator_9>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_14 {}
impl Clone for allocator_traits_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub fn allocate_1(__a: Ptr<allocator_10>, mut __n: u64) -> Ptr<u16> {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub fn deallocate(__a: Ptr<allocator_10>, mut __p: Ptr<u16>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub fn max_size(__a: Ptr<allocator_10>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_15 {}
impl Clone for allocator_traits_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_15 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_15 {
    pub fn allocate_1(__a: Ptr<allocator_11>, mut __n: u64) -> Ptr<u32> {
        unimplemented!()
    }
}
impl allocator_traits_15 {
    pub fn deallocate(__a: Ptr<allocator_11>, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_15 {
    pub fn max_size(__a: Ptr<allocator_11>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_16 {}
impl Clone for char_traits_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_16 {
    pub fn assign_1(__c1: Ptr<i8>, __c2: Ptr<i8>) {
        unimplemented!()
    }
}
impl char_traits_16 {
    pub fn lt(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_16 {
    pub fn compare(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_16 {
    pub fn length(mut __s: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_16 {
    pub fn find(mut __s: Ptr<i8>, mut __n: usize, __a: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_16 {
    pub fn move_(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_16 {
    pub fn copy(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_16 {
    pub fn assign_9(mut __s: Ptr<i8>, mut __n: usize, mut __a: i8) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_16 {
    pub fn eof() -> i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_17 {}
impl Clone for char_traits_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_17 {
    pub fn assign_1(__c1: Ptr<i32>, __c2: Ptr<i32>) {
        unimplemented!()
    }
}
impl char_traits_17 {
    pub fn copy(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl char_traits_17 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_17 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_18 {}
impl Clone for char_traits_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_18 {
    pub fn assign_1(__c1: Ptr<u16>, __c2: Ptr<u16>) {
        unimplemented!()
    }
}
impl char_traits_18 {
    pub fn eq(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_18 {
    pub fn lt(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_18 {
    pub fn copy(mut __s1: Ptr<u16>, mut __s2: Ptr<u16>, mut __n: usize) -> Ptr<u16> {
        unimplemented!()
    }
}
impl char_traits_18 {
    pub fn eq_int_type(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_18 {
    pub fn eof() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_19 {}
impl Clone for char_traits_19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_19 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_19 {
    pub fn assign_1(__c1: Ptr<u32>, __c2: Ptr<u32>) {
        unimplemented!()
    }
}
impl char_traits_19 {
    pub fn eq(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_19 {
    pub fn lt(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_19 {
    pub fn copy(mut __s1: Ptr<u32>, mut __s2: Ptr<u32>, mut __n: usize) -> Ptr<u32> {
        unimplemented!()
    }
}
impl char_traits_19 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_19 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_20 {}
impl Clone for basic_string_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_20 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_basic_string_char__std_char_traits_char___std_allocator_char___npos_22: Value<
        u64,
    > = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_basic_string_char___Alloc_hider {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<i8>,
}
impl Clone for std_basic_string_char___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_char___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub fn new_1(mut __dat: Ptr<i8>, __a: Ptr<allocator_8>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub fn new_2(mut __dat: Ptr<i8>, __a: Option<Ptr<allocator_8>>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {}
impl std_basic_string_char___Alloc_hider {}
impl std_basic_string_char___Alloc_hider {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std_basic_string_char___Alloc_hider>,
    ) -> Ptr<std_basic_string_char___Alloc_hider> {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub fn move_assign(
        &self,
        _a0: Ptr<std_basic_string_char___Alloc_hider>,
    ) -> Ptr<std_basic_string_char___Alloc_hider> {
        unimplemented!()
    }
}
pub type anon_23 = u32;
pub const anon_23__S_local_capacity: anon_23 = 15;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_21 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i8]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_21 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn new_2(__a: Ptr<allocator_8>) -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn copy_from(__str: Ptr<basic_string_20>) -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn new_3(__str: Ptr<basic_string_20>, mut __pos: u64, mut __n: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn new_4(mut __s: Ptr<i8>, mut __n: u64, __a: Option<Ptr<allocator_8>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn move_from(__str: Ptr<basic_string_20>) -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn new_5(mut __s: Ptr<i8>, __a: Option<Ptr<allocator_8>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn new_6(mut __n: u64, mut __c: i8, __a: Option<Ptr<allocator_8>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn new_7(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_8>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_31 {}
impl Clone for basic_string_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_basic_string_wchar_t___Alloc_hider {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<i32>,
}
impl Clone for std_basic_string_wchar_t___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_wchar_t___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {
    pub fn new_1(mut __dat: Ptr<i32>, __a: Ptr<allocator_9>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {}
impl std_basic_string_wchar_t___Alloc_hider {}
impl std_basic_string_wchar_t___Alloc_hider {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std_basic_string_wchar_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_wchar_t___Alloc_hider> {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {
    pub fn move_assign(
        &self,
        _a0: Ptr<std_basic_string_wchar_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_wchar_t___Alloc_hider> {
        unimplemented!()
    }
}
pub type anon_33 = u32;
pub const anon_33__S_local_capacity: anon_33 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_32 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub fn copy_from(__str: Ptr<basic_string_31>) -> Self {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub fn new_1(mut __s: Ptr<i32>, mut __n: u64, __a: Option<Ptr<allocator_9>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub fn move_from(__str: Ptr<basic_string_31>) -> Self {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub fn new_2(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_9>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_34 {}
impl Clone for basic_string_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_basic_string_char16_t___Alloc_hider {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<u16>,
}
impl Clone for std_basic_string_char16_t___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_char16_t___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {
    pub fn new_1(mut __dat: Ptr<u16>, __a: Ptr<allocator_10>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {}
impl std_basic_string_char16_t___Alloc_hider {}
impl std_basic_string_char16_t___Alloc_hider {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std_basic_string_char16_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_char16_t___Alloc_hider> {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {
    pub fn move_assign(
        &self,
        _a0: Ptr<std_basic_string_char16_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_char16_t___Alloc_hider> {
        unimplemented!()
    }
}
pub type anon_36 = u32;
pub const anon_36__S_local_capacity: anon_36 = 7;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_35 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u16]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_34 {
    pub fn copy_from(__str: Ptr<basic_string_34>) -> Self {
        unimplemented!()
    }
}
impl basic_string_34 {
    pub fn new_1(mut __s: Ptr<u16>, mut __n: u64, __a: Option<Ptr<allocator_10>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_34 {
    pub fn move_from(__str: Ptr<basic_string_34>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_37 {}
impl Clone for basic_string_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_basic_string_char32_t___Alloc_hider {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<u32>,
}
impl Clone for std_basic_string_char32_t___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_char32_t___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {
    pub fn new_1(mut __dat: Ptr<u32>, __a: Ptr<allocator_11>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {}
impl std_basic_string_char32_t___Alloc_hider {}
impl std_basic_string_char32_t___Alloc_hider {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std_basic_string_char32_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_char32_t___Alloc_hider> {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {
    pub fn move_assign(
        &self,
        _a0: Ptr<std_basic_string_char32_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_char32_t___Alloc_hider> {
        unimplemented!()
    }
}
pub type anon_39 = u32;
pub const anon_39__S_local_capacity: anon_39 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_38 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_37 {
    pub fn copy_from(__str: Ptr<basic_string_37>) -> Self {
        unimplemented!()
    }
}
impl basic_string_37 {
    pub fn new_1(mut __s: Ptr<u32>, mut __n: u64, __a: Option<Ptr<allocator_11>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_37 {
    pub fn move_from(__str: Ptr<basic_string_37>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __mbstate_t {
    #[offset(0)]
    pub __count: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub __value: anon_40,
}
impl Clone for __mbstate_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct anon_40 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_41 {}
impl Clone for integral_constant_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_42: Value<bool> = unimplemented!();
);
impl integral_constant_41 {}
impl integral_constant_41 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_43 {}
impl Clone for integral_constant_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__false__value_44: Value<bool> = unimplemented!();
);
impl integral_constant_43 {}
impl integral_constant_43 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_45 {}
impl Clone for integral_constant_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_46 {}
impl Clone for integral_constant_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_47 {}
impl Clone for enable_if_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_48 {}
impl Clone for enable_if_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_49 {}
impl Clone for enable_if_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_50 {}
impl Clone for enable_if_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_51 {}
impl Clone for enable_if_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_52 {}
impl Clone for enable_if_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_53 {}
impl Clone for enable_if_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_54 {}
impl Clone for __type_identity_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_55 {}
impl Clone for __type_identity_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_56 {}
impl Clone for __type_identity_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_57 {}
impl Clone for __type_identity_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_58 {}
impl Clone for __type_identity_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_59 {}
impl Clone for __type_identity_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_60 {}
impl Clone for __type_identity_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__61 {}
impl Clone for __or__61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__62 {}
impl Clone for __or__62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__63 {}
impl Clone for __or__63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__64 {}
impl Clone for __or__64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__65 {}
impl Clone for __or__65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__66 {}
impl Clone for __or__66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__67 {}
impl Clone for __or__67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__68 {}
impl Clone for __or__68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__69 {}
impl Clone for __or__69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__70 {}
impl Clone for __or__70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__71 {}
impl Clone for __or__71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__72 {}
impl Clone for __or__72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__73 {}
impl Clone for __or__73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__74 {}
impl Clone for __and__74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__75 {}
impl Clone for __and__75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__76 {}
impl Clone for __and__76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__77 {}
impl Clone for __and__77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__78 {}
impl Clone for __and__78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__79 {}
impl Clone for __and__79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__80 {}
impl Clone for __not__80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__81 {}
impl Clone for __not__81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__82 {}
impl Clone for __not__82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__83 {}
impl Clone for __not__83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__84 {}
impl Clone for __not__84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__85 {}
impl Clone for __not__85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_86 {}
impl Clone for is_reference_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_87 {}
impl Clone for is_reference_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_88 {}
impl Clone for is_reference_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_89 {}
impl Clone for is_reference_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_90 {}
impl Clone for is_reference_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_91 {}
impl Clone for is_reference_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_92 {}
impl Clone for is_reference_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_93 {}
impl Clone for is_function_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_94 {}
impl Clone for is_function_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_95 {}
impl Clone for is_function_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_96 {}
impl Clone for is_function_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_97 {}
impl Clone for is_function_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_98 {}
impl Clone for is_function_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_99 {}
impl Clone for is_function_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_100 {}
impl Clone for is_void_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_101 {}
impl Clone for is_void_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_102 {}
impl Clone for is_void_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_103 {}
impl Clone for is_void_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_104 {}
impl Clone for is_void_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_105 {}
impl Clone for is_void_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_106 {}
impl Clone for is_void_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_107 {}
impl Clone for is_void_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_108 {}
impl Clone for is_void_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_109 {}
impl Clone for is_void_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_110 {}
impl Clone for is_void_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_111 {}
impl Clone for is_void_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_112 {}
impl Clone for is_void_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_113 {}
impl Clone for is_void_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_114 {}
impl Clone for is_void_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_115 {}
impl Clone for is_void_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_116 {}
impl Clone for remove_cv_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_117 {}
impl Clone for remove_cv_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_118 {}
impl Clone for remove_cv_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_119 {}
impl Clone for remove_cv_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_120 {}
impl Clone for remove_cv_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_121 {}
impl Clone for is_const_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_122 {}
impl Clone for is_const_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_123 {}
impl Clone for is_const_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_124 {}
impl Clone for is_const_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_125 {}
impl Clone for is_const_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_126 {}
impl Clone for __is_array_unknown_bounds_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_127 {}
impl Clone for __is_array_unknown_bounds_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_128 {}
impl Clone for __is_array_unknown_bounds_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_129 {}
impl Clone for __is_array_unknown_bounds_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_130 {}
impl Clone for __is_array_unknown_bounds_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_131 {}
impl Clone for __is_array_unknown_bounds_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_132 {}
impl Clone for __is_array_unknown_bounds_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_133 {}
impl Clone for __is_integral_helper_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_134 {}
impl Clone for __is_integral_helper_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_135 {}
impl Clone for __is_integral_helper_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_136 {}
impl Clone for __is_integral_helper_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_137 {}
impl Clone for __is_integral_helper_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_138 {}
impl Clone for is_integral_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_139 {}
impl Clone for is_integral_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_140 {}
impl Clone for is_integral_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_141 {}
impl Clone for is_integral_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_142 {}
impl Clone for is_integral_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_143 {}
impl Clone for is_same_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_144 {}
impl Clone for is_same_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_145 {}
impl Clone for is_same_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_146 {}
impl Clone for is_same_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_147 {}
impl Clone for is_same_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_148 {}
impl Clone for is_same_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_149 {}
impl Clone for is_same_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_150 {}
impl Clone for is_same_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_151 {}
impl Clone for is_same_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_152 {}
impl Clone for is_same_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_153 {}
impl Clone for is_same_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_154 {}
impl Clone for is_same_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_155 {}
impl Clone for is_same_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_156 {}
impl Clone for is_same_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_157 {}
impl Clone for is_same_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_158 {}
impl Clone for is_same_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_159 {}
impl Clone for is_same_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_160 {}
impl Clone for is_same_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_161 {}
impl Clone for is_same_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_162 {}
impl Clone for is_volatile_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_163 {}
impl Clone for is_volatile_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_164 {}
impl Clone for is_volatile_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_165 {}
impl Clone for is_volatile_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_166 {}
impl Clone for is_volatile_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___do_is_destructible_impl {}
impl Clone for std___do_is_destructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_destructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___do_is_nt_destructible_impl {}
impl Clone for std___do_is_nt_destructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_nt_destructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_167 {}
impl Clone for is_nothrow_default_constructible_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_168 {}
impl Clone for is_nothrow_default_constructible_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_169 {}
impl Clone for is_nothrow_copy_constructible_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_170 {}
impl Clone for is_nothrow_copy_constructible_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_171 {}
impl Clone for is_nothrow_copy_constructible_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_172 {}
impl Clone for is_nothrow_copy_constructible_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_173 {}
impl Clone for is_nothrow_copy_constructible_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___do_is_implicitly_default_constructible_impl {}
impl Clone for std___do_is_implicitly_default_constructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_implicitly_default_constructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_174 {}
impl Clone for is_convertible_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_175 {}
impl Clone for is_convertible_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_176 {}
impl Clone for is_convertible_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_177 {}
impl Clone for is_convertible_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_178 {}
impl Clone for is_convertible_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_179 {}
impl Clone for is_convertible_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_180 {}
impl Clone for is_convertible_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_181 {}
impl Clone for is_convertible_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_182 {}
impl Clone for is_convertible_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_183 {}
impl Clone for is_convertible_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_184 {}
impl Clone for is_convertible_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_185 {}
impl Clone for is_convertible_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_186 {}
impl Clone for is_convertible_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_187 {}
impl Clone for remove_reference_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_188 {}
impl Clone for remove_reference_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_189 {}
impl Clone for remove_reference_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_190 {}
impl Clone for remove_reference_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_191 {}
impl Clone for remove_reference_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_192 {}
impl Clone for remove_reference_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_193 {}
impl Clone for remove_reference_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_194 {}
impl Clone for __cv_selector_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_195 {}
impl Clone for __cv_selector_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_196 {}
impl Clone for __cv_selector_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_197 {}
impl Clone for __cv_selector_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_198 {}
impl Clone for __match_cv_qualifiers_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_199 {}
impl Clone for __match_cv_qualifiers_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_200 {}
impl Clone for __match_cv_qualifiers_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_201 {}
impl Clone for __match_cv_qualifiers_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_202 {}
impl Clone for __match_cv_qualifiers_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_203 {}
impl Clone for __make_unsigned_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_204 {}
impl Clone for __make_unsigned_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_205 {}
impl Clone for __make_unsigned_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_206 {}
impl Clone for __make_unsigned_selector_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_207 {}
impl Clone for __make_unsigned_selector_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_208 {}
impl Clone for __make_unsigned_selector_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_209 {}
impl Clone for __make_unsigned_selector_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_210 {}
impl Clone for __make_unsigned_selector_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_211 {}
impl Clone for __make_unsigned_selector_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___make_unsigned_selector_base {}
impl Clone for std___make_unsigned_selector_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___make_unsigned_selector_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_215 {}
impl Clone for _List_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_216 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_217 {}
impl Clone for _List_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_218 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_219 {}
impl Clone for _List_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_220 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_221 {}
impl Clone for _List_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_222 {}
impl Clone for _List_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_223 {}
impl Clone for _List_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_224 {}
impl Clone for __select_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_225 {}
impl Clone for __select_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_226 {}
impl Clone for __select_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_227 {}
impl Clone for __select_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_228 {}
impl Clone for __select_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_229 {}
impl Clone for __make_signed_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_230 {}
impl Clone for __make_signed_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_231 {}
impl Clone for __make_signed_selector_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_232 {}
impl Clone for __make_signed_selector_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_233 {}
impl Clone for __make_signed_selector_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_234 {}
impl Clone for __make_signed_selector_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_235 {}
impl Clone for __make_signed_selector_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std___aligned_storage_max_align_t {}
impl Clone for std___aligned_storage_max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___aligned_storage_max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct common_type_236 {}
impl Clone for common_type_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___failure_type {}
impl Clone for std___failure_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___failure_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___do_common_type_impl {}
impl Clone for std___do_common_type_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_common_type_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___invoke_memfun_ref {}
impl Clone for std___invoke_memfun_ref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memfun_ref {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___invoke_memfun_deref {}
impl Clone for std___invoke_memfun_deref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memfun_deref {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___invoke_memobj_ref {}
impl Clone for std___invoke_memobj_ref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memobj_ref {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___invoke_memobj_deref {}
impl Clone for std___invoke_memobj_deref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memobj_deref {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___invoke_other {}
impl Clone for std___invoke_other {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_other {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___result_of_memfun_ref_impl {}
impl Clone for std___result_of_memfun_ref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memfun_ref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___result_of_memfun_deref_impl {}
impl Clone for std___result_of_memfun_deref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memfun_deref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___result_of_memobj_ref_impl {}
impl Clone for std___result_of_memobj_ref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memobj_ref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___result_of_memobj_deref_impl {}
impl Clone for std___result_of_memobj_deref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memobj_deref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___result_of_other_impl {}
impl Clone for std___result_of_other_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_other_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___swappable_details___do_is_swappable_impl {}
impl Clone for std___swappable_details___do_is_swappable_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_details___do_is_swappable_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___swappable_details___do_is_nothrow_swappable_impl {}
impl Clone for std___swappable_details___do_is_nothrow_swappable_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_details___do_is_nothrow_swappable_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___swappable_with_details___do_is_swappable_with_impl {}
impl Clone for std___swappable_with_details___do_is_swappable_with_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_with_details___do_is_swappable_with_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___swappable_with_details___do_is_nothrow_swappable_with_impl {}
impl Clone for std___swappable_with_details___do_is_nothrow_swappable_with_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_with_details___do_is_nothrow_swappable_with_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___nonesuchbase {}
impl Clone for std___nonesuchbase {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___nonesuchbase {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___nonesuchbase {}
impl std___nonesuchbase {}
impl std___nonesuchbase {
    pub fn copy_assign(&self, _a0: Ptr<std___nonesuchbase>) -> Ptr<std___nonesuchbase> {
        unimplemented!()
    }
}
impl std___nonesuchbase {
    pub fn move_assign(&self, _a0: Ptr<std___nonesuchbase>) -> Ptr<std___nonesuchbase> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___nonesuch {}
impl Clone for std___nonesuch {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___nonesuch {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static is_array_v_237: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_238: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_239: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_240: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_241: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_242: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_243: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_244: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_245: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_246: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_247: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_248: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_249: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_250: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_251: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_252: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_253: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_254: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_255: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_256: Value<bool> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_257 {}
impl Clone for _Char_types_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_258 {}
impl Clone for _Char_types_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_259 {}
impl Clone for char_traits_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_259 {
    pub fn eq(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_259 {
    pub fn length(mut __p: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_259 {
    pub fn find(mut __s: Ptr<i8>, mut __n: usize, __a: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_260 {}
impl Clone for char_traits_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_260 {
    pub fn eq(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_260 {
    pub fn lt(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_260 {
    pub fn compare(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_260 {
    pub fn length(mut __p: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl char_traits_260 {
    pub fn find(mut __s: Ptr<i32>, mut __n: usize, __a: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_exception {}
impl Clone for std_exception {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_exception {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn copy_from(_a0: Ptr<std_exception>) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn copy_assign(&self, _a0: Ptr<std_exception>) -> Ptr<std_exception> {
        unimplemented!()
    }
}
impl std_exception {
    pub fn move_from(_a0: Ptr<std_exception>) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn move_assign(&self, _a0: Ptr<std_exception>) -> Ptr<std_exception> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_bad_alloc {}
impl Clone for std_bad_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bad_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub fn copy_from(_a0: Ptr<std_bad_alloc>) -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub fn copy_assign(&self, _a0: Ptr<std_bad_alloc>) -> Ptr<std_bad_alloc> {
        unimplemented!()
    }
}
pub type std_align_val_t = usize;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_nothrow_t {}
impl Clone for std_nothrow_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_nothrow_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_nothrow_t {}
impl std_nothrow_t {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_261 {}
impl Clone for __new_allocator_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_261 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_261 {
    pub fn copy_from(_a0: Ptr<__new_allocator_261>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_261 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_261>) -> Ptr<__new_allocator_261> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_262 {}
impl Clone for __new_allocator_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_262 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_262 {
    pub fn copy_from(_a0: Ptr<__new_allocator_262>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_262 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_262>) -> Ptr<__new_allocator_262> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_263 {}
impl Clone for __new_allocator_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_263 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_263 {
    pub fn copy_from(_a0: Ptr<__new_allocator_263>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_263 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_263>) -> Ptr<__new_allocator_263> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_264 {}
impl Clone for __new_allocator_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_264 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_264 {
    pub fn copy_from(_a0: Ptr<__new_allocator_264>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_264 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_264>) -> Ptr<__new_allocator_264> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___true_type {}
impl Clone for std___true_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___true_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___false_type {}
impl Clone for std___false_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___false_type {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___false_type {}
impl std___false_type {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_265 {}
impl Clone for __are_same_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_266 = u32;
pub const anon_266___value: anon_266 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_267 {}
impl Clone for __are_same_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_268 = u32;
pub const anon_268___value: anon_268 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_269 {}
impl Clone for __are_same_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_270 = u32;
pub const anon_270___value: anon_270 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_271 {}
impl Clone for __are_same_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_272 = u32;
pub const anon_272___value: anon_272 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_273 {}
impl Clone for __are_same_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_274 = u32;
pub const anon_274___value: anon_274 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_275 {}
impl Clone for __are_same_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_276 = u32;
pub const anon_276___value: anon_276 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_277 {}
impl Clone for __is_integer_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_278 = u32;
pub const anon_278___value: anon_278 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_279 {}
impl Clone for __is_integer_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_280 = u32;
pub const anon_280___value: anon_280 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_281 {}
impl Clone for __is_integer_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_282 = u32;
pub const anon_282___value: anon_282 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_283 {}
impl Clone for __is_integer_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_284 = u32;
pub const anon_284___value: anon_284 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_285 {}
impl Clone for __is_integer_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_286 = u32;
pub const anon_286___value: anon_286 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_287 {}
impl Clone for __is_integer_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_288 = u32;
pub const anon_288___value: anon_288 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_289 {}
impl Clone for __is_integer_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_290 = u32;
pub const anon_290___value: anon_290 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_291 {}
impl Clone for __is_integer_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_292 = u32;
pub const anon_292___value: anon_292 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_293 {}
impl Clone for __memcpyable_integer_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(96)]
pub struct lconv {
    #[offset(0)]
    #[byte_size(8)]
    pub decimal_point: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub thousands_sep: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub grouping: Ptr<i8>,
    #[offset(24)]
    #[byte_size(8)]
    pub int_curr_symbol: Ptr<i8>,
    #[offset(32)]
    #[byte_size(8)]
    pub currency_symbol: Ptr<i8>,
    #[offset(40)]
    #[byte_size(8)]
    pub mon_decimal_point: Ptr<i8>,
    #[offset(48)]
    #[byte_size(8)]
    pub mon_thousands_sep: Ptr<i8>,
    #[offset(56)]
    #[byte_size(8)]
    pub mon_grouping: Ptr<i8>,
    #[offset(64)]
    #[byte_size(8)]
    pub positive_sign: Ptr<i8>,
    #[offset(72)]
    #[byte_size(8)]
    pub negative_sign: Ptr<i8>,
    #[offset(80)]
    pub int_frac_digits: i8,
    #[offset(81)]
    pub frac_digits: i8,
    #[offset(82)]
    pub p_cs_precedes: i8,
    #[offset(83)]
    pub p_sep_by_space: i8,
    #[offset(84)]
    pub n_cs_precedes: i8,
    #[offset(85)]
    pub n_sep_by_space: i8,
    #[offset(86)]
    pub p_sign_posn: i8,
    #[offset(87)]
    pub n_sign_posn: i8,
    #[offset(88)]
    pub int_p_cs_precedes: i8,
    #[offset(89)]
    pub int_p_sep_by_space: i8,
    #[offset(90)]
    pub int_n_cs_precedes: i8,
    #[offset(91)]
    pub int_n_sep_by_space: i8,
    #[offset(92)]
    pub int_p_sign_posn: i8,
    #[offset(93)]
    pub int_n_sign_posn: i8,
}
impl Clone for lconv {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for lconv {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_input_iterator_tag {}
impl Clone for std_input_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_input_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_input_iterator_tag {}
impl std_input_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_output_iterator_tag {}
impl Clone for std_output_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_output_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_output_iterator_tag {}
impl std_output_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_forward_iterator_tag {}
impl Clone for std_forward_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_forward_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_forward_iterator_tag {}
impl std_forward_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_bidirectional_iterator_tag {}
impl Clone for std_bidirectional_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bidirectional_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bidirectional_iterator_tag {}
impl std_bidirectional_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_random_access_iterator_tag {}
impl Clone for std_random_access_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_random_access_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_random_access_iterator_tag {}
impl std_random_access_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_294 {}
impl Clone for iterator_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_295 {}
impl Clone for iterator_traits_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_296 {}
impl Clone for iterator_traits_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_297 {}
impl Clone for iterator_traits_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_298 {}
impl Clone for iterator_traits_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_299 {}
impl Clone for iterator_traits_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_300 {}
impl Clone for __ptr_traits_ptr_to_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_300 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_301 {}
impl Clone for __ptr_traits_ptr_to_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_301 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_302 {}
impl Clone for __ptr_traits_ptr_to_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_302 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_303 {}
impl Clone for __ptr_traits_ptr_to_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_303 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_304 {}
impl Clone for __ptr_traits_ptr_to_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_304 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_305 {}
impl Clone for __ptr_traits_ptr_to_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_305 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_306 {}
impl Clone for __ptr_traits_ptr_to_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_306 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_307 {}
impl Clone for __ptr_traits_ptr_to_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_307 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_308 {}
impl Clone for pointer_traits_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_309 {}
impl Clone for pointer_traits_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_310 {}
impl Clone for pointer_traits_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_311 {}
impl Clone for pointer_traits_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_312 {}
impl Clone for pointer_traits_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_313 {}
impl Clone for pointer_traits_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_314 {}
impl Clone for pointer_traits_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_315 {}
impl Clone for pointer_traits_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_316 {}
impl Clone for binary_function_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_317 {}
impl Clone for greater_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_318 {}
impl Clone for less_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_319 {}
impl Clone for greater_equal_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_320 {}
impl Clone for less_equal_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_321 {}
impl Clone for __is_integer_nonstrict_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_322 = u32;
pub const anon_322___width: anon_322 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_323 {}
impl Clone for __is_integer_nonstrict_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_324 = u32;
pub const anon_324___width: anon_324 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_325 {}
impl Clone for __is_integer_nonstrict_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_326 = u32;
pub const anon_326___width: anon_326 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_327 {}
impl Clone for __is_integer_nonstrict_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_328 = u32;
pub const anon_328___width: anon_328 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_329 {}
impl Clone for __is_integer_nonstrict_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_330 = u32;
pub const anon_330___width: anon_330 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_331 {}
impl Clone for __numeric_traits_integer_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____is_signed_332: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____digits_333: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____max_334: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____min_335: Value<i32> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_336 {}
impl Clone for __numeric_traits_integer_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_337: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_338: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_339: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_340 {}
impl Clone for __numeric_traits_integer_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_341: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____digits_342: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____max_343: Value<u32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_344 {}
impl Clone for __numeric_traits_integer_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_345: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_346: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_347: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_348 {}
impl Clone for __numeric_traits_integer_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_349: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_350: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_351: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_352 {}
impl Clone for __numeric_traits_floating_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_float____max_exponent10_353: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_354 {}
impl Clone for __numeric_traits_floating_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_double____max_exponent10_355: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_356 {}
impl Clone for __numeric_traits_floating_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_357: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_358 {}
impl Clone for __numeric_traits_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_359 {}
impl Clone for __numeric_traits_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_360 {}
impl Clone for __numeric_traits_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_361 {}
impl Clone for __numeric_traits_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_362 {}
impl Clone for __numeric_traits_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_in_place_t {}
impl Clone for std_in_place_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_in_place_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_in_place_t {}
impl std_in_place_t {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std__Swallow_assign {}
impl Clone for std__Swallow_assign {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Swallow_assign {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_piecewise_construct_t {}
impl Clone for std_piecewise_construct_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_piecewise_construct_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_piecewise_construct_t {}
impl std_piecewise_construct_t {}
thread_local!(
    pub static piecewise_construct_363: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_364 {}
impl Clone for tuple_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_364 {}
impl tuple_364 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __gnu_cxx___ops__Iter_less_iter {}
impl Clone for __gnu_cxx___ops__Iter_less_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_less_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_iter {}
impl __gnu_cxx___ops__Iter_less_iter {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __gnu_cxx___ops__Iter_less_val {}
impl Clone for __gnu_cxx___ops__Iter_less_val {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_less_val {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_val {}
impl __gnu_cxx___ops__Iter_less_val {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __gnu_cxx___ops__Val_less_iter {}
impl Clone for __gnu_cxx___ops__Val_less_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Val_less_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Val_less_iter {}
impl __gnu_cxx___ops__Val_less_iter {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __gnu_cxx___ops__Iter_equal_to_iter {}
impl Clone for __gnu_cxx___ops__Iter_equal_to_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_equal_to_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_equal_to_iter {}
impl __gnu_cxx___ops__Iter_equal_to_iter {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __gnu_cxx___ops__Iter_equal_to_val {}
impl Clone for __gnu_cxx___ops__Iter_equal_to_val {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_equal_to_val {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_equal_to_val {}
impl __gnu_cxx___ops__Iter_equal_to_val {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_365 {}
impl Clone for initializer_list_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_365 {}
impl initializer_list_365 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_366 {}
impl Clone for initializer_list_366 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_366 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_366 {}
impl initializer_list_366 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_367 {}
impl Clone for initializer_list_367 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_367 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_367 {}
impl initializer_list_367 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_368 {}
impl Clone for initializer_list_368 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_368 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_368 {}
impl initializer_list_368 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___allocator_traits_base {}
impl Clone for std___allocator_traits_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___allocator_traits_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_370 {}
impl Clone for __is_allocator_370 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_370 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_371 {}
impl Clone for __is_allocator_371 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_371 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_372 {}
impl Clone for __is_allocator_372 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_372 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_373 {}
impl Clone for __is_allocator_373 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_373 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_374 {}
impl Clone for __alloc_traits_374 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_374 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_375 {}
impl Clone for rebind_375 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_375 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_374 {
    pub fn _S_select_on_copy(__a: Ptr<allocator_8>) -> allocator_8 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_376 {}
impl Clone for __alloc_traits_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_377 {}
impl Clone for rebind_377 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_377 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_378 {}
impl Clone for __alloc_traits_378 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_378 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_379 {}
impl Clone for rebind_379 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_379 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_380 {}
impl Clone for __alloc_traits_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_380 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_381 {}
impl Clone for rebind_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_382 {}
impl Clone for __hash_base_382 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_382 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_383 {}
impl Clone for __hash_base_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_384 {}
impl Clone for __hash_base_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_384 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_385 {}
impl Clone for __hash_base_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_386 {}
impl Clone for __hash_base_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_387 {}
impl Clone for __hash_base_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_388 {}
impl Clone for __hash_base_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_389 {}
impl Clone for __hash_base_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_390 {}
impl Clone for __hash_base_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_391 {}
impl Clone for __hash_base_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_392 {}
impl Clone for __hash_base_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_393 {}
impl Clone for __hash_base_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_394 {}
impl Clone for __hash_base_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_395 {}
impl Clone for __hash_base_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_396 {}
impl Clone for __hash_base_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_397 {}
impl Clone for __hash_base_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_398 {}
impl Clone for __hash_base_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_399 {}
impl Clone for __hash_base_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_400 {}
impl Clone for __hash_base_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_401 {}
impl Clone for __hash_base_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_401 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_402 {}
impl Clone for __hash_base_402 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_402 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_403 {}
impl Clone for __hash_base_403 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_403 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_404 {}
impl Clone for __hash_base_404 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_404 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_405 {}
impl Clone for __hash_base_405 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_405 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_406 {}
impl Clone for __hash_base_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_407 {}
impl Clone for hash_407 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_407 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_408 {}
impl Clone for hash_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_408 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_409 {}
impl Clone for hash_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_410 {}
impl Clone for hash_410 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_410 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_411 {}
impl Clone for hash_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std__Hash_impl {}
impl Clone for std__Hash_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Hash_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash(mut __ptr: AnyPtr, mut __clength: usize, __seed: Option<usize>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_412(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_413(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_24 {}
impl Clone for basic_string_view_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_24 {
    pub fn copy_from(_a0: Ptr<basic_string_view_24>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_24 {
    pub fn new_1(mut __str: Ptr<i8>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_24 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_24>) -> Ptr<basic_string_view_24> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_414 {}
impl Clone for basic_string_view_414 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_414 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_414 {
    pub fn copy_from(_a0: Ptr<basic_string_view_414>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_414 {
    pub fn new_1(mut __str: Ptr<i32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_414 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_414>) -> Ptr<basic_string_view_414> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_415 {}
impl Clone for basic_string_view_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_415 {
    pub fn copy_from(_a0: Ptr<basic_string_view_415>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_415 {
    pub fn new_1(mut __str: Ptr<u16>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_415 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_415>) -> Ptr<basic_string_view_415> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_416 {}
impl Clone for basic_string_view_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_416 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_416 {
    pub fn copy_from(_a0: Ptr<basic_string_view_416>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_416 {
    pub fn new_1(mut __str: Ptr<u32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_416 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_416>) -> Ptr<basic_string_view_416> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct div_t {
    #[offset(0)]
    pub quot: i32,
    #[offset(4)]
    pub rem: i32,
}
impl Clone for div_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for div_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct ldiv_t {
    #[offset(0)]
    pub quot: i64,
    #[offset(8)]
    pub rem: i64,
}
impl Clone for ldiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ldiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct lldiv_t {
    #[offset(0)]
    pub quot: i64,
    #[offset(8)]
    pub rem: i64,
}
impl Clone for lldiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for lldiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl lldiv_t {}
impl lldiv_t {}
impl lldiv_t {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct __sigset_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __val: Value<Box<[u64]>>,
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __sigset_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timeval {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_usec: i64,
}
impl Clone for timeval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timeval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timespec {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_nsec: i64,
}
impl Clone for timespec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timespec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct fd_set {
    #[offset(0)]
    #[byte_size(128)]
    pub fds_bits: Value<Box<[i64]>>,
}
impl Clone for fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __atomic_wide_counter {
    #[offset(0)]
    pub __value64: u64,
    #[offset(0)]
    #[byte_size(8)]
    pub __value32: anon_417,
}
impl Clone for __atomic_wide_counter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __atomic_wide_counter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_417 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct __pthread_internal_list {
    #[offset(0)]
    #[byte_size(8)]
    pub __prev: Ptr<__pthread_internal_list>,
    #[offset(8)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_list>,
}
impl Clone for __pthread_internal_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __pthread_internal_slist {
    #[offset(0)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_slist>,
}
impl Clone for __pthread_internal_slist {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_slist {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct __pthread_mutex_s {
    #[offset(0)]
    pub __lock: i32,
    #[offset(4)]
    pub __count: u32,
    #[offset(8)]
    pub __owner: i32,
    #[offset(12)]
    pub __nusers: u32,
    #[offset(16)]
    pub __kind: i32,
    #[offset(20)]
    pub __spins: i16,
    #[offset(22)]
    pub __elision: i16,
    #[offset(24)]
    #[byte_size(16)]
    pub __list: __pthread_internal_list,
}
impl Clone for __pthread_mutex_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct __pthread_rwlock_arch_t {
    #[offset(0)]
    pub __readers: u32,
    #[offset(4)]
    pub __writers: u32,
    #[offset(8)]
    pub __wrphase_futex: u32,
    #[offset(12)]
    pub __writers_futex: u32,
    #[offset(16)]
    pub __pad3: u32,
    #[offset(20)]
    pub __pad4: u32,
    #[offset(24)]
    pub __cur_writer: i32,
    #[offset(28)]
    pub __shared: i32,
    #[offset(32)]
    pub __rwelision: i8,
    #[offset(33)]
    #[byte_size(7)]
    pub __pad1: Value<Box<[u8]>>,
    #[offset(40)]
    pub __pad2: u64,
    #[offset(48)]
    pub __flags: u32,
}
impl Clone for __pthread_rwlock_arch_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_rwlock_arch_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct __pthread_cond_s {
    #[offset(0)]
    #[byte_size(8)]
    pub __wseq: __atomic_wide_counter,
    #[offset(8)]
    #[byte_size(8)]
    pub __g1_start: __atomic_wide_counter,
    #[offset(16)]
    #[byte_size(8)]
    pub __g_refs: Value<Box<[u32]>>,
    #[offset(24)]
    #[byte_size(8)]
    pub __g_size: Value<Box<[u32]>>,
    #[offset(32)]
    pub __g1_orig_size: u32,
    #[offset(36)]
    pub __wrefs: u32,
    #[offset(40)]
    #[byte_size(8)]
    pub __g_signals: Value<Box<[u32]>>,
}
impl Clone for __pthread_cond_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct __once_flag {
    #[offset(0)]
    pub __data: i32,
}
impl Clone for __once_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __once_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_mutexattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_mutexattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_condattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_condattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_attr_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_attr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct pthread_mutex_t {
    #[offset(0)]
    #[byte_size(40)]
    pub __data: __pthread_mutex_s,
    #[offset(0)]
    #[byte_size(40)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_mutex_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct pthread_cond_t {
    #[offset(0)]
    #[byte_size(48)]
    pub __data: __pthread_cond_s,
    #[offset(0)]
    #[byte_size(48)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_cond_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_cond_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_rwlock_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __data: __pthread_rwlock_arch_t,
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_rwlock_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct pthread_rwlockattr_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_rwlockattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct pthread_barrier_t {
    #[offset(0)]
    #[byte_size(32)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_barrier_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_barrierattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_barrierattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct random_data {
    #[offset(0)]
    #[byte_size(8)]
    pub fptr: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub rptr: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub state: Ptr<i32>,
    #[offset(24)]
    pub rand_type: i32,
    #[offset(28)]
    pub rand_deg: i32,
    #[offset(32)]
    pub rand_sep: i32,
    #[offset(40)]
    #[byte_size(8)]
    pub end_ptr: Ptr<i32>,
}
impl Clone for random_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for random_data {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct drand48_data {
    #[offset(0)]
    #[byte_size(6)]
    pub __x: Value<Box<[u16]>>,
    #[offset(6)]
    #[byte_size(6)]
    pub __old_x: Value<Box<[u16]>>,
    #[offset(12)]
    pub __c: u16,
    #[offset(14)]
    pub __init: u16,
    #[offset(16)]
    pub __a: u64,
}
impl Clone for drand48_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for drand48_data {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _G_fpos_t {
    #[offset(0)]
    pub __pos: i64,
    #[offset(8)]
    #[byte_size(8)]
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _G_fpos64_t {
    #[offset(0)]
    pub __pos: i64,
    #[offset(8)]
    #[byte_size(8)]
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos64_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos64_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(216)]
pub struct _IO_FILE {
    #[offset(0)]
    pub _flags: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub _IO_read_ptr: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub _IO_read_end: Ptr<i8>,
    #[offset(24)]
    #[byte_size(8)]
    pub _IO_read_base: Ptr<i8>,
    #[offset(32)]
    #[byte_size(8)]
    pub _IO_write_base: Ptr<i8>,
    #[offset(40)]
    #[byte_size(8)]
    pub _IO_write_ptr: Ptr<i8>,
    #[offset(48)]
    #[byte_size(8)]
    pub _IO_write_end: Ptr<i8>,
    #[offset(56)]
    #[byte_size(8)]
    pub _IO_buf_base: Ptr<i8>,
    #[offset(64)]
    #[byte_size(8)]
    pub _IO_buf_end: Ptr<i8>,
    #[offset(72)]
    #[byte_size(8)]
    pub _IO_save_base: Ptr<i8>,
    #[offset(80)]
    #[byte_size(8)]
    pub _IO_backup_base: Ptr<i8>,
    #[offset(88)]
    #[byte_size(8)]
    pub _IO_save_end: Ptr<i8>,
    #[offset(96)]
    #[byte_size(8)]
    pub _markers: Ptr<_IO_marker>,
    #[offset(104)]
    #[byte_size(8)]
    pub _chain: Ptr<_IO_FILE>,
    #[offset(112)]
    pub _fileno: i32,
    #[offset(116)]
    pub _flags2: i32,
    #[offset(120)]
    pub _old_offset: i64,
    #[offset(128)]
    pub _cur_column: u16,
    #[offset(130)]
    pub _vtable_offset: i8,
    #[offset(131)]
    #[byte_size(1)]
    pub _shortbuf: Value<Box<[i8]>>,
    #[offset(136)]
    #[byte_size(8)]
    pub _lock: AnyPtr,
    #[offset(144)]
    pub _offset: i64,
    #[offset(152)]
    #[byte_size(8)]
    pub _codecvt: Ptr<_IO_codecvt>,
    #[offset(160)]
    #[byte_size(8)]
    pub _wide_data: Ptr<_IO_wide_data>,
    #[offset(168)]
    #[byte_size(8)]
    pub _freeres_list: Ptr<_IO_FILE>,
    #[offset(176)]
    #[byte_size(8)]
    pub _freeres_buf: AnyPtr,
    #[offset(184)]
    pub __pad5: usize,
    #[offset(192)]
    pub _mode: i32,
    #[offset(196)]
    #[byte_size(20)]
    pub _unused2: Value<Box<[i8]>>,
}
impl Clone for _IO_FILE {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_FILE {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct _IO_cookie_io_functions_t {
    #[offset(0)]
    #[byte_size(8)]
    pub read: FnPtr<fn(AnyPtr, Ptr<i8>, usize) -> i64>,
    #[offset(8)]
    #[byte_size(8)]
    pub write: FnPtr<fn(AnyPtr, Ptr<i8>, usize) -> i64>,
    #[offset(16)]
    #[byte_size(8)]
    pub seek: FnPtr<fn(AnyPtr, Ptr<i64>, i32) -> i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub close: FnPtr<fn(AnyPtr) -> i32>,
}
impl Clone for _IO_cookie_io_functions_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_cookie_io_functions_t {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __integer_to_chars_is_unsigned_418: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_419: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_420: Value<bool> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct max_align_t {
    #[offset(0)]
    pub __clang_max_align_nonce1: i64,
    #[offset(16)]
    #[byte_size(16)]
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___erased_type {}
impl Clone for std___erased_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___erased_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_allocator_arg_t {}
impl Clone for std_allocator_arg_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_allocator_arg_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_allocator_arg_t {}
impl std_allocator_arg_t {}
thread_local!(
    pub static allocator_arg_421: Value<std_allocator_arg_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___uses_alloc_base {}
impl Clone for std___uses_alloc_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___uses_alloc_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___uses_alloc0 {
    #[offset(0)]
    #[byte_size(1)]
    pub _M_a: std___uses_alloc0__Sink,
}
impl Clone for std___uses_alloc0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___uses_alloc0 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let empty: Value<basic_string_20> = Rc::new(RefCell::new(basic_string_20::new_1()));
    (if (({ basic_string_20Impl::length(&empty.as_pointer()) }) == 0_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"empty.length() == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let s1: Value<basic_string_20> = Rc::new(RefCell::new(basic_string_20::new_5(
        { Ptr::<i8>::from_string_literal(b"hello") },
        None,
    )));
    (if (({ basic_string_20Impl::length(&s1.as_pointer()) }) == 5_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s1.length() == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ basic_string_20Impl::size(&s1.as_pointer()) })
        == ({ basic_string_20Impl::length(&s1.as_pointer()) }))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s1.size() == s1.length()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&s1.as_pointer(), 0_u64) }).read()) as i32)
        == (('h' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s1[0] == \'h\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&s1.as_pointer(), 1_u64) }).read()) as i32)
        == (('e' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s1[1] == \'e\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&s1.as_pointer(), 2_u64) }).read()) as i32)
        == (('l' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s1[2] == \'l\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&s1.as_pointer(), 3_u64) }).read()) as i32)
        == (('l' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s1[3] == \'l\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&s1.as_pointer(), 4_u64) }).read()) as i32)
        == (('o' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s1[4] == \'o\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let ___lhs: Ptr<basic_string_20> = s1.as_pointer();
        operator_eq_423(___lhs, Ptr::<i8>::from_string_literal(b"hello"))
    }) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s1 == \"hello\""),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut p1: Ptr<i8> = ({ basic_string_20Impl::data_159(&s1.as_pointer()) });
    (if (((elem!(p1, 0).read()) as i32) == (('h' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"p1[0] == \'h\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(p1, 1).read()) as i32) == (('e' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"p1[1] == \'e\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(p1, 2).read()) as i32) == (('l' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"p1[2] == \'l\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(p1, 3).read()) as i32) == (('l' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"p1[3] == \'l\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(p1, 4).read()) as i32) == (('o' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"p1[4] == \'o\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let s2: Value<basic_string_20> = Rc::new(RefCell::new(basic_string_20::new_6(
        { 10_u64 },
        { ('a' as i8) },
        None,
    )));
    let mut p2: Ptr<i8> = ({ basic_string_20Impl::data_159(&s2.as_pointer()) });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < ({ basic_string_20Impl::size(&s2.as_pointer()) })) {
        (if (((elem!(p2, i).read()) as i32) == (('a' as i8) as i32))
            && (((({ basic_string_20Impl::operator_index_84(&s2.as_pointer(), (i as u64)) }).read())
                as i32)
                == (('a' as i8) as i32))
        {
            &(0);
        } else {
            ({
                __assert_fail_422(
                    Ptr::<i8>::from_string_literal(b"p2[i] == \'a\' && s2[i] == \'a\'"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    (if (({ basic_string_20Impl::length(&s2.as_pointer()) }) == 10_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s2.length() == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ basic_string_20Impl::size(&s2.as_pointer()) })
        == ({ basic_string_20Impl::length(&s2.as_pointer()) }))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s2.size() == s2.length()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ basic_string_20Impl::operator_index_84(&s2.as_pointer(), 0_u64) }).write(('b' as i8));
    ({ basic_string_20Impl::operator_index_84(&s2.as_pointer(), 1_u64) }).write(('c' as i8));
    (if (((({ basic_string_20Impl::operator_index_84(&s2.as_pointer(), 0_u64) }).read()) as i32)
        == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s2[0] == \'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&s2.as_pointer(), 1_u64) }).read()) as i32)
        == (('c' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s2[1] == \'c\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut i: u32 = 2_u32;
    'loop_: while ((i as u64) < ({ basic_string_20Impl::size(&s2.as_pointer()) })) {
        (if (((elem!(p2, i).read()) as i32) == (('a' as i8) as i32))
            && (((({ basic_string_20Impl::operator_index_84(&s2.as_pointer(), (i as u64)) }).read())
                as i32)
                == (('a' as i8) as i32))
        {
            &(0);
        } else {
            ({
                __assert_fail_422(
                    Ptr::<i8>::from_string_literal(b"p2[i] == \'a\' && s2[i] == \'a\'"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    let s3: Value<basic_string_20> = Rc::new(RefCell::new(
        ({ basic_string_20Impl::substr(&s2.as_pointer(), 2_u64, 5_u64) }),
    ));
    (if (({ basic_string_20Impl::length(&s3.as_pointer()) }) == 5_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s3.length() == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ basic_string_20Impl::size(&s3.as_pointer()) })
        == ({ basic_string_20Impl::length(&s3.as_pointer()) }))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s3.size() == s3.length()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut p3: Ptr<i8> = ({ basic_string_20Impl::data_159(&s3.as_pointer()) });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < ({ basic_string_20Impl::size(&s3.as_pointer()) })) {
        (if ({ ((elem!(p3, i).read()) as i32) } == {
            ((({ basic_string_20Impl::operator_index_84(&s3.as_pointer(), (i as u64)) }).read())
                as i32)
        }) {
            &(0);
        } else {
            ({
                __assert_fail_422(
                    Ptr::<i8>::from_string_literal(b"p3[i] == s3[i]"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    let s4: Value<basic_string_20> = Rc::new(RefCell::new(
        ({
            let ___n: u64 = ({
                basic_string_20Impl::find_last_of_179(
                    &s1.as_pointer(),
                    Ptr::<i8>::from_string_literal(b"l"),
                    None,
                )
            });
            basic_string_20Impl::substr(&s1.as_pointer(), 1_u64, ___n)
        }),
    ));
    (if (({ basic_string_20Impl::length(&s4.as_pointer()) }) == 3_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s4.length() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ basic_string_20Impl::size(&s4.as_pointer()) })
        == ({ basic_string_20Impl::length(&s4.as_pointer()) }))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s4.size() == s4.length()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut p4: Ptr<i8> = ({ basic_string_20Impl::data_159(&s4.as_pointer()) });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < ({ basic_string_20Impl::size(&s4.as_pointer()) })) {
        (if ({ ((elem!(p4, i).read()) as i32) } == {
            ((({ basic_string_20Impl::operator_index_84(&s4.as_pointer(), (i as u64)) }).read())
                as i32)
        }) {
            &(0);
        } else {
            ({
                __assert_fail_422(
                    Ptr::<i8>::from_string_literal(b"p4[i] == s4[i]"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    let s5: Value<basic_string_20> = Rc::new(RefCell::new(
        ({
            let ___lhs: Ptr<basic_string_20> = s1.as_pointer();
            operator_add_424(___lhs, Ptr::<i8>::from_string_literal(b", world"))
        }),
    ));
    (if (({ basic_string_20Impl::length(&s5.as_pointer()) }) == 12_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s5.length() == 12"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ basic_string_20Impl::size(&s5.as_pointer()) })
        == ({ basic_string_20Impl::length(&s5.as_pointer()) }))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"s5.size() == s5.length()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut p5: Ptr<i8> = ({ basic_string_20Impl::data_159(&s5.as_pointer()) });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < ({ basic_string_20Impl::size(&s5.as_pointer()) })) {
        (if ({ ((elem!(p5, i).read()) as i32) } == {
            ((({ basic_string_20Impl::operator_index_84(&s5.as_pointer(), (i as u64)) }).read())
                as i32)
        }) {
            &(0);
        } else {
            ({
                __assert_fail_422(
                    Ptr::<i8>::from_string_literal(b"p5[i] == s5[i]"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    let arr: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('b' as i8),
        ('a' as i8),
        ('r' as i8),
        (' ' as i8),
        ('f' as i8),
        ('o' as i8),
        ('o' as i8),
    ])));
    let string: Value<basic_string_20> = Rc::new(RefCell::new(basic_string_20::new_4(
        { (arr.as_pointer() as Ptr<i8>) },
        { 3_u64 },
        None,
    )));
    (if (({ basic_string_20Impl::size(&string.as_pointer()) }) == 3_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string.size() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 0_u64) }).read())
        as i32)
        == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[0] == \'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 1_u64) }).read())
        as i32)
        == (('a' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[1] == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 2_u64) }).read())
        as i32)
        == (('r' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[2] == \'r\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let ___lhs: Ptr<basic_string_20> = string.as_pointer();
        operator_eq_423(___lhs, Ptr::<i8>::from_string_literal(b"bar"))
    }) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string == \"bar\""),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ basic_string_20Impl::resize_75(&string.as_pointer(), 3_u64) });
    (if (({ basic_string_20Impl::size(&string.as_pointer()) }) == 3_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string.size() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 0_u64) }).read())
        as i32)
        == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[0] == \'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 1_u64) }).read())
        as i32)
        == (('a' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[1] == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 2_u64) }).read())
        as i32)
        == (('r' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[2] == \'r\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let ___lhs: Ptr<basic_string_20> = string.as_pointer();
        operator_eq_423(___lhs, Ptr::<i8>::from_string_literal(b"bar"))
    }) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string == \"bar\""),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ basic_string_20Impl::resize_75(&string.as_pointer(), 5_u64) });
    (if (({ basic_string_20Impl::size(&string.as_pointer()) }) == 5_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string.size() == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 0_u64) }).read())
        as i32)
        == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[0] == \'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 1_u64) }).read())
        as i32)
        == (('a' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[1] == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 2_u64) }).read())
        as i32)
        == (('r' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[2] == \'r\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 3_u64) }).read())
        as i32)
        == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[3] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 4_u64) }).read())
        as i32)
        == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[4] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 3_u64) }).write(('a' as i8));
    ({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 4_u64) }).write(('b' as i8));
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 3_u64) }).read())
        as i32)
        == (('a' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[3] == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 4_u64) }).read())
        as i32)
        == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[4] == \'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 3_u64) }).write(0_i8);
    ({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 4_u64) }).write(0_i8);
    ({ basic_string_20Impl::resize_75(&string.as_pointer(), 4_u64) });
    (if (({ basic_string_20Impl::size(&string.as_pointer()) }) == 4_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string.size() == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 0_u64) }).read())
        as i32)
        == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[0] == \'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 1_u64) }).read())
        as i32)
        == (('a' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[1] == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 2_u64) }).read())
        as i32)
        == (('r' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[2] == \'r\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&string.as_pointer(), 3_u64) }).read())
        as i32)
        == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"string[3] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let result: Value<basic_string_20> = Rc::new(RefCell::new(
        ({
            let ___lhs: Ptr<basic_string_20> = string.as_pointer();
            operator_add_424(___lhs, Ptr::<i8>::from_string_literal(b" foo"))
        }),
    ));
    (if (({ basic_string_20Impl::size(&result.as_pointer()) }) == 8_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"result.size() == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&result.as_pointer(), 0_u64) }).read())
        as i32)
        == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"result[0] == \'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&result.as_pointer(), 1_u64) }).read())
        as i32)
        == (('a' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"result[1] == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&result.as_pointer(), 2_u64) }).read())
        as i32)
        == (('r' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"result[2] == \'r\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&result.as_pointer(), 3_u64) }).read())
        as i32)
        == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"result[3] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&result.as_pointer(), 4_u64) }).read())
        as i32)
        == ((' ' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"result[4] == \' \'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&result.as_pointer(), 5_u64) }).read())
        as i32)
        == (('f' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"result[5] == \'f\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&result.as_pointer(), 6_u64) }).read())
        as i32)
        == (('o' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"result[6] == \'o\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&result.as_pointer(), 7_u64) }).read())
        as i32)
        == (('o' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"result[7] == \'o\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let substr_0: Value<basic_string_20> = Rc::new(RefCell::new(
        ({ basic_string_20Impl::substr(&result.as_pointer(), 5_u64, 3_u64) }),
    ));
    (if (({ basic_string_20Impl::size(&substr_0.as_pointer()) }) == 3_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_0.size() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_0.as_pointer(), 0_u64) }).read())
        as i32)
        == (('f' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_0[0] == \'f\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_0.as_pointer(), 1_u64) }).read())
        as i32)
        == (('o' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_0[1] == \'o\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_0.as_pointer(), 2_u64) }).read())
        as i32)
        == (('o' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_0[2] == \'o\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let substr_1: Value<basic_string_20> = Rc::new(RefCell::new(
        ({ basic_string_20Impl::substr(&result.as_pointer(), 0_u64, 5_u64) }),
    ));
    (if (({ basic_string_20Impl::size(&substr_1.as_pointer()) }) == 5_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_1.size() == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_1.as_pointer(), 0_u64) }).read())
        as i32)
        == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_1[0] == \'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_1.as_pointer(), 1_u64) }).read())
        as i32)
        == (('a' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_1[1] == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_1.as_pointer(), 2_u64) }).read())
        as i32)
        == (('r' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_1[2] == \'r\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_1.as_pointer(), 3_u64) }).read())
        as i32)
        == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_1[3] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_1.as_pointer(), 4_u64) }).read())
        as i32)
        == ((' ' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_1[4] == \' \'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let substr_2: Value<basic_string_20> = Rc::new(RefCell::new(
        ({ basic_string_20Impl::substr(&result.as_pointer(), 0_u64, 15_u64) }),
    ));
    (if (({ basic_string_20Impl::size(&substr_2.as_pointer()) }) == 8_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_2.size() == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_2.as_pointer(), 0_u64) }).read())
        as i32)
        == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_2[0] == \'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_2.as_pointer(), 1_u64) }).read())
        as i32)
        == (('a' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_2[1] == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_2.as_pointer(), 2_u64) }).read())
        as i32)
        == (('r' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_2[2] == \'r\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_2.as_pointer(), 3_u64) }).read())
        as i32)
        == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_2[3] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_2.as_pointer(), 4_u64) }).read())
        as i32)
        == ((' ' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_2[4] == \' \'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_2.as_pointer(), 5_u64) }).read())
        as i32)
        == (('f' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_2[5] == \'f\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_2.as_pointer(), 6_u64) }).read())
        as i32)
        == (('o' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_2[6] == \'o\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ basic_string_20Impl::operator_index_84(&substr_2.as_pointer(), 7_u64) }).read())
        as i32)
        == (('o' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"substr_2[7] == \'o\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut pos: u64 = ({
        basic_string_20Impl::find_last_of_179(
            &result.as_pointer(),
            Ptr::<i8>::from_string_literal(b"b"),
            None,
        )
    });
    (if (pos == 0_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"pos == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    pos = ({
        basic_string_20Impl::find_last_of_179(
            &result.as_pointer(),
            Ptr::<i8>::from_string_literal(b"f"),
            None,
        )
    });
    (if (pos == 5_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"pos == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    pos = ({
        basic_string_20Impl::find_last_of_179(
            &result.as_pointer(),
            Ptr::<i8>::from_string_literal(b"o"),
            None,
        )
    });
    (if (pos == 7_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"pos == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    pos = ({
        basic_string_20Impl::find_last_of_179(
            &result.as_pointer(),
            Ptr::<i8>::from_string_literal(b"x"),
            None,
        )
    });
    (if (pos == (-1_i64 as u64)) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"pos == -1ul"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let string_to_cast: Value<basic_string_20> = Rc::new(RefCell::new(basic_string_20::new_5(
        { Ptr::<i8>::from_string_literal(b"cast") },
        None,
    )));
    let mut output_data: Ptr<u8> =
        ({ basic_string_20Impl::operator_index_84(&string_to_cast.as_pointer(), 0_u64) })
            .reinterpret_cast::<u8>();
    (if (((output_data.read()) as i32) == (('c' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"*output_data == \'c\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(output_data, 1).read()) as i32) == (('a' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"output_data[1] == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(output_data, 2).read()) as i32) == (('s' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"output_data[2] == \'s\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(output_data, 3).read()) as i32) == (('t' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"output_data[3] == \'t\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut t0: u64 = ({ basic_string_20Impl::length(&s1.as_pointer()) });
    let mut t1: u64 = (t0).wrapping_add(((p1.read()) as u64));
    let mut t2: u64 = (t1).wrapping_add(({ basic_string_20Impl::length(&s2.as_pointer()) }));
    let mut t3: u64 = (t2).wrapping_add(({ basic_string_20Impl::size(&s3.as_pointer()) }));
    let mut t4: u64 = (t3).wrapping_add(({ basic_string_20Impl::size(&s4.as_pointer()) }));
    (if ((t4).wrapping_add(({ basic_string_20Impl::size(&s5.as_pointer()) })) == 139_u64) {
        &(0);
    } else {
        ({
            __assert_fail_422(
                Ptr::<i8>::from_string_literal(b"t4 + s5.size() == 139"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn __assert_fail_422(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn operator_eq_423(__lhs: Ptr<basic_string_20>, mut __rhs: Ptr<i8>) -> bool {
    unimplemented!()
}
pub fn operator_add_424(__lhs: Ptr<basic_string_20>, mut __rhs: Ptr<i8>) -> basic_string_20 {
    unimplemented!()
}
pub trait __new_allocator_261Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i8> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i8>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_261Impl for Ptr<__new_allocator_261> {}
pub trait __new_allocator_262Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_262Impl for Ptr<__new_allocator_262> {}
pub trait __new_allocator_263Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<u16> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<u16>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_263Impl for Ptr<__new_allocator_263> {}
pub trait __new_allocator_264Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<u32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_264Impl for Ptr<__new_allocator_264> {}
pub trait basic_string_20Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_20>) -> Ptr<basic_string_20> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_20>) -> Ptr<basic_string_20> {
        unimplemented!()
    }
    fn to_basic_string_view_24(&self) -> basic_string_view_24 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn resize_74(&self, mut __n: u64, mut __c: i8) {
        unimplemented!()
    }
    fn resize_75(&self, mut __n: u64) {
        unimplemented!()
    }
    fn capacity(&self) -> u64 {
        unimplemented!()
    }
    fn reserve_79(&self, __res: u64) {
        unimplemented!()
    }
    fn operator_index_84(&self, mut __pos: u64) -> Ptr<i8> {
        unimplemented!()
    }
    fn append_98(&self, mut __s: Ptr<i8>, mut __n: u64) -> Ptr<basic_string_20> {
        unimplemented!()
    }
    fn append_100(&self, mut __n: u64, mut __c: i8) -> Ptr<basic_string_20> {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn data_158(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn data_159(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn get_allocator(&self) -> allocator_8 {
        unimplemented!()
    }
    fn find_last_of_178(&self, mut __s: Ptr<i8>, mut __pos: u64, mut __n: u64) -> u64 {
        unimplemented!()
    }
    fn find_last_of_179(&self, mut __s: Ptr<i8>, __pos: Option<u64>) -> u64 {
        unimplemented!()
    }
    fn substr(&self, mut __pos: u64, mut __n: u64) -> basic_string_20 {
        unimplemented!()
    }
    fn __resize_and_overwrite_25(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_26(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_27(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_28(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_29(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_30(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
}
impl basic_string_20Impl for Ptr<basic_string_20> {}
pub trait basic_string_31Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_31>) -> Ptr<basic_string_31> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_31>) -> Ptr<basic_string_31> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_31Impl for Ptr<basic_string_31> {}
pub trait basic_string_34Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_34>) -> Ptr<basic_string_34> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_34>) -> Ptr<basic_string_34> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_34Impl for Ptr<basic_string_34> {}
pub trait basic_string_37Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_37>) -> Ptr<basic_string_37> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_37>) -> Ptr<basic_string_37> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_37Impl for Ptr<basic_string_37> {}
pub trait basic_string_view_24Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl basic_string_view_24Impl for Ptr<basic_string_view_24> {}
pub trait basic_string_view_414Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_view_414Impl for Ptr<basic_string_view_414> {}
pub trait basic_string_view_415Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u16> {
        unimplemented!()
    }
}
impl basic_string_view_415Impl for Ptr<basic_string_view_415> {}
pub trait basic_string_view_416Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u32> {
        unimplemented!()
    }
}
impl basic_string_view_416Impl for Ptr<basic_string_view_416> {}
pub trait integral_constant_41Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_41Impl for Ptr<integral_constant_41> {}
pub fn __cpp2rust_init_globals() {}
