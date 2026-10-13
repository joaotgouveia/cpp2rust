extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_0 {}
impl Clone for allocator_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_0 {}
impl allocator_0 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_1 {}
impl Clone for allocator_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_1 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_1 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_1 {
    pub unsafe fn copy_from(__a: *const allocator_1) -> Self {
        unimplemented!()
    }
}
impl allocator_1 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_1) -> *mut allocator_1 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_2 {}
impl Clone for allocator_2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_2 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_2 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_2 {
    pub unsafe fn copy_from(__a: *const allocator_2) -> Self {
        unimplemented!()
    }
}
impl allocator_2 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_2) -> *mut allocator_2 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_3 {}
impl Clone for allocator_3 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_3 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_3 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_3 {
    pub unsafe fn copy_from(__a: *const allocator_3) -> Self {
        unimplemented!()
    }
}
impl allocator_3 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_3) -> *mut allocator_3 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_4 {}
impl Clone for allocator_4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_4 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_4 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_4 {
    pub unsafe fn copy_from(__a: *const allocator_4) -> Self {
        unimplemented!()
    }
}
impl allocator_4 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_4) -> *mut allocator_4 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_5 {}
impl Clone for allocator_traits_5 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_5 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_5 {
    pub unsafe fn allocate_1(__a: *mut allocator_1, mut __n: u64) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl allocator_traits_5 {
    pub unsafe fn deallocate(__a: *mut allocator_1, mut __p: *mut libc::c_char, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_5 {
    pub unsafe fn max_size(__a: *const allocator_1) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_6 {}
impl Clone for allocator_traits_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_6 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_6 {
    pub unsafe fn allocate_1(__a: *mut allocator_2, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl allocator_traits_6 {
    pub unsafe fn deallocate(__a: *mut allocator_2, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_6 {
    pub unsafe fn max_size(__a: *const allocator_2) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_7 {}
impl Clone for allocator_traits_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_7 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_7 {
    pub unsafe fn allocate_1(__a: *mut allocator_3, mut __n: u64) -> *mut u16 {
        unimplemented!()
    }
}
impl allocator_traits_7 {
    pub unsafe fn deallocate(__a: *mut allocator_3, mut __p: *mut u16, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_7 {
    pub unsafe fn max_size(__a: *const allocator_3) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_8 {}
impl Clone for allocator_traits_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_8 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_8 {
    pub unsafe fn allocate_1(__a: *mut allocator_4, mut __n: u64) -> *mut u32 {
        unimplemented!()
    }
}
impl allocator_traits_8 {
    pub unsafe fn deallocate(__a: *mut allocator_4, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_8 {
    pub unsafe fn max_size(__a: *const allocator_4) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_9 {}
impl Clone for char_traits_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_9 {
    pub unsafe fn assign_1(__c1: *mut libc::c_char, __c2: *const libc::c_char) {
        unimplemented!()
    }
}
impl char_traits_9 {
    pub unsafe fn lt(__c1: *const libc::c_char, __c2: *const libc::c_char) -> bool {
        unimplemented!()
    }
}
impl char_traits_9 {
    pub unsafe fn compare(
        mut __s1: *const libc::c_char,
        mut __s2: *const libc::c_char,
        mut __n: usize,
    ) -> i32 {
        unimplemented!()
    }
}
impl char_traits_9 {
    pub unsafe fn length(mut __s: *const libc::c_char) -> usize {
        unimplemented!()
    }
}
impl char_traits_9 {
    pub unsafe fn copy(
        mut __s1: *mut libc::c_char,
        mut __s2: *const libc::c_char,
        mut __n: usize,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl char_traits_9 {
    pub unsafe fn eof() -> i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_10 {}
impl Clone for char_traits_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_10 {
    pub unsafe fn assign_1(__c1: *mut i32, __c2: *const i32) {
        unimplemented!()
    }
}
impl char_traits_10 {
    pub unsafe fn copy(mut __s1: *mut i32, mut __s2: *const i32, mut __n: usize) -> *mut i32 {
        unimplemented!()
    }
}
impl char_traits_10 {
    pub unsafe fn eq_int_type(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_10 {
    pub unsafe fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_11 {}
impl Clone for char_traits_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn assign_1(__c1: *mut u16, __c2: *const u16) {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn eq(__c1: *const u16, __c2: *const u16) -> bool {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn lt(__c1: *const u16, __c2: *const u16) -> bool {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn copy(mut __s1: *mut u16, mut __s2: *const u16, mut __n: usize) -> *mut u16 {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn eq_int_type(__c1: *const u16, __c2: *const u16) -> bool {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn eof() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_12 {}
impl Clone for char_traits_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_12 {
    pub unsafe fn assign_1(__c1: *mut u32, __c2: *const u32) {
        unimplemented!()
    }
}
impl char_traits_12 {
    pub unsafe fn eq(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_12 {
    pub unsafe fn lt(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_12 {
    pub unsafe fn copy(mut __s1: *mut u32, mut __s2: *const u32, mut __n: usize) -> *mut u32 {
        unimplemented!()
    }
}
impl char_traits_12 {
    pub unsafe fn eq_int_type(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_12 {
    pub unsafe fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_13 {}
impl Clone for basic_string_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_basic_string_char___Alloc_hider {
    pub _M_p: *mut libc::c_char,
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
    pub unsafe fn new_1(mut __dat: *mut libc::c_char, __a: *const allocator_1) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub unsafe fn new_2(mut __dat: *mut libc::c_char, __a: Option<*mut allocator_1>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {}
impl std_basic_string_char___Alloc_hider {}
impl std_basic_string_char___Alloc_hider {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_basic_string_char___Alloc_hider,
    ) -> *mut std_basic_string_char___Alloc_hider {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_basic_string_char___Alloc_hider,
    ) -> *mut std_basic_string_char___Alloc_hider {
        unimplemented!()
    }
}
pub type anon_15 = u32;
pub const anon_15__S_local_capacity: anon_15 = 15;
#[derive(VaArg, FnPtrArg)]
pub struct anon_14 {
    pub _M_local_buf: [libc::c_char; 16],
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn copy_from(__str: *const basic_string_13) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn new_2(
        mut __s: *const libc::c_char,
        mut __n: u64,
        __a: Option<*const allocator_1>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn move_from(__str: *mut basic_string_13) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_13) -> *mut basic_string_13 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_13) -> *mut basic_string_13 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn to_basic_string_view_16(&self) -> basic_string_view_16 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn capacity(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn reserve_79(&mut self, mut __res: u64) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn c_str(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn data_158(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn new_3(mut __s: *const libc::c_char, __a: Option<*const allocator_1>) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn new_4(
        mut __beg: *mut libc::c_char,
        mut __end: *mut libc::c_char,
        __a: Option<*const allocator_1>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_17(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_19(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_21(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_23(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_25(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_27(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_29 {}
impl Clone for basic_string_29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_basic_string_wchar_t___Alloc_hider {
    pub _M_p: *mut i32,
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
    pub unsafe fn new_1(mut __dat: *mut i32, __a: *const allocator_2) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {}
impl std_basic_string_wchar_t___Alloc_hider {}
impl std_basic_string_wchar_t___Alloc_hider {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_basic_string_wchar_t___Alloc_hider,
    ) -> *mut std_basic_string_wchar_t___Alloc_hider {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_basic_string_wchar_t___Alloc_hider,
    ) -> *mut std_basic_string_wchar_t___Alloc_hider {
        unimplemented!()
    }
}
pub type anon_31 = u32;
pub const anon_31__S_local_capacity: anon_31 = 3;
#[derive(VaArg, FnPtrArg)]
pub struct anon_30 {
    pub _M_local_buf: [i32; 4],
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_30 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_29 {
    pub unsafe fn copy_from(__str: *const basic_string_29) -> Self {
        unimplemented!()
    }
}
impl basic_string_29 {
    pub unsafe fn new_1(
        mut __s: *const i32,
        mut __n: u64,
        __a: Option<*const allocator_2>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_29 {
    pub unsafe fn move_from(__str: *mut basic_string_29) -> Self {
        unimplemented!()
    }
}
impl basic_string_29 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_29) -> *mut basic_string_29 {
        unimplemented!()
    }
}
impl basic_string_29 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_29) -> *mut basic_string_29 {
        unimplemented!()
    }
}
impl basic_string_29 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_29 {
    pub unsafe fn c_str(&self) -> *const i32 {
        unimplemented!()
    }
}
impl basic_string_29 {
    pub unsafe fn new_2(
        mut __beg: *const libc::c_char,
        mut __end: *const libc::c_char,
        __a: Option<*const allocator_2>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_32 {}
impl Clone for basic_string_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_basic_string_char16_t___Alloc_hider {
    pub _M_p: *mut u16,
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
    pub unsafe fn new_1(mut __dat: *mut u16, __a: *const allocator_3) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {}
impl std_basic_string_char16_t___Alloc_hider {}
impl std_basic_string_char16_t___Alloc_hider {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_basic_string_char16_t___Alloc_hider,
    ) -> *mut std_basic_string_char16_t___Alloc_hider {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_basic_string_char16_t___Alloc_hider,
    ) -> *mut std_basic_string_char16_t___Alloc_hider {
        unimplemented!()
    }
}
pub type anon_34 = u32;
pub const anon_34__S_local_capacity: anon_34 = 7;
#[derive(VaArg, FnPtrArg)]
pub struct anon_33 {
    pub _M_local_buf: [u16; 8],
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_33 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_32 {
    pub unsafe fn copy_from(__str: *const basic_string_32) -> Self {
        unimplemented!()
    }
}
impl basic_string_32 {
    pub unsafe fn new_1(
        mut __s: *const u16,
        mut __n: u64,
        __a: Option<*const allocator_3>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_32 {
    pub unsafe fn move_from(__str: *mut basic_string_32) -> Self {
        unimplemented!()
    }
}
impl basic_string_32 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_32) -> *mut basic_string_32 {
        unimplemented!()
    }
}
impl basic_string_32 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_32) -> *mut basic_string_32 {
        unimplemented!()
    }
}
impl basic_string_32 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_35 {}
impl Clone for basic_string_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_basic_string_char32_t___Alloc_hider {
    pub _M_p: *mut u32,
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
    pub unsafe fn new_1(mut __dat: *mut u32, __a: *const allocator_4) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {}
impl std_basic_string_char32_t___Alloc_hider {}
impl std_basic_string_char32_t___Alloc_hider {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_basic_string_char32_t___Alloc_hider,
    ) -> *mut std_basic_string_char32_t___Alloc_hider {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_basic_string_char32_t___Alloc_hider,
    ) -> *mut std_basic_string_char32_t___Alloc_hider {
        unimplemented!()
    }
}
pub type anon_37 = u32;
pub const anon_37__S_local_capacity: anon_37 = 3;
#[derive(VaArg, FnPtrArg)]
pub struct anon_36 {
    pub _M_local_buf: [u32; 4],
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_36 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_35 {
    pub unsafe fn copy_from(__str: *const basic_string_35) -> Self {
        unimplemented!()
    }
}
impl basic_string_35 {
    pub unsafe fn new_1(
        mut __s: *const u32,
        mut __n: u64,
        __a: Option<*const allocator_4>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_35 {
    pub unsafe fn move_from(__str: *mut basic_string_35) -> Self {
        unimplemented!()
    }
}
impl basic_string_35 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_35) -> *mut basic_string_35 {
        unimplemented!()
    }
}
impl basic_string_35 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_35) -> *mut basic_string_35 {
        unimplemented!()
    }
}
impl basic_string_35 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: anon_38,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_38 {
    pub __wch: u32,
    pub __wchb: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_39 {}
impl Clone for integral_constant_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_integral_constant_bool__true__value_40: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_39 {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_39 {}
impl integral_constant_39 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_integral_constant_bool__false__value_42: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_41 {}
impl integral_constant_41 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_45 {}
impl Clone for enable_if_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_46 {}
impl Clone for enable_if_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_51 {}
impl Clone for __type_identity_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_52 {}
impl Clone for __type_identity_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_53 {}
impl Clone for __type_identity_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__58 {}
impl Clone for __or__58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__59 {}
impl Clone for __or__59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__60 {}
impl Clone for __or__60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__71 {}
impl Clone for __and__71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__72 {}
impl Clone for __and__72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__73 {}
impl Clone for __not__73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__74 {}
impl Clone for __not__74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_75 {}
impl Clone for is_reference_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_76 {}
impl Clone for is_reference_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_77 {}
impl Clone for is_reference_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_78 {}
impl Clone for is_reference_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_79 {}
impl Clone for is_reference_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_80 {}
impl Clone for is_reference_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_81 {}
impl Clone for is_reference_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_82 {}
impl Clone for is_function_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_83 {}
impl Clone for is_function_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_84 {}
impl Clone for is_function_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_85 {}
impl Clone for is_function_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_86 {}
impl Clone for is_function_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_87 {}
impl Clone for is_function_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_88 {}
impl Clone for is_function_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_89 {}
impl Clone for is_void_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_90 {}
impl Clone for is_void_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_91 {}
impl Clone for is_void_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_92 {}
impl Clone for is_void_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_93 {}
impl Clone for is_void_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_94 {}
impl Clone for is_void_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_95 {}
impl Clone for is_void_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_96 {}
impl Clone for is_void_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_97 {}
impl Clone for is_void_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_98 {}
impl Clone for is_void_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_99 {}
impl Clone for is_void_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_105 {}
impl Clone for remove_cv_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_106 {}
impl Clone for remove_cv_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_107 {}
impl Clone for remove_cv_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_108 {}
impl Clone for remove_cv_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_109 {}
impl Clone for remove_cv_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_110 {}
impl Clone for is_const_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_111 {}
impl Clone for is_const_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_112 {}
impl Clone for is_const_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_113 {}
impl Clone for is_const_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_114 {}
impl Clone for is_const_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_115 {}
impl Clone for __is_array_unknown_bounds_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_116 {}
impl Clone for __is_array_unknown_bounds_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_117 {}
impl Clone for __is_array_unknown_bounds_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_118 {}
impl Clone for __is_array_unknown_bounds_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_119 {}
impl Clone for __is_array_unknown_bounds_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_120 {}
impl Clone for __is_array_unknown_bounds_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_121 {}
impl Clone for __is_array_unknown_bounds_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_122 {}
impl Clone for __is_integral_helper_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_123 {}
impl Clone for __is_integral_helper_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_124 {}
impl Clone for __is_integral_helper_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_125 {}
impl Clone for __is_integral_helper_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_126 {}
impl Clone for __is_integral_helper_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_127 {}
impl Clone for is_integral_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_128 {}
impl Clone for is_integral_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_129 {}
impl Clone for is_integral_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_130 {}
impl Clone for is_integral_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_131 {}
impl Clone for is_integral_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_132 {}
impl Clone for is_same_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_133 {}
impl Clone for is_same_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_134 {}
impl Clone for is_same_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_135 {}
impl Clone for is_same_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_136 {}
impl Clone for is_same_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_137 {}
impl Clone for is_same_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_138 {}
impl Clone for is_same_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_139 {}
impl Clone for is_same_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_140 {}
impl Clone for is_same_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_141 {}
impl Clone for is_same_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_142 {}
impl Clone for is_same_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_151 {}
impl Clone for is_volatile_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_152 {}
impl Clone for is_volatile_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_153 {}
impl Clone for is_volatile_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_154 {}
impl Clone for is_volatile_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_155 {}
impl Clone for is_volatile_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_156 {}
impl Clone for is_nothrow_default_constructible_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_157 {}
impl Clone for is_nothrow_default_constructible_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_158 {}
impl Clone for is_nothrow_copy_constructible_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_159 {}
impl Clone for is_nothrow_copy_constructible_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_160 {}
impl Clone for is_nothrow_copy_constructible_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_161 {}
impl Clone for is_nothrow_copy_constructible_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_162 {}
impl Clone for is_nothrow_copy_constructible_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_163 {}
impl Clone for is_convertible_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_164 {}
impl Clone for is_convertible_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_165 {}
impl Clone for is_convertible_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_166 {}
impl Clone for is_convertible_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_167 {}
impl Clone for is_convertible_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_168 {}
impl Clone for remove_reference_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_169 {}
impl Clone for remove_reference_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_170 {}
impl Clone for remove_reference_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_171 {}
impl Clone for remove_reference_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_172 {}
impl Clone for remove_reference_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_173 {}
impl Clone for remove_reference_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_174 {}
impl Clone for remove_reference_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_175 {}
impl Clone for __cv_selector_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_176 {}
impl Clone for __cv_selector_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_177 {}
impl Clone for __cv_selector_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_178 {}
impl Clone for __cv_selector_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_179 {}
impl Clone for __match_cv_qualifiers_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_180 {}
impl Clone for __match_cv_qualifiers_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_181 {}
impl Clone for __match_cv_qualifiers_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_182 {}
impl Clone for __match_cv_qualifiers_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_183 {}
impl Clone for __match_cv_qualifiers_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_184 {}
impl Clone for __make_unsigned_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_185 {}
impl Clone for __make_unsigned_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_186 {}
impl Clone for __make_unsigned_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_187 {}
impl Clone for __make_unsigned_selector_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_188 {}
impl Clone for __make_unsigned_selector_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_189 {}
impl Clone for __make_unsigned_selector_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_190 {}
impl Clone for __make_unsigned_selector_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_191 {}
impl Clone for __make_unsigned_selector_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_192 {}
impl Clone for __make_unsigned_selector_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_196 {}
impl Clone for _List_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_197 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_198 {}
impl Clone for _List_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_199 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_200 {}
impl Clone for _List_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_201 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_202 {}
impl Clone for _List_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_203 {}
impl Clone for _List_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_204 {}
impl Clone for _List_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_205 {}
impl Clone for __select_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_206 {}
impl Clone for __select_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_207 {}
impl Clone for __select_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_208 {}
impl Clone for __select_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_209 {}
impl Clone for __select_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_210 {}
impl Clone for __make_signed_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_211 {}
impl Clone for __make_signed_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_212 {}
impl Clone for __make_signed_selector_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_213 {}
impl Clone for __make_signed_selector_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_214 {}
impl Clone for __make_signed_selector_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_215 {}
impl Clone for __make_signed_selector_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_216 {}
impl Clone for __make_signed_selector_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct common_type_217 {}
impl Clone for common_type_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std___nonesuchbase,
    ) -> *mut std___nonesuchbase {
        unimplemented!()
    }
}
impl std___nonesuchbase {
    pub unsafe fn move_assign(&mut self, _a0: *mut std___nonesuchbase) -> *mut std___nonesuchbase {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut is_array_v_218: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_219: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_220: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_221: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_222: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_223: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_224: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_225: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_226: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_227: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_228: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_229: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_230: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_231: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_232: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_233: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_234: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_235: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_236: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_237: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Char_types_238 {}
impl Clone for _Char_types_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Char_types_239 {}
impl Clone for _Char_types_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_240 {}
impl Clone for char_traits_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_240 {
    pub unsafe fn eq(__c1: *const libc::c_char, __c2: *const libc::c_char) -> bool {
        unimplemented!()
    }
}
impl char_traits_240 {
    pub unsafe fn length(mut __p: *const libc::c_char) -> usize {
        unimplemented!()
    }
}
impl char_traits_240 {
    pub unsafe fn find(
        mut __s: *const libc::c_char,
        mut __n: usize,
        __a: *const libc::c_char,
    ) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_241 {}
impl Clone for char_traits_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_241 {
    pub unsafe fn eq(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_241 {
    pub unsafe fn lt(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_241 {
    pub unsafe fn compare(mut __s1: *const i32, mut __s2: *const i32, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_241 {
    pub unsafe fn length(mut __p: *const i32) -> usize {
        unimplemented!()
    }
}
impl char_traits_241 {
    pub unsafe fn find(mut __s: *const i32, mut __n: usize, __a: *const i32) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_from(_a0: *const std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_from(_a0: *mut std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_assign(&mut self, _a0: *mut std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_from(_a0: *const std_bad_alloc) -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_bad_alloc) -> *mut std_bad_alloc {
        unimplemented!()
    }
}
pub type std_align_val_t = usize;
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_242 {}
impl Clone for __new_allocator_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_242 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_242 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_242) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_242 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_242,
    ) -> *mut __new_allocator_242 {
        unimplemented!()
    }
}
impl __new_allocator_242 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl __new_allocator_242 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut libc::c_char, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_242 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_243 {}
impl Clone for __new_allocator_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_243 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_243 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_243) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_243 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_243,
    ) -> *mut __new_allocator_243 {
        unimplemented!()
    }
}
impl __new_allocator_243 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        unimplemented!()
    }
}
impl __new_allocator_243 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_243 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_244 {}
impl Clone for __new_allocator_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_244 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_244 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_244) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_244 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_244,
    ) -> *mut __new_allocator_244 {
        unimplemented!()
    }
}
impl __new_allocator_244 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u16 {
        unimplemented!()
    }
}
impl __new_allocator_244 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u16, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_244 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_245 {}
impl Clone for __new_allocator_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_245 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_245 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_245) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_245 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_245,
    ) -> *mut __new_allocator_245 {
        unimplemented!()
    }
}
impl __new_allocator_245 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u32 {
        unimplemented!()
    }
}
impl __new_allocator_245 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_245 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_246 {}
impl Clone for __are_same_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_247 = u32;
pub const anon_247___value: anon_247 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_248 {}
impl Clone for __are_same_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_249 = u32;
pub const anon_249___value: anon_249 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_250 {}
impl Clone for __are_same_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_251 = u32;
pub const anon_251___value: anon_251 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_252 {}
impl Clone for __are_same_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_253 = u32;
pub const anon_253___value: anon_253 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_254 {}
impl Clone for __are_same_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_255 = u32;
pub const anon_255___value: anon_255 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_256 {}
impl Clone for __are_same_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_257 = u32;
pub const anon_257___value: anon_257 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_258 {}
impl Clone for __is_integer_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_259 = u32;
pub const anon_259___value: anon_259 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_260 {}
impl Clone for __is_integer_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_261 = u32;
pub const anon_261___value: anon_261 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_262 {}
impl Clone for __is_integer_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_263 = u32;
pub const anon_263___value: anon_263 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_264 {}
impl Clone for __is_integer_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_265 = u32;
pub const anon_265___value: anon_265 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_266 {}
impl Clone for __is_integer_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_267 = u32;
pub const anon_267___value: anon_267 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_268 {}
impl Clone for __is_integer_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_269 = u32;
pub const anon_269___value: anon_269 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_270 {}
impl Clone for __is_integer_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_271 = u32;
pub const anon_271___value: anon_271 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_272 {}
impl Clone for __is_integer_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_273 = u32;
pub const anon_273___value: anon_273 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_274 {}
impl Clone for __memcpyable_integer_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_275 {}
impl Clone for iterator_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_276 {}
impl Clone for iterator_traits_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_277 {}
impl Clone for iterator_traits_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_278 {}
impl Clone for iterator_traits_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_279 {}
impl Clone for iterator_traits_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_280 {}
impl Clone for iterator_traits_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_281 {}
impl Clone for __ptr_traits_ptr_to_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_281 {
    pub unsafe fn pointer_to(__r: *mut libc::c_char) -> *mut libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_282 {}
impl Clone for __ptr_traits_ptr_to_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_282 {
    pub unsafe fn pointer_to(__r: *const libc::c_char) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_283 {}
impl Clone for __ptr_traits_ptr_to_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_283 {
    pub unsafe fn pointer_to(__r: *mut i32) -> *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_284 {}
impl Clone for __ptr_traits_ptr_to_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_284 {
    pub unsafe fn pointer_to(__r: *const i32) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_285 {}
impl Clone for __ptr_traits_ptr_to_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_285 {
    pub unsafe fn pointer_to(__r: *mut u16) -> *mut u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_286 {}
impl Clone for __ptr_traits_ptr_to_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_286 {
    pub unsafe fn pointer_to(__r: *const u16) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_287 {}
impl Clone for __ptr_traits_ptr_to_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_287 {
    pub unsafe fn pointer_to(__r: *mut u32) -> *mut u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_288 {}
impl Clone for __ptr_traits_ptr_to_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_288 {
    pub unsafe fn pointer_to(__r: *const u32) -> *const u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_289 {}
impl Clone for pointer_traits_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_290 {}
impl Clone for pointer_traits_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_291 {}
impl Clone for pointer_traits_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_292 {}
impl Clone for pointer_traits_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_293 {}
impl Clone for pointer_traits_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_294 {}
impl Clone for pointer_traits_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_295 {}
impl Clone for pointer_traits_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_296 {}
impl Clone for pointer_traits_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_297 {}
impl Clone for binary_function_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_298 {}
impl Clone for greater_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_299 {}
impl Clone for less_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_300 {}
impl Clone for greater_equal_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_301 {}
impl Clone for less_equal_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_302 {}
impl Clone for __is_integer_nonstrict_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_303 = u32;
pub const anon_303___width: anon_303 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_304 {}
impl Clone for __is_integer_nonstrict_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_305 = u32;
pub const anon_305___width: anon_305 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_306 {}
impl Clone for __is_integer_nonstrict_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_307 = u32;
pub const anon_307___width: anon_307 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_308 {}
impl Clone for __is_integer_nonstrict_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_309 = u32;
pub const anon_309___width: anon_309 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_310 {}
impl Clone for __is_integer_nonstrict_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_311 = u32;
pub const anon_311___width: anon_311 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_312 {}
impl Clone for __numeric_traits_integer_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_int____is_signed_313: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____digits_314: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____max_315: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____min_316: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_317 {}
impl Clone for __numeric_traits_integer_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_318: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_319: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_320: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_321 {}
impl Clone for __numeric_traits_integer_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_322:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____digits_323: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____max_324: std::cell::LazyCell<
    u32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_325 {}
impl Clone for __numeric_traits_integer_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_326:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_327: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_328: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_329 {}
impl Clone for __numeric_traits_integer_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_330:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_331:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_332:
    std::cell::LazyCell<u64> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_333 {}
impl Clone for __numeric_traits_floating_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_float____max_exponent10_334:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_335 {}
impl Clone for __numeric_traits_floating_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_double____max_exponent10_336:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_337 {}
impl Clone for __numeric_traits_floating_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_338:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_339 {}
impl Clone for __numeric_traits_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_340 {}
impl Clone for __numeric_traits_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_341 {}
impl Clone for __numeric_traits_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_342 {}
impl Clone for __numeric_traits_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_343 {}
impl Clone for __numeric_traits_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut piecewise_construct_344: std::cell::LazyCell<std_piecewise_construct_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_345 {}
impl Clone for tuple_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_345 {}
impl tuple_345 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_346 {}
impl Clone for initializer_list_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_346 {}
impl initializer_list_346 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_347 {}
impl Clone for initializer_list_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_347 {}
impl initializer_list_347 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_348 {}
impl Clone for initializer_list_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_348 {}
impl initializer_list_348 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_349 {}
impl Clone for initializer_list_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_349 {}
impl initializer_list_349 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_351 {}
impl Clone for __is_allocator_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_352 {}
impl Clone for __is_allocator_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_353 {}
impl Clone for __is_allocator_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_354 {}
impl Clone for __is_allocator_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_355 {}
impl Clone for __alloc_traits_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_356 {}
impl Clone for rebind_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_357 {}
impl Clone for __alloc_traits_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_358 {}
impl Clone for rebind_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_359 {}
impl Clone for __alloc_traits_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_360 {}
impl Clone for rebind_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_361 {}
impl Clone for __alloc_traits_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_362 {}
impl Clone for rebind_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_363 {}
impl Clone for __hash_base_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_364 {}
impl Clone for __hash_base_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_365 {}
impl Clone for __hash_base_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_366 {}
impl Clone for __hash_base_366 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_366 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_367 {}
impl Clone for __hash_base_367 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_367 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_368 {}
impl Clone for __hash_base_368 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_368 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_369 {}
impl Clone for __hash_base_369 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_369 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_370 {}
impl Clone for __hash_base_370 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_370 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_371 {}
impl Clone for __hash_base_371 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_371 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_372 {}
impl Clone for __hash_base_372 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_372 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_373 {}
impl Clone for __hash_base_373 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_373 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_374 {}
impl Clone for __hash_base_374 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_374 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_375 {}
impl Clone for __hash_base_375 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_375 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_376 {}
impl Clone for __hash_base_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_377 {}
impl Clone for __hash_base_377 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_377 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_378 {}
impl Clone for __hash_base_378 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_378 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_379 {}
impl Clone for __hash_base_379 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_379 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_380 {}
impl Clone for __hash_base_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_380 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_381 {}
impl Clone for __hash_base_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_388 {}
impl Clone for hash_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_389 {}
impl Clone for hash_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_390 {}
impl Clone for hash_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_391 {}
impl Clone for hash_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_392 {}
impl Clone for hash_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn hash(
        mut __ptr: *const ::libc::c_void,
        mut __clength: usize,
        mut __seed: Option<usize>,
    ) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_393(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_394(__val: *const f64) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_16 {}
impl Clone for basic_string_view_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_16 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_16) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_16 {
    pub unsafe fn new_1(mut __str: *const libc::c_char, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_16 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_16,
    ) -> *mut basic_string_view_16 {
        unimplemented!()
    }
}
impl basic_string_view_16 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_16 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_16 {
    pub unsafe fn data(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_395 {}
impl Clone for basic_string_view_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_395 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_395) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_395 {
    pub unsafe fn new_1(mut __str: *const i32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_395 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_395,
    ) -> *mut basic_string_view_395 {
        unimplemented!()
    }
}
impl basic_string_view_395 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_395 {
    pub unsafe fn data(&self) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_396 {}
impl Clone for basic_string_view_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_396 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_396) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_396 {
    pub unsafe fn new_1(mut __str: *const u16, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_396 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_396,
    ) -> *mut basic_string_view_396 {
        unimplemented!()
    }
}
impl basic_string_view_396 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_396 {
    pub unsafe fn data(&self) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_397 {}
impl Clone for basic_string_view_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_397 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_397) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_397 {
    pub unsafe fn new_1(mut __str: *const u32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_397 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_397,
    ) -> *mut basic_string_view_397 {
        unimplemented!()
    }
}
impl basic_string_view_397 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_397 {
    pub unsafe fn data(&self) -> *const u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct div_t {
    pub quot: i32,
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
#[derive(VaArg, FnPtrArg)]
pub struct ldiv_t {
    pub quot: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct lldiv_t {
    pub quot: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct __sigset_t {
    pub __val: [u64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct timeval {
    pub tv_sec: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct timespec {
    pub tv_sec: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct fd_set {
    pub fds_bits: [i64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct __atomic_wide_counter {
    pub __value64: u64,
    pub __value32: anon_398,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_398 {
    pub __low: u32,
    pub __high: u32,
}
impl Clone for anon_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __rwelision: i8,
    pub __pad1: [u8; 7],
    pub __pad2: u64,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [u32; 2],
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
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
#[derive(VaArg, FnPtrArg)]
pub struct __once_flag {
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_condattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_attr_t {
    pub __size: [libc::c_char; 56],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrier_t {
    pub __size: [libc::c_char; 32],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrierattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: i32,
    pub rand_deg: i32,
    pub rand_sep: i32,
    pub end_ptr: *mut i32,
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
#[derive(VaArg, FnPtrArg)]
pub struct drand48_data {
    pub __x: [u16; 3],
    pub __old_x: [u16; 3],
    pub __c: u16,
    pub __init: u16,
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
#[derive(VaArg, FnPtrArg)]
pub struct _G_fpos_t {
    pub __pos: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct _G_fpos64_t {
    pub __pos: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut ::libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::libc::c_void,
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
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
#[derive(VaArg, FnPtrArg)]
pub struct _IO_cookie_io_functions_t {
    pub read: Option<unsafe fn(*mut ::libc::c_void, *mut libc::c_char, usize) -> i64>,
    pub write: Option<unsafe fn(*mut ::libc::c_void, *const libc::c_char, usize) -> i64>,
    pub seek: Option<unsafe fn(*mut ::libc::c_void, *mut i64, i32) -> i32>,
    pub close: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
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
pub static mut __integer_to_chars_is_unsigned_399: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_400: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_401: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut allocator_arg_402: std::cell::LazyCell<std_allocator_arg_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg)]
pub struct std___uses_alloc0 {
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
pub unsafe fn next_403() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_404)).prefix_inc();
}
static mut counter_404: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn marker_405(mut tag: u8) -> u8 {
    return ((((tag as i32) << (3)) | (2)) as u8);
}
pub static mut signature_406: std::cell::LazyCell<[u8; 3]> = std::cell::LazyCell::new(|| unsafe {
    [
        (unsafe { marker_405(1_u8) }),
        4_u8,
        (('B' as libc::c_char) as u8),
    ]
});
pub static mut single_407: std::cell::LazyCell<u8> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { marker_405(2_u8) }) });
pub static mut from_call_408: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { next_403() }) });
pub static mut depends_on_call_409: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut from_call_408)) + (1))
    });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Ctor {
    pub v: i32,
}
impl Ctor {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            v: (unsafe { next_403() }),
        };
        this
    }
    pub unsafe fn new_2(mut x: i32) -> Self {
        let mut this = Self { v: x };
        this
    }
}
impl Default for Ctor {
    fn default() -> Self {
        unsafe { Ctor::new_1() }
    }
}
pub static mut default_ctor_410: std::cell::LazyCell<Ctor> =
    std::cell::LazyCell::new(|| unsafe { Ctor::new_1() });
pub static mut arg_ctor_411: std::cell::LazyCell<Ctor> =
    std::cell::LazyCell::new(|| unsafe { Ctor::new_2({ 7 }) });
pub static mut str_412: std::cell::LazyCell<basic_string_13> =
    std::cell::LazyCell::new(|| unsafe { basic_string_13::new_3({ c"abc".as_ptr() }, None) });
pub static mut Holder_inline_member_414: std::cell::LazyCell<Ctor> =
    std::cell::LazyCell::new(|| unsafe { Ctor::new_2({ 5 }) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {}
pub static mut Holder_member_413: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { next_403() }) });
pub unsafe fn local_static_415() -> i32 {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut once_416))
        + ((*std::cell::LazyCell::force_mut(&mut *&raw mut local_ctor_417)).v));
}
static mut once_416: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { next_403() }) });
static mut local_ctor_417: std::cell::LazyCell<Ctor> =
    std::cell::LazyCell::new(|| unsafe { Ctor::new_2({ 3 }) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Singleton {
    pub hits: i32,
}
impl Singleton {
    pub unsafe fn new() -> Self {
        let mut this = Self { hits: 0 };
        this
    }
    pub unsafe fn instance() -> *mut Singleton {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut s_418));
    }
}
impl Default for Singleton {
    fn default() -> Self {
        unsafe { Singleton::new() }
    }
}
static mut s_418: std::cell::LazyCell<Singleton> =
    std::cell::LazyCell::new(|| unsafe { Singleton::new() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut signature_406))[(0) as usize] as i32)
        == (10))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"signature[0] == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut signature_406))[(1) as usize] as i32)
        == (4))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"signature[1] == 0x04".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut single_407)) as i32) == (18)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"single == 18".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut from_call_408)) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"from_call == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut depends_on_call_409)) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"depends_on_call == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctor_410)).v) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"default_ctor.v == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut arg_ctor_411)).v) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"arg_ctor.v == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let ___lhs: *const basic_string_13 =
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut str_412));
        operator_eq_420(___lhs, c"abc".as_ptr())
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"str == \"abc\"".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut Holder_member_413)) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"Holder::member == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut Holder_inline_member_414)).v) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"Holder::inline_member.v == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { local_static_415() }) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"local_static() == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { local_static_415() }) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"local_static() == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(unsafe { Singleton::instance() })).hits.postfix_inc();
    (*(unsafe { Singleton::instance() })).hits.postfix_inc();
    (if (((*(unsafe { Singleton::instance() })).hits) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"Singleton::instance().hits == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { Singleton::instance() }) == (unsafe { Singleton::instance() })) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_419(
                c"&Singleton::instance() == &Singleton::instance()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std___uses_alloc0__Sink;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn __assert_fail_419(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn operator_eq_420(
    __lhs: *const basic_string_13,
    mut __rhs: *const libc::c_char,
) -> bool {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const signature_406);
    std::cell::LazyCell::force(&*&raw const single_407);
    std::cell::LazyCell::force(&*&raw const from_call_408);
    std::cell::LazyCell::force(&*&raw const depends_on_call_409);
    std::cell::LazyCell::force(&*&raw const default_ctor_410);
    std::cell::LazyCell::force(&*&raw const arg_ctor_411);
    std::cell::LazyCell::force(&*&raw const str_412);
    std::cell::LazyCell::force(&*&raw const Holder_inline_member_414);
    std::cell::LazyCell::force(&*&raw const Holder_member_413);
}
