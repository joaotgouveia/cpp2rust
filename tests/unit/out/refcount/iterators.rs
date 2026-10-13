extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub struct allocator_12 {}
impl Clone for allocator_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_12 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_12 {
    pub fn copy_from(__a: Ptr<allocator_12>) -> Self {
        unimplemented!()
    }
}
impl allocator_12 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_12>) -> Ptr<allocator_12> {
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
    pub fn allocate_1(__a: Ptr<allocator_8>, mut __n: u64) -> Ptr<i8> {
        unimplemented!()
    }
}
impl allocator_traits_13 {
    pub fn deallocate(__a: Ptr<allocator_8>, mut __p: Ptr<i8>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_13 {
    pub fn max_size(__a: Ptr<allocator_8>) -> u64 {
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
    pub fn allocate_1(__a: Ptr<allocator_9>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub fn deallocate(__a: Ptr<allocator_9>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub fn max_size(__a: Ptr<allocator_9>) -> u64 {
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
    pub fn allocate_1(__a: Ptr<allocator_10>, mut __n: u64) -> Ptr<u16> {
        unimplemented!()
    }
}
impl allocator_traits_15 {
    pub fn deallocate(__a: Ptr<allocator_10>, mut __p: Ptr<u16>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_15 {
    pub fn max_size(__a: Ptr<allocator_10>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_16 {}
impl Clone for allocator_traits_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_16 {
    pub fn allocate_1(__a: Ptr<allocator_11>, mut __n: u64) -> Ptr<u32> {
        unimplemented!()
    }
}
impl allocator_traits_16 {
    pub fn deallocate(__a: Ptr<allocator_11>, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_16 {
    pub fn max_size(__a: Ptr<allocator_11>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_17 {}
impl Clone for allocator_traits_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_17 {
    pub fn allocate_1(__a: Ptr<allocator_12>, mut __n: u64) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl allocator_traits_17 {
    pub fn deallocate(__a: Ptr<allocator_12>, mut __p: Ptr<Ptr<i32>>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_17 {
    pub fn max_size(__a: Ptr<allocator_12>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_17 {
    pub fn construct_18(__a: Ptr<allocator_12>, mut __p: Ptr<Ptr<i32>>, __args: Ptr<Ptr<i32>>) {
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
    pub fn assign_1(__c1: Ptr<i8>, __c2: Ptr<i8>) {
        unimplemented!()
    }
}
impl char_traits_19 {
    pub fn lt(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_19 {
    pub fn length(mut __s: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_19 {
    pub fn copy(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_19 {
    pub fn eof() -> i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_20 {}
impl Clone for char_traits_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_20 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_20 {
    pub fn assign_1(__c1: Ptr<i32>, __c2: Ptr<i32>) {
        unimplemented!()
    }
}
impl char_traits_20 {
    pub fn copy(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl char_traits_20 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_20 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_21 {}
impl Clone for char_traits_21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_21 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_21 {
    pub fn assign_1(__c1: Ptr<u16>, __c2: Ptr<u16>) {
        unimplemented!()
    }
}
impl char_traits_21 {
    pub fn eq(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_21 {
    pub fn lt(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_21 {
    pub fn copy(mut __s1: Ptr<u16>, mut __s2: Ptr<u16>, mut __n: usize) -> Ptr<u16> {
        unimplemented!()
    }
}
impl char_traits_21 {
    pub fn eq_int_type(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_21 {
    pub fn eof() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_22 {}
impl Clone for char_traits_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_22 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_22 {
    pub fn assign_1(__c1: Ptr<u32>, __c2: Ptr<u32>) {
        unimplemented!()
    }
}
impl char_traits_22 {
    pub fn eq(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_22 {
    pub fn lt(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_22 {
    pub fn copy(mut __s1: Ptr<u32>, mut __s2: Ptr<u32>, mut __n: usize) -> Ptr<u32> {
        unimplemented!()
    }
}
impl char_traits_22 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_22 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_23 {}
impl Clone for basic_string_23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_23 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
pub type anon_25 = u32;
pub const anon_25__S_local_capacity: anon_25 = 15;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_24 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i8]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_23 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl basic_string_23 {
    pub fn copy_from(__str: Ptr<basic_string_23>) -> Self {
        unimplemented!()
    }
}
impl basic_string_23 {
    pub fn new_2(mut __s: Ptr<i8>, mut __n: u64, __a: Option<Ptr<allocator_8>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_23 {
    pub fn move_from(__str: Ptr<basic_string_23>) -> Self {
        unimplemented!()
    }
}
impl basic_string_23 {
    pub fn new_3(mut __s: Ptr<i8>, __a: Option<Ptr<allocator_8>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_23 {
    pub fn new_4(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_8>>) -> Self {
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
pub type anon_36 = u32;
pub const anon_36__S_local_capacity: anon_36 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_35 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i32]>>,
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
    pub fn new_1(mut __s: Ptr<i32>, mut __n: u64, __a: Option<Ptr<allocator_9>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_34 {
    pub fn move_from(__str: Ptr<basic_string_34>) -> Self {
        unimplemented!()
    }
}
impl basic_string_34 {
    pub fn new_2(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_9>>) -> Self {
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
pub type anon_39 = u32;
pub const anon_39__S_local_capacity: anon_39 = 7;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_38 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u16]>>,
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
    pub fn new_1(mut __s: Ptr<u16>, mut __n: u64, __a: Option<Ptr<allocator_10>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_37 {
    pub fn move_from(__str: Ptr<basic_string_37>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_40 {}
impl Clone for basic_string_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_40 {
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
pub type anon_42 = u32;
pub const anon_42__S_local_capacity: anon_42 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_41 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_40 {
    pub fn copy_from(__str: Ptr<basic_string_40>) -> Self {
        unimplemented!()
    }
}
impl basic_string_40 {
    pub fn new_1(mut __s: Ptr<u32>, mut __n: u64, __a: Option<Ptr<allocator_11>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_40 {
    pub fn move_from(__str: Ptr<basic_string_40>) -> Self {
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
    pub __value: anon_43,
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
pub struct anon_43 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_43 {
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
pub struct integral_constant_44 {}
impl Clone for integral_constant_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_45: Value<bool> = unimplemented!();
);
impl integral_constant_44 {}
impl integral_constant_44 {}
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
thread_local!(
    pub static std_integral_constant_bool__false__value_47: Value<bool> = unimplemented!();
);
impl integral_constant_46 {}
impl integral_constant_46 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_48 {}
impl Clone for integral_constant_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_49 {}
impl Clone for integral_constant_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_49 {
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
pub struct enable_if_54 {}
impl Clone for enable_if_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_55 {}
impl Clone for enable_if_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_56 {}
impl Clone for enable_if_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_56 {
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
pub struct __type_identity_61 {}
impl Clone for __type_identity_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_62 {}
impl Clone for __type_identity_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_63 {}
impl Clone for __type_identity_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_64 {}
impl Clone for __type_identity_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_65 {}
impl Clone for __type_identity_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_66 {}
impl Clone for __type_identity_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_66 {
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
pub struct __or__74 {}
impl Clone for __or__74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__75 {}
impl Clone for __or__75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__76 {}
impl Clone for __or__76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__77 {}
impl Clone for __or__77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__78 {}
impl Clone for __or__78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__79 {}
impl Clone for __or__79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__80 {}
impl Clone for __or__80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__81 {}
impl Clone for __or__81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__82 {}
impl Clone for __or__82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__83 {}
impl Clone for __or__83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__84 {}
impl Clone for __or__84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__85 {}
impl Clone for __or__85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__86 {}
impl Clone for __or__86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__87 {}
impl Clone for __and__87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__88 {}
impl Clone for __and__88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__89 {}
impl Clone for __and__89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__90 {}
impl Clone for __and__90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__91 {}
impl Clone for __and__91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__92 {}
impl Clone for __not__92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__93 {}
impl Clone for __not__93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__94 {}
impl Clone for __not__94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_95 {}
impl Clone for is_reference_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_96 {}
impl Clone for is_reference_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_97 {}
impl Clone for is_reference_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_98 {}
impl Clone for is_reference_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_99 {}
impl Clone for is_reference_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_100 {}
impl Clone for is_reference_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_101 {}
impl Clone for is_reference_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_102 {}
impl Clone for is_reference_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_103 {}
impl Clone for is_reference_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_104 {}
impl Clone for is_reference_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_105 {}
impl Clone for is_function_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_106 {}
impl Clone for is_function_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_107 {}
impl Clone for is_function_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_108 {}
impl Clone for is_function_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_109 {}
impl Clone for is_function_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_110 {}
impl Clone for is_function_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_111 {}
impl Clone for is_function_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_112 {}
impl Clone for is_function_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_113 {}
impl Clone for is_function_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_114 {}
impl Clone for is_function_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_114 {
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
pub struct is_void_116 {}
impl Clone for is_void_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_117 {}
impl Clone for is_void_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_118 {}
impl Clone for is_void_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_119 {}
impl Clone for is_void_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_120 {}
impl Clone for is_void_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_121 {}
impl Clone for is_void_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_122 {}
impl Clone for is_void_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_123 {}
impl Clone for is_void_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_124 {}
impl Clone for is_void_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_125 {}
impl Clone for is_void_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_126 {}
impl Clone for is_void_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_127 {}
impl Clone for is_void_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_128 {}
impl Clone for is_void_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_129 {}
impl Clone for is_void_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_130 {}
impl Clone for is_void_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_131 {}
impl Clone for is_void_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_132 {}
impl Clone for is_void_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_133 {}
impl Clone for is_void_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_134 {}
impl Clone for remove_cv_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_135 {}
impl Clone for remove_cv_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_136 {}
impl Clone for remove_cv_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_137 {}
impl Clone for remove_cv_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_138 {}
impl Clone for remove_cv_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_139 {}
impl Clone for remove_cv_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_140 {}
impl Clone for is_const_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_141 {}
impl Clone for is_const_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_142 {}
impl Clone for is_const_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_143 {}
impl Clone for is_const_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_144 {}
impl Clone for is_const_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_145 {}
impl Clone for __is_array_unknown_bounds_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_146 {}
impl Clone for __is_array_unknown_bounds_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_147 {}
impl Clone for __is_array_unknown_bounds_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_148 {}
impl Clone for __is_array_unknown_bounds_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_149 {}
impl Clone for __is_array_unknown_bounds_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_150 {}
impl Clone for __is_array_unknown_bounds_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_151 {}
impl Clone for __is_array_unknown_bounds_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_152 {}
impl Clone for __is_array_unknown_bounds_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_153 {}
impl Clone for __is_array_unknown_bounds_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_154 {}
impl Clone for __is_array_unknown_bounds_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_155 {}
impl Clone for __is_integral_helper_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_156 {}
impl Clone for __is_integral_helper_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_157 {}
impl Clone for __is_integral_helper_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_158 {}
impl Clone for __is_integral_helper_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_159 {}
impl Clone for __is_integral_helper_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_160 {}
impl Clone for __is_integral_helper_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_161 {}
impl Clone for is_integral_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_162 {}
impl Clone for is_integral_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_163 {}
impl Clone for is_integral_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_164 {}
impl Clone for is_integral_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_165 {}
impl Clone for is_integral_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_166 {}
impl Clone for is_integral_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_167 {}
impl Clone for __is_floating_point_helper_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_168 {}
impl Clone for is_floating_point_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_169 {}
impl Clone for is_pointer_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_170 {}
impl Clone for is_pointer_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_171 {}
impl Clone for is_enum_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_172 {}
impl Clone for is_arithmetic_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_173 {}
impl Clone for is_scalar_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_174 {}
impl Clone for is_same_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_175 {}
impl Clone for is_same_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_176 {}
impl Clone for is_same_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_177 {}
impl Clone for is_same_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_178 {}
impl Clone for is_same_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_179 {}
impl Clone for is_same_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_180 {}
impl Clone for is_same_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_181 {}
impl Clone for is_same_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_182 {}
impl Clone for is_same_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_183 {}
impl Clone for is_same_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_184 {}
impl Clone for is_same_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_185 {}
impl Clone for is_same_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_186 {}
impl Clone for is_same_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_187 {}
impl Clone for is_same_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_188 {}
impl Clone for is_same_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_189 {}
impl Clone for is_same_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_190 {}
impl Clone for is_same_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_191 {}
impl Clone for is_same_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_192 {}
impl Clone for is_same_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_193 {}
impl Clone for is_same_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_194 {}
impl Clone for is_same_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_195 {}
impl Clone for is_volatile_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_196 {}
impl Clone for is_volatile_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_197 {}
impl Clone for is_volatile_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_198 {}
impl Clone for is_volatile_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_199 {}
impl Clone for is_volatile_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_199 {
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
pub struct __is_destructible_safe_200 {}
impl Clone for __is_destructible_safe_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_200 {
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
pub struct is_constructible_201 {}
impl Clone for is_constructible_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_202 {}
impl Clone for is_move_constructible_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_203 {}
impl Clone for is_nothrow_default_constructible_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_204 {}
impl Clone for is_nothrow_default_constructible_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_205 {}
impl Clone for is_nothrow_default_constructible_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_206 {}
impl Clone for is_nothrow_copy_constructible_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_207 {}
impl Clone for is_nothrow_copy_constructible_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_208 {}
impl Clone for is_nothrow_copy_constructible_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_209 {}
impl Clone for is_nothrow_copy_constructible_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_210 {}
impl Clone for is_nothrow_copy_constructible_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_211 {}
impl Clone for is_nothrow_copy_constructible_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_211 {
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
pub struct is_trivially_destructible_212 {}
impl Clone for is_trivially_destructible_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_213 {}
impl Clone for is_convertible_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_214 {}
impl Clone for is_convertible_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_215 {}
impl Clone for is_convertible_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_216 {}
impl Clone for is_convertible_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_217 {}
impl Clone for is_convertible_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_218 {}
impl Clone for __is_nothrow_new_constructible_impl_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_219 {}
impl Clone for remove_reference_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_220 {}
impl Clone for remove_reference_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_221 {}
impl Clone for remove_reference_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_222 {}
impl Clone for remove_reference_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_223 {}
impl Clone for remove_reference_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_224 {}
impl Clone for remove_reference_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_225 {}
impl Clone for remove_reference_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_226 {}
impl Clone for remove_reference_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_227 {}
impl Clone for remove_reference_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_228 {}
impl Clone for __cv_selector_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_229 {}
impl Clone for __cv_selector_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_230 {}
impl Clone for __cv_selector_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_231 {}
impl Clone for __cv_selector_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_232 {}
impl Clone for __match_cv_qualifiers_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_233 {}
impl Clone for __match_cv_qualifiers_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_234 {}
impl Clone for __match_cv_qualifiers_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_235 {}
impl Clone for __match_cv_qualifiers_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_236 {}
impl Clone for __match_cv_qualifiers_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_237 {}
impl Clone for __make_unsigned_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_238 {}
impl Clone for __make_unsigned_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_239 {}
impl Clone for __make_unsigned_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_240 {}
impl Clone for __make_unsigned_selector_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_241 {}
impl Clone for __make_unsigned_selector_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_242 {}
impl Clone for __make_unsigned_selector_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_243 {}
impl Clone for __make_unsigned_selector_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_244 {}
impl Clone for __make_unsigned_selector_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_245 {}
impl Clone for __make_unsigned_selector_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_245 {
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
pub struct _List_249 {}
impl Clone for _List_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_250 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_251 {}
impl Clone for _List_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_252 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_253 {}
impl Clone for _List_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_254 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_255 {}
impl Clone for _List_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_256 {}
impl Clone for _List_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_257 {}
impl Clone for _List_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_258 {}
impl Clone for __select_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_259 {}
impl Clone for __select_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_260 {}
impl Clone for __select_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_261 {}
impl Clone for __select_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_262 {}
impl Clone for __select_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_263 {}
impl Clone for __make_signed_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_264 {}
impl Clone for __make_signed_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_265 {}
impl Clone for __make_signed_selector_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_266 {}
impl Clone for __make_signed_selector_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_267 {}
impl Clone for __make_signed_selector_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_268 {}
impl Clone for __make_signed_selector_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_269 {}
impl Clone for __make_signed_selector_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_269 {
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
pub struct common_type_270 {}
impl Clone for common_type_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_270 {
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
    pub static is_array_v_271: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_272: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_273: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_274: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_275: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_276: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_277: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_278: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_279: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_280: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_281: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_282: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_283: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_284: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_285: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_286: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_287: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_288: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_289: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_290: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_291: Value<bool> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_292 {}
impl Clone for _Char_types_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_293 {}
impl Clone for _Char_types_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_294 {}
impl Clone for char_traits_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_294 {
    pub fn eq(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_294 {
    pub fn length(mut __p: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_294 {
    pub fn find(mut __s: Ptr<i8>, mut __n: usize, __a: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_295 {}
impl Clone for char_traits_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_295 {
    pub fn eq(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_295 {
    pub fn lt(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_295 {
    pub fn compare(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_295 {
    pub fn length(mut __p: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl char_traits_295 {
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
pub struct __new_allocator_296 {}
impl Clone for __new_allocator_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_296 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_296 {
    pub fn copy_from(_a0: Ptr<__new_allocator_296>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_296 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_296>) -> Ptr<__new_allocator_296> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_297 {}
impl Clone for __new_allocator_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_297 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_297 {
    pub fn copy_from(_a0: Ptr<__new_allocator_297>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_297 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_297>) -> Ptr<__new_allocator_297> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_298 {}
impl Clone for __new_allocator_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_298 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_298 {
    pub fn copy_from(_a0: Ptr<__new_allocator_298>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_298 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_298>) -> Ptr<__new_allocator_298> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_299 {}
impl Clone for __new_allocator_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_299 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_299 {
    pub fn copy_from(_a0: Ptr<__new_allocator_299>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_299 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_299>) -> Ptr<__new_allocator_299> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_300 {}
impl Clone for __new_allocator_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_300 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_300 {
    pub fn copy_from(_a0: Ptr<__new_allocator_300>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_300 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_300>) -> Ptr<__new_allocator_300> {
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
pub struct __are_same_302 {}
impl Clone for __are_same_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_303 = u32;
pub const anon_303___value: anon_303 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_304 {}
impl Clone for __are_same_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_305 = u32;
pub const anon_305___value: anon_305 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_306 {}
impl Clone for __are_same_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_307 = u32;
pub const anon_307___value: anon_307 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_308 {}
impl Clone for __are_same_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_309 = u32;
pub const anon_309___value: anon_309 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_310 {}
impl Clone for __are_same_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_311 = u32;
pub const anon_311___value: anon_311 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_312 {}
impl Clone for __are_same_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_313 = u32;
pub const anon_313___value: anon_313 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_314 {}
impl Clone for __is_integer_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_315 = u32;
pub const anon_315___value: anon_315 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_316 {}
impl Clone for __is_integer_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_317 = u32;
pub const anon_317___value: anon_317 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_318 {}
impl Clone for __is_integer_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_319 = u32;
pub const anon_319___value: anon_319 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_320 {}
impl Clone for __is_integer_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_321 = u32;
pub const anon_321___value: anon_321 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_322 {}
impl Clone for __is_integer_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_323 = u32;
pub const anon_323___value: anon_323 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_324 {}
impl Clone for __is_integer_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_325 = u32;
pub const anon_325___value: anon_325 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_326 {}
impl Clone for __is_integer_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_327 = u32;
pub const anon_327___value: anon_327 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_328 {}
impl Clone for __is_integer_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_329 = u32;
pub const anon_329___value: anon_329 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_330 {}
impl Clone for __memcpyable_integer_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_330 {
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
pub struct iterator_331 {}
impl Clone for iterator_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_332 {}
impl Clone for iterator_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_332 {}
impl iterator_332 {}
impl iterator_332 {}
impl iterator_332 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_332>) -> Ptr<iterator_332> {
        unimplemented!()
    }
}
impl iterator_332 {
    pub fn move_assign(&self, _a0: Ptr<iterator_332>) -> Ptr<iterator_332> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_333 {}
impl Clone for iterator_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_333 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_333>) -> Ptr<iterator_333> {
        unimplemented!()
    }
}
impl iterator_333 {
    pub fn move_assign(&self, _a0: Ptr<iterator_333>) -> Ptr<iterator_333> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_334 {}
impl Clone for iterator_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_334 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_334>) -> Ptr<iterator_334> {
        unimplemented!()
    }
}
impl iterator_334 {
    pub fn move_assign(&self, _a0: Ptr<iterator_334>) -> Ptr<iterator_334> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_335 {}
impl Clone for iterator_traits_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_336 {}
impl Clone for iterator_traits_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_337 {}
impl Clone for iterator_traits_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_338 {}
impl Clone for iterator_traits_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_339 {}
impl Clone for iterator_traits_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_340 {}
impl Clone for iterator_traits_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_341 {}
impl Clone for iterator_traits_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_342 {}
impl Clone for iterator_traits_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_343 {}
impl Clone for iterator_traits_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_344 {}
impl Clone for __iterator_traits_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_345 {}
impl Clone for __iterator_traits_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_346 {}
impl Clone for __ptr_traits_ptr_to_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_346 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_347 {}
impl Clone for __ptr_traits_ptr_to_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_347 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_348 {}
impl Clone for __ptr_traits_ptr_to_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_348 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_349 {}
impl Clone for __ptr_traits_ptr_to_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_349 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_350 {}
impl Clone for __ptr_traits_ptr_to_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_350 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_351 {}
impl Clone for __ptr_traits_ptr_to_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_351 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_352 {}
impl Clone for __ptr_traits_ptr_to_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_352 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_353 {}
impl Clone for __ptr_traits_ptr_to_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_353 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_354 {}
impl Clone for __ptr_traits_ptr_to_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_355 {}
impl Clone for pointer_traits_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_356 {}
impl Clone for pointer_traits_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_357 {}
impl Clone for pointer_traits_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_358 {}
impl Clone for pointer_traits_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_359 {}
impl Clone for pointer_traits_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_360 {}
impl Clone for pointer_traits_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_361 {}
impl Clone for pointer_traits_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_362 {}
impl Clone for pointer_traits_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_363 {}
impl Clone for pointer_traits_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_364 {}
impl Clone for reverse_iterator_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_364 {
    pub fn copy_from(__x: Ptr<reverse_iterator_364>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_364 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_364>) -> Ptr<reverse_iterator_364> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_365 {}
impl Clone for reverse_iterator_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_365 {
    pub fn copy_from(__x: Ptr<reverse_iterator_365>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_365 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_365>) -> Ptr<reverse_iterator_365> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_27 {}
impl Clone for __normal_iterator_27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_27 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_27 {
    pub fn new_1(__i: Ptr<Ptr<i8>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_27 {}
impl __normal_iterator_27 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_366 {}
impl Clone for __normal_iterator_366 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_366 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_366 {
    pub fn new_1(__i: Ptr<Ptr<Ptr<i32>>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_366 {}
impl __normal_iterator_366 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_367 {}
impl Clone for __normal_iterator_367 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_367 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_367 {
    pub fn new_1(__i: Ptr<Ptr<Ptr<i32>>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_367 {}
impl __normal_iterator_367 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_368 {}
impl Clone for binary_function_368 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_368 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_369 {}
impl Clone for greater_369 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_369 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_370 {}
impl Clone for less_370 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_370 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_371 {}
impl Clone for greater_equal_371 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_371 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_372 {}
impl Clone for less_equal_372 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_372 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_373 {}
impl Clone for __is_integer_nonstrict_373 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_373 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_374 = u32;
pub const anon_374___width: anon_374 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_375 {}
impl Clone for __is_integer_nonstrict_375 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_375 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_376 = u32;
pub const anon_376___width: anon_376 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_377 {}
impl Clone for __is_integer_nonstrict_377 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_377 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_378 = u32;
pub const anon_378___width: anon_378 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_379 {}
impl Clone for __is_integer_nonstrict_379 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_379 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_380 = u32;
pub const anon_380___width: anon_380 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_381 {}
impl Clone for __is_integer_nonstrict_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_382 = u32;
pub const anon_382___width: anon_382 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_383 {}
impl Clone for __numeric_traits_integer_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____is_signed_384: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____digits_385: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____max_386: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____min_387: Value<i32> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_388 {}
impl Clone for __numeric_traits_integer_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_389: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_390: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_391: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_392 {}
impl Clone for __numeric_traits_integer_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_393: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____digits_394: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____max_395: Value<u32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_396 {}
impl Clone for __numeric_traits_integer_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_397: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_398: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_399: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_400 {}
impl Clone for __numeric_traits_integer_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_401: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_402: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_403: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_404 {}
impl Clone for __numeric_traits_floating_404 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_404 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_float____max_exponent10_405: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_406 {}
impl Clone for __numeric_traits_floating_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_double____max_exponent10_407: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_408 {}
impl Clone for __numeric_traits_floating_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_408 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_409: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_410 {}
impl Clone for __numeric_traits_410 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_410 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_411 {}
impl Clone for __numeric_traits_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_412 {}
impl Clone for __numeric_traits_412 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_412 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_413 {}
impl Clone for __numeric_traits_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_414 {}
impl Clone for __numeric_traits_414 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_414 {
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
    pub static piecewise_construct_415: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_416 {}
impl Clone for tuple_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_416 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_416 {}
impl tuple_416 {}
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
pub struct initializer_list_417 {}
impl Clone for initializer_list_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_417 {}
impl initializer_list_417 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_418 {}
impl Clone for initializer_list_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_418 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_418 {}
impl initializer_list_418 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_419 {}
impl Clone for initializer_list_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_419 {}
impl initializer_list_419 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_420 {}
impl Clone for initializer_list_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_420 {}
impl initializer_list_420 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_421 {}
impl Clone for initializer_list_421 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_421 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
pub struct __is_move_insertable_423 {}
impl Clone for __is_move_insertable_423 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_423 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_424 {}
impl Clone for __is_allocator_424 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_424 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_425 {}
impl Clone for __is_allocator_425 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_425 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_426 {}
impl Clone for __is_allocator_426 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_426 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_427 {}
impl Clone for __is_allocator_427 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_427 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_428 {}
impl Clone for __alloc_traits_428 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_428 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_429 {}
impl Clone for rebind_429 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_429 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_430 {}
impl Clone for __alloc_traits_430 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_430 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_431 {}
impl Clone for rebind_431 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_431 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_432 {}
impl Clone for __alloc_traits_432 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_432 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_433 {}
impl Clone for rebind_433 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_433 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_434 {}
impl Clone for __alloc_traits_434 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_434 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_435 {}
impl Clone for rebind_435 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_435 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_436 {}
impl Clone for __alloc_traits_436 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_436 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_437 {}
impl Clone for rebind_437 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_437 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_438 {}
impl Clone for __hash_base_438 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_438 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_439 {}
impl Clone for __hash_base_439 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_439 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_440 {}
impl Clone for __hash_base_440 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_440 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_441 {}
impl Clone for __hash_base_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_442 {}
impl Clone for __hash_base_442 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_442 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_443 {}
impl Clone for __hash_base_443 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_443 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_444 {}
impl Clone for __hash_base_444 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_444 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_445 {}
impl Clone for __hash_base_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_446 {}
impl Clone for __hash_base_446 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_446 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_447 {}
impl Clone for __hash_base_447 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_447 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_448 {}
impl Clone for __hash_base_448 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_448 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_449 {}
impl Clone for __hash_base_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_450 {}
impl Clone for __hash_base_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_451 {}
impl Clone for __hash_base_451 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_451 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_452 {}
impl Clone for __hash_base_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_452 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_453 {}
impl Clone for __hash_base_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_454 {}
impl Clone for __hash_base_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_454 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_455 {}
impl Clone for __hash_base_455 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_455 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_456 {}
impl Clone for __hash_base_456 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_456 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_457 {}
impl Clone for __hash_base_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_458 {}
impl Clone for __hash_base_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_459 {}
impl Clone for __hash_base_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_460 {}
impl Clone for __hash_base_460 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_460 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_461 {}
impl Clone for __hash_base_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_462 {}
impl Clone for __hash_base_462 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_462 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_463 {}
impl Clone for hash_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_463 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_464 {}
impl Clone for hash_464 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_464 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_465 {}
impl Clone for hash_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_466 {}
impl Clone for hash_466 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_466 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_467 {}
impl Clone for hash_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_467 {
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
    pub fn hash_468(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_469(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_26 {}
impl Clone for basic_string_view_26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_26 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_26 {
    pub fn copy_from(_a0: Ptr<basic_string_view_26>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_26 {
    pub fn new_1(mut __str: Ptr<i8>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_26 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_26>) -> Ptr<basic_string_view_26> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_470 {}
impl Clone for basic_string_view_470 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_470 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_470 {
    pub fn copy_from(_a0: Ptr<basic_string_view_470>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_470 {
    pub fn new_1(mut __str: Ptr<i32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_470 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_470>) -> Ptr<basic_string_view_470> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_471 {}
impl Clone for basic_string_view_471 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_471 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_471 {
    pub fn copy_from(_a0: Ptr<basic_string_view_471>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_471 {
    pub fn new_1(mut __str: Ptr<u16>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_471 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_471>) -> Ptr<basic_string_view_471> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_472 {}
impl Clone for basic_string_view_472 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_472 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_472 {
    pub fn copy_from(_a0: Ptr<basic_string_view_472>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_472 {
    pub fn new_1(mut __str: Ptr<u32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_472 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_472>) -> Ptr<basic_string_view_472> {
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
    pub __value32: anon_473,
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
pub struct anon_473 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_473 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_473 {
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
    pub static __integer_to_chars_is_unsigned_474: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_475: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_476: Value<bool> = unimplemented!();
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
    pub static allocator_arg_477: Value<std_allocator_arg_t> = unimplemented!();
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_478 {}
impl Clone for __is_bitwise_relocatable_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_479 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl,
}
impl Clone for _Vector_base_479 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_479 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<Ptr<i32>>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<Ptr<i32>>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<Ptr<i32>>,
}
impl Clone for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {}
impl Clone for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    pub fn move_from(
        __x: Ptr<std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_479 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_479 {
    pub fn move_from(_a0: Ptr<_Vector_base_479>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_480 {}
impl Clone for vector_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_int_ptr___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<Ptr<i32>>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_479>,
}
impl Clone for std_vector_int_ptr___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_int_ptr___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_int_ptr___Guard_alloc {
    pub fn new(mut __s: Ptr<Ptr<i32>>, mut __l: u64, __vect: Ptr<_Vector_base_479>) -> Self {
        unimplemented!()
    }
}
impl vector_480 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_480 {
    pub fn copy_from(__x: Ptr<vector_480>) -> Self {
        unimplemented!()
    }
}
impl vector_480 {
    pub fn move_from(_a0: Ptr<vector_480>) -> Self {
        unimplemented!()
    }
}
pub type anon_482 = u32;
pub const anon_482__S_word_bit: anon_482 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Bit_reference {}
impl Clone for std__Bit_reference {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_reference {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub fn copy_from(_a0: Ptr<std__Bit_reference>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Bit_iterator_base {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<u64>,
    #[offset(8)]
    pub _M_offset: u32,
}
impl Clone for std__Bit_iterator_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_iterator_base {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub fn new(mut __x: Ptr<u64>, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {
    pub fn copy_assign(&self, _a0: Ptr<std__Bit_iterator_base>) -> Ptr<std__Bit_iterator_base> {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub fn move_assign(&self, _a0: Ptr<std__Bit_iterator_base>) -> Ptr<std__Bit_iterator_base> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Bit_iterator {}
impl Clone for std__Bit_iterator {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_iterator {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub fn new_1(mut __x: Ptr<u64>, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {}
impl std__Bit_iterator {}
impl std__Bit_iterator {
    pub fn copy_assign(&self, _a0: Ptr<std__Bit_iterator>) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub fn move_assign(&self, _a0: Ptr<std__Bit_iterator>) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Bit_const_iterator {}
impl Clone for std__Bit_const_iterator {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_const_iterator {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {
    pub fn copy_assign(&self, _a0: Ptr<std__Bit_const_iterator>) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub fn move_assign(&self, _a0: Ptr<std__Bit_const_iterator>) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<basic_string_23> = Rc::new(RefCell::new(basic_string_23::new_3(
        { Ptr::<i8>::from_string_literal(b"hello") },
        None,
    )));
    {
        let __range1: Ptr<basic_string_23> = x.as_pointer();
        let __begin1: Value<__normal_iterator_27> =
            Rc::new(RefCell::new(({ basic_string_23Impl::begin_59(&__range1) })));
        let __end1: Value<__normal_iterator_27> =
            Rc::new(RefCell::new(({ basic_string_23Impl::end_61(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_27> = __begin1.as_pointer();
            operator_ne_483(___lhs, __end1.as_pointer())
        }) {
            let c: Ptr<i8> = ({ __normal_iterator_27Impl::operator_deref(&__begin1.as_pointer()) });
            c.with_mut(|__v| __v.prefix_inc());
            ({ __normal_iterator_27Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<basic_string_23> = x.as_pointer();
        let __begin1: Value<__normal_iterator_27> =
            Rc::new(RefCell::new(({ basic_string_23Impl::begin_59(&__range1) })));
        let __end1: Value<__normal_iterator_27> =
            Rc::new(RefCell::new(({ basic_string_23Impl::end_61(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_27> = __begin1.as_pointer();
            operator_ne_483(___lhs, __end1.as_pointer())
        }) {
            let c: Ptr<i8> = ({ __normal_iterator_27Impl::operator_deref(&__begin1.as_pointer()) });
            println!("{}", ((c.read()) as i32) as u8 as char);
            ({ __normal_iterator_27Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<basic_string_23> = x.as_pointer();
        let __begin1: Value<__normal_iterator_27> =
            Rc::new(RefCell::new(({ basic_string_23Impl::begin_59(&__range1) })));
        let __end1: Value<__normal_iterator_27> =
            Rc::new(RefCell::new(({ basic_string_23Impl::end_61(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_27> = __begin1.as_pointer();
            operator_ne_483(___lhs, __end1.as_pointer())
        }) {
            let mut c: i8 =
                (({ __normal_iterator_27Impl::operator_deref(&__begin1.as_pointer()) }).read());
            println!("{}", (c as i32) as u8 as char);
            ({ __normal_iterator_27Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    let v: Value<vector_480> = Rc::new(RefCell::new(<vector_480>::default()));
    ({
        let ___x: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(2)));
        vector_480Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(3)));
        vector_480Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    {
        let __range1: Ptr<vector_480> = v.as_pointer();
        let __begin1: Value<__normal_iterator_366> =
            Rc::new(RefCell::new(({ vector_480Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_366> =
            Rc::new(RefCell::new(({ vector_480Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_366> = __begin1.as_pointer();
            operator_ne_484(___lhs, __end1.as_pointer())
        }) {
            let mut p: Ptr<i32> =
                (({ __normal_iterator_366Impl::operator_deref(&__begin1.as_pointer()) }).read());
            println!("{}", (p.read()));
            ({ __normal_iterator_366Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn operator_ne_483(__lhs: Ptr<__normal_iterator_27>, __rhs: Ptr<__normal_iterator_27>) -> bool {
    unimplemented!()
}
pub fn operator_ne_484(
    __lhs: Ptr<__normal_iterator_366>,
    __rhs: Ptr<__normal_iterator_366>,
) -> bool {
    unimplemented!()
}
pub trait _Vector_base_479Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_12> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_12> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<Ptr<i32>>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_479Impl for Ptr<_Vector_base_479> {}
pub trait __new_allocator_296Impl {
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
impl __new_allocator_296Impl for Ptr<__new_allocator_296> {}
pub trait __new_allocator_297Impl {
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
impl __new_allocator_297Impl for Ptr<__new_allocator_297> {}
pub trait __new_allocator_298Impl {
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
impl __new_allocator_298Impl for Ptr<__new_allocator_298> {}
pub trait __new_allocator_299Impl {
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
impl __new_allocator_299Impl for Ptr<__new_allocator_299> {}
pub trait __new_allocator_300Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<Ptr<i32>>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_301(&self, mut __p: Ptr<Ptr<i32>>, __args: Ptr<Ptr<i32>>) {
        unimplemented!()
    }
}
impl __new_allocator_300Impl for Ptr<__new_allocator_300> {}
pub trait __normal_iterator_27Impl {
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_27> {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<i8>> {
        unimplemented!()
    }
}
impl __normal_iterator_27Impl for Ptr<__normal_iterator_27> {}
pub trait __normal_iterator_366Impl {
    fn operator_deref(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_366> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_366 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<Ptr<i32>>> {
        unimplemented!()
    }
}
impl __normal_iterator_366Impl for Ptr<__normal_iterator_366> {}
pub trait __normal_iterator_367Impl {
    fn base(&self) -> Ptr<Ptr<Ptr<i32>>> {
        unimplemented!()
    }
}
impl __normal_iterator_367Impl for Ptr<__normal_iterator_367> {}
pub trait basic_string_23Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_23>) -> Ptr<basic_string_23> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_23>) -> Ptr<basic_string_23> {
        unimplemented!()
    }
    fn to_basic_string_view_26(&self) -> basic_string_view_26 {
        unimplemented!()
    }
    fn begin_59(&self) -> __normal_iterator_27 {
        unimplemented!()
    }
    fn end_61(&self) -> __normal_iterator_27 {
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
    fn capacity(&self) -> u64 {
        unimplemented!()
    }
    fn reserve_79(&self, __res: u64) {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn data_158(&self) -> Ptr<i8> {
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
    fn __resize_and_overwrite_31(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_32(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_33(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
}
impl basic_string_23Impl for Ptr<basic_string_23> {}
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
    fn c_str(&self) -> Ptr<i32> {
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
pub trait basic_string_40Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_40>) -> Ptr<basic_string_40> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_40>) -> Ptr<basic_string_40> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_40Impl for Ptr<basic_string_40> {}
pub trait basic_string_view_26Impl {
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
impl basic_string_view_26Impl for Ptr<basic_string_view_26> {}
pub trait basic_string_view_470Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_view_470Impl for Ptr<basic_string_view_470> {}
pub trait basic_string_view_471Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u16> {
        unimplemented!()
    }
}
impl basic_string_view_471Impl for Ptr<basic_string_view_471> {}
pub trait basic_string_view_472Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u32> {
        unimplemented!()
    }
}
impl basic_string_view_472Impl for Ptr<basic_string_view_472> {}
pub trait initializer_list_421Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_421Impl for Ptr<initializer_list_421> {}
pub trait integral_constant_44Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_44Impl for Ptr<integral_constant_44> {}
pub trait std__Bit_const_iteratorImpl {
    fn operator_deref(&self) -> bool {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut __i: i64) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut __i: i64) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
}
impl std__Bit_const_iteratorImpl for Ptr<std__Bit_const_iterator> {}
pub trait std__Bit_iteratorImpl {
    fn operator_deref(&self) -> std__Bit_reference {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut __i: i64) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut __i: i64) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
}
impl std__Bit_iteratorImpl for Ptr<std__Bit_iterator> {}
pub trait std__Bit_iterator_baseImpl {
    fn _M_assume_normalized(&self) {
        unimplemented!()
    }
    fn _M_bump_up(&self) {
        unimplemented!()
    }
    fn _M_bump_down(&self) {
        unimplemented!()
    }
    fn _M_incr(&self, mut __i: i64) {
        unimplemented!()
    }
}
impl std__Bit_iterator_baseImpl for Ptr<std__Bit_iterator_base> {}
pub trait std__Bit_referenceImpl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_assign_5(&self, mut __x: bool) -> Ptr<std__Bit_reference> {
        unimplemented!()
    }
    fn copy_assign(&self, __x: Ptr<std__Bit_reference>) -> Ptr<std__Bit_reference> {
        unimplemented!()
    }
}
impl std__Bit_referenceImpl for Ptr<std__Bit_reference> {}
pub trait vector_480Impl {
    fn copy_assign(&self, __x: Ptr<vector_480>) -> Ptr<vector_480> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_480>) -> Ptr<vector_480> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_366 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_367 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_366 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_367 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<Ptr<i32>>) {
        unimplemented!()
    }
    fn emplace_back_481(&self, __args: Ptr<Ptr<i32>>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl vector_480Impl for Ptr<vector_480> {}
pub fn __cpp2rust_init_globals() {}
