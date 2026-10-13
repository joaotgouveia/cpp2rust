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
    pub fn length(mut __s: Ptr<i8>) -> usize {
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
pub type anon_22 = u32;
pub const anon_22__S_local_capacity: anon_22 = 15;
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
    pub fn copy_from(__str: Ptr<basic_string_20>) -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn new_2(mut __s: Ptr<i8>, mut __n: u64, __a: Option<Ptr<allocator_8>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn move_from(__str: Ptr<basic_string_20>) -> Self {
        unimplemented!()
    }
}
impl basic_string_20 {
    pub fn new_3(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_8>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_30 {}
impl Clone for basic_string_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_30 {
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
pub type anon_32 = u32;
pub const anon_32__S_local_capacity: anon_32 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_31 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub fn copy_from(__str: Ptr<basic_string_30>) -> Self {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub fn new_1(mut __s: Ptr<i32>, mut __n: u64, __a: Option<Ptr<allocator_9>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub fn move_from(__str: Ptr<basic_string_30>) -> Self {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub fn new_2(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_9>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_33 {}
impl Clone for basic_string_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_33 {
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
pub type anon_35 = u32;
pub const anon_35__S_local_capacity: anon_35 = 7;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_34 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u16]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_33 {
    pub fn copy_from(__str: Ptr<basic_string_33>) -> Self {
        unimplemented!()
    }
}
impl basic_string_33 {
    pub fn new_1(mut __s: Ptr<u16>, mut __n: u64, __a: Option<Ptr<allocator_10>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_33 {
    pub fn move_from(__str: Ptr<basic_string_33>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_36 {}
impl Clone for basic_string_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_36 {
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
pub type anon_38 = u32;
pub const anon_38__S_local_capacity: anon_38 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_37 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_37 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_36 {
    pub fn copy_from(__str: Ptr<basic_string_36>) -> Self {
        unimplemented!()
    }
}
impl basic_string_36 {
    pub fn new_1(mut __s: Ptr<u32>, mut __n: u64, __a: Option<Ptr<allocator_11>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_36 {
    pub fn move_from(__str: Ptr<basic_string_36>) -> Self {
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
    pub __value: anon_39,
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
pub struct anon_39 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_39 {
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
impl __locale_struct {}
impl __locale_struct {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(264)]
pub struct basic_ios_40 {}
impl Clone for basic_ios_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ios_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(264)]
pub struct basic_ios_41 {}
impl Clone for basic_ios_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ios_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct basic_streambuf_42 {}
impl Clone for basic_streambuf_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct basic_streambuf_43 {}
impl Clone for basic_streambuf_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(280)]
pub struct basic_istream_44 {}
impl Clone for basic_istream_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_istream_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(280)]
pub struct basic_istream_45 {}
impl Clone for basic_istream_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_istream_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(272)]
pub struct basic_ostream_46 {}
impl Clone for basic_ostream_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ostream_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(272)]
pub struct basic_ostream_47 {}
impl Clone for basic_ostream_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ostream_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(288)]
pub struct basic_iostream_48 {}
impl Clone for basic_iostream_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_iostream_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(288)]
pub struct basic_iostream_49 {}
impl Clone for basic_iostream_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_iostream_49 {
    fn default() -> Self {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_type_info {}
impl Clone for std_type_info {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_type_info {
    fn default() -> Self {
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
thread_local!(
    pub static nothrow_50: Value<std_nothrow_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_51 {}
impl Clone for integral_constant_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_52: Value<bool> = unimplemented!();
);
impl integral_constant_51 {}
impl integral_constant_51 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_53 {}
impl Clone for integral_constant_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__false__value_54: Value<bool> = unimplemented!();
);
impl integral_constant_53 {}
impl integral_constant_53 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_55 {}
impl Clone for integral_constant_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_56 {}
impl Clone for integral_constant_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_57 {}
impl Clone for enable_if_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_58 {}
impl Clone for enable_if_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_59 {}
impl Clone for enable_if_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_60 {}
impl Clone for enable_if_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_61 {}
impl Clone for enable_if_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_62 {}
impl Clone for enable_if_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_63 {}
impl Clone for enable_if_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_64 {}
impl Clone for enable_if_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_65 {}
impl Clone for enable_if_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_66 {}
impl Clone for __conditional_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_67 {}
impl Clone for __conditional_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_68 {}
impl Clone for __type_identity_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_69 {}
impl Clone for __type_identity_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_70 {}
impl Clone for __type_identity_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_71 {}
impl Clone for __type_identity_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_72 {}
impl Clone for __type_identity_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_73 {}
impl Clone for __type_identity_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_74 {}
impl Clone for __type_identity_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_75 {}
impl Clone for __type_identity_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_76 {}
impl Clone for __type_identity_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_77 {}
impl Clone for __type_identity_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_78 {}
impl Clone for __type_identity_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_79 {}
impl Clone for __type_identity_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_80 {}
impl Clone for __type_identity_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_81 {}
impl Clone for __type_identity_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_81 {
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
pub struct __or__87 {}
impl Clone for __or__87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__88 {}
impl Clone for __or__88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__89 {}
impl Clone for __or__89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__90 {}
impl Clone for __or__90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__91 {}
impl Clone for __or__91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__92 {}
impl Clone for __or__92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__93 {}
impl Clone for __or__93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__94 {}
impl Clone for __or__94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__95 {}
impl Clone for __or__95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__96 {}
impl Clone for __or__96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__97 {}
impl Clone for __or__97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__98 {}
impl Clone for __or__98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__99 {}
impl Clone for __or__99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__100 {}
impl Clone for __or__100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__101 {}
impl Clone for __or__101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__102 {}
impl Clone for __and__102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__103 {}
impl Clone for __and__103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__104 {}
impl Clone for __and__104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__105 {}
impl Clone for __and__105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__106 {}
impl Clone for __and__106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__107 {}
impl Clone for __and__107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__108 {}
impl Clone for __and__108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__109 {}
impl Clone for __and__109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__110 {}
impl Clone for __and__110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__111 {}
impl Clone for __and__111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__112 {}
impl Clone for __and__112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__113 {}
impl Clone for __and__113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__114 {}
impl Clone for __and__114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__115 {}
impl Clone for __and__115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__116 {}
impl Clone for __and__116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__117 {}
impl Clone for __and__117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__118 {}
impl Clone for __and__118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__119 {}
impl Clone for __and__119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__120 {}
impl Clone for __and__120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__121 {}
impl Clone for __and__121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__122 {}
impl Clone for __not__122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__123 {}
impl Clone for __not__123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__124 {}
impl Clone for __not__124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__125 {}
impl Clone for __not__125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__126 {}
impl Clone for __not__126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__127 {}
impl Clone for __not__127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__128 {}
impl Clone for __not__128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__129 {}
impl Clone for __not__129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__130 {}
impl Clone for __not__130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__131 {}
impl Clone for __not__131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_132 {}
impl Clone for is_reference_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_133 {}
impl Clone for is_reference_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_134 {}
impl Clone for is_reference_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_135 {}
impl Clone for is_reference_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_136 {}
impl Clone for is_reference_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_137 {}
impl Clone for is_reference_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_138 {}
impl Clone for is_reference_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_139 {}
impl Clone for is_reference_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_140 {}
impl Clone for is_reference_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_141 {}
impl Clone for is_reference_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_142 {}
impl Clone for is_reference_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_143 {}
impl Clone for is_reference_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_144 {}
impl Clone for is_reference_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_145 {}
impl Clone for is_reference_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_146 {}
impl Clone for is_function_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_147 {}
impl Clone for is_function_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_148 {}
impl Clone for is_function_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_149 {}
impl Clone for is_function_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_150 {}
impl Clone for is_function_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_151 {}
impl Clone for is_function_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_152 {}
impl Clone for is_function_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_153 {}
impl Clone for is_function_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_154 {}
impl Clone for is_function_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_155 {}
impl Clone for is_function_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_156 {}
impl Clone for is_function_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_157 {}
impl Clone for is_function_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_158 {}
impl Clone for is_void_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_159 {}
impl Clone for is_void_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_160 {}
impl Clone for is_void_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_161 {}
impl Clone for is_void_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_162 {}
impl Clone for is_void_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_163 {}
impl Clone for is_void_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_164 {}
impl Clone for is_void_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_165 {}
impl Clone for is_void_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_166 {}
impl Clone for is_void_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_167 {}
impl Clone for is_void_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_168 {}
impl Clone for is_void_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_169 {}
impl Clone for is_void_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_170 {}
impl Clone for is_void_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_171 {}
impl Clone for is_void_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_172 {}
impl Clone for is_void_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_173 {}
impl Clone for is_void_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_174 {}
impl Clone for is_void_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_175 {}
impl Clone for is_void_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_176 {}
impl Clone for is_void_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_177 {}
impl Clone for is_void_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_178 {}
impl Clone for is_void_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_179 {}
impl Clone for is_void_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_180 {}
impl Clone for remove_cv_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_181 {}
impl Clone for remove_cv_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_182 {}
impl Clone for remove_cv_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_183 {}
impl Clone for remove_cv_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_184 {}
impl Clone for remove_cv_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_185 {}
impl Clone for remove_cv_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_186 {}
impl Clone for remove_cv_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_187 {}
impl Clone for remove_cv_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_188 {}
impl Clone for is_const_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_189 {}
impl Clone for is_const_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_190 {}
impl Clone for is_const_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_191 {}
impl Clone for is_const_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_192 {}
impl Clone for is_const_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_193 {}
impl Clone for __is_array_unknown_bounds_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_194 {}
impl Clone for __is_array_unknown_bounds_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_195 {}
impl Clone for __is_array_unknown_bounds_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_196 {}
impl Clone for __is_array_unknown_bounds_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_197 {}
impl Clone for __is_array_unknown_bounds_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_198 {}
impl Clone for __is_array_unknown_bounds_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_199 {}
impl Clone for __is_array_unknown_bounds_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_200 {}
impl Clone for __is_array_unknown_bounds_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_201 {}
impl Clone for __is_array_unknown_bounds_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_202 {}
impl Clone for __is_array_unknown_bounds_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_203 {}
impl Clone for __is_array_unknown_bounds_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_204 {}
impl Clone for __is_array_unknown_bounds_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_205 {}
impl Clone for __is_integral_helper_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_206 {}
impl Clone for __is_integral_helper_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_207 {}
impl Clone for __is_integral_helper_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_208 {}
impl Clone for __is_integral_helper_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_209 {}
impl Clone for __is_integral_helper_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_210 {}
impl Clone for is_integral_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_211 {}
impl Clone for is_integral_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_212 {}
impl Clone for is_integral_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_213 {}
impl Clone for is_integral_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_214 {}
impl Clone for is_integral_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_215 {}
impl Clone for is_pointer_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_rvalue_reference_216 {}
impl Clone for is_rvalue_reference_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_rvalue_reference_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_object_pointer_217 {}
impl Clone for is_member_object_pointer_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_object_pointer_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_function_pointer_218 {}
impl Clone for is_member_function_pointer_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_class_219 {}
impl Clone for is_class_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_class_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_220 {}
impl Clone for is_same_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_221 {}
impl Clone for is_same_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_222 {}
impl Clone for is_same_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_223 {}
impl Clone for is_same_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_224 {}
impl Clone for is_same_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_225 {}
impl Clone for is_same_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_226 {}
impl Clone for is_same_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_227 {}
impl Clone for is_same_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_228 {}
impl Clone for is_same_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_229 {}
impl Clone for is_same_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_230 {}
impl Clone for is_same_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_231 {}
impl Clone for is_same_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_232 {}
impl Clone for is_same_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_233 {}
impl Clone for is_same_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_234 {}
impl Clone for is_same_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_235 {}
impl Clone for is_same_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_236 {}
impl Clone for is_same_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_237 {}
impl Clone for is_same_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_238 {}
impl Clone for is_same_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_239 {}
impl Clone for is_volatile_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_240 {}
impl Clone for is_volatile_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_241 {}
impl Clone for is_volatile_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_242 {}
impl Clone for is_volatile_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_243 {}
impl Clone for is_volatile_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_empty_244 {}
impl Clone for is_empty_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_empty_245 {}
impl Clone for is_empty_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_245 {
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
pub struct is_default_constructible_246 {}
impl Clone for is_default_constructible_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_247 {}
impl Clone for is_default_constructible_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_248 {}
impl Clone for is_move_constructible_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_249 {}
impl Clone for is_move_constructible_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_250 {}
impl Clone for is_move_constructible_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_251 {}
impl Clone for is_move_constructible_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_252 {}
impl Clone for is_nothrow_default_constructible_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_253 {}
impl Clone for is_nothrow_default_constructible_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_254 {}
impl Clone for is_nothrow_default_constructible_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_255 {}
impl Clone for is_nothrow_default_constructible_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_256 {}
impl Clone for is_nothrow_copy_constructible_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_257 {}
impl Clone for is_nothrow_copy_constructible_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_258 {}
impl Clone for is_nothrow_copy_constructible_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_259 {}
impl Clone for is_nothrow_copy_constructible_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_260 {}
impl Clone for is_nothrow_copy_constructible_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_261 {}
impl Clone for is_nothrow_move_constructible_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_262 {}
impl Clone for is_nothrow_move_constructible_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_263 {}
impl Clone for is_nothrow_move_constructible_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_264 {}
impl Clone for is_assignable_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_265 {}
impl Clone for is_assignable_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_266 {}
impl Clone for is_assignable_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_267 {}
impl Clone for is_assignable_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_268 {}
impl Clone for is_move_assignable_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_269 {}
impl Clone for is_move_assignable_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_270 {}
impl Clone for is_move_assignable_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_271 {}
impl Clone for is_move_assignable_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_272 {}
impl Clone for is_nothrow_move_assignable_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_273 {}
impl Clone for is_nothrow_move_assignable_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_274 {}
impl Clone for is_nothrow_move_assignable_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_274 {
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
pub struct __is_implicitly_default_constructible_impl_275 {}
impl Clone for __is_implicitly_default_constructible_impl_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_impl_276 {}
impl Clone for __is_implicitly_default_constructible_impl_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_safe_277 {}
impl Clone for __is_implicitly_default_constructible_safe_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_safe_278 {}
impl Clone for __is_implicitly_default_constructible_safe_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_279 {}
impl Clone for __is_implicitly_default_constructible_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_280 {}
impl Clone for __is_implicitly_default_constructible_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_281 {}
impl Clone for is_convertible_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_282 {}
impl Clone for is_convertible_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_283 {}
impl Clone for is_convertible_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_284 {}
impl Clone for is_convertible_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_285 {}
impl Clone for is_convertible_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_286 {}
impl Clone for is_convertible_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_287 {}
impl Clone for is_convertible_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_288 {}
impl Clone for is_convertible_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_289 {}
impl Clone for is_convertible_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_290 {}
impl Clone for is_convertible_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_291 {}
impl Clone for is_convertible_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_292 {}
impl Clone for remove_reference_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_293 {}
impl Clone for remove_reference_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_294 {}
impl Clone for remove_reference_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_295 {}
impl Clone for remove_reference_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_296 {}
impl Clone for remove_reference_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_297 {}
impl Clone for remove_reference_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_298 {}
impl Clone for remove_reference_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_299 {}
impl Clone for remove_reference_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_300 {}
impl Clone for remove_reference_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_301 {}
impl Clone for remove_reference_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_302 {}
impl Clone for remove_reference_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_303 {}
impl Clone for remove_reference_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_304 {}
impl Clone for remove_reference_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_305 {}
impl Clone for remove_reference_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_306 {}
impl Clone for remove_reference_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct add_lvalue_reference_307 {}
impl Clone for add_lvalue_reference_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_lvalue_reference_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_308 {}
impl Clone for __cv_selector_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_309 {}
impl Clone for __cv_selector_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_310 {}
impl Clone for __cv_selector_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_311 {}
impl Clone for __cv_selector_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_312 {}
impl Clone for __match_cv_qualifiers_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_313 {}
impl Clone for __match_cv_qualifiers_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_314 {}
impl Clone for __match_cv_qualifiers_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_315 {}
impl Clone for __match_cv_qualifiers_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_316 {}
impl Clone for __match_cv_qualifiers_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_317 {}
impl Clone for __make_unsigned_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_318 {}
impl Clone for __make_unsigned_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_319 {}
impl Clone for __make_unsigned_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_320 {}
impl Clone for __make_unsigned_selector_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_321 {}
impl Clone for __make_unsigned_selector_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_322 {}
impl Clone for __make_unsigned_selector_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_323 {}
impl Clone for __make_unsigned_selector_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_324 {}
impl Clone for __make_unsigned_selector_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_325 {}
impl Clone for __make_unsigned_selector_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_325 {
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
pub struct _List_329 {}
impl Clone for _List_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_330 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_331 {}
impl Clone for _List_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_332 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_333 {}
impl Clone for _List_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_334 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_335 {}
impl Clone for _List_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_336 {}
impl Clone for _List_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_337 {}
impl Clone for _List_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_338 {}
impl Clone for __select_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_339 {}
impl Clone for __select_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_340 {}
impl Clone for __select_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_341 {}
impl Clone for __select_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_342 {}
impl Clone for __select_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_343 {}
impl Clone for __make_signed_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_344 {}
impl Clone for __make_signed_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_345 {}
impl Clone for __make_signed_selector_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_346 {}
impl Clone for __make_signed_selector_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_347 {}
impl Clone for __make_signed_selector_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_348 {}
impl Clone for __make_signed_selector_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_349 {}
impl Clone for __make_signed_selector_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct add_pointer_350 {}
impl Clone for add_pointer_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_pointer_350 {
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
pub struct common_type_351 {}
impl Clone for common_type_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __success_type_352 {}
impl Clone for __success_type_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __success_type_352 {
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
pub struct __result_of_success_353 {}
impl Clone for __result_of_success_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_success_353 {
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
pub struct __result_of_impl_354 {}
impl Clone for __result_of_impl_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_impl_354 {
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
pub struct __invoke_result_355 {}
impl Clone for __invoke_result_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __invoke_result_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_356 {}
impl Clone for __is_tuple_like_impl_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_357 {}
impl Clone for __is_tuple_like_impl_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_358 {}
impl Clone for __is_tuple_like_impl_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_359 {}
impl Clone for __is_tuple_like_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_360 {}
impl Clone for __is_tuple_like_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_361 {}
impl Clone for __is_tuple_like_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_361 {
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
pub struct __is_invocable_impl_362 {}
impl Clone for __is_invocable_impl_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_impl_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_invocable_363 {}
impl Clone for __is_invocable_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_363 {
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
    pub static is_array_v_364: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_365: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_366: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_367: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_368: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_369: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_370: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_371: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_372: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_373: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_374: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_375: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_376: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_377: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_378: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_379: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_380: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_381: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_382: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_383: Value<bool> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std___exception_ptr_exception_ptr {}
impl Clone for std___exception_ptr_exception_ptr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___exception_ptr_exception_ptr {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub fn copy_from(__other: Ptr<std___exception_ptr_exception_ptr>) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub fn move_from(__o: Ptr<std___exception_ptr_exception_ptr>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_nested_exception {}
impl Clone for std_nested_exception {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_nested_exception {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub fn copy_from(_a0: Ptr<std_nested_exception>) -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub fn copy_assign(&self, _a0: Ptr<std_nested_exception>) -> Ptr<std_nested_exception> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_384 {}
impl Clone for _Char_types_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_384 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_385 {}
impl Clone for _Char_types_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_386 {}
impl Clone for char_traits_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_386 {
    pub fn eq(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_386 {
    pub fn length(mut __p: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_386 {
    pub fn find(mut __s: Ptr<i8>, mut __n: usize, __a: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_387 {}
impl Clone for char_traits_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_387 {
    pub fn eq(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_387 {
    pub fn lt(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_387 {
    pub fn compare(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_387 {
    pub fn length(mut __p: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl char_traits_387 {
    pub fn find(mut __s: Ptr<i32>, mut __n: usize, __a: Ptr<i32>) -> Ptr<i32> {
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
pub type anon_388 = u32;
pub const anon_388__ISupper: anon_388 = 256;
pub const anon_388__ISlower: anon_388 = 512;
pub const anon_388__ISalpha: anon_388 = 1024;
pub const anon_388__ISdigit: anon_388 = 2048;
pub const anon_388__ISxdigit: anon_388 = 4096;
pub const anon_388__ISspace: anon_388 = 8192;
pub const anon_388__ISprint: anon_388 = 16384;
pub const anon_388__ISgraph: anon_388 = 32768;
pub const anon_388__ISblank: anon_388 = 1;
pub const anon_388__IScntrl: anon_388 = 2;
pub const anon_388__ISpunct: anon_388 = 4;
pub const anon_388__ISalnum: anon_388 = 8;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(576)]
pub struct ctype_389 {}
impl Clone for ctype_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1344)]
pub struct ctype_390 {}
impl Clone for ctype_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_get_391 {}
impl Clone for num_get_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_get_392 {}
impl Clone for num_get_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_put_393 {}
impl Clone for num_put_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_put_394 {}
impl Clone for num_put_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct numpunct_395 {}
impl Clone for numpunct_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct numpunct_396 {}
impl Clone for numpunct_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct collate_397 {}
impl Clone for collate_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct collate_398 {}
impl Clone for collate_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_398 {
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
#[byte_size(4)]
pub struct sched_param {
    #[offset(0)]
    pub sched_priority: i32,
}
impl Clone for sched_param {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sched_param {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct cpu_set_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __bits: Value<Box<[u64]>>,
}
impl Clone for cpu_set_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for cpu_set_t {
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
#[byte_size(208)]
pub struct timex {
    #[offset(0)]
    pub modes: u32,
    #[offset(8)]
    pub offset: i64,
    #[offset(16)]
    pub freq: i64,
    #[offset(24)]
    pub maxerror: i64,
    #[offset(32)]
    pub esterror: i64,
    #[offset(40)]
    pub status: i32,
    #[offset(48)]
    pub constant: i64,
    #[offset(56)]
    pub precision: i64,
    #[offset(64)]
    pub tolerance: i64,
    #[offset(72)]
    #[byte_size(16)]
    pub time: timeval,
    #[offset(88)]
    pub tick: i64,
    #[offset(96)]
    pub ppsfreq: i64,
    #[offset(104)]
    pub jitter: i64,
    #[offset(112)]
    pub shift: i32,
    #[offset(120)]
    pub stabil: i64,
    #[offset(128)]
    pub jitcnt: i64,
    #[offset(136)]
    pub calcnt: i64,
    #[offset(144)]
    pub errcnt: i64,
    #[offset(152)]
    pub stbcnt: i64,
    #[offset(160)]
    pub tai: i32,
    #[offset(164)]
    pub anon_399: i32,
    #[offset(168)]
    pub anon_400: i32,
    #[offset(172)]
    pub anon_401: i32,
    #[offset(176)]
    pub anon_402: i32,
    #[offset(180)]
    pub anon_403: i32,
    #[offset(184)]
    pub anon_404: i32,
    #[offset(188)]
    pub anon_405: i32,
    #[offset(192)]
    pub anon_406: i32,
    #[offset(196)]
    pub anon_407: i32,
    #[offset(200)]
    pub anon_408: i32,
    #[offset(204)]
    pub anon_409: i32,
}
impl Clone for timex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timex {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct tm {
    #[offset(0)]
    pub tm_sec: i32,
    #[offset(4)]
    pub tm_min: i32,
    #[offset(8)]
    pub tm_hour: i32,
    #[offset(12)]
    pub tm_mday: i32,
    #[offset(16)]
    pub tm_mon: i32,
    #[offset(20)]
    pub tm_year: i32,
    #[offset(24)]
    pub tm_wday: i32,
    #[offset(28)]
    pub tm_yday: i32,
    #[offset(32)]
    pub tm_isdst: i32,
    #[offset(40)]
    pub tm_gmtoff: i64,
    #[offset(48)]
    #[byte_size(8)]
    pub tm_zone: Ptr<i8>,
}
impl Clone for tm {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tm {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct itimerspec {
    #[offset(0)]
    #[byte_size(16)]
    pub it_interval: timespec,
    #[offset(16)]
    #[byte_size(16)]
    pub it_value: timespec,
}
impl Clone for itimerspec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for itimerspec {
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
    pub __value32: anon_410,
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
pub struct anon_410 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_410 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_410 {
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
#[byte_size(200)]
pub struct __jmp_buf_tag {
    #[offset(0)]
    #[byte_size(64)]
    pub __jmpbuf: Value<Box<[i64]>>,
    #[offset(64)]
    pub __mask_was_saved: i32,
    #[offset(72)]
    #[byte_size(128)]
    pub __saved_mask: __sigset_t,
}
impl Clone for __jmp_buf_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __jmp_buf_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_411 = u32;
pub const anon_411_PTHREAD_MUTEX_TIMED_NP: anon_411 = 0;
pub const anon_411_PTHREAD_MUTEX_RECURSIVE_NP: anon_411 = 1;
pub const anon_411_PTHREAD_MUTEX_ERRORCHECK_NP: anon_411 = 2;
pub const anon_411_PTHREAD_MUTEX_ADAPTIVE_NP: anon_411 = 3;
pub const anon_411_PTHREAD_MUTEX_NORMAL: anon_411 = 0;
pub const anon_411_PTHREAD_MUTEX_RECURSIVE: anon_411 = 1;
pub const anon_411_PTHREAD_MUTEX_ERRORCHECK: anon_411 = 2;
pub const anon_411_PTHREAD_MUTEX_DEFAULT: anon_411 = 0;
pub const anon_411_PTHREAD_MUTEX_FAST_NP: anon_411 = 0;
pub type anon_412 = u32;
pub const anon_412_PTHREAD_MUTEX_STALLED: anon_412 = 0;
pub const anon_412_PTHREAD_MUTEX_STALLED_NP: anon_412 = 0;
pub const anon_412_PTHREAD_MUTEX_ROBUST: anon_412 = 1;
pub const anon_412_PTHREAD_MUTEX_ROBUST_NP: anon_412 = 1;
pub type anon_413 = u32;
pub const anon_413_PTHREAD_RWLOCK_PREFER_READER_NP: anon_413 = 0;
pub const anon_413_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_413 = 1;
pub const anon_413_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_413 = 2;
pub const anon_413_PTHREAD_RWLOCK_DEFAULT_NP: anon_413 = 0;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct _pthread_cleanup_buffer {
    #[offset(0)]
    #[byte_size(8)]
    pub __routine: FnPtr<fn(AnyPtr)>,
    #[offset(8)]
    #[byte_size(8)]
    pub __arg: AnyPtr,
    #[offset(16)]
    pub __canceltype: i32,
    #[offset(24)]
    #[byte_size(8)]
    pub __prev: Ptr<_pthread_cleanup_buffer>,
}
impl Clone for _pthread_cleanup_buffer {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _pthread_cleanup_buffer {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_414 = u32;
pub const anon_414_PTHREAD_CANCEL_DEFERRED: anon_414 = 0;
pub const anon_414_PTHREAD_CANCEL_ASYNCHRONOUS: anon_414 = 1;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(72)]
pub struct __cancel_jmp_buf_tag {
    #[offset(0)]
    #[byte_size(64)]
    pub __cancel_jmp_buf: Value<Box<[i64]>>,
    #[offset(64)]
    pub __mask_was_saved: i32,
}
impl Clone for __cancel_jmp_buf_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cancel_jmp_buf_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(104)]
pub struct __pthread_unwind_buf_t {
    #[offset(0)]
    #[byte_size(72)]
    pub __cancel_jmp_buf: Value<Box<[__cancel_jmp_buf_tag]>>,
    #[offset(72)]
    #[byte_size(32)]
    pub __pad: Value<Box<[AnyPtr]>>,
}
impl Clone for __pthread_unwind_buf_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_unwind_buf_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct __pthread_cleanup_frame {
    #[offset(0)]
    #[byte_size(8)]
    pub __cancel_routine: FnPtr<fn(AnyPtr)>,
    #[offset(8)]
    #[byte_size(8)]
    pub __cancel_arg: AnyPtr,
    #[offset(16)]
    pub __do_it: i32,
    #[offset(20)]
    pub __cancel_type: i32,
}
impl Clone for __pthread_cleanup_frame {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cleanup_frame {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct __pthread_cleanup_class {}
impl Clone for __pthread_cleanup_class {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cleanup_class {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __libc_single_threaded_415: Value<i8> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_416 {}
impl Clone for __new_allocator_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_416 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_416 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_416 {
    pub fn copy_from(_a0: Ptr<__new_allocator_416>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_416 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_416>) -> Ptr<__new_allocator_416> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_417 {}
impl Clone for __new_allocator_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub fn copy_from(_a0: Ptr<__new_allocator_417>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_417>) -> Ptr<__new_allocator_417> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_418 {}
impl Clone for __new_allocator_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_418 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_418 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_418 {
    pub fn copy_from(_a0: Ptr<__new_allocator_418>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_418 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_418>) -> Ptr<__new_allocator_418> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_419 {}
impl Clone for __new_allocator_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_419 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_419 {
    pub fn copy_from(_a0: Ptr<__new_allocator_419>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_419 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_419>) -> Ptr<__new_allocator_419> {
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
pub struct __are_same_420 {}
impl Clone for __are_same_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_421 = u32;
pub const anon_421___value: anon_421 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_422 {}
impl Clone for __are_same_422 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_422 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_423 = u32;
pub const anon_423___value: anon_423 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_424 {}
impl Clone for __are_same_424 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_424 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_425 = u32;
pub const anon_425___value: anon_425 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_426 {}
impl Clone for __are_same_426 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_426 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_427 = u32;
pub const anon_427___value: anon_427 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_428 {}
impl Clone for __are_same_428 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_428 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_429 = u32;
pub const anon_429___value: anon_429 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_430 {}
impl Clone for __are_same_430 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_430 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_431 = u32;
pub const anon_431___value: anon_431 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_432 {}
impl Clone for __is_integer_432 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_432 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_433 = u32;
pub const anon_433___value: anon_433 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_434 {}
impl Clone for __is_integer_434 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_434 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_435 = u32;
pub const anon_435___value: anon_435 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_436 {}
impl Clone for __is_integer_436 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_436 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_437 = u32;
pub const anon_437___value: anon_437 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_438 {}
impl Clone for __is_integer_438 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_438 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_439 = u32;
pub const anon_439___value: anon_439 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_440 {}
impl Clone for __is_integer_440 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_440 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_441 = u32;
pub const anon_441___value: anon_441 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_442 {}
impl Clone for __is_integer_442 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_442 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_443 = u32;
pub const anon_443___value: anon_443 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_444 {}
impl Clone for __is_integer_444 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_444 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_445 = u32;
pub const anon_445___value: anon_445 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_446 {}
impl Clone for __is_integer_446 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_446 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_447 = u32;
pub const anon_447___value: anon_447 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_448 {}
impl Clone for __is_integer_448 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_448 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_449 = u32;
pub const anon_449___value: anon_449 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_450 {}
impl Clone for __is_integer_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_451 = u32;
pub const anon_451___value: anon_451 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_452 {}
impl Clone for __memcpyable_integer_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_452 {
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
pub struct iterator_453 {}
impl Clone for iterator_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_454 {}
impl Clone for iterator_traits_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_454 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_455 {}
impl Clone for iterator_traits_455 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_455 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_456 {}
impl Clone for iterator_traits_456 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_456 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_457 {}
impl Clone for iterator_traits_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_458 {}
impl Clone for iterator_traits_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_type_459 {}
impl Clone for __conditional_type_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_460 {}
impl Clone for __ptr_traits_ptr_to_460 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_460 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_460 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_461 {}
impl Clone for __ptr_traits_ptr_to_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_461 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_462 {}
impl Clone for __ptr_traits_ptr_to_462 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_462 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_462 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_463 {}
impl Clone for __ptr_traits_ptr_to_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_463 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_463 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_464 {}
impl Clone for __ptr_traits_ptr_to_464 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_464 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_464 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_465 {}
impl Clone for __ptr_traits_ptr_to_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_465 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_466 {}
impl Clone for __ptr_traits_ptr_to_466 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_466 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_466 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_467 {}
impl Clone for __ptr_traits_ptr_to_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_467 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_468 {}
impl Clone for pointer_traits_468 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_468 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_469 {}
impl Clone for pointer_traits_469 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_469 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_470 {}
impl Clone for pointer_traits_470 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_470 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_471 {}
impl Clone for pointer_traits_471 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_471 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_472 {}
impl Clone for pointer_traits_472 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_472 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_473 {}
impl Clone for pointer_traits_473 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_473 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_474 {}
impl Clone for pointer_traits_474 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_474 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_475 {}
impl Clone for pointer_traits_475 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_475 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_476 {}
impl Clone for __normal_iterator_476 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_476 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_476 {}
impl __normal_iterator_476 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_477 {}
impl Clone for binary_function_477 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_477 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_478 {}
impl Clone for binary_function_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_478 {}
impl binary_function_478 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_479 {}
impl Clone for binary_function_479 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_479 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_479 {}
impl binary_function_479 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_480 {}
impl Clone for greater_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_481 {}
impl Clone for less_481 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_481 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_482 {}
impl Clone for less_482 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_482 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_482 {}
impl less_482 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_483 {}
impl Clone for less_483 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_483 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_483 {}
impl less_483 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_484 {}
impl Clone for greater_equal_484 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_484 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_485 {}
impl Clone for less_equal_485 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_485 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_486 {}
impl Clone for __is_integer_nonstrict_486 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_486 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_487 = u32;
pub const anon_487___width: anon_487 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_488 {}
impl Clone for __is_integer_nonstrict_488 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_488 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_489 = u32;
pub const anon_489___width: anon_489 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_490 {}
impl Clone for __is_integer_nonstrict_490 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_490 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_491 = u32;
pub const anon_491___width: anon_491 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_492 {}
impl Clone for __is_integer_nonstrict_492 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_492 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_493 = u32;
pub const anon_493___width: anon_493 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_494 {}
impl Clone for __is_integer_nonstrict_494 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_494 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_495 = u32;
pub const anon_495___width: anon_495 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_496 {}
impl Clone for __is_integer_nonstrict_496 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_496 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_497 = u32;
pub const anon_497___width: anon_497 = 8;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_498 {}
impl Clone for __is_integer_nonstrict_498 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_498 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_499 = u32;
pub const anon_499___width: anon_499 = 16;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_500 {}
impl Clone for __numeric_traits_integer_500 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_500 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____is_signed_501: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____digits_502: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____max_503: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____min_504: Value<i32> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_505 {}
impl Clone for __numeric_traits_integer_505 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_505 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_506: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_507: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_508: Value<i64> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____min_509: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_510 {}
impl Clone for __numeric_traits_integer_510 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_510 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_511: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____digits_512: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____max_513: Value<u32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_514 {}
impl Clone for __numeric_traits_integer_514 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_514 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_515: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_516: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_517: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_518 {}
impl Clone for __numeric_traits_integer_518 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_518 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_519: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_520: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_521: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_522 {}
impl Clone for __numeric_traits_integer_522 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_522 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_char____is_signed_523: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_char____digits_524: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_char____max_525: Value<i8> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_526 {}
impl Clone for __numeric_traits_integer_526 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_526 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____is_signed_527: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____digits_528: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____max_529: Value<i16> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____min_530: Value<i16> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_531 {}
impl Clone for __numeric_traits_floating_531 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_531 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_float____max_exponent10_532: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_533 {}
impl Clone for __numeric_traits_floating_533 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_533 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_double____max_exponent10_534: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_535 {}
impl Clone for __numeric_traits_floating_535 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_535 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_536: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_537 {}
impl Clone for __numeric_traits_537 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_537 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_538 {}
impl Clone for __numeric_traits_538 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_538 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_539 {}
impl Clone for __numeric_traits_539 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_539 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_540 {}
impl Clone for __numeric_traits_540 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_540 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_541 {}
impl Clone for __numeric_traits_541 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_541 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_542 {}
impl Clone for __numeric_traits_542 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_542 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_543 {}
impl Clone for __numeric_traits_543 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_543 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_544 {}
impl Clone for __numeric_traits_544 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_544 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_element_545 {}
impl Clone for tuple_element_545 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_545 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_element_546 {}
impl Clone for tuple_element_546 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_546 {
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
pub struct _Nth_type_547 {}
impl Clone for _Nth_type_547 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_547 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Nth_type_548 {}
impl Clone for _Nth_type_548 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_548 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
    pub static piecewise_construct_549: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_550 {}
impl Clone for tuple_550 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_550 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_550 {}
impl tuple_550 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct tuple_551 {}
impl Clone for tuple_551 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_551 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_551 {
    pub fn copy_from(_a0: Ptr<tuple_551>) -> Self {
        unimplemented!()
    }
}
impl tuple_551 {
    pub fn move_from(_a0: Ptr<tuple_551>) -> Self {
        unimplemented!()
    }
}
impl tuple_551 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
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
pub struct initializer_list_552 {}
impl Clone for initializer_list_552 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_552 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_552 {}
impl initializer_list_552 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_553 {}
impl Clone for initializer_list_553 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_553 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_553 {}
impl initializer_list_553 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_554 {}
impl Clone for initializer_list_554 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_554 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_554 {}
impl initializer_list_554 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_555 {}
impl Clone for initializer_list_555 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_555 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_555 {}
impl initializer_list_555 {}
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
pub struct __is_allocator_557 {}
impl Clone for __is_allocator_557 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_557 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_558 {}
impl Clone for __is_allocator_558 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_558 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_559 {}
impl Clone for __is_allocator_559 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_559 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_560 {}
impl Clone for __is_allocator_560 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_560 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_561 {}
impl Clone for __alloc_traits_561 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_561 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_562 {}
impl Clone for rebind_562 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_562 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_563 {}
impl Clone for __alloc_traits_563 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_563 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_564 {}
impl Clone for rebind_564 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_564 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_565 {}
impl Clone for __alloc_traits_565 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_565 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_566 {}
impl Clone for rebind_566 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_566 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_567 {}
impl Clone for __alloc_traits_567 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_567 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_568 {}
impl Clone for rebind_568 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_568 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_569 {}
impl Clone for __hash_base_569 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_569 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_570 {}
impl Clone for __hash_base_570 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_570 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_571 {}
impl Clone for __hash_base_571 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_571 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_572 {}
impl Clone for __hash_base_572 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_572 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_573 {}
impl Clone for __hash_base_573 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_573 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_574 {}
impl Clone for __hash_base_574 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_574 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_575 {}
impl Clone for __hash_base_575 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_575 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_576 {}
impl Clone for __hash_base_576 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_576 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_577 {}
impl Clone for __hash_base_577 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_577 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_578 {}
impl Clone for __hash_base_578 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_578 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_579 {}
impl Clone for __hash_base_579 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_579 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_580 {}
impl Clone for __hash_base_580 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_580 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_581 {}
impl Clone for __hash_base_581 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_581 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_582 {}
impl Clone for __hash_base_582 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_582 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_583 {}
impl Clone for __hash_base_583 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_583 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_584 {}
impl Clone for __hash_base_584 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_584 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_585 {}
impl Clone for __hash_base_585 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_585 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_586 {}
impl Clone for __hash_base_586 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_586 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_587 {}
impl Clone for __hash_base_587 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_587 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_588 {}
impl Clone for __hash_base_588 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_588 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_589 {}
impl Clone for __hash_base_589 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_589 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_590 {}
impl Clone for __hash_base_590 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_590 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_591 {}
impl Clone for __hash_base_591 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_591 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_592 {}
impl Clone for __hash_base_592 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_592 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_593 {}
impl Clone for __hash_base_593 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_593 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_594 {}
impl Clone for __hash_base_594 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_594 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_595 {}
impl Clone for __hash_base_595 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_595 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_596 {}
impl Clone for hash_596 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_596 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_597 {}
impl Clone for hash_597 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_597 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_598 {}
impl Clone for hash_598 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_598 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_599 {}
impl Clone for hash_599 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_599 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_600 {}
impl Clone for hash_600 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_600 {
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
    pub fn hash_601(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_602(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_603(__val: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn __hash_combine_604(__val: Ptr<Ptr<std_error_category>>, mut __hash: usize) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_23 {}
impl Clone for basic_string_view_23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_23 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_23 {
    pub fn copy_from(_a0: Ptr<basic_string_view_23>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_23 {
    pub fn new_1(mut __str: Ptr<i8>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_23 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_23>) -> Ptr<basic_string_view_23> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_605 {}
impl Clone for basic_string_view_605 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_605 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_605 {
    pub fn copy_from(_a0: Ptr<basic_string_view_605>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_605 {
    pub fn new_1(mut __str: Ptr<i32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_605 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_605>) -> Ptr<basic_string_view_605> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_606 {}
impl Clone for basic_string_view_606 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_606 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_606 {
    pub fn copy_from(_a0: Ptr<basic_string_view_606>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_606 {
    pub fn new_1(mut __str: Ptr<u16>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_606 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_606>) -> Ptr<basic_string_view_606> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_607 {}
impl Clone for basic_string_view_607 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_607 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_607 {
    pub fn copy_from(_a0: Ptr<basic_string_view_607>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_607 {
    pub fn new_1(mut __str: Ptr<u32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_607 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_607>) -> Ptr<basic_string_view_607> {
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
    pub static __integer_to_chars_is_unsigned_608: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_609: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_610: Value<bool> = unimplemented!();
);
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
    pub static allocator_arg_611: Value<std_allocator_arg_t> = unimplemented!();
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
pub struct __is_empty_non_tuple_612 {}
impl Clone for __is_empty_non_tuple_612 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_612 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_empty_non_tuple_613 {}
impl Clone for __is_empty_non_tuple_613 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_613 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Head_base_614 {
    #[offset(0)]
    #[byte_size(1)]
    pub _M_head_impl: default_delete_615,
}
impl Clone for _Head_base_614 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_614 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_614 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_614 {
    pub fn copy_from(_a0: Ptr<_Head_base_614>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_614 {
    pub fn move_from(_a0: Ptr<_Head_base_614>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_614 {
    pub fn _M_head_12(__b: Ptr<_Head_base_614>) -> Ptr<default_delete_615> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Head_base_616 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_head_impl: Ptr<i32>,
}
impl Clone for _Head_base_616 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_616 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_616 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_616 {
    pub fn copy_from(_a0: Ptr<_Head_base_616>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_616 {
    pub fn move_from(_a0: Ptr<_Head_base_616>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_616 {
    pub fn _M_head_12(__b: Ptr<_Head_base_616>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl _Head_base_616 {
    pub fn _M_head_13(__b: Ptr<_Head_base_616>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Tuple_impl_617 {}
impl Clone for _Tuple_impl_617 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_617 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_617 {
    pub fn _M_head_1(__t: Ptr<_Tuple_impl_617>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl _Tuple_impl_617 {
    pub fn _M_head_2(__t: Ptr<_Tuple_impl_617>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl _Tuple_impl_617 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_617 {
    pub fn copy_from(_a0: Ptr<_Tuple_impl_617>) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_617 {
    pub fn move_from(_a0: Ptr<_Tuple_impl_617>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Tuple_impl_618 {}
impl Clone for _Tuple_impl_618 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_618 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_618 {
    pub fn _M_head_1(__t: Ptr<_Tuple_impl_618>) -> Ptr<default_delete_615> {
        unimplemented!()
    }
}
impl _Tuple_impl_618 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_618 {
    pub fn copy_from(_a0: Ptr<_Tuple_impl_618>) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_618 {
    pub fn move_from(__in: Ptr<_Tuple_impl_618>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _TupleConstraints_619 {}
impl Clone for _TupleConstraints_619 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _TupleConstraints_619 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _TupleConstraints_619 {
    pub fn __is_implicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
impl _TupleConstraints_619 {
    pub fn __is_explicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_locale {}
impl Clone for std_locale {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_locale_ctype_620: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_numeric_621: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_collate_622: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_time_623: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_monetary_624: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_messages_625: Value<i32> = unimplemented!();
);
pub type anon_626 = u32;
pub const anon_626__S_categories_size: anon_626 = 12;
impl std_locale {
    pub fn copy_from(__other: Ptr<std_locale>) -> Self {
        unimplemented!()
    }
}
impl std_locale {
    pub fn new_1(mut __s: Ptr<i8>) -> Self {
        unimplemented!()
    }
}
impl std_locale {
    pub fn new_2(__base: Ptr<std_locale>, mut __s: Ptr<i8>, mut __cat: i32) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_locale_facet {}
impl Clone for std_locale_facet {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale_facet {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_locale_id {}
impl Clone for std_locale_id {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale_id {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(40)]
pub struct std_locale__Impl {}
impl Clone for std_locale__Impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale__Impl {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_errc = i32;
pub const std_errc_address_family_not_supported: std_errc = 97;
pub const std_errc_address_in_use: std_errc = 98;
pub const std_errc_address_not_available: std_errc = 99;
pub const std_errc_already_connected: std_errc = 106;
pub const std_errc_argument_list_too_long: std_errc = 7;
pub const std_errc_argument_out_of_domain: std_errc = 33;
pub const std_errc_bad_address: std_errc = 14;
pub const std_errc_bad_file_descriptor: std_errc = 9;
pub const std_errc_bad_message: std_errc = 74;
pub const std_errc_broken_pipe: std_errc = 32;
pub const std_errc_connection_aborted: std_errc = 103;
pub const std_errc_connection_already_in_progress: std_errc = 114;
pub const std_errc_connection_refused: std_errc = 111;
pub const std_errc_connection_reset: std_errc = 104;
pub const std_errc_cross_device_link: std_errc = 18;
pub const std_errc_destination_address_required: std_errc = 89;
pub const std_errc_device_or_resource_busy: std_errc = 16;
pub const std_errc_directory_not_empty: std_errc = 39;
pub const std_errc_executable_format_error: std_errc = 8;
pub const std_errc_file_exists: std_errc = 17;
pub const std_errc_file_too_large: std_errc = 27;
pub const std_errc_filename_too_long: std_errc = 36;
pub const std_errc_function_not_supported: std_errc = 38;
pub const std_errc_host_unreachable: std_errc = 113;
pub const std_errc_identifier_removed: std_errc = 43;
pub const std_errc_illegal_byte_sequence: std_errc = 84;
pub const std_errc_inappropriate_io_control_operation: std_errc = 25;
pub const std_errc_interrupted: std_errc = 4;
pub const std_errc_invalid_argument: std_errc = 22;
pub const std_errc_invalid_seek: std_errc = 29;
pub const std_errc_io_error: std_errc = 5;
pub const std_errc_is_a_directory: std_errc = 21;
pub const std_errc_message_size: std_errc = 90;
pub const std_errc_network_down: std_errc = 100;
pub const std_errc_network_reset: std_errc = 102;
pub const std_errc_network_unreachable: std_errc = 101;
pub const std_errc_no_buffer_space: std_errc = 105;
pub const std_errc_no_child_process: std_errc = 10;
pub const std_errc_no_link: std_errc = 67;
pub const std_errc_no_lock_available: std_errc = 37;
pub const std_errc_no_message_available: std_errc = 61;
pub const std_errc_no_message: std_errc = 42;
pub const std_errc_no_protocol_option: std_errc = 92;
pub const std_errc_no_space_on_device: std_errc = 28;
pub const std_errc_no_stream_resources: std_errc = 63;
pub const std_errc_no_such_device_or_address: std_errc = 6;
pub const std_errc_no_such_device: std_errc = 19;
pub const std_errc_no_such_file_or_directory: std_errc = 2;
pub const std_errc_no_such_process: std_errc = 3;
pub const std_errc_not_a_directory: std_errc = 20;
pub const std_errc_not_a_socket: std_errc = 88;
pub const std_errc_not_a_stream: std_errc = 60;
pub const std_errc_not_connected: std_errc = 107;
pub const std_errc_not_enough_memory: std_errc = 12;
pub const std_errc_not_supported: std_errc = 95;
pub const std_errc_operation_canceled: std_errc = 125;
pub const std_errc_operation_in_progress: std_errc = 115;
pub const std_errc_operation_not_permitted: std_errc = 1;
pub const std_errc_operation_not_supported: std_errc = 95;
pub const std_errc_operation_would_block: std_errc = 11;
pub const std_errc_owner_dead: std_errc = 130;
pub const std_errc_permission_denied: std_errc = 13;
pub const std_errc_protocol_error: std_errc = 71;
pub const std_errc_protocol_not_supported: std_errc = 93;
pub const std_errc_read_only_file_system: std_errc = 30;
pub const std_errc_resource_deadlock_would_occur: std_errc = 35;
pub const std_errc_resource_unavailable_try_again: std_errc = 11;
pub const std_errc_result_out_of_range: std_errc = 34;
pub const std_errc_state_not_recoverable: std_errc = 131;
pub const std_errc_stream_timeout: std_errc = 62;
pub const std_errc_text_file_busy: std_errc = 26;
pub const std_errc_timed_out: std_errc = 110;
pub const std_errc_too_many_files_open_in_system: std_errc = 23;
pub const std_errc_too_many_files_open: std_errc = 24;
pub const std_errc_too_many_links: std_errc = 31;
pub const std_errc_too_many_symbolic_link_levels: std_errc = 40;
pub const std_errc_value_too_large: std_errc = 75;
pub const std_errc_wrong_protocol_type: std_errc = 91;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std___cow_string {
    #[offset(0)]
    #[byte_size(8)]
    pub anon_627: anon_627,
}
impl Clone for std___cow_string {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___cow_string {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_627 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<i8>,
    #[offset(0)]
    #[byte_size(8)]
    pub _M_bytes: Value<Box<[i8]>>,
}
impl Clone for anon_627 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_627 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___cow_string {
    pub fn copy_from(_a0: Ptr<std___cow_string>) -> Self {
        unimplemented!()
    }
}
impl std___cow_string {
    pub fn move_from(_a0: Ptr<std___cow_string>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_logic_error {}
impl Clone for std_logic_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_logic_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_logic_error {
    pub fn move_from(_a0: Ptr<std_logic_error>) -> Self {
        unimplemented!()
    }
}
impl std_logic_error {
    pub fn copy_from(_a0: Ptr<std_logic_error>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_runtime_error {}
impl Clone for std_runtime_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_runtime_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub fn new_1(__arg: Ptr<basic_string_20>) -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub fn move_from(_a0: Ptr<std_runtime_error>) -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub fn copy_from(_a0: Ptr<std_runtime_error>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_628 {}
impl Clone for is_error_code_enum_628 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_628 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_629 {}
impl Clone for is_error_code_enum_629 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_629 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_630 {}
impl Clone for is_error_code_enum_630 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_630 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_631 {}
impl Clone for is_error_code_enum_631 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_631 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_632 {}
impl Clone for is_error_code_enum_632 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_632 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_633 {}
impl Clone for is_error_code_enum_633 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_633 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_634 {}
impl Clone for is_error_code_enum_634 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_634 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_635 {}
impl Clone for is_error_condition_enum_635 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_635 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_636 {}
impl Clone for is_error_condition_enum_636 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_636 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_637 {}
impl Clone for is_error_condition_enum_637 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_637 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_error_code {}
impl Clone for std_error_code {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_error_code {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub fn new_1(mut __v: i32, __cat: Ptr<std_error_category>) -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub fn copy_from(_a0: Ptr<std_error_code>) -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub fn copy_assign(&self, _a0: Ptr<std_error_code>) -> Ptr<std_error_code> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_error_condition {}
impl Clone for std_error_condition {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_error_condition {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub fn new_1(mut __v: i32, __cat: Ptr<std_error_category>) -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub fn copy_from(_a0: Ptr<std_error_condition>) -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub fn copy_assign(&self, _a0: Ptr<std_error_condition>) -> Ptr<std_error_condition> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct std_system_error {}
impl Clone for std_system_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_system_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub fn new_1(__ec: std_error_code, mut __what: Ptr<i8>) -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub fn copy_from(_a0: Ptr<std_system_error>) -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub fn copy_assign(&self, _a0: Ptr<std_system_error>) -> Ptr<std_system_error> {
        unimplemented!()
    }
}
pub type std__Ios_Fmtflags = i32;
pub const std__Ios_Fmtflags__S_boolalpha: std__Ios_Fmtflags = 1;
pub const std__Ios_Fmtflags__S_dec: std__Ios_Fmtflags = 2;
pub const std__Ios_Fmtflags__S_fixed: std__Ios_Fmtflags = 4;
pub const std__Ios_Fmtflags__S_hex: std__Ios_Fmtflags = 8;
pub const std__Ios_Fmtflags__S_internal: std__Ios_Fmtflags = 16;
pub const std__Ios_Fmtflags__S_left: std__Ios_Fmtflags = 32;
pub const std__Ios_Fmtflags__S_oct: std__Ios_Fmtflags = 64;
pub const std__Ios_Fmtflags__S_right: std__Ios_Fmtflags = 128;
pub const std__Ios_Fmtflags__S_scientific: std__Ios_Fmtflags = 256;
pub const std__Ios_Fmtflags__S_showbase: std__Ios_Fmtflags = 512;
pub const std__Ios_Fmtflags__S_showpoint: std__Ios_Fmtflags = 1024;
pub const std__Ios_Fmtflags__S_showpos: std__Ios_Fmtflags = 2048;
pub const std__Ios_Fmtflags__S_skipws: std__Ios_Fmtflags = 4096;
pub const std__Ios_Fmtflags__S_unitbuf: std__Ios_Fmtflags = 8192;
pub const std__Ios_Fmtflags__S_uppercase: std__Ios_Fmtflags = 16384;
pub const std__Ios_Fmtflags__S_adjustfield: std__Ios_Fmtflags = 176;
pub const std__Ios_Fmtflags__S_basefield: std__Ios_Fmtflags = 74;
pub const std__Ios_Fmtflags__S_floatfield: std__Ios_Fmtflags = 260;
pub const std__Ios_Fmtflags__S_ios_fmtflags_end: std__Ios_Fmtflags = 65536;
pub const std__Ios_Fmtflags__S_ios_fmtflags_max: std__Ios_Fmtflags = 2147483647;
pub const std__Ios_Fmtflags__S_ios_fmtflags_min: std__Ios_Fmtflags = -2147483648;
pub type std__Ios_Openmode = i32;
pub const std__Ios_Openmode__S_app: std__Ios_Openmode = 1;
pub const std__Ios_Openmode__S_ate: std__Ios_Openmode = 2;
pub const std__Ios_Openmode__S_bin: std__Ios_Openmode = 4;
pub const std__Ios_Openmode__S_in: std__Ios_Openmode = 8;
pub const std__Ios_Openmode__S_out: std__Ios_Openmode = 16;
pub const std__Ios_Openmode__S_trunc: std__Ios_Openmode = 32;
pub const std__Ios_Openmode__S_noreplace: std__Ios_Openmode = 64;
pub const std__Ios_Openmode__S_ios_openmode_end: std__Ios_Openmode = 65536;
pub const std__Ios_Openmode__S_ios_openmode_max: std__Ios_Openmode = 2147483647;
pub const std__Ios_Openmode__S_ios_openmode_min: std__Ios_Openmode = -2147483648;
pub type std__Ios_Iostate = i32;
pub const std__Ios_Iostate__S_goodbit: std__Ios_Iostate = 0;
pub const std__Ios_Iostate__S_badbit: std__Ios_Iostate = 1;
pub const std__Ios_Iostate__S_eofbit: std__Ios_Iostate = 2;
pub const std__Ios_Iostate__S_failbit: std__Ios_Iostate = 4;
pub const std__Ios_Iostate__S_ios_iostate_end: std__Ios_Iostate = 65536;
pub const std__Ios_Iostate__S_ios_iostate_max: std__Ios_Iostate = 2147483647;
pub const std__Ios_Iostate__S_ios_iostate_min: std__Ios_Iostate = -2147483648;
pub type std__Ios_Seekdir = u32;
pub const std__Ios_Seekdir__S_beg: std__Ios_Seekdir = 0;
pub const std__Ios_Seekdir__S_cur: std__Ios_Seekdir = 1;
pub const std__Ios_Seekdir__S_end: std__Ios_Seekdir = 2;
pub const std__Ios_Seekdir__S_ios_seekdir_end: std__Ios_Seekdir = 65536;
pub type std_io_errc = i32;
pub const std_io_errc_stream: std_io_errc = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(216)]
pub struct std_ios_base {}
impl Clone for std_ios_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ios_base {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_ios_base_boolalpha_638: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_dec_639: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_fixed_640: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_hex_641: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_internal_642: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_left_643: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_oct_644: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_right_645: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_scientific_646: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showbase_647: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showpoint_648: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showpos_649: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_skipws_650: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_unitbuf_651: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_uppercase_652: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_adjustfield_653: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_basefield_654: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_floatfield_655: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_badbit_656: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_eofbit_657: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_failbit_658: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_goodbit_659: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_in_660: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_out_661: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_cur_662: Value<std__Ios_Seekdir> = unimplemented!();
);
pub type std_ios_base_event = u32;
pub const std_ios_base_event_erase_event: std_ios_base_event = 0;
pub const std_ios_base_event_imbue_event: std_ios_base_event = 1;
pub const std_ios_base_event_copyfmt_event: std_ios_base_event = 2;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_ios_base__Callback_list {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_next: Ptr<std_ios_base__Callback_list>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_fn: FnPtr<fn(std_ios_base_event, Ptr<std_ios_base>, i32)>,
    #[offset(16)]
    pub _M_index: i32,
    #[offset(20)]
    pub _M_refcount: i32,
}
impl Clone for std_ios_base__Callback_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ios_base__Callback_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_ios_base__Words {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_pword: AnyPtr,
    #[offset(8)]
    pub _M_iword: i64,
}
impl Clone for std_ios_base__Words {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ios_base__Words {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_ios_base__Words {}
impl std_ios_base__Words {}
impl std_ios_base__Words {
    pub fn copy_assign(&self, _a0: Ptr<std_ios_base__Words>) -> Ptr<std_ios_base__Words> {
        unimplemented!()
    }
}
impl std_ios_base__Words {
    pub fn move_assign(&self, _a0: Ptr<std_ios_base__Words>) -> Ptr<std_ios_base__Words> {
        unimplemented!()
    }
}
pub type anon_663 = u32;
pub const anon_663__S_local_word_size: anon_663 = 8;
pub type anon_664 = i32;
pub const anon_664___ISwupper: anon_664 = 0;
pub const anon_664___ISwlower: anon_664 = 1;
pub const anon_664___ISwalpha: anon_664 = 2;
pub const anon_664___ISwdigit: anon_664 = 3;
pub const anon_664___ISwxdigit: anon_664 = 4;
pub const anon_664___ISwspace: anon_664 = 5;
pub const anon_664___ISwprint: anon_664 = 6;
pub const anon_664___ISwgraph: anon_664 = 7;
pub const anon_664___ISwblank: anon_664 = 8;
pub const anon_664___ISwcntrl: anon_664 = 9;
pub const anon_664___ISwpunct: anon_664 = 10;
pub const anon_664___ISwalnum: anon_664 = 11;
pub const anon_664__ISwupper: anon_664 = 16777216;
pub const anon_664__ISwlower: anon_664 = 33554432;
pub const anon_664__ISwalpha: anon_664 = 67108864;
pub const anon_664__ISwdigit: anon_664 = 134217728;
pub const anon_664__ISwxdigit: anon_664 = 268435456;
pub const anon_664__ISwspace: anon_664 = 536870912;
pub const anon_664__ISwprint: anon_664 = 1073741824;
pub const anon_664__ISwgraph: anon_664 = -2147483648;
pub const anon_664__ISwblank: anon_664 = 65536;
pub const anon_664__ISwcntrl: anon_664 = 131072;
pub const anon_664__ISwpunct: anon_664 = 262144;
pub const anon_664__ISwalnum: anon_664 = 524288;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ctype_base {}
impl Clone for std_ctype_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ctype_base {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_ctype_base_upper_665: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_lower_666: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_alpha_667: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_digit_668: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_xdigit_669: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_space_670: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_print_671: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_graph_672: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_cntrl_673: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_punct_674: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_alnum_675: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_blank_676: Value<u16> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___num_base {}
impl Clone for std___num_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___num_base {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_677 = u32;
pub const anon_677__S_ominus: anon_677 = 0;
pub const anon_677__S_oplus: anon_677 = 1;
pub const anon_677__S_ox: anon_677 = 2;
pub const anon_677__S_oX: anon_677 = 3;
pub const anon_677__S_odigits: anon_677 = 4;
pub const anon_677__S_odigits_end: anon_677 = 20;
pub const anon_677__S_oudigits: anon_677 = 20;
pub const anon_677__S_oudigits_end: anon_677 = 36;
pub const anon_677__S_oe: anon_677 = 18;
pub const anon_677__S_oE: anon_677 = 34;
pub const anon_677__S_oend: anon_677 = 36;
thread_local!(
    pub static std___num_base__S_atoms_out_678: Value<Ptr<i8>> = unimplemented!();
);
thread_local!(
    pub static std___num_base__S_atoms_in_679: Value<Ptr<i8>> = unimplemented!();
);
pub type anon_680 = u32;
pub const anon_680__S_iminus: anon_680 = 0;
pub const anon_680__S_iplus: anon_680 = 1;
pub const anon_680__S_ix: anon_680 = 2;
pub const anon_680__S_iX: anon_680 = 3;
pub const anon_680__S_izero: anon_680 = 4;
pub const anon_680__S_ie: anon_680 = 18;
pub const anon_680__S_iE: anon_680 = 24;
pub const anon_680__S_iend: anon_680 = 26;
thread_local!(
    pub static cout_681: Value<basic_ostream_46> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct default_delete_615 {}
impl Clone for default_delete_615 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for default_delete_615 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl default_delete_615 {}
impl default_delete_615 {}
impl default_delete_615 {
    pub fn copy_assign(&self, _a0: Ptr<default_delete_615>) -> Ptr<default_delete_615> {
        unimplemented!()
    }
}
impl default_delete_615 {
    pub fn move_assign(&self, _a0: Ptr<default_delete_615>) -> Ptr<default_delete_615> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __uniq_ptr_impl_682 {}
impl Clone for __uniq_ptr_impl_682 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_impl_682 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Ptr_683 {}
impl Clone for _Ptr_683 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_683 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_682 {
    pub fn new_1(mut __p: Ptr<i32>) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_682 {
    pub fn move_from(__u: Ptr<__uniq_ptr_impl_682>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __uniq_ptr_data_684 {}
impl Clone for __uniq_ptr_data_684 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_data_684 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_684 {
    pub fn move_from(_a0: Ptr<__uniq_ptr_data_684>) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_684 {
    pub fn move_assign(&self, _a0: Ptr<__uniq_ptr_data_684>) -> Ptr<__uniq_ptr_data_684> {
        unimplemented!()
    }
}
impl __uniq_ptr_data_684 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct unique_ptr_685 {}
impl Clone for unique_ptr_685 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unique_ptr_685 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unique_ptr_685 {
    pub fn move_from(_a0: Ptr<unique_ptr_685>) -> Self {
        unimplemented!()
    }
}
impl unique_ptr_685 {
    pub fn move_assign(&self, _a0: Ptr<unique_ptr_685>) -> Ptr<unique_ptr_685> {
        unimplemented!()
    }
}
impl unique_ptr_685 {
    pub fn new(mut __p: Ptr<i32>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _MakeUniq_686 {}
impl Clone for _MakeUniq_686 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _MakeUniq_686 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type __gnu_cxx__Lock_policy = u32;
pub const __gnu_cxx__Lock_policy__S_single: __gnu_cxx__Lock_policy = 0;
pub const __gnu_cxx__Lock_policy__S_mutex: __gnu_cxx__Lock_policy = 1;
pub const __gnu_cxx__Lock_policy__S_atomic: __gnu_cxx__Lock_policy = 2;
thread_local!(
    pub static __default_lock_policy_687: Value<__gnu_cxx__Lock_policy> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __gnu_cxx___concurrence_lock_error {}
impl Clone for __gnu_cxx___concurrence_lock_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_lock_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {
    fn what(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {}
impl __gnu_cxx___concurrence_lock_error {}
impl __gnu_cxx___concurrence_lock_error {
    pub fn copy_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_lock_error>,
    ) -> Ptr<__gnu_cxx___concurrence_lock_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {
    pub fn move_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_lock_error>,
    ) -> Ptr<__gnu_cxx___concurrence_lock_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __gnu_cxx___concurrence_unlock_error {}
impl Clone for __gnu_cxx___concurrence_unlock_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_unlock_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {
    fn what(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {}
impl __gnu_cxx___concurrence_unlock_error {}
impl __gnu_cxx___concurrence_unlock_error {
    pub fn copy_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_unlock_error>,
    ) -> Ptr<__gnu_cxx___concurrence_unlock_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {
    pub fn move_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_unlock_error>,
    ) -> Ptr<__gnu_cxx___concurrence_unlock_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __gnu_cxx___concurrence_broadcast_error {}
impl Clone for __gnu_cxx___concurrence_broadcast_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_broadcast_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {
    fn what(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {}
impl __gnu_cxx___concurrence_broadcast_error {}
impl __gnu_cxx___concurrence_broadcast_error {
    pub fn copy_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_broadcast_error>,
    ) -> Ptr<__gnu_cxx___concurrence_broadcast_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {
    pub fn move_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_broadcast_error>,
    ) -> Ptr<__gnu_cxx___concurrence_broadcast_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __gnu_cxx___concurrence_wait_error {}
impl Clone for __gnu_cxx___concurrence_wait_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_wait_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {
    fn what(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {}
impl __gnu_cxx___concurrence_wait_error {}
impl __gnu_cxx___concurrence_wait_error {
    pub fn copy_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_wait_error>,
    ) -> Ptr<__gnu_cxx___concurrence_wait_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {
    pub fn move_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_wait_error>,
    ) -> Ptr<__gnu_cxx___concurrence_wait_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(40)]
pub struct __gnu_cxx___mutex {}
impl Clone for __gnu_cxx___mutex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___mutex {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(40)]
pub struct __gnu_cxx___recursive_mutex {}
impl Clone for __gnu_cxx___recursive_mutex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___recursive_mutex {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __gnu_cxx___scoped_lock {}
impl Clone for __gnu_cxx___scoped_lock {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___scoped_lock {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___scoped_lock {
    pub fn new(__name: Ptr<__gnu_cxx___mutex>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(48)]
pub struct __gnu_cxx___cond {}
impl Clone for __gnu_cxx___cond {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___cond {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_bad_weak_ptr {}
impl Clone for std_bad_weak_ptr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bad_weak_ptr {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bad_weak_ptr {}
impl std_bad_weak_ptr {
    pub fn copy_assign(&self, _a0: Ptr<std_bad_weak_ptr>) -> Ptr<std_bad_weak_ptr> {
        unimplemented!()
    }
}
impl std_bad_weak_ptr {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(40)]
pub struct _Mutex_base_688 {}
impl Clone for _Mutex_base_688 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_688 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_689 = u32;
pub const anon_689__S_need_barriers: anon_689 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Mutex_base_690 {}
impl Clone for _Mutex_base_690 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_690 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Mutex_base_690 {}
impl _Mutex_base_690 {}
impl _Mutex_base_690 {
    pub fn copy_assign(&self, _a0: Ptr<_Mutex_base_690>) -> Ptr<_Mutex_base_690> {
        unimplemented!()
    }
}
impl _Mutex_base_690 {
    pub fn move_assign(&self, _a0: Ptr<_Mutex_base_690>) -> Ptr<_Mutex_base_690> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Mutex_base_691 {}
impl Clone for _Mutex_base_691 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_691 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_692 = u32;
pub const anon_692__S_need_barriers: anon_692 = 0;
impl _Mutex_base_691 {}
impl _Mutex_base_691 {}
impl _Mutex_base_691 {
    pub fn copy_assign(&self, _a0: Ptr<_Mutex_base_691>) -> Ptr<_Mutex_base_691> {
        unimplemented!()
    }
}
impl _Mutex_base_691 {
    pub fn move_assign(&self, _a0: Ptr<_Mutex_base_691>) -> Ptr<_Mutex_base_691> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Sp_counted_ptr_693 {}
impl Clone for _Sp_counted_ptr_693 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_693 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct _Sp_counted_ptr_694 {}
impl Clone for _Sp_counted_ptr_694 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_694 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Sp_counted_ptr_695 {}
impl Clone for _Sp_counted_ptr_695 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_695 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std__Sp_make_shared_tag {}
impl Clone for std__Sp_make_shared_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Sp_make_shared_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Sp_alloc_shared_tag_696 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_a: Ptr<allocator_7>,
}
impl Clone for _Sp_alloc_shared_tag_696 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_alloc_shared_tag_696 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___sp_array_delete {}
impl Clone for std___sp_array_delete {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___sp_array_delete {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Sp_owner_less_697 {}
impl Clone for _Sp_owner_less_697 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_owner_less_697 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_memory_order = i32;
pub const std_memory_order_memory_order_relaxed: std_memory_order = 0;
pub const std_memory_order_memory_order_consume: std_memory_order = 1;
pub const std_memory_order_memory_order_acquire: std_memory_order = 2;
pub const std_memory_order_memory_order_release: std_memory_order = 3;
pub const std_memory_order_memory_order_acq_rel: std_memory_order = 4;
pub const std_memory_order_memory_order_seq_cst: std_memory_order = 5;
pub type std___memory_order_modifier = u32;
pub const std___memory_order_modifier___memory_order_mask: std___memory_order_modifier = 65535;
pub const std___memory_order_modifier___memory_order_modifier_mask: std___memory_order_modifier =
    4294901760;
pub const std___memory_order_modifier___memory_order_hle_acquire: std___memory_order_modifier =
    65536;
pub const std___memory_order_modifier___memory_order_hle_release: std___memory_order_modifier =
    131072;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___atomic_flag_base {
    #[offset(0)]
    pub _M_i: bool,
}
impl Clone for std___atomic_flag_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___atomic_flag_base {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___atomic_flag_base {}
impl std___atomic_flag_base {}
impl std___atomic_flag_base {
    pub fn copy_assign(&self, _a0: Ptr<std___atomic_flag_base>) -> Ptr<std___atomic_flag_base> {
        unimplemented!()
    }
}
impl std___atomic_flag_base {
    pub fn move_assign(&self, _a0: Ptr<std___atomic_flag_base>) -> Ptr<std___atomic_flag_base> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_atomic_flag {}
impl Clone for std_atomic_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_atomic_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(2)]
pub struct std__Sp_locker {}
impl Clone for std__Sp_locker {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Sp_locker {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_pointer_safety = i32;
pub const std_pointer_safety_relaxed: std_pointer_safety = 0;
pub const std_pointer_safety_preferred: std_pointer_safety = 1;
pub const std_pointer_safety_strict: std_pointer_safety = 2;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pstl_execution_sequenced_policy {}
impl Clone for __pstl_execution_sequenced_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_sequenced_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pstl_execution_parallel_policy {}
impl Clone for __pstl_execution_parallel_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_parallel_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pstl_execution_parallel_unsequenced_policy {}
impl Clone for __pstl_execution_parallel_unsequenced_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_parallel_unsequenced_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pstl_execution_unsequenced_policy {}
impl Clone for __pstl_execution_unsequenced_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_unsequenced_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn identity_698(mut x: i32) -> i32 {
    return x;
}
pub fn swap_by_ptr_699(mut a: Ptr<i32>, mut b: Ptr<i32>) {
    let mut tmp: i32 = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn swap_by_ref_700(a: Ptr<i32>, b: Ptr<i32>) {
    let mut tmp: i32 = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: i32 = 0;
    ({
        let ___out: Ptr<basic_ostream_46> =
            ({ basic_ostream_46Impl::operator_shl_11(&cout_681.with(|v| v.as_pointer()), x) });
        operator_shl_701(___out, ('\n' as i8))
    });
    let mut a: i32 = 1;
    let mut b: i32 = ({ identity_698(a) });
    ({
        let ___out: Ptr<basic_ostream_46> =
            ({ basic_ostream_46Impl::operator_shl_11(&cout_681.with(|v| v.as_pointer()), b) });
        operator_shl_701(___out, ('\n' as i8))
    });
    let c: Value<i32> = Rc::new(RefCell::new(2));
    let mut p: Ptr<i32> = (c.as_pointer());
    ({
        let ___out: Ptr<basic_ostream_46> = ({
            let ___n: i32 = (p.read());
            basic_ostream_46Impl::operator_shl_11(&cout_681.with(|v| v.as_pointer()), ___n)
        });
        operator_shl_701(___out, ('\n' as i8))
    });
    let d: Value<i32> = Rc::new(RefCell::new(3));
    let e: Value<i32> = Rc::new(RefCell::new(4));
    ({ swap_by_ptr_699((d.as_pointer()), (e.as_pointer())) });
    let f: Value<i32> = Rc::new(RefCell::new(4));
    let g: Value<i32> = Rc::new(RefCell::new(5));
    ({ swap_by_ref_700(f.as_pointer(), g.as_pointer()) });
    let mut h: Ptr<i32> = Ptr::alloc(6);
    ({
        let ___out: Ptr<basic_ostream_46> = ({
            let ___n: i32 = (h.read());
            basic_ostream_46Impl::operator_shl_11(&cout_681.with(|v| v.as_pointer()), ___n)
        });
        operator_shl_701(___out, ('\n' as i8))
    });
    h.delete();
    let mut i: Ptr<i32> = Ptr::alloc_array(Box::new([7, 8, 0_i32]));
    ({
        let ___out: Ptr<basic_ostream_46> = ({
            let ___n: i32 = (elem!(i, 1).read());
            basic_ostream_46Impl::operator_shl_11(
                &({
                    let ___out: Ptr<basic_ostream_46> = ({
                        let ___n: i32 = (elem!(i, 0).read());
                        basic_ostream_46Impl::operator_shl_11(
                            &cout_681.with(|v| v.as_pointer()),
                            ___n,
                        )
                    });
                    operator_shl_701(___out, (' ' as i8))
                }),
                ___n,
            )
        });
        operator_shl_701(___out, ('\n' as i8))
    });
    i.delete();
    ({ swap_by_ptr_699(Ptr::alloc(7), Ptr::alloc(8)) });
    ({
        swap_by_ptr_699(
            Ptr::alloc(7).offset((0) as isize),
            Ptr::alloc(8).offset((0) as isize),
        )
    });
    ({ swap_by_ref_700(Ptr::alloc(9), Ptr::alloc(10)) });
    ({
        swap_by_ref_700(
            (Ptr::alloc(9)).offset((0) as isize),
            (Ptr::alloc(10)).offset((0) as isize),
        )
    });
    let j: Value<unique_ptr_685> = Rc::new(RefCell::new(unique_ptr_685::new({ Ptr::alloc(11) })));
    let mut k: Ptr<i32> = ({ unique_ptr_685Impl::get(&j.as_pointer()) });
    ({
        let ___out: Ptr<basic_ostream_46> = ({
            let ___n: i32 = (k.read());
            basic_ostream_46Impl::operator_shl_11(&cout_681.with(|v| v.as_pointer()), ___n)
        });
        operator_shl_701(___out, ('\n' as i8))
    });
    let l: Value<unique_ptr_685> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(11));
            make_unique_702(___args.as_pointer())
        }),
    ));
    let mut m: Ptr<i32> = ({ unique_ptr_685Impl::get(&l.as_pointer()) });
    ({
        let ___out: Ptr<basic_ostream_46> = ({
            let ___n: i32 = (m.read());
            basic_ostream_46Impl::operator_shl_11(&cout_681.with(|v| v.as_pointer()), ___n)
        });
        operator_shl_701(___out, ('\n' as i8))
    });
    (if ((*c.borrow()) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_703(
                Ptr::<i8>::from_string_literal(b"c == 2"),
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
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std_error_category;
pub fn operator_shl_701(__out: Ptr<basic_ostream_46>, __c: i8) -> Ptr<basic_ostream_46> {
    unimplemented!()
}
pub fn make_unique_702(__args: Ptr<i32>) -> unique_ptr_685 {
    unimplemented!()
}
pub fn __assert_fail_703(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait __gnu_cxx___mutexImpl {
    fn lock(&self) {
        unimplemented!()
    }
    fn unlock(&self) {
        unimplemented!()
    }
    fn gthread_mutex(&self) -> Ptr<pthread_mutex_t> {
        unimplemented!()
    }
}
impl __gnu_cxx___mutexImpl for Ptr<__gnu_cxx___mutex> {}
pub trait __gnu_cxx___recursive_mutexImpl {
    fn gthread_recursive_mutex(&self) -> Ptr<pthread_mutex_t> {
        unimplemented!()
    }
}
impl __gnu_cxx___recursive_mutexImpl for Ptr<__gnu_cxx___recursive_mutex> {}
pub trait __new_allocator_416Impl {
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
impl __new_allocator_416Impl for Ptr<__new_allocator_416> {}
pub trait __new_allocator_417Impl {
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
impl __new_allocator_417Impl for Ptr<__new_allocator_417> {}
pub trait __new_allocator_418Impl {
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
impl __new_allocator_418Impl for Ptr<__new_allocator_418> {}
pub trait __new_allocator_419Impl {
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
impl __new_allocator_419Impl for Ptr<__new_allocator_419> {}
pub trait __uniq_ptr_impl_682Impl {
    fn move_assign(&self, __u: Ptr<__uniq_ptr_impl_682>) -> Ptr<__uniq_ptr_impl_682> {
        unimplemented!()
    }
    fn _M_ptr_6(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn _M_ptr_7(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deleter_8(&self) -> Ptr<default_delete_615> {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_682Impl for Ptr<__uniq_ptr_impl_682> {}
pub trait basic_ostream_46Impl {
    fn operator_shl_11(&self, mut __n: i32) -> Ptr<basic_ostream_46> {
        unimplemented!()
    }
    fn put(&self, mut __c: i8) -> Ptr<basic_ostream_46> {
        unimplemented!()
    }
}
impl basic_ostream_46Impl for Ptr<basic_ostream_46> {}
pub trait basic_string_20Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_20>) -> Ptr<basic_string_20> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_20>) -> Ptr<basic_string_20> {
        unimplemented!()
    }
    fn to_basic_string_view_23(&self) -> basic_string_view_23 {
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
    fn insert_119(&self, mut __pos1: u64, __str: Ptr<basic_string_20>) -> Ptr<basic_string_20> {
        unimplemented!()
    }
    fn insert_122(&self, mut __pos: u64, mut __s: Ptr<i8>) -> Ptr<basic_string_20> {
        unimplemented!()
    }
    fn replace_133(
        &self,
        mut __pos: u64,
        mut __n1: u64,
        mut __s: Ptr<i8>,
        mut __n2: u64,
    ) -> Ptr<basic_string_20> {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn data_158(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn __resize_and_overwrite_24(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
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
}
impl basic_string_20Impl for Ptr<basic_string_20> {}
pub trait basic_string_30Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_30>) -> Ptr<basic_string_30> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_30>) -> Ptr<basic_string_30> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_30Impl for Ptr<basic_string_30> {}
pub trait basic_string_33Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_33>) -> Ptr<basic_string_33> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_33>) -> Ptr<basic_string_33> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_33Impl for Ptr<basic_string_33> {}
pub trait basic_string_36Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_36>) -> Ptr<basic_string_36> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_36>) -> Ptr<basic_string_36> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_36Impl for Ptr<basic_string_36> {}
pub trait basic_string_view_23Impl {
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
impl basic_string_view_23Impl for Ptr<basic_string_view_23> {}
pub trait basic_string_view_605Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_view_605Impl for Ptr<basic_string_view_605> {}
pub trait basic_string_view_606Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u16> {
        unimplemented!()
    }
}
impl basic_string_view_606Impl for Ptr<basic_string_view_606> {}
pub trait basic_string_view_607Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u32> {
        unimplemented!()
    }
}
impl basic_string_view_607Impl for Ptr<basic_string_view_607> {}
pub trait default_delete_615Impl {
    fn operator_call(&self, mut __ptr: Ptr<i32>) {
        unimplemented!()
    }
}
impl default_delete_615Impl for Ptr<default_delete_615> {}
pub trait integral_constant_51Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_51Impl for Ptr<integral_constant_51> {}
pub trait less_482Impl {
    fn operator_call(
        &self,
        mut __x: Ptr<std_error_category>,
        mut __y: Ptr<std_error_category>,
    ) -> bool {
        unimplemented!()
    }
}
impl less_482Impl for Ptr<less_482> {}
pub trait less_483Impl {
    fn operator_call(&self, mut __x: Ptr<i8>, mut __y: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl less_483Impl for Ptr<less_483> {}
pub trait std___cow_stringImpl {
    fn copy_assign(&self, _a0: Ptr<std___cow_string>) -> Ptr<std___cow_string> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<std___cow_string>) -> Ptr<std___cow_string> {
        unimplemented!()
    }
}
impl std___cow_stringImpl for Ptr<std___cow_string> {}
pub trait std___exception_ptr_exception_ptrImpl {
    fn copy_assign(
        &self,
        __other: Ptr<std___exception_ptr_exception_ptr>,
    ) -> Ptr<std___exception_ptr_exception_ptr> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        __o: Ptr<std___exception_ptr_exception_ptr>,
    ) -> Ptr<std___exception_ptr_exception_ptr> {
        unimplemented!()
    }
    fn swap(&self, __other: Ptr<std___exception_ptr_exception_ptr>) {
        unimplemented!()
    }
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptrImpl for Ptr<std___exception_ptr_exception_ptr> {}
pub trait std_error_codeImpl {
    fn assign(&self, mut __v: i32, __cat: Ptr<std_error_category>) {
        unimplemented!()
    }
    fn value(&self) -> i32 {
        unimplemented!()
    }
    fn category(&self) -> Ptr<std_error_category> {
        unimplemented!()
    }
    fn message(&self) -> basic_string_20 {
        unimplemented!()
    }
}
impl std_error_codeImpl for Ptr<std_error_code> {}
pub trait std_error_conditionImpl {
    fn assign(&self, mut __v: i32, __cat: Ptr<std_error_category>) {
        unimplemented!()
    }
    fn value(&self) -> i32 {
        unimplemented!()
    }
    fn category(&self) -> Ptr<std_error_category> {
        unimplemented!()
    }
}
impl std_error_conditionImpl for Ptr<std_error_condition> {}
pub trait std_ios_baseImpl {
    fn setf_8(&self, mut __fmtfl: std__Ios_Fmtflags) -> std__Ios_Fmtflags {
        unimplemented!()
    }
    fn setf_9(
        &self,
        mut __fmtfl: std__Ios_Fmtflags,
        mut __mask: std__Ios_Fmtflags,
    ) -> std__Ios_Fmtflags {
        unimplemented!()
    }
    fn unsetf(&self, mut __mask: std__Ios_Fmtflags) {
        unimplemented!()
    }
    fn width_13(&self) -> i64 {
        unimplemented!()
    }
}
impl std_ios_baseImpl for Ptr<std_ios_base> {}
pub trait std_localeImpl {
    fn copy_assign(&self, __other: Ptr<std_locale>) -> Ptr<std_locale> {
        unimplemented!()
    }
    fn operator_eq(&self, __other: Ptr<std_locale>) -> bool {
        unimplemented!()
    }
}
impl std_localeImpl for Ptr<std_locale> {}
pub trait std_logic_errorImpl {
    fn move_assign(&self, _a0: Ptr<std_logic_error>) -> Ptr<std_logic_error> {
        unimplemented!()
    }
    fn copy_assign(&self, _a0: Ptr<std_logic_error>) -> Ptr<std_logic_error> {
        unimplemented!()
    }
}
impl std_logic_errorImpl for Ptr<std_logic_error> {}
pub trait std_runtime_errorImpl {
    fn move_assign(&self, _a0: Ptr<std_runtime_error>) -> Ptr<std_runtime_error> {
        unimplemented!()
    }
    fn copy_assign(&self, _a0: Ptr<std_runtime_error>) -> Ptr<std_runtime_error> {
        unimplemented!()
    }
}
impl std_runtime_errorImpl for Ptr<std_runtime_error> {}
pub trait std_type_infoImpl {
    fn name(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_eq(&self, __arg: Ptr<std_type_info>) -> bool {
        unimplemented!()
    }
}
impl std_type_infoImpl for Ptr<std_type_info> {}
pub trait tuple_551Impl {
    fn copy_assign(&self, __in: Ptr<tuple_551>) -> Ptr<tuple_551> {
        unimplemented!()
    }
    fn move_assign(&self, __in: Ptr<tuple_551>) -> Ptr<tuple_551> {
        unimplemented!()
    }
}
impl tuple_551Impl for Ptr<tuple_551> {}
pub trait unique_ptr_685Impl {
    fn get(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn get_deleter_17(&self) -> Ptr<default_delete_615> {
        unimplemented!()
    }
}
impl unique_ptr_685Impl for Ptr<unique_ptr_685> {}
pub fn __cpp2rust_init_globals() {}
