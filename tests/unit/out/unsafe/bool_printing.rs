extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
    pub unsafe fn length(mut __s: *const libc::c_char) -> usize {
        unimplemented!()
    }
}
impl char_traits_9 {
    pub unsafe fn move_(
        mut __s1: *mut libc::c_char,
        mut __s2: *const libc::c_char,
        mut __n: usize,
    ) -> *mut libc::c_char {
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
    pub unsafe fn insert_119(
        &mut self,
        mut __pos1: u64,
        __str: *const basic_string_13,
    ) -> *mut basic_string_13 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn insert_122(
        &mut self,
        mut __pos: u64,
        mut __s: *const libc::c_char,
    ) -> *mut basic_string_13 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn replace_133(
        &mut self,
        mut __pos: u64,
        mut __n1: u64,
        mut __s: *const libc::c_char,
        mut __n2: u64,
    ) -> *mut basic_string_13 {
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
    pub unsafe fn new_3(
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
impl __locale_struct {}
impl __locale_struct {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_ios_39 {}
impl Clone for basic_ios_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ios_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_streambuf_41 {}
impl Clone for basic_streambuf_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_istream_43 {}
impl Clone for basic_istream_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_istream_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_ostream_45 {}
impl Clone for basic_ostream_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ostream_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_ostream_45 {
    pub unsafe fn operator_shl_8(&mut self, mut __n: bool) -> *mut basic_ostream_45 {
        unimplemented!()
    }
}
impl basic_ostream_45 {
    pub unsafe fn put(&mut self, mut __c: libc::c_char) -> *mut basic_ostream_45 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_iostream_47 {}
impl Clone for basic_iostream_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_iostream_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
impl std_type_info {
    pub unsafe fn name(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl std_type_info {
    pub unsafe fn operator_eq(&self, __arg: *const std_type_info) -> bool {
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
pub static mut std_integral_constant_bool__true__value_50: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_49 {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_49 {}
impl integral_constant_49 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_integral_constant_bool__false__value_52: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_51 {}
impl integral_constant_51 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_54 {}
impl Clone for integral_constant_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_67 {}
impl Clone for __type_identity_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__92 {}
impl Clone for __and__92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__93 {}
impl Clone for __and__93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__94 {}
impl Clone for __and__94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__95 {}
impl Clone for __and__95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__96 {}
impl Clone for __and__96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__97 {}
impl Clone for __and__97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__98 {}
impl Clone for __and__98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__99 {}
impl Clone for __and__99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__100 {}
impl Clone for __not__100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__101 {}
impl Clone for __not__101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__102 {}
impl Clone for __not__102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__103 {}
impl Clone for __not__103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__104 {}
impl Clone for __not__104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__105 {}
impl Clone for __not__105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__106 {}
impl Clone for __not__106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__107 {}
impl Clone for __not__107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_108 {}
impl Clone for is_reference_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_109 {}
impl Clone for is_reference_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_110 {}
impl Clone for is_reference_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_111 {}
impl Clone for is_reference_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_112 {}
impl Clone for is_reference_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_113 {}
impl Clone for is_reference_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_114 {}
impl Clone for is_reference_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_115 {}
impl Clone for is_reference_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_116 {}
impl Clone for is_reference_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_117 {}
impl Clone for is_reference_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_118 {}
impl Clone for is_function_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_119 {}
impl Clone for is_function_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_120 {}
impl Clone for is_function_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_121 {}
impl Clone for is_function_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_122 {}
impl Clone for is_function_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_123 {}
impl Clone for is_function_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_124 {}
impl Clone for is_function_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_125 {}
impl Clone for is_function_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_126 {}
impl Clone for is_function_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_127 {}
impl Clone for is_function_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_134 {}
impl Clone for is_void_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_135 {}
impl Clone for is_void_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_136 {}
impl Clone for is_void_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_137 {}
impl Clone for is_void_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_138 {}
impl Clone for is_void_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_139 {}
impl Clone for is_void_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_140 {}
impl Clone for is_void_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_141 {}
impl Clone for is_void_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_142 {}
impl Clone for is_void_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_143 {}
impl Clone for is_void_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_144 {}
impl Clone for is_void_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_145 {}
impl Clone for is_void_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_146 {}
impl Clone for is_void_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_147 {}
impl Clone for remove_cv_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_148 {}
impl Clone for remove_cv_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_149 {}
impl Clone for remove_cv_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_150 {}
impl Clone for remove_cv_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_151 {}
impl Clone for remove_cv_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_152 {}
impl Clone for remove_cv_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_153 {}
impl Clone for remove_cv_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_154 {}
impl Clone for remove_cv_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_155 {}
impl Clone for is_const_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_156 {}
impl Clone for is_const_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_157 {}
impl Clone for is_const_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_158 {}
impl Clone for is_const_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_159 {}
impl Clone for is_const_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_160 {}
impl Clone for __is_array_unknown_bounds_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_161 {}
impl Clone for __is_array_unknown_bounds_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_162 {}
impl Clone for __is_array_unknown_bounds_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_163 {}
impl Clone for __is_array_unknown_bounds_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_164 {}
impl Clone for __is_array_unknown_bounds_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_165 {}
impl Clone for __is_array_unknown_bounds_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_166 {}
impl Clone for __is_array_unknown_bounds_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_167 {}
impl Clone for __is_array_unknown_bounds_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_168 {}
impl Clone for __is_array_unknown_bounds_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_169 {}
impl Clone for __is_array_unknown_bounds_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_170 {}
impl Clone for __is_integral_helper_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_171 {}
impl Clone for __is_integral_helper_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_172 {}
impl Clone for __is_integral_helper_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_173 {}
impl Clone for __is_integral_helper_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_174 {}
impl Clone for __is_integral_helper_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_175 {}
impl Clone for is_integral_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_176 {}
impl Clone for is_integral_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_177 {}
impl Clone for is_integral_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_178 {}
impl Clone for is_integral_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_179 {}
impl Clone for is_integral_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_class_180 {}
impl Clone for is_class_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_class_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_195 {}
impl Clone for is_same_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_196 {}
impl Clone for is_same_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_197 {}
impl Clone for is_same_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_198 {}
impl Clone for is_same_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_199 {}
impl Clone for is_same_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_200 {}
impl Clone for is_volatile_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_201 {}
impl Clone for is_volatile_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_202 {}
impl Clone for is_volatile_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_203 {}
impl Clone for is_volatile_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_204 {}
impl Clone for is_volatile_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_204 {
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
pub struct is_move_constructible_205 {}
impl Clone for is_move_constructible_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_206 {}
impl Clone for is_move_constructible_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_207 {}
impl Clone for is_move_constructible_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_208 {}
impl Clone for is_nothrow_default_constructible_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_209 {}
impl Clone for is_nothrow_default_constructible_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_212 {}
impl Clone for is_nothrow_copy_constructible_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_213 {}
impl Clone for is_nothrow_copy_constructible_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_214 {}
impl Clone for is_nothrow_copy_constructible_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_215 {}
impl Clone for is_nothrow_move_constructible_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_216 {}
impl Clone for is_nothrow_move_constructible_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_217 {}
impl Clone for is_nothrow_move_constructible_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_218 {}
impl Clone for is_move_assignable_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_219 {}
impl Clone for is_move_assignable_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_220 {}
impl Clone for is_move_assignable_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_221 {}
impl Clone for is_nothrow_move_assignable_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_222 {}
impl Clone for is_nothrow_move_assignable_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_223 {}
impl Clone for is_nothrow_move_assignable_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_223 {
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
pub struct is_convertible_224 {}
impl Clone for is_convertible_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_225 {}
impl Clone for is_convertible_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_226 {}
impl Clone for is_convertible_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_227 {}
impl Clone for is_convertible_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_228 {}
impl Clone for is_convertible_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_229 {}
impl Clone for is_convertible_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_230 {}
impl Clone for is_convertible_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_231 {}
impl Clone for is_convertible_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_232 {}
impl Clone for is_convertible_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_233 {}
impl Clone for is_convertible_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_234 {}
impl Clone for is_convertible_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_235 {}
impl Clone for remove_reference_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_236 {}
impl Clone for remove_reference_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_237 {}
impl Clone for remove_reference_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_238 {}
impl Clone for remove_reference_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_239 {}
impl Clone for remove_reference_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_240 {}
impl Clone for remove_reference_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_241 {}
impl Clone for remove_reference_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_242 {}
impl Clone for remove_reference_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_243 {}
impl Clone for remove_reference_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_244 {}
impl Clone for remove_reference_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_245 {}
impl Clone for remove_reference_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_246 {}
impl Clone for __cv_selector_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_247 {}
impl Clone for __cv_selector_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_248 {}
impl Clone for __cv_selector_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_249 {}
impl Clone for __cv_selector_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_250 {}
impl Clone for __match_cv_qualifiers_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_251 {}
impl Clone for __match_cv_qualifiers_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_252 {}
impl Clone for __match_cv_qualifiers_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_253 {}
impl Clone for __match_cv_qualifiers_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_254 {}
impl Clone for __match_cv_qualifiers_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_255 {}
impl Clone for __make_unsigned_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_256 {}
impl Clone for __make_unsigned_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_257 {}
impl Clone for __make_unsigned_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_258 {}
impl Clone for __make_unsigned_selector_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_259 {}
impl Clone for __make_unsigned_selector_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_260 {}
impl Clone for __make_unsigned_selector_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_261 {}
impl Clone for __make_unsigned_selector_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_262 {}
impl Clone for __make_unsigned_selector_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_263 {}
impl Clone for __make_unsigned_selector_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_263 {
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
pub struct _List_267 {}
impl Clone for _List_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_268 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_269 {}
impl Clone for _List_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_270 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_271 {}
impl Clone for _List_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_272 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_273 {}
impl Clone for _List_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_274 {}
impl Clone for _List_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_275 {}
impl Clone for _List_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_276 {}
impl Clone for __select_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_277 {}
impl Clone for __select_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_278 {}
impl Clone for __select_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_279 {}
impl Clone for __select_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_280 {}
impl Clone for __select_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_281 {}
impl Clone for __make_signed_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_282 {}
impl Clone for __make_signed_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_283 {}
impl Clone for __make_signed_selector_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_284 {}
impl Clone for __make_signed_selector_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_285 {}
impl Clone for __make_signed_selector_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_286 {}
impl Clone for __make_signed_selector_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_287 {}
impl Clone for __make_signed_selector_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct add_pointer_288 {}
impl Clone for add_pointer_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_pointer_288 {
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
pub struct common_type_289 {}
impl Clone for common_type_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_289 {
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
pub struct __is_tuple_like_impl_290 {}
impl Clone for __is_tuple_like_impl_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_impl_291 {}
impl Clone for __is_tuple_like_impl_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_impl_292 {}
impl Clone for __is_tuple_like_impl_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_293 {}
impl Clone for __is_tuple_like_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_294 {}
impl Clone for __is_tuple_like_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_295 {}
impl Clone for __is_tuple_like_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_295 {
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
pub static mut is_array_v_296: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_297: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_298: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_299: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_300: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_301: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_302: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_303: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_304: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_305: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_306: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_307: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_308: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_309: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_310: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_311: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_312: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_313: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_314: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_315: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(__other: *const std___exception_ptr_exception_ptr) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn move_from(__o: *mut std___exception_ptr_exception_ptr) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn copy_assign(
        &mut self,
        __other: *const std___exception_ptr_exception_ptr,
    ) -> *mut std___exception_ptr_exception_ptr {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn move_assign(
        &mut self,
        __o: *mut std___exception_ptr_exception_ptr,
    ) -> *mut std___exception_ptr_exception_ptr {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn swap(&mut self, __other: *mut std___exception_ptr_exception_ptr) {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(_a0: *const std_nested_exception) -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_nested_exception,
    ) -> *mut std_nested_exception {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Char_types_316 {}
impl Clone for _Char_types_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Char_types_317 {}
impl Clone for _Char_types_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_318 {}
impl Clone for char_traits_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_318 {
    pub unsafe fn eq(__c1: *const libc::c_char, __c2: *const libc::c_char) -> bool {
        unimplemented!()
    }
}
impl char_traits_318 {
    pub unsafe fn length(mut __p: *const libc::c_char) -> usize {
        unimplemented!()
    }
}
impl char_traits_318 {
    pub unsafe fn find(
        mut __s: *const libc::c_char,
        mut __n: usize,
        __a: *const libc::c_char,
    ) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_319 {}
impl Clone for char_traits_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_319 {
    pub unsafe fn eq(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_319 {
    pub unsafe fn lt(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_319 {
    pub unsafe fn compare(mut __s1: *const i32, mut __s2: *const i32, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_319 {
    pub unsafe fn length(mut __p: *const i32) -> usize {
        unimplemented!()
    }
}
impl char_traits_319 {
    pub unsafe fn find(mut __s: *const i32, mut __n: usize, __a: *const i32) -> *const i32 {
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
pub type anon_320 = u32;
pub const anon_320__ISupper: anon_320 = 256;
pub const anon_320__ISlower: anon_320 = 512;
pub const anon_320__ISalpha: anon_320 = 1024;
pub const anon_320__ISdigit: anon_320 = 2048;
pub const anon_320__ISxdigit: anon_320 = 4096;
pub const anon_320__ISspace: anon_320 = 8192;
pub const anon_320__ISprint: anon_320 = 16384;
pub const anon_320__ISgraph: anon_320 = 32768;
pub const anon_320__ISblank: anon_320 = 1;
pub const anon_320__IScntrl: anon_320 = 2;
pub const anon_320__ISpunct: anon_320 = 4;
pub const anon_320__ISalnum: anon_320 = 8;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct ctype_321 {}
impl Clone for ctype_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct ctype_322 {}
impl Clone for ctype_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_get_323 {}
impl Clone for num_get_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_get_324 {}
impl Clone for num_get_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_put_325 {}
impl Clone for num_put_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_put_326 {}
impl Clone for num_put_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numpunct_327 {}
impl Clone for numpunct_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numpunct_328 {}
impl Clone for numpunct_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct collate_329 {}
impl Clone for collate_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct collate_330 {}
impl Clone for collate_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_330 {
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
pub struct sched_param {
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
#[derive(VaArg, FnPtrArg)]
pub struct cpu_set_t {
    pub __bits: [u64; 16],
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
pub struct timex {
    pub modes: u32,
    pub offset: i64,
    pub freq: i64,
    pub maxerror: i64,
    pub esterror: i64,
    pub status: i32,
    pub constant: i64,
    pub precision: i64,
    pub tolerance: i64,
    pub time: timeval,
    pub tick: i64,
    pub ppsfreq: i64,
    pub jitter: i64,
    pub shift: i32,
    pub stabil: i64,
    pub jitcnt: i64,
    pub calcnt: i64,
    pub errcnt: i64,
    pub stbcnt: i64,
    pub tai: i32,
    pub anon_331: i32,
    pub anon_332: i32,
    pub anon_333: i32,
    pub anon_334: i32,
    pub anon_335: i32,
    pub anon_336: i32,
    pub anon_337: i32,
    pub anon_338: i32,
    pub anon_339: i32,
    pub anon_340: i32,
    pub anon_341: i32,
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
#[derive(VaArg, FnPtrArg)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const libc::c_char,
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
#[derive(VaArg, FnPtrArg)]
pub struct itimerspec {
    pub it_interval: timespec,
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
#[derive(VaArg, FnPtrArg)]
pub struct __atomic_wide_counter {
    pub __value64: u64,
    pub __value32: anon_342,
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
pub struct anon_342 {
    pub __low: u32,
    pub __high: u32,
}
impl Clone for anon_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_342 {
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
pub struct __jmp_buf_tag {
    pub __jmpbuf: [i64; 8],
    pub __mask_was_saved: i32,
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
pub type anon_343 = u32;
pub const anon_343_PTHREAD_MUTEX_TIMED_NP: anon_343 = 0;
pub const anon_343_PTHREAD_MUTEX_RECURSIVE_NP: anon_343 = 1;
pub const anon_343_PTHREAD_MUTEX_ERRORCHECK_NP: anon_343 = 2;
pub const anon_343_PTHREAD_MUTEX_ADAPTIVE_NP: anon_343 = 3;
pub const anon_343_PTHREAD_MUTEX_NORMAL: anon_343 = 0;
pub const anon_343_PTHREAD_MUTEX_RECURSIVE: anon_343 = 1;
pub const anon_343_PTHREAD_MUTEX_ERRORCHECK: anon_343 = 2;
pub const anon_343_PTHREAD_MUTEX_DEFAULT: anon_343 = 0;
pub const anon_343_PTHREAD_MUTEX_FAST_NP: anon_343 = 0;
pub type anon_344 = u32;
pub const anon_344_PTHREAD_MUTEX_STALLED: anon_344 = 0;
pub const anon_344_PTHREAD_MUTEX_STALLED_NP: anon_344 = 0;
pub const anon_344_PTHREAD_MUTEX_ROBUST: anon_344 = 1;
pub const anon_344_PTHREAD_MUTEX_ROBUST_NP: anon_344 = 1;
pub type anon_345 = u32;
pub const anon_345_PTHREAD_RWLOCK_PREFER_READER_NP: anon_345 = 0;
pub const anon_345_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_345 = 1;
pub const anon_345_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_345 = 2;
pub const anon_345_PTHREAD_RWLOCK_DEFAULT_NP: anon_345 = 0;
#[derive(VaArg, FnPtrArg)]
pub struct _pthread_cleanup_buffer {
    pub __routine: Option<unsafe fn(*mut ::libc::c_void)>,
    pub __arg: *mut ::libc::c_void,
    pub __canceltype: i32,
    pub __prev: *mut _pthread_cleanup_buffer,
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
pub type anon_346 = u32;
pub const anon_346_PTHREAD_CANCEL_DEFERRED: anon_346 = 0;
pub const anon_346_PTHREAD_CANCEL_ASYNCHRONOUS: anon_346 = 1;
#[derive(VaArg, FnPtrArg)]
pub struct __cancel_jmp_buf_tag {
    pub __cancel_jmp_buf: [i64; 8],
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__cancel_jmp_buf_tag; 1],
    pub __pad: [*mut ::libc::c_void; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine: Option<unsafe fn(*mut ::libc::c_void)>,
    pub __cancel_arg: *mut ::libc::c_void,
    pub __do_it: i32,
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut __libc_single_threaded_347: std::cell::LazyCell<libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_348 {}
impl Clone for __new_allocator_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_348 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_348 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_348) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_348 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_348,
    ) -> *mut __new_allocator_348 {
        unimplemented!()
    }
}
impl __new_allocator_348 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl __new_allocator_348 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut libc::c_char, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_348 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_349 {}
impl Clone for __new_allocator_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_349 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_349 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_349) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_349 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_349,
    ) -> *mut __new_allocator_349 {
        unimplemented!()
    }
}
impl __new_allocator_349 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        unimplemented!()
    }
}
impl __new_allocator_349 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_349 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_350 {}
impl Clone for __new_allocator_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_350 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_350 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_350) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_350 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_350,
    ) -> *mut __new_allocator_350 {
        unimplemented!()
    }
}
impl __new_allocator_350 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u16 {
        unimplemented!()
    }
}
impl __new_allocator_350 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u16, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_350 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_351 {}
impl Clone for __new_allocator_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_351 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_351 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_351) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_351 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_351,
    ) -> *mut __new_allocator_351 {
        unimplemented!()
    }
}
impl __new_allocator_351 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u32 {
        unimplemented!()
    }
}
impl __new_allocator_351 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_351 {
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
pub struct __are_same_352 {}
impl Clone for __are_same_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_353 = u32;
pub const anon_353___value: anon_353 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_354 {}
impl Clone for __are_same_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_355 = u32;
pub const anon_355___value: anon_355 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_356 {}
impl Clone for __are_same_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_357 = u32;
pub const anon_357___value: anon_357 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_358 {}
impl Clone for __are_same_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_359 = u32;
pub const anon_359___value: anon_359 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_360 {}
impl Clone for __are_same_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_361 = u32;
pub const anon_361___value: anon_361 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_362 {}
impl Clone for __are_same_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_363 = u32;
pub const anon_363___value: anon_363 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_364 {}
impl Clone for __is_integer_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_365 = u32;
pub const anon_365___value: anon_365 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_366 {}
impl Clone for __is_integer_366 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_366 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_367 = u32;
pub const anon_367___value: anon_367 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_368 {}
impl Clone for __is_integer_368 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_368 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_369 = u32;
pub const anon_369___value: anon_369 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_370 {}
impl Clone for __is_integer_370 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_370 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_371 = u32;
pub const anon_371___value: anon_371 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_372 {}
impl Clone for __is_integer_372 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_372 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_373 = u32;
pub const anon_373___value: anon_373 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_374 {}
impl Clone for __is_integer_374 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_374 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_375 = u32;
pub const anon_375___value: anon_375 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_376 {}
impl Clone for __is_integer_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_377 = u32;
pub const anon_377___value: anon_377 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_378 {}
impl Clone for __is_integer_378 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_378 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_379 = u32;
pub const anon_379___value: anon_379 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_380 {}
impl Clone for __is_integer_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_380 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_381 = u32;
pub const anon_381___value: anon_381 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_382 {}
impl Clone for __is_integer_382 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_382 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_383 = u32;
pub const anon_383___value: anon_383 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_384 {}
impl Clone for __memcpyable_integer_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_384 {
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
pub struct iterator_385 {}
impl Clone for iterator_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_386 {}
impl Clone for iterator_traits_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_387 {}
impl Clone for iterator_traits_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_388 {}
impl Clone for iterator_traits_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_389 {}
impl Clone for iterator_traits_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_390 {}
impl Clone for iterator_traits_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_type_391 {}
impl Clone for __conditional_type_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_392 {}
impl Clone for __ptr_traits_ptr_to_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_392 {
    pub unsafe fn pointer_to(__r: *mut libc::c_char) -> *mut libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_393 {}
impl Clone for __ptr_traits_ptr_to_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_393 {
    pub unsafe fn pointer_to(__r: *const libc::c_char) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_394 {}
impl Clone for __ptr_traits_ptr_to_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_394 {
    pub unsafe fn pointer_to(__r: *mut i32) -> *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_395 {}
impl Clone for __ptr_traits_ptr_to_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_395 {
    pub unsafe fn pointer_to(__r: *const i32) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_396 {}
impl Clone for __ptr_traits_ptr_to_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_396 {
    pub unsafe fn pointer_to(__r: *mut u16) -> *mut u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_397 {}
impl Clone for __ptr_traits_ptr_to_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_397 {
    pub unsafe fn pointer_to(__r: *const u16) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_398 {}
impl Clone for __ptr_traits_ptr_to_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_398 {
    pub unsafe fn pointer_to(__r: *mut u32) -> *mut u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_399 {}
impl Clone for __ptr_traits_ptr_to_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_399 {
    pub unsafe fn pointer_to(__r: *const u32) -> *const u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_400 {}
impl Clone for pointer_traits_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_401 {}
impl Clone for pointer_traits_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_401 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_402 {}
impl Clone for pointer_traits_402 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_402 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_403 {}
impl Clone for pointer_traits_403 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_403 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_404 {}
impl Clone for pointer_traits_404 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_404 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_405 {}
impl Clone for pointer_traits_405 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_405 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_406 {}
impl Clone for pointer_traits_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_407 {}
impl Clone for pointer_traits_407 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_407 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_408 {}
impl Clone for __normal_iterator_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_408 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_408 {}
impl __normal_iterator_408 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_409 {}
impl Clone for binary_function_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_410 {}
impl Clone for binary_function_410 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_410 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_410 {}
impl binary_function_410 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_411 {}
impl Clone for binary_function_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_411 {}
impl binary_function_411 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_412 {}
impl Clone for greater_412 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_412 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_413 {}
impl Clone for less_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_414 {}
impl Clone for less_414 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_414 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_414 {
    pub unsafe fn operator_call(
        &self,
        mut __x: *const std_error_category,
        mut __y: *const std_error_category,
    ) -> bool {
        unimplemented!()
    }
}
impl less_414 {}
impl less_414 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_415 {}
impl Clone for less_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_415 {
    pub unsafe fn operator_call(
        &self,
        mut __x: *const libc::c_char,
        mut __y: *const libc::c_char,
    ) -> bool {
        unimplemented!()
    }
}
impl less_415 {}
impl less_415 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_416 {}
impl Clone for greater_equal_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_416 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_417 {}
impl Clone for less_equal_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_418 {}
impl Clone for __is_integer_nonstrict_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_418 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_419 = u32;
pub const anon_419___width: anon_419 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_420 {}
impl Clone for __is_integer_nonstrict_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_421 = u32;
pub const anon_421___width: anon_421 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_422 {}
impl Clone for __is_integer_nonstrict_422 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_422 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_423 = u32;
pub const anon_423___width: anon_423 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_424 {}
impl Clone for __is_integer_nonstrict_424 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_424 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_425 = u32;
pub const anon_425___width: anon_425 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_426 {}
impl Clone for __is_integer_nonstrict_426 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_426 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_427 = u32;
pub const anon_427___width: anon_427 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_428 {}
impl Clone for __is_integer_nonstrict_428 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_428 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_429 = u32;
pub const anon_429___width: anon_429 = 8;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_430 {}
impl Clone for __is_integer_nonstrict_430 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_430 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_431 = u32;
pub const anon_431___width: anon_431 = 16;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_432 {}
impl Clone for __numeric_traits_integer_432 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_432 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_int____is_signed_433: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____digits_434: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____max_435: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____min_436: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_437 {}
impl Clone for __numeric_traits_integer_437 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_437 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_438: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_439: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_440: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____min_441: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_442 {}
impl Clone for __numeric_traits_integer_442 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_442 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_443:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____digits_444: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____max_445: std::cell::LazyCell<
    u32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_446 {}
impl Clone for __numeric_traits_integer_446 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_446 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_447:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_448: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_449: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_450 {}
impl Clone for __numeric_traits_integer_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_451:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_452:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_453:
    std::cell::LazyCell<u64> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_454 {}
impl Clone for __numeric_traits_integer_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_454 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_char____is_signed_455: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_char____digits_456: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_char____max_457: std::cell::LazyCell<
    libc::c_char,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_458 {}
impl Clone for __numeric_traits_integer_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_short____is_signed_459: std::cell::LazyCell<
    bool,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____digits_460: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____max_461: std::cell::LazyCell<i16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____min_462: std::cell::LazyCell<i16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_463 {}
impl Clone for __numeric_traits_floating_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_463 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_float____max_exponent10_464:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_465 {}
impl Clone for __numeric_traits_floating_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_double____max_exponent10_466:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_467 {}
impl Clone for __numeric_traits_floating_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_468:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_469 {}
impl Clone for __numeric_traits_469 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_469 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_470 {}
impl Clone for __numeric_traits_470 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_470 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_471 {}
impl Clone for __numeric_traits_471 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_471 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_472 {}
impl Clone for __numeric_traits_472 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_472 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_473 {}
impl Clone for __numeric_traits_473 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_473 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_474 {}
impl Clone for __numeric_traits_474 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_474 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_475 {}
impl Clone for __numeric_traits_475 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_475 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_476 {}
impl Clone for __numeric_traits_476 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_476 {
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
pub static mut piecewise_construct_477: std::cell::LazyCell<std_piecewise_construct_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_478 {}
impl Clone for tuple_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_478 {}
impl tuple_478 {}
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
pub struct initializer_list_479 {}
impl Clone for initializer_list_479 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_479 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_479 {}
impl initializer_list_479 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_480 {}
impl Clone for initializer_list_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_480 {}
impl initializer_list_480 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_481 {}
impl Clone for initializer_list_481 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_481 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_481 {}
impl initializer_list_481 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_482 {}
impl Clone for initializer_list_482 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_482 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_482 {}
impl initializer_list_482 {}
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
pub struct __is_allocator_484 {}
impl Clone for __is_allocator_484 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_484 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_485 {}
impl Clone for __is_allocator_485 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_485 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_486 {}
impl Clone for __is_allocator_486 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_486 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_487 {}
impl Clone for __is_allocator_487 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_487 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_488 {}
impl Clone for __alloc_traits_488 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_488 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_489 {}
impl Clone for rebind_489 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_489 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_490 {}
impl Clone for __alloc_traits_490 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_490 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_491 {}
impl Clone for rebind_491 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_491 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_492 {}
impl Clone for __alloc_traits_492 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_492 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_493 {}
impl Clone for rebind_493 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_493 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_494 {}
impl Clone for __alloc_traits_494 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_494 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_495 {}
impl Clone for rebind_495 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_495 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_496 {}
impl Clone for __hash_base_496 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_496 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_497 {}
impl Clone for __hash_base_497 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_497 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_498 {}
impl Clone for __hash_base_498 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_498 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_499 {}
impl Clone for __hash_base_499 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_499 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_500 {}
impl Clone for __hash_base_500 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_500 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_501 {}
impl Clone for __hash_base_501 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_501 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_502 {}
impl Clone for __hash_base_502 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_502 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_503 {}
impl Clone for __hash_base_503 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_503 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_504 {}
impl Clone for __hash_base_504 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_504 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_505 {}
impl Clone for __hash_base_505 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_505 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_506 {}
impl Clone for __hash_base_506 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_506 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_507 {}
impl Clone for __hash_base_507 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_507 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_508 {}
impl Clone for __hash_base_508 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_508 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_509 {}
impl Clone for __hash_base_509 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_509 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_510 {}
impl Clone for __hash_base_510 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_510 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_511 {}
impl Clone for __hash_base_511 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_511 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_512 {}
impl Clone for __hash_base_512 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_512 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_513 {}
impl Clone for __hash_base_513 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_513 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_514 {}
impl Clone for __hash_base_514 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_514 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_515 {}
impl Clone for __hash_base_515 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_515 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_516 {}
impl Clone for __hash_base_516 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_516 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_517 {}
impl Clone for __hash_base_517 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_517 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_518 {}
impl Clone for __hash_base_518 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_518 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_519 {}
impl Clone for __hash_base_519 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_519 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_520 {}
impl Clone for __hash_base_520 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_520 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_521 {}
impl Clone for __hash_base_521 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_521 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_522 {}
impl Clone for __hash_base_522 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_522 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_523 {}
impl Clone for hash_523 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_523 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_524 {}
impl Clone for hash_524 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_524 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_525 {}
impl Clone for hash_525 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_525 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_526 {}
impl Clone for hash_526 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_526 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_527 {}
impl Clone for hash_527 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_527 {
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
    pub unsafe fn hash_528(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_529(__val: *const f64) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_530(__val: *const i32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn __hash_combine_531(
        __val: *const *const std_error_category,
        mut __hash: usize,
    ) -> usize {
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
pub struct basic_string_view_532 {}
impl Clone for basic_string_view_532 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_532 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_532 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_532) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_532 {
    pub unsafe fn new_1(mut __str: *const i32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_532 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_532,
    ) -> *mut basic_string_view_532 {
        unimplemented!()
    }
}
impl basic_string_view_532 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_532 {
    pub unsafe fn data(&self) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_533 {}
impl Clone for basic_string_view_533 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_533 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_533 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_533) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_533 {
    pub unsafe fn new_1(mut __str: *const u16, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_533 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_533,
    ) -> *mut basic_string_view_533 {
        unimplemented!()
    }
}
impl basic_string_view_533 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_533 {
    pub unsafe fn data(&self) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_534 {}
impl Clone for basic_string_view_534 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_534 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_534 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_534) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_534 {
    pub unsafe fn new_1(mut __str: *const u32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_534 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_534,
    ) -> *mut basic_string_view_534 {
        unimplemented!()
    }
}
impl basic_string_view_534 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_534 {
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
pub static mut __integer_to_chars_is_unsigned_535: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_536: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_537: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
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
pub static mut allocator_arg_538: std::cell::LazyCell<std_allocator_arg_t> =
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_locale_ctype_539: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_numeric_540: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_collate_541: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_time_542: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_monetary_543: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_messages_544: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type anon_545 = u32;
pub const anon_545__S_categories_size: anon_545 = 12;
impl std_locale {
    pub unsafe fn copy_from(__other: *const std_locale) -> Self {
        unimplemented!()
    }
}
impl std_locale {
    pub unsafe fn new_1(mut __s: *const libc::c_char) -> Self {
        unimplemented!()
    }
}
impl std_locale {
    pub unsafe fn new_2(
        __base: *const std_locale,
        mut __s: *const libc::c_char,
        mut __cat: i32,
    ) -> Self {
        unimplemented!()
    }
}
impl std_locale {
    pub unsafe fn copy_assign(&mut self, __other: *const std_locale) -> *const std_locale {
        unimplemented!()
    }
}
impl std_locale {
    pub unsafe fn operator_eq(&self, __other: *const std_locale) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg)]
pub struct std___cow_string {
    pub anon_546: anon_546,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_546 {
    pub _M_p: *const libc::c_char,
    pub _M_bytes: [libc::c_char; 8],
}
impl Clone for anon_546 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_546 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___cow_string {
    pub unsafe fn copy_from(_a0: *const std___cow_string) -> Self {
        unimplemented!()
    }
}
impl std___cow_string {
    pub unsafe fn copy_assign(&mut self, _a0: *const std___cow_string) -> *mut std___cow_string {
        unimplemented!()
    }
}
impl std___cow_string {
    pub unsafe fn move_from(_a0: *mut std___cow_string) -> Self {
        unimplemented!()
    }
}
impl std___cow_string {
    pub unsafe fn move_assign(&mut self, _a0: *mut std___cow_string) -> *mut std___cow_string {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn move_from(_a0: *mut std_logic_error) -> Self {
        unimplemented!()
    }
}
impl std_logic_error {
    pub unsafe fn move_assign(&mut self, _a0: *mut std_logic_error) -> *mut std_logic_error {
        unimplemented!()
    }
}
impl std_logic_error {
    pub unsafe fn copy_from(_a0: *const std_logic_error) -> Self {
        unimplemented!()
    }
}
impl std_logic_error {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_logic_error) -> *mut std_logic_error {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(__arg: *const basic_string_13) -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub unsafe fn move_from(_a0: *mut std_runtime_error) -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub unsafe fn move_assign(&mut self, _a0: *mut std_runtime_error) -> *mut std_runtime_error {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub unsafe fn copy_from(_a0: *const std_runtime_error) -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_runtime_error) -> *mut std_runtime_error {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_547 {}
impl Clone for is_error_code_enum_547 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_547 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_548 {}
impl Clone for is_error_code_enum_548 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_548 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_549 {}
impl Clone for is_error_code_enum_549 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_549 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_550 {}
impl Clone for is_error_code_enum_550 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_550 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_551 {}
impl Clone for is_error_code_enum_551 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_551 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_552 {}
impl Clone for is_error_code_enum_552 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_552 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_condition_enum_553 {}
impl Clone for is_error_condition_enum_553 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_553 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_condition_enum_554 {}
impl Clone for is_error_condition_enum_554 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_554 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(mut __v: i32, __cat: *const std_error_category) -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn copy_from(_a0: *const std_error_code) -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_error_code) -> *mut std_error_code {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn assign(&mut self, mut __v: i32, __cat: *const std_error_category) {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn value(&self) -> i32 {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn category(&self) -> *const std_error_category {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn message(&self) -> basic_string_13 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(mut __v: i32, __cat: *const std_error_category) -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn copy_from(_a0: *const std_error_condition) -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_error_condition,
    ) -> *mut std_error_condition {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn assign(&mut self, mut __v: i32, __cat: *const std_error_category) {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn value(&self) -> i32 {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn category(&self) -> *const std_error_category {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(mut __ec: std_error_code, mut __what: *const libc::c_char) -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub unsafe fn copy_from(_a0: *const std_system_error) -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_system_error) -> *mut std_system_error {
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_ios_base_boolalpha_555: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_dec_556: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_fixed_557: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_hex_558: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_internal_559: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_left_560: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_oct_561: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_right_562: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_scientific_563: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showbase_564: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showpoint_565: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showpos_566: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_skipws_567: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_unitbuf_568: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_uppercase_569: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_adjustfield_570: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_basefield_571: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_floatfield_572: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_badbit_573: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_eofbit_574: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_failbit_575: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_goodbit_576: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_in_577: std::cell::LazyCell<std__Ios_Openmode> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_out_578: std::cell::LazyCell<std__Ios_Openmode> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_cur_579: std::cell::LazyCell<std__Ios_Seekdir> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type std_ios_base_event = u32;
pub const std_ios_base_event_erase_event: std_ios_base_event = 0;
pub const std_ios_base_event_imbue_event: std_ios_base_event = 1;
pub const std_ios_base_event_copyfmt_event: std_ios_base_event = 2;
#[derive(VaArg, FnPtrArg)]
pub struct std_ios_base__Callback_list {
    pub _M_next: *mut std_ios_base__Callback_list,
    pub _M_fn: Option<unsafe fn(std_ios_base_event, *mut std_ios_base, i32)>,
    pub _M_index: i32,
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
#[derive(VaArg, FnPtrArg)]
pub struct std_ios_base__Words {
    pub _M_pword: *mut ::libc::c_void,
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
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_ios_base__Words,
    ) -> *mut std_ios_base__Words {
        unimplemented!()
    }
}
impl std_ios_base__Words {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_ios_base__Words,
    ) -> *mut std_ios_base__Words {
        unimplemented!()
    }
}
pub type anon_580 = u32;
pub const anon_580__S_local_word_size: anon_580 = 8;
impl std_ios_base {
    pub unsafe fn setf_8(&mut self, mut __fmtfl: std__Ios_Fmtflags) -> std__Ios_Fmtflags {
        unimplemented!()
    }
}
impl std_ios_base {
    pub unsafe fn setf_9(
        &mut self,
        mut __fmtfl: std__Ios_Fmtflags,
        mut __mask: std__Ios_Fmtflags,
    ) -> std__Ios_Fmtflags {
        unimplemented!()
    }
}
impl std_ios_base {
    pub unsafe fn unsetf(&mut self, mut __mask: std__Ios_Fmtflags) {
        unimplemented!()
    }
}
impl std_ios_base {
    pub unsafe fn width_13(&self) -> i64 {
        unimplemented!()
    }
}
pub type anon_581 = i32;
pub const anon_581___ISwupper: anon_581 = 0;
pub const anon_581___ISwlower: anon_581 = 1;
pub const anon_581___ISwalpha: anon_581 = 2;
pub const anon_581___ISwdigit: anon_581 = 3;
pub const anon_581___ISwxdigit: anon_581 = 4;
pub const anon_581___ISwspace: anon_581 = 5;
pub const anon_581___ISwprint: anon_581 = 6;
pub const anon_581___ISwgraph: anon_581 = 7;
pub const anon_581___ISwblank: anon_581 = 8;
pub const anon_581___ISwcntrl: anon_581 = 9;
pub const anon_581___ISwpunct: anon_581 = 10;
pub const anon_581___ISwalnum: anon_581 = 11;
pub const anon_581__ISwupper: anon_581 = 16777216;
pub const anon_581__ISwlower: anon_581 = 33554432;
pub const anon_581__ISwalpha: anon_581 = 67108864;
pub const anon_581__ISwdigit: anon_581 = 134217728;
pub const anon_581__ISwxdigit: anon_581 = 268435456;
pub const anon_581__ISwspace: anon_581 = 536870912;
pub const anon_581__ISwprint: anon_581 = 1073741824;
pub const anon_581__ISwgraph: anon_581 = -2147483648;
pub const anon_581__ISwblank: anon_581 = 65536;
pub const anon_581__ISwcntrl: anon_581 = 131072;
pub const anon_581__ISwpunct: anon_581 = 262144;
pub const anon_581__ISwalnum: anon_581 = 524288;
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_ctype_base_upper_582: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_lower_583: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_alpha_584: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_digit_585: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_xdigit_586: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_space_587: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_print_588: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_graph_589: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_cntrl_590: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_punct_591: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_alnum_592: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_blank_593: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub type anon_594 = u32;
pub const anon_594__S_ominus: anon_594 = 0;
pub const anon_594__S_oplus: anon_594 = 1;
pub const anon_594__S_ox: anon_594 = 2;
pub const anon_594__S_oX: anon_594 = 3;
pub const anon_594__S_odigits: anon_594 = 4;
pub const anon_594__S_odigits_end: anon_594 = 20;
pub const anon_594__S_oudigits: anon_594 = 20;
pub const anon_594__S_oudigits_end: anon_594 = 36;
pub const anon_594__S_oe: anon_594 = 18;
pub const anon_594__S_oE: anon_594 = 34;
pub const anon_594__S_oend: anon_594 = 36;
pub static mut std___num_base__S_atoms_out_595: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std___num_base__S_atoms_in_596: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type anon_597 = u32;
pub const anon_597__S_iminus: anon_597 = 0;
pub const anon_597__S_iplus: anon_597 = 1;
pub const anon_597__S_ix: anon_597 = 2;
pub const anon_597__S_iX: anon_597 = 3;
pub const anon_597__S_izero: anon_597 = 4;
pub const anon_597__S_ie: anon_597 = 18;
pub const anon_597__S_iE: anon_597 = 24;
pub const anon_597__S_iend: anon_597 = 26;
pub static mut cout_598: std::cell::LazyCell<basic_ostream_45> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn foo_599() -> bool {
    return true;
}
pub unsafe fn bar_600() -> bool {
    return true;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 0;
    let mut i2: i32 = 1;
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            basic_ostream_45::operator_shl_8(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_598)),
                true,
            )
        }));
        operator_shl_601(___out, ('\n' as libc::c_char))
    });
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            basic_ostream_45::operator_shl_8(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_598)),
                false,
            )
        }));
        operator_shl_601(___out, ('\n' as libc::c_char))
    });
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            basic_ostream_45::operator_shl_8(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_598)),
                ((i1) != (i2)),
            )
        }));
        operator_shl_601(___out, ('\n' as libc::c_char))
    });
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            basic_ostream_45::operator_shl_8(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_598)),
                ((i1) == (i2)),
            )
        }));
        operator_shl_601(___out, ('\n' as libc::c_char))
    });
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            basic_ostream_45::operator_shl_8(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_598)),
                (unsafe { foo_599() }),
            )
        }));
        operator_shl_601(___out, ('\n' as libc::c_char))
    });
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            basic_ostream_45::operator_shl_8(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_598)),
                (unsafe { bar_600() }),
            )
        }));
        operator_shl_601(___out, ('\n' as libc::c_char))
    });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std___uses_alloc0__Sink;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std_error_category;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_wide_data;
pub unsafe fn operator_shl_601(
    __out: *mut basic_ostream_45,
    mut __c: libc::c_char,
) -> *mut basic_ostream_45 {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
