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
pub struct allocator_5 {}
impl Clone for allocator_5 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_5 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_5 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_5 {
    pub unsafe fn copy_from(__a: *const allocator_5) -> Self {
        unimplemented!()
    }
}
impl allocator_5 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_5) -> *mut allocator_5 {
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
    pub unsafe fn allocate_1(__a: *mut allocator_1, mut __n: u64) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl allocator_traits_6 {
    pub unsafe fn deallocate(__a: *mut allocator_1, mut __p: *mut libc::c_char, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_6 {
    pub unsafe fn max_size(__a: *const allocator_1) -> u64 {
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
    pub unsafe fn allocate_1(__a: *mut allocator_2, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl allocator_traits_7 {
    pub unsafe fn deallocate(__a: *mut allocator_2, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_7 {
    pub unsafe fn max_size(__a: *const allocator_2) -> u64 {
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
    pub unsafe fn allocate_1(__a: *mut allocator_3, mut __n: u64) -> *mut u16 {
        unimplemented!()
    }
}
impl allocator_traits_8 {
    pub unsafe fn deallocate(__a: *mut allocator_3, mut __p: *mut u16, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_8 {
    pub unsafe fn max_size(__a: *const allocator_3) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_9 {}
impl Clone for allocator_traits_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_9 {
    pub unsafe fn allocate_1(__a: *mut allocator_4, mut __n: u64) -> *mut u32 {
        unimplemented!()
    }
}
impl allocator_traits_9 {
    pub unsafe fn deallocate(__a: *mut allocator_4, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_9 {
    pub unsafe fn max_size(__a: *const allocator_4) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_10 {}
impl Clone for allocator_traits_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_10 {
    pub unsafe fn allocate_1(__a: *mut allocator_5, mut __n: u64) -> *mut u8 {
        unimplemented!()
    }
}
impl allocator_traits_10 {
    pub unsafe fn deallocate(__a: *mut allocator_5, mut __p: *mut u8, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_10 {
    pub unsafe fn max_size(__a: *const allocator_5) -> u64 {
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
    pub unsafe fn assign_1(__c1: *mut libc::c_char, __c2: *const libc::c_char) {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn lt(__c1: *const libc::c_char, __c2: *const libc::c_char) -> bool {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn length(mut __s: *const libc::c_char) -> usize {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn move_(
        mut __s1: *mut libc::c_char,
        mut __s2: *const libc::c_char,
        mut __n: usize,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn copy(
        mut __s1: *mut libc::c_char,
        mut __s2: *const libc::c_char,
        mut __n: usize,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl char_traits_11 {
    pub unsafe fn eof() -> i32 {
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
    pub unsafe fn assign_1(__c1: *mut i32, __c2: *const i32) {
        unimplemented!()
    }
}
impl char_traits_12 {
    pub unsafe fn copy(mut __s1: *mut i32, mut __s2: *const i32, mut __n: usize) -> *mut i32 {
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
pub struct char_traits_13 {}
impl Clone for char_traits_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_13 {
    pub unsafe fn assign_1(__c1: *mut u16, __c2: *const u16) {
        unimplemented!()
    }
}
impl char_traits_13 {
    pub unsafe fn eq(__c1: *const u16, __c2: *const u16) -> bool {
        unimplemented!()
    }
}
impl char_traits_13 {
    pub unsafe fn lt(__c1: *const u16, __c2: *const u16) -> bool {
        unimplemented!()
    }
}
impl char_traits_13 {
    pub unsafe fn copy(mut __s1: *mut u16, mut __s2: *const u16, mut __n: usize) -> *mut u16 {
        unimplemented!()
    }
}
impl char_traits_13 {
    pub unsafe fn eq_int_type(__c1: *const u16, __c2: *const u16) -> bool {
        unimplemented!()
    }
}
impl char_traits_13 {
    pub unsafe fn eof() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_14 {}
impl Clone for char_traits_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_14 {
    pub unsafe fn assign_1(__c1: *mut u32, __c2: *const u32) {
        unimplemented!()
    }
}
impl char_traits_14 {
    pub unsafe fn eq(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_14 {
    pub unsafe fn lt(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_14 {
    pub unsafe fn copy(mut __s1: *mut u32, mut __s2: *const u32, mut __n: usize) -> *mut u32 {
        unimplemented!()
    }
}
impl char_traits_14 {
    pub unsafe fn eq_int_type(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_14 {
    pub unsafe fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_15 {}
impl Clone for basic_string_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_15 {
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
pub type anon_17 = u32;
pub const anon_17__S_local_capacity: anon_17 = 15;
#[derive(VaArg, FnPtrArg)]
pub struct anon_16 {
    pub _M_local_buf: [libc::c_char; 16],
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn copy_from(__str: *const basic_string_15) -> Self {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn new_2(
        mut __s: *const libc::c_char,
        mut __n: u64,
        __a: Option<*const allocator_1>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn move_from(__str: *mut basic_string_15) -> Self {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_15) -> *mut basic_string_15 {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_15) -> *mut basic_string_15 {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn to_basic_string_view_18(&self) -> basic_string_view_18 {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn capacity(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn reserve_79(&mut self, mut __res: u64) {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn insert_119(
        &mut self,
        mut __pos1: u64,
        __str: *const basic_string_15,
    ) -> *mut basic_string_15 {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn insert_122(
        &mut self,
        mut __pos: u64,
        mut __s: *const libc::c_char,
    ) -> *mut basic_string_15 {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn replace_133(
        &mut self,
        mut __pos: u64,
        mut __n1: u64,
        mut __s: *const libc::c_char,
        mut __n2: u64,
    ) -> *mut basic_string_15 {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn c_str(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn data_158(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn new_3(mut __s: *const libc::c_char, __a: Option<*const allocator_1>) -> Self {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn new_4(
        mut __beg: *mut libc::c_char,
        mut __end: *mut libc::c_char,
        __a: Option<*const allocator_1>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn __resize_and_overwrite_19(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn __resize_and_overwrite_21(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn __resize_and_overwrite_23(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn __resize_and_overwrite_25(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn __resize_and_overwrite_27(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_15 {
    pub unsafe fn __resize_and_overwrite_29(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub type anon_33 = u32;
pub const anon_33__S_local_capacity: anon_33 = 3;
#[derive(VaArg, FnPtrArg)]
pub struct anon_32 {
    pub _M_local_buf: [i32; 4],
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
    pub unsafe fn copy_from(__str: *const basic_string_31) -> Self {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub unsafe fn new_1(
        mut __s: *const i32,
        mut __n: u64,
        __a: Option<*const allocator_2>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub unsafe fn move_from(__str: *mut basic_string_31) -> Self {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_31) -> *mut basic_string_31 {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_31) -> *mut basic_string_31 {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub unsafe fn c_str(&self) -> *const i32 {
        unimplemented!()
    }
}
impl basic_string_31 {
    pub unsafe fn new_2(
        mut __beg: *const libc::c_char,
        mut __end: *const libc::c_char,
        __a: Option<*const allocator_2>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub type anon_36 = u32;
pub const anon_36__S_local_capacity: anon_36 = 7;
#[derive(VaArg, FnPtrArg)]
pub struct anon_35 {
    pub _M_local_buf: [u16; 8],
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
    pub unsafe fn copy_from(__str: *const basic_string_34) -> Self {
        unimplemented!()
    }
}
impl basic_string_34 {
    pub unsafe fn new_1(
        mut __s: *const u16,
        mut __n: u64,
        __a: Option<*const allocator_3>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_34 {
    pub unsafe fn move_from(__str: *mut basic_string_34) -> Self {
        unimplemented!()
    }
}
impl basic_string_34 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_34) -> *mut basic_string_34 {
        unimplemented!()
    }
}
impl basic_string_34 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_34) -> *mut basic_string_34 {
        unimplemented!()
    }
}
impl basic_string_34 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub type anon_39 = u32;
pub const anon_39__S_local_capacity: anon_39 = 3;
#[derive(VaArg, FnPtrArg)]
pub struct anon_38 {
    pub _M_local_buf: [u32; 4],
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
    pub unsafe fn copy_from(__str: *const basic_string_37) -> Self {
        unimplemented!()
    }
}
impl basic_string_37 {
    pub unsafe fn new_1(
        mut __s: *const u32,
        mut __n: u64,
        __a: Option<*const allocator_4>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_37 {
    pub unsafe fn move_from(__str: *mut basic_string_37) -> Self {
        unimplemented!()
    }
}
impl basic_string_37 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_37) -> *mut basic_string_37 {
        unimplemented!()
    }
}
impl basic_string_37 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_37) -> *mut basic_string_37 {
        unimplemented!()
    }
}
impl basic_string_37 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __mbstate_t {
    pub __count: i32,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_40 {
    pub __wch: u32,
    pub __wchb: [libc::c_char; 4],
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
impl basic_ios_41 {
    pub unsafe fn rdstate(&mut self) -> std__Ios_Iostate {
        unimplemented!()
    }
}
impl basic_ios_41 {
    pub unsafe fn clear(&mut self, mut __state: std__Ios_Iostate) {
        unimplemented!()
    }
}
impl basic_ios_41 {
    pub unsafe fn setstate(&mut self, mut __state: std__Ios_Iostate) {
        unimplemented!()
    }
}
impl basic_ios_41 {
    pub unsafe fn widen(&mut self, mut __c: libc::c_char) -> libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_ios_42 {}
impl Clone for basic_ios_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ios_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_streambuf_44 {}
impl Clone for basic_streambuf_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_istream_46 {}
impl Clone for basic_istream_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_istream_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl basic_ostream_47 {
    pub unsafe fn operator_shl_3(
        &mut self,
        mut __pf: Option<unsafe fn(*mut basic_ostream_47) -> *mut basic_ostream_47>,
    ) -> *mut basic_ostream_47 {
        unimplemented!()
    }
}
impl basic_ostream_47 {
    pub unsafe fn operator_shl_5(
        &mut self,
        mut __pf: Option<unsafe fn(*mut std_ios_base) -> *mut std_ios_base>,
    ) -> *mut basic_ostream_47 {
        unimplemented!()
    }
}
impl basic_ostream_47 {
    pub unsafe fn operator_shl_11(&mut self, mut __n: i32) -> *mut basic_ostream_47 {
        unimplemented!()
    }
}
impl basic_ostream_47 {
    pub unsafe fn put(&mut self, mut __c: libc::c_char) -> *mut basic_ostream_47 {
        unimplemented!()
    }
}
impl basic_ostream_47 {
    pub unsafe fn flush(&mut self) -> *mut basic_ostream_47 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_ostream_48 {}
impl Clone for basic_ostream_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ostream_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_iostream_50 {}
impl Clone for basic_iostream_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_iostream_50 {
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
pub static mut std_integral_constant_bool__true__value_52: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_51 {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
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
pub static mut std_integral_constant_bool__false__value_54: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_53 {}
impl integral_constant_53 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__101 {}
impl Clone for __and__101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__114 {}
impl Clone for __not__114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__115 {}
impl Clone for __not__115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__116 {}
impl Clone for __not__116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__117 {}
impl Clone for __not__117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__118 {}
impl Clone for __not__118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__119 {}
impl Clone for __not__119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__120 {}
impl Clone for __not__120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__121 {}
impl Clone for __not__121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_122 {}
impl Clone for is_reference_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_123 {}
impl Clone for is_reference_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_124 {}
impl Clone for is_reference_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_125 {}
impl Clone for is_reference_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_126 {}
impl Clone for is_reference_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_127 {}
impl Clone for is_reference_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_128 {}
impl Clone for is_reference_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_129 {}
impl Clone for is_reference_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_130 {}
impl Clone for is_reference_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_131 {}
impl Clone for is_reference_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_135 {}
impl Clone for is_function_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_136 {}
impl Clone for is_function_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_137 {}
impl Clone for is_function_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_138 {}
impl Clone for is_function_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_139 {}
impl Clone for is_function_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_140 {}
impl Clone for is_function_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_141 {}
impl Clone for is_function_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_142 {}
impl Clone for is_function_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_143 {}
impl Clone for is_function_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_144 {}
impl Clone for is_function_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_145 {}
impl Clone for is_function_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_148 {}
impl Clone for is_void_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_149 {}
impl Clone for is_void_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_150 {}
impl Clone for is_void_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_151 {}
impl Clone for is_void_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_152 {}
impl Clone for is_void_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_153 {}
impl Clone for is_void_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_154 {}
impl Clone for is_void_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_155 {}
impl Clone for is_void_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_156 {}
impl Clone for is_void_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_157 {}
impl Clone for is_void_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_170 {}
impl Clone for remove_cv_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_171 {}
impl Clone for remove_cv_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_172 {}
impl Clone for remove_cv_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_173 {}
impl Clone for remove_cv_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_174 {}
impl Clone for remove_cv_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_175 {}
impl Clone for remove_cv_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_176 {}
impl Clone for remove_cv_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_177 {}
impl Clone for remove_cv_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_178 {}
impl Clone for remove_cv_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_179 {}
impl Clone for is_const_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_180 {}
impl Clone for is_const_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_181 {}
impl Clone for is_const_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_182 {}
impl Clone for is_const_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_183 {}
impl Clone for is_const_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_184 {}
impl Clone for __is_array_unknown_bounds_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_185 {}
impl Clone for __is_array_unknown_bounds_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_186 {}
impl Clone for __is_array_unknown_bounds_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_187 {}
impl Clone for __is_array_unknown_bounds_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_188 {}
impl Clone for __is_array_unknown_bounds_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_189 {}
impl Clone for __is_array_unknown_bounds_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_190 {}
impl Clone for __is_array_unknown_bounds_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_191 {}
impl Clone for __is_array_unknown_bounds_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_192 {}
impl Clone for __is_array_unknown_bounds_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_197 {}
impl Clone for __is_integral_helper_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_198 {}
impl Clone for __is_integral_helper_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_199 {}
impl Clone for __is_integral_helper_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_200 {}
impl Clone for __is_integral_helper_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_201 {}
impl Clone for __is_integral_helper_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_202 {}
impl Clone for __is_integral_helper_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_203 {}
impl Clone for is_integral_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_204 {}
impl Clone for is_integral_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_205 {}
impl Clone for is_integral_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_206 {}
impl Clone for is_integral_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_207 {}
impl Clone for is_integral_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_208 {}
impl Clone for is_integral_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_class_209 {}
impl Clone for is_class_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_class_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_210 {}
impl Clone for is_arithmetic_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_211 {}
impl Clone for is_scalar_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_212 {}
impl Clone for is_same_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_213 {}
impl Clone for is_same_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_214 {}
impl Clone for is_same_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_215 {}
impl Clone for is_same_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_216 {}
impl Clone for is_same_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_217 {}
impl Clone for is_same_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_218 {}
impl Clone for is_same_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_219 {}
impl Clone for is_same_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_232 {}
impl Clone for is_volatile_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_233 {}
impl Clone for is_volatile_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_234 {}
impl Clone for is_volatile_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_235 {}
impl Clone for is_volatile_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_236 {}
impl Clone for is_volatile_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_236 {
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
pub struct __is_destructible_safe_237 {}
impl Clone for __is_destructible_safe_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_237 {
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
pub struct is_move_constructible_238 {}
impl Clone for is_move_constructible_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_239 {}
impl Clone for is_move_constructible_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_240 {}
impl Clone for is_move_constructible_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_241 {}
impl Clone for is_nothrow_default_constructible_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_242 {}
impl Clone for is_nothrow_default_constructible_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_243 {}
impl Clone for is_nothrow_copy_constructible_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_244 {}
impl Clone for is_nothrow_copy_constructible_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_245 {}
impl Clone for is_nothrow_copy_constructible_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_246 {}
impl Clone for is_nothrow_copy_constructible_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_247 {}
impl Clone for is_nothrow_copy_constructible_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_248 {}
impl Clone for is_nothrow_copy_constructible_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_249 {}
impl Clone for is_nothrow_copy_constructible_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_250 {}
impl Clone for is_nothrow_move_constructible_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_251 {}
impl Clone for is_nothrow_move_constructible_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_252 {}
impl Clone for is_nothrow_move_constructible_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_253 {}
impl Clone for is_move_assignable_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_254 {}
impl Clone for is_move_assignable_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_255 {}
impl Clone for is_move_assignable_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_256 {}
impl Clone for is_nothrow_move_assignable_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_257 {}
impl Clone for is_nothrow_move_assignable_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_258 {}
impl Clone for is_nothrow_move_assignable_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_258 {
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
pub struct is_trivially_destructible_259 {}
impl Clone for is_trivially_destructible_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_260 {}
impl Clone for is_convertible_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_261 {}
impl Clone for is_convertible_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_262 {}
impl Clone for is_convertible_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_263 {}
impl Clone for is_convertible_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_264 {}
impl Clone for is_convertible_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_265 {}
impl Clone for is_convertible_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_266 {}
impl Clone for is_convertible_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_267 {}
impl Clone for is_convertible_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_268 {}
impl Clone for is_convertible_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_269 {}
impl Clone for is_convertible_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_270 {}
impl Clone for is_convertible_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_271 {}
impl Clone for remove_reference_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_272 {}
impl Clone for remove_reference_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_273 {}
impl Clone for remove_reference_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_274 {}
impl Clone for remove_reference_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_275 {}
impl Clone for remove_reference_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_276 {}
impl Clone for remove_reference_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_277 {}
impl Clone for remove_reference_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_278 {}
impl Clone for remove_reference_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_279 {}
impl Clone for remove_reference_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_280 {}
impl Clone for remove_reference_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_281 {}
impl Clone for remove_reference_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_282 {}
impl Clone for remove_reference_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_283 {}
impl Clone for __cv_selector_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_284 {}
impl Clone for __cv_selector_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_285 {}
impl Clone for __cv_selector_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_286 {}
impl Clone for __cv_selector_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_287 {}
impl Clone for __match_cv_qualifiers_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_288 {}
impl Clone for __match_cv_qualifiers_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_289 {}
impl Clone for __match_cv_qualifiers_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_290 {}
impl Clone for __match_cv_qualifiers_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_291 {}
impl Clone for __match_cv_qualifiers_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_292 {}
impl Clone for __make_unsigned_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_293 {}
impl Clone for __make_unsigned_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_294 {}
impl Clone for __make_unsigned_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_295 {}
impl Clone for __make_unsigned_selector_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_296 {}
impl Clone for __make_unsigned_selector_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_297 {}
impl Clone for __make_unsigned_selector_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_298 {}
impl Clone for __make_unsigned_selector_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_299 {}
impl Clone for __make_unsigned_selector_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_300 {}
impl Clone for __make_unsigned_selector_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_300 {
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
pub struct _List_304 {}
impl Clone for _List_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_305 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_306 {}
impl Clone for _List_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_307 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_308 {}
impl Clone for _List_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_309 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_310 {}
impl Clone for _List_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_311 {}
impl Clone for _List_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_312 {}
impl Clone for _List_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_313 {}
impl Clone for __select_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_314 {}
impl Clone for __select_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_315 {}
impl Clone for __select_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_316 {}
impl Clone for __select_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_317 {}
impl Clone for __select_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_318 {}
impl Clone for __make_signed_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_319 {}
impl Clone for __make_signed_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_320 {}
impl Clone for __make_signed_selector_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_321 {}
impl Clone for __make_signed_selector_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_322 {}
impl Clone for __make_signed_selector_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_323 {}
impl Clone for __make_signed_selector_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_324 {}
impl Clone for __make_signed_selector_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_pointer_325 {}
impl Clone for remove_pointer_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct add_pointer_326 {}
impl Clone for add_pointer_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_pointer_326 {
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
pub struct common_type_327 {}
impl Clone for common_type_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_327 {
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
pub struct __is_tuple_like_impl_328 {}
impl Clone for __is_tuple_like_impl_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_impl_329 {}
impl Clone for __is_tuple_like_impl_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_impl_330 {}
impl Clone for __is_tuple_like_impl_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_331 {}
impl Clone for __is_tuple_like_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_332 {}
impl Clone for __is_tuple_like_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_333 {}
impl Clone for __is_tuple_like_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_333 {
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
pub static mut is_array_v_334: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_335: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_336: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_337: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_338: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_339: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_340: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_341: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_342: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_343: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_344: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_345: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_346: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_347: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_348: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_349: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_350: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_351: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_352: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_353: std::cell::LazyCell<bool> =
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
pub struct _Char_types_354 {}
impl Clone for _Char_types_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Char_types_355 {}
impl Clone for _Char_types_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_356 {}
impl Clone for char_traits_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_356 {
    pub unsafe fn eq(__c1: *const libc::c_char, __c2: *const libc::c_char) -> bool {
        unimplemented!()
    }
}
impl char_traits_356 {
    pub unsafe fn length(mut __p: *const libc::c_char) -> usize {
        unimplemented!()
    }
}
impl char_traits_356 {
    pub unsafe fn find(
        mut __s: *const libc::c_char,
        mut __n: usize,
        __a: *const libc::c_char,
    ) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_357 {}
impl Clone for char_traits_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_357 {
    pub unsafe fn eq(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_357 {
    pub unsafe fn lt(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_357 {
    pub unsafe fn compare(mut __s1: *const i32, mut __s2: *const i32, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_357 {
    pub unsafe fn length(mut __p: *const i32) -> usize {
        unimplemented!()
    }
}
impl char_traits_357 {
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
pub type anon_358 = u32;
pub const anon_358__ISupper: anon_358 = 256;
pub const anon_358__ISlower: anon_358 = 512;
pub const anon_358__ISalpha: anon_358 = 1024;
pub const anon_358__ISdigit: anon_358 = 2048;
pub const anon_358__ISxdigit: anon_358 = 4096;
pub const anon_358__ISspace: anon_358 = 8192;
pub const anon_358__ISprint: anon_358 = 16384;
pub const anon_358__ISgraph: anon_358 = 32768;
pub const anon_358__ISblank: anon_358 = 1;
pub const anon_358__IScntrl: anon_358 = 2;
pub const anon_358__ISpunct: anon_358 = 4;
pub const anon_358__ISalnum: anon_358 = 8;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct ctype_359 {}
impl Clone for ctype_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl ctype_359 {
    pub unsafe fn widen_11(&mut self, mut __c: libc::c_char) -> libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct ctype_360 {}
impl Clone for ctype_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_get_361 {}
impl Clone for num_get_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_get_362 {}
impl Clone for num_get_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_put_363 {}
impl Clone for num_put_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_put_364 {}
impl Clone for num_put_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numpunct_365 {}
impl Clone for numpunct_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numpunct_366 {}
impl Clone for numpunct_366 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_366 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct collate_367 {}
impl Clone for collate_367 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_367 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct collate_368 {}
impl Clone for collate_368 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_368 {
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
    pub anon_369: i32,
    pub anon_370: i32,
    pub anon_371: i32,
    pub anon_372: i32,
    pub anon_373: i32,
    pub anon_374: i32,
    pub anon_375: i32,
    pub anon_376: i32,
    pub anon_377: i32,
    pub anon_378: i32,
    pub anon_379: i32,
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
    pub __value32: anon_380,
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
pub struct anon_380 {
    pub __low: u32,
    pub __high: u32,
}
impl Clone for anon_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_380 {
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
pub type anon_381 = u32;
pub const anon_381_PTHREAD_MUTEX_TIMED_NP: anon_381 = 0;
pub const anon_381_PTHREAD_MUTEX_RECURSIVE_NP: anon_381 = 1;
pub const anon_381_PTHREAD_MUTEX_ERRORCHECK_NP: anon_381 = 2;
pub const anon_381_PTHREAD_MUTEX_ADAPTIVE_NP: anon_381 = 3;
pub const anon_381_PTHREAD_MUTEX_NORMAL: anon_381 = 0;
pub const anon_381_PTHREAD_MUTEX_RECURSIVE: anon_381 = 1;
pub const anon_381_PTHREAD_MUTEX_ERRORCHECK: anon_381 = 2;
pub const anon_381_PTHREAD_MUTEX_DEFAULT: anon_381 = 0;
pub const anon_381_PTHREAD_MUTEX_FAST_NP: anon_381 = 0;
pub type anon_382 = u32;
pub const anon_382_PTHREAD_MUTEX_STALLED: anon_382 = 0;
pub const anon_382_PTHREAD_MUTEX_STALLED_NP: anon_382 = 0;
pub const anon_382_PTHREAD_MUTEX_ROBUST: anon_382 = 1;
pub const anon_382_PTHREAD_MUTEX_ROBUST_NP: anon_382 = 1;
pub type anon_383 = u32;
pub const anon_383_PTHREAD_RWLOCK_PREFER_READER_NP: anon_383 = 0;
pub const anon_383_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_383 = 1;
pub const anon_383_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_383 = 2;
pub const anon_383_PTHREAD_RWLOCK_DEFAULT_NP: anon_383 = 0;
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
pub type anon_384 = u32;
pub const anon_384_PTHREAD_CANCEL_DEFERRED: anon_384 = 0;
pub const anon_384_PTHREAD_CANCEL_ASYNCHRONOUS: anon_384 = 1;
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
pub static mut __libc_single_threaded_385: std::cell::LazyCell<libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_386 {}
impl Clone for __new_allocator_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_386 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_386 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_386) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_386 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_386,
    ) -> *mut __new_allocator_386 {
        unimplemented!()
    }
}
impl __new_allocator_386 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl __new_allocator_386 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut libc::c_char, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_386 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_387 {}
impl Clone for __new_allocator_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_387 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_387 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_387) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_387 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_387,
    ) -> *mut __new_allocator_387 {
        unimplemented!()
    }
}
impl __new_allocator_387 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        unimplemented!()
    }
}
impl __new_allocator_387 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_387 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_388 {}
impl Clone for __new_allocator_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_388) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_388,
    ) -> *mut __new_allocator_388 {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u16 {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u16, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_389 {}
impl Clone for __new_allocator_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_389) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_389,
    ) -> *mut __new_allocator_389 {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u32 {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_390 {}
impl Clone for __new_allocator_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_390) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_390,
    ) -> *mut __new_allocator_390 {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u8 {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u8, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_390 {
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
pub struct __are_same_391 {}
impl Clone for __are_same_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_392 = u32;
pub const anon_392___value: anon_392 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_393 {}
impl Clone for __are_same_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_394 = u32;
pub const anon_394___value: anon_394 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_395 {}
impl Clone for __are_same_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_396 = u32;
pub const anon_396___value: anon_396 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_397 {}
impl Clone for __are_same_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_398 = u32;
pub const anon_398___value: anon_398 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_399 {}
impl Clone for __are_same_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_400 = u32;
pub const anon_400___value: anon_400 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_401 {}
impl Clone for __are_same_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_401 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_402 = u32;
pub const anon_402___value: anon_402 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_403 {}
impl Clone for __is_integer_403 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_403 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_404 = u32;
pub const anon_404___value: anon_404 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_405 {}
impl Clone for __is_integer_405 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_405 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_406 = u32;
pub const anon_406___value: anon_406 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_407 {}
impl Clone for __is_integer_407 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_407 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_408 = u32;
pub const anon_408___value: anon_408 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_409 {}
impl Clone for __is_integer_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_410 = u32;
pub const anon_410___value: anon_410 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_411 {}
impl Clone for __is_integer_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_412 = u32;
pub const anon_412___value: anon_412 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_413 {}
impl Clone for __is_integer_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_414 = u32;
pub const anon_414___value: anon_414 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_415 {}
impl Clone for __is_integer_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_416 = u32;
pub const anon_416___value: anon_416 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_417 {}
impl Clone for __is_integer_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_418 = u32;
pub const anon_418___value: anon_418 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_419 {}
impl Clone for __is_integer_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_420 = u32;
pub const anon_420___value: anon_420 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_421 {}
impl Clone for __is_integer_421 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_421 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_422 = u32;
pub const anon_422___value: anon_422 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nonvolatile_trivially_copyable_423 {}
impl Clone for __is_nonvolatile_trivially_copyable_423 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_423 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_424 = u32;
pub const anon_424___value: anon_424 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_425 {}
impl Clone for __memcpyable_425 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_425 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_426 {}
impl Clone for __memcpyable_integer_426 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_426 {
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
pub struct iterator_427 {}
impl Clone for iterator_427 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_427 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_428 {}
impl Clone for iterator_428 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_428 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_428 {}
impl iterator_428 {}
impl iterator_428 {}
impl iterator_428 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_428) -> *mut iterator_428 {
        unimplemented!()
    }
}
impl iterator_428 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_428) -> *mut iterator_428 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_429 {}
impl Clone for iterator_429 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_429 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_429 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_429) -> *mut iterator_429 {
        unimplemented!()
    }
}
impl iterator_429 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_429) -> *mut iterator_429 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_430 {}
impl Clone for iterator_430 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_430 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_430 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_430) -> *mut iterator_430 {
        unimplemented!()
    }
}
impl iterator_430 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_430) -> *mut iterator_430 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_431 {}
impl Clone for iterator_traits_431 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_431 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_432 {}
impl Clone for iterator_traits_432 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_432 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_433 {}
impl Clone for iterator_traits_433 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_433 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_434 {}
impl Clone for iterator_traits_434 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_434 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_435 {}
impl Clone for iterator_traits_435 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_435 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_436 {}
impl Clone for iterator_traits_436 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_436 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_437 {}
impl Clone for iterator_traits_437 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_437 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_438 {}
impl Clone for iterator_traits_438 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_438 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_439 {}
impl Clone for __iterator_traits_439 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_439 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_440 {}
impl Clone for __iterator_traits_440 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_440 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_type_441 {}
impl Clone for __conditional_type_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_442 {}
impl Clone for __ptr_traits_ptr_to_442 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_442 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_442 {
    pub unsafe fn pointer_to(__r: *mut libc::c_char) -> *mut libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_443 {}
impl Clone for __ptr_traits_ptr_to_443 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_443 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_443 {
    pub unsafe fn pointer_to(__r: *const libc::c_char) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_444 {}
impl Clone for __ptr_traits_ptr_to_444 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_444 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_444 {
    pub unsafe fn pointer_to(__r: *mut i32) -> *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_445 {}
impl Clone for __ptr_traits_ptr_to_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_445 {
    pub unsafe fn pointer_to(__r: *const i32) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_446 {}
impl Clone for __ptr_traits_ptr_to_446 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_446 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_446 {
    pub unsafe fn pointer_to(__r: *mut u16) -> *mut u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_447 {}
impl Clone for __ptr_traits_ptr_to_447 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_447 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_447 {
    pub unsafe fn pointer_to(__r: *const u16) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_448 {}
impl Clone for __ptr_traits_ptr_to_448 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_448 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_448 {
    pub unsafe fn pointer_to(__r: *mut u32) -> *mut u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_449 {}
impl Clone for __ptr_traits_ptr_to_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_449 {
    pub unsafe fn pointer_to(__r: *const u32) -> *const u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_450 {}
impl Clone for pointer_traits_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_451 {}
impl Clone for pointer_traits_451 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_451 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_452 {}
impl Clone for pointer_traits_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_452 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_453 {}
impl Clone for pointer_traits_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_454 {}
impl Clone for pointer_traits_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_454 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_455 {}
impl Clone for pointer_traits_455 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_455 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_456 {}
impl Clone for pointer_traits_456 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_456 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_457 {}
impl Clone for pointer_traits_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_458 {}
impl Clone for reverse_iterator_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_458 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_458) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_458 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_458,
    ) -> *mut reverse_iterator_458 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_459 {}
impl Clone for reverse_iterator_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_459 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_459) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_459 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_459,
    ) -> *mut reverse_iterator_459 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_460 {}
impl Clone for __normal_iterator_460 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_460 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_460 {}
impl __normal_iterator_460 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_461 {}
impl Clone for binary_function_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_462 {}
impl Clone for binary_function_462 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_462 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_462 {}
impl binary_function_462 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_463 {}
impl Clone for binary_function_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_463 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_463 {}
impl binary_function_463 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_464 {}
impl Clone for greater_464 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_464 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_465 {}
impl Clone for less_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_466 {}
impl Clone for less_466 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_466 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_466 {
    pub unsafe fn operator_call(
        &self,
        mut __x: *const std_error_category,
        mut __y: *const std_error_category,
    ) -> bool {
        unimplemented!()
    }
}
impl less_466 {}
impl less_466 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_467 {}
impl Clone for less_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_467 {
    pub unsafe fn operator_call(
        &self,
        mut __x: *const libc::c_char,
        mut __y: *const libc::c_char,
    ) -> bool {
        unimplemented!()
    }
}
impl less_467 {}
impl less_467 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_468 {}
impl Clone for greater_equal_468 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_468 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_469 {}
impl Clone for less_equal_469 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_469 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_470 {}
impl Clone for __is_integer_nonstrict_470 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_470 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_471 = u32;
pub const anon_471___width: anon_471 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_472 {}
impl Clone for __is_integer_nonstrict_472 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_472 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_473 = u32;
pub const anon_473___width: anon_473 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_474 {}
impl Clone for __is_integer_nonstrict_474 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_474 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_475 = u32;
pub const anon_475___width: anon_475 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_476 {}
impl Clone for __is_integer_nonstrict_476 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_476 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_477 = u32;
pub const anon_477___width: anon_477 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_478 {}
impl Clone for __is_integer_nonstrict_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_479 = u32;
pub const anon_479___width: anon_479 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_480 {}
impl Clone for __is_integer_nonstrict_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_481 = u32;
pub const anon_481___width: anon_481 = 8;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_482 {}
impl Clone for __is_integer_nonstrict_482 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_482 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_483 = u32;
pub const anon_483___width: anon_483 = 16;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_484 {}
impl Clone for __numeric_traits_integer_484 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_484 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_int____is_signed_485: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____digits_486: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____max_487: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____min_488: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_489 {}
impl Clone for __numeric_traits_integer_489 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_489 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_490: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_491: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_492: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____min_493: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_494 {}
impl Clone for __numeric_traits_integer_494 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_494 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_495:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____digits_496: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____max_497: std::cell::LazyCell<
    u32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_498 {}
impl Clone for __numeric_traits_integer_498 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_498 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_499:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_500: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_501: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_502 {}
impl Clone for __numeric_traits_integer_502 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_502 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_503:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_504:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_505:
    std::cell::LazyCell<u64> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_506 {}
impl Clone for __numeric_traits_integer_506 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_506 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_char____is_signed_507: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_char____digits_508: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_char____max_509: std::cell::LazyCell<
    libc::c_char,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut __gnu_cxx___numeric_traits_integer_short____is_signed_511: std::cell::LazyCell<
    bool,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____digits_512: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____max_513: std::cell::LazyCell<i16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____min_514: std::cell::LazyCell<i16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_515 {}
impl Clone for __numeric_traits_floating_515 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_515 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_float____max_exponent10_516:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_517 {}
impl Clone for __numeric_traits_floating_517 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_517 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_double____max_exponent10_518:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_519 {}
impl Clone for __numeric_traits_floating_519 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_519 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_520:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_521 {}
impl Clone for __numeric_traits_521 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_521 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_522 {}
impl Clone for __numeric_traits_522 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_522 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_523 {}
impl Clone for __numeric_traits_523 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_523 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_524 {}
impl Clone for __numeric_traits_524 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_524 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_525 {}
impl Clone for __numeric_traits_525 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_525 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_526 {}
impl Clone for __numeric_traits_526 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_526 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_527 {}
impl Clone for __numeric_traits_527 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_527 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_528 {}
impl Clone for __numeric_traits_528 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_528 {
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
pub static mut piecewise_construct_529: std::cell::LazyCell<std_piecewise_construct_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_530 {}
impl Clone for tuple_530 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_530 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_530 {}
impl tuple_530 {}
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
pub struct initializer_list_531 {}
impl Clone for initializer_list_531 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_531 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_531 {}
impl initializer_list_531 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_532 {}
impl Clone for initializer_list_532 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_532 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_532 {}
impl initializer_list_532 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_533 {}
impl Clone for initializer_list_533 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_533 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_533 {}
impl initializer_list_533 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_534 {}
impl Clone for initializer_list_534 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_534 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_534 {}
impl initializer_list_534 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_535 {}
impl Clone for initializer_list_535 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_535 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_535 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl initializer_list_535 {
    pub unsafe fn begin(&self) -> *const bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_536 {}
impl Clone for initializer_list_536 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_536 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_536 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl initializer_list_536 {
    pub unsafe fn begin(&self) -> *const u8 {
        unimplemented!()
    }
}
impl initializer_list_536 {
    pub unsafe fn end(&self) -> *const u8 {
        unimplemented!()
    }
}
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
pub struct __is_allocator_538 {}
impl Clone for __is_allocator_538 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_538 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_539 {}
impl Clone for __is_allocator_539 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_539 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_540 {}
impl Clone for __is_allocator_540 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_540 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_541 {}
impl Clone for __is_allocator_541 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_541 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_542 {}
impl Clone for __alloc_traits_542 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_542 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_543 {}
impl Clone for rebind_543 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_543 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_544 {}
impl Clone for __alloc_traits_544 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_544 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_545 {}
impl Clone for rebind_545 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_545 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_546 {}
impl Clone for __alloc_traits_546 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_546 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_547 {}
impl Clone for rebind_547 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_547 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_548 {}
impl Clone for __alloc_traits_548 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_548 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_549 {}
impl Clone for rebind_549 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_549 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_550 {}
impl Clone for __alloc_traits_550 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_550 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_551 {}
impl Clone for rebind_551 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_551 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_552 {}
impl Clone for __hash_base_552 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_552 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_553 {}
impl Clone for __hash_base_553 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_553 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_554 {}
impl Clone for __hash_base_554 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_554 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_555 {}
impl Clone for __hash_base_555 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_555 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_556 {}
impl Clone for __hash_base_556 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_556 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_557 {}
impl Clone for __hash_base_557 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_557 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_558 {}
impl Clone for __hash_base_558 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_558 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_559 {}
impl Clone for __hash_base_559 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_559 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_560 {}
impl Clone for __hash_base_560 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_560 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_561 {}
impl Clone for __hash_base_561 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_561 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_562 {}
impl Clone for __hash_base_562 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_562 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_563 {}
impl Clone for __hash_base_563 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_563 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_564 {}
impl Clone for __hash_base_564 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_564 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_565 {}
impl Clone for __hash_base_565 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_565 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_566 {}
impl Clone for __hash_base_566 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_566 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_567 {}
impl Clone for __hash_base_567 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_567 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_568 {}
impl Clone for __hash_base_568 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_568 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_579 {}
impl Clone for hash_579 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_579 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_580 {}
impl Clone for hash_580 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_580 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_581 {}
impl Clone for hash_581 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_581 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_582 {}
impl Clone for hash_582 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_582 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_583 {}
impl Clone for hash_583 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_583 {
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
    pub unsafe fn hash_584(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_585(__val: *const f64) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_586(__val: *const i32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn __hash_combine_587(
        __val: *const *const std_error_category,
        mut __hash: usize,
    ) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_18 {}
impl Clone for basic_string_view_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_18 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_18) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_18 {
    pub unsafe fn new_1(mut __str: *const libc::c_char, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_18 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_18,
    ) -> *mut basic_string_view_18 {
        unimplemented!()
    }
}
impl basic_string_view_18 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_18 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_18 {
    pub unsafe fn data(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_588 {}
impl Clone for basic_string_view_588 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_588 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_588 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_588) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_588 {
    pub unsafe fn new_1(mut __str: *const i32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_588 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_588,
    ) -> *mut basic_string_view_588 {
        unimplemented!()
    }
}
impl basic_string_view_588 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_588 {
    pub unsafe fn data(&self) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_589 {}
impl Clone for basic_string_view_589 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_589 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_589 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_589) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_589 {
    pub unsafe fn new_1(mut __str: *const u16, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_589 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_589,
    ) -> *mut basic_string_view_589 {
        unimplemented!()
    }
}
impl basic_string_view_589 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_589 {
    pub unsafe fn data(&self) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_590 {}
impl Clone for basic_string_view_590 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_590 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_590 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_590) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_590 {
    pub unsafe fn new_1(mut __str: *const u32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_590 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_590,
    ) -> *mut basic_string_view_590 {
        unimplemented!()
    }
}
impl basic_string_view_590 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_590 {
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
pub static mut __integer_to_chars_is_unsigned_591: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_592: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_593: std::cell::LazyCell<bool> =
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
pub static mut allocator_arg_594: std::cell::LazyCell<std_allocator_arg_t> =
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
pub static mut std_locale_ctype_595: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_numeric_596: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_collate_597: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_time_598: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_monetary_599: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_messages_600: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type anon_601 = u32;
pub const anon_601__S_categories_size: anon_601 = 12;
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
    pub anon_602: anon_602,
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
pub struct anon_602 {
    pub _M_p: *const libc::c_char,
    pub _M_bytes: [libc::c_char; 8],
}
impl Clone for anon_602 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_602 {
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
    pub unsafe fn new_1(__arg: *const basic_string_15) -> Self {
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
pub struct is_error_code_enum_603 {}
impl Clone for is_error_code_enum_603 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_603 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_604 {}
impl Clone for is_error_code_enum_604 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_604 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_605 {}
impl Clone for is_error_code_enum_605 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_605 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_606 {}
impl Clone for is_error_code_enum_606 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_606 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_607 {}
impl Clone for is_error_code_enum_607 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_607 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_608 {}
impl Clone for is_error_code_enum_608 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_608 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_609 {}
impl Clone for is_error_code_enum_609 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_609 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_610 {}
impl Clone for is_error_code_enum_610 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_610 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_611 {}
impl Clone for is_error_code_enum_611 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_611 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_612 {}
impl Clone for is_error_code_enum_612 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_612 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_613 {}
impl Clone for is_error_code_enum_613 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_613 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_condition_enum_614 {}
impl Clone for is_error_condition_enum_614 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_614 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_condition_enum_615 {}
impl Clone for is_error_condition_enum_615 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_615 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_condition_enum_616 {}
impl Clone for is_error_condition_enum_616 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_616 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_condition_enum_617 {}
impl Clone for is_error_condition_enum_617 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_617 {
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
    pub unsafe fn message(&self) -> basic_string_15 {
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
pub static mut std_ios_base_boolalpha_618: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_dec_619: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_fixed_620: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_hex_621: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_internal_622: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_left_623: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_oct_624: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_right_625: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_scientific_626: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showbase_627: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showpoint_628: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showpos_629: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_skipws_630: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_unitbuf_631: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_uppercase_632: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_adjustfield_633: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_basefield_634: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_floatfield_635: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_badbit_636: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_eofbit_637: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_failbit_638: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_goodbit_639: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_in_640: std::cell::LazyCell<std__Ios_Openmode> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_out_641: std::cell::LazyCell<std__Ios_Openmode> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_cur_642: std::cell::LazyCell<std__Ios_Seekdir> =
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
pub type anon_643 = u32;
pub const anon_643__S_local_word_size: anon_643 = 8;
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
pub type anon_644 = i32;
pub const anon_644___ISwupper: anon_644 = 0;
pub const anon_644___ISwlower: anon_644 = 1;
pub const anon_644___ISwalpha: anon_644 = 2;
pub const anon_644___ISwdigit: anon_644 = 3;
pub const anon_644___ISwxdigit: anon_644 = 4;
pub const anon_644___ISwspace: anon_644 = 5;
pub const anon_644___ISwprint: anon_644 = 6;
pub const anon_644___ISwgraph: anon_644 = 7;
pub const anon_644___ISwblank: anon_644 = 8;
pub const anon_644___ISwcntrl: anon_644 = 9;
pub const anon_644___ISwpunct: anon_644 = 10;
pub const anon_644___ISwalnum: anon_644 = 11;
pub const anon_644__ISwupper: anon_644 = 16777216;
pub const anon_644__ISwlower: anon_644 = 33554432;
pub const anon_644__ISwalpha: anon_644 = 67108864;
pub const anon_644__ISwdigit: anon_644 = 134217728;
pub const anon_644__ISwxdigit: anon_644 = 268435456;
pub const anon_644__ISwspace: anon_644 = 536870912;
pub const anon_644__ISwprint: anon_644 = 1073741824;
pub const anon_644__ISwgraph: anon_644 = -2147483648;
pub const anon_644__ISwblank: anon_644 = 65536;
pub const anon_644__ISwcntrl: anon_644 = 131072;
pub const anon_644__ISwpunct: anon_644 = 262144;
pub const anon_644__ISwalnum: anon_644 = 524288;
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
pub static mut std_ctype_base_upper_645: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_lower_646: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_alpha_647: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_digit_648: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_xdigit_649: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_space_650: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_print_651: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_graph_652: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_cntrl_653: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_punct_654: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_alnum_655: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_blank_656: std::cell::LazyCell<u16> =
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
pub type anon_657 = u32;
pub const anon_657__S_ominus: anon_657 = 0;
pub const anon_657__S_oplus: anon_657 = 1;
pub const anon_657__S_ox: anon_657 = 2;
pub const anon_657__S_oX: anon_657 = 3;
pub const anon_657__S_odigits: anon_657 = 4;
pub const anon_657__S_odigits_end: anon_657 = 20;
pub const anon_657__S_oudigits: anon_657 = 20;
pub const anon_657__S_oudigits_end: anon_657 = 36;
pub const anon_657__S_oe: anon_657 = 18;
pub const anon_657__S_oE: anon_657 = 34;
pub const anon_657__S_oend: anon_657 = 36;
pub static mut std___num_base__S_atoms_out_658: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std___num_base__S_atoms_in_659: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type anon_660 = u32;
pub const anon_660__S_iminus: anon_660 = 0;
pub const anon_660__S_iplus: anon_660 = 1;
pub const anon_660__S_ix: anon_660 = 2;
pub const anon_660__S_iX: anon_660 = 3;
pub const anon_660__S_izero: anon_660 = 4;
pub const anon_660__S_ie: anon_660 = 18;
pub const anon_660__S_iE: anon_660 = 24;
pub const anon_660__S_iend: anon_660 = 26;
pub static mut cout_661: std::cell::LazyCell<basic_ostream_47> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_662 {
    pub _M_impl: std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl,
}
impl Clone for _Vector_base_662 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_662 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    pub _M_start: *mut u8,
    pub _M_finish: *mut u8,
    pub _M_end_of_storage: *mut u8,
}
impl Clone for std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {}
impl Clone for std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {
    pub unsafe fn new_1(__a: *const allocator_5) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_662 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_5 {
        unimplemented!()
    }
}
impl _Vector_base_662 {
    pub unsafe fn new_1(__a: *const allocator_5) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_662 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_662) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_662 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut u8 {
        unimplemented!()
    }
}
impl _Vector_base_662 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut u8, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_663 {}
impl Clone for vector_663 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_663 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_663 {
    pub unsafe fn copy_from(__x: *const vector_663) -> Self {
        unimplemented!()
    }
}
impl vector_663 {
    pub unsafe fn move_from(_a0: *mut vector_663) -> Self {
        unimplemented!()
    }
}
impl vector_663 {
    pub unsafe fn new_1(mut __l: initializer_list_536, __a: Option<*const allocator_5>) -> Self {
        unimplemented!()
    }
}
impl vector_663 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_663) -> *mut vector_663 {
        unimplemented!()
    }
}
impl vector_663 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_663) -> *mut vector_663 {
        unimplemented!()
    }
}
impl vector_663 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_663 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut u8 {
        unimplemented!()
    }
}
pub type anon_664 = u32;
pub const anon_664__S_word_bit: anon_664 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(_a0: *const std__Bit_reference) -> Self {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn operator_assign_5(&mut self, mut __x: bool) -> *mut std__Bit_reference {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn copy_assign(
        &mut self,
        __x: *const std__Bit_reference,
    ) -> *mut std__Bit_reference {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Bit_iterator_base {
    pub _M_p: *mut u64,
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
    pub unsafe fn _M_assume_normalized(&self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn new(mut __x: *mut u64, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_bump_up(&mut self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_bump_down(&mut self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_incr(&mut self, mut __i: i64) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std__Bit_iterator_base,
    ) -> *mut std__Bit_iterator_base {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std__Bit_iterator_base,
    ) -> *mut std__Bit_iterator_base {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(mut __x: *mut u64, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_deref(&self) -> std__Bit_reference {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_add_assign(&mut self, mut __i: i64) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_sub_assign(&mut self, mut __i: i64) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {}
impl std__Bit_iterator {}
impl std__Bit_iterator {
    pub unsafe fn copy_assign(&mut self, _a0: *const std__Bit_iterator) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn move_assign(&mut self, _a0: *mut std__Bit_iterator) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl std__Bit_const_iterator {
    pub unsafe fn operator_deref(&self) -> bool {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_add_assign(&mut self, mut __i: i64) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_sub_assign(&mut self, mut __i: i64) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std__Bit_const_iterator,
    ) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std__Bit_const_iterator,
    ) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vec_: vector_663 =
        vector_663::new_1({ __cpp2rust_unimplemented::<initializer_list_536>() }, None);
    let mut i: i32 = 27;
    let mut str: basic_string_15 = basic_string_15::new_3({ c"foo.".as_ptr() }, None);
    (unsafe {
        basic_ostream_47::operator_shl_5(
            &mut (*(unsafe {
                let ___pf: Option<unsafe fn(*mut basic_ostream_47) -> *mut basic_ostream_47> =
                    Some(endl_665);
                basic_ostream_47::operator_shl_3(
                    &mut (*(unsafe {
                        let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                            let ___n: i32 = i;
                            basic_ostream_47::operator_shl_11(
                                &mut (*(unsafe {
                                    let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                                        let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                                            let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                                                basic_ostream_47::operator_shl_11(
                                                    &mut (*(unsafe {
                                                        let ___out: *mut basic_ostream_47 =
                                                            &mut (*(unsafe {
                                                                basic_ostream_47::operator_shl_5(
                                                                    &mut (*(unsafe {
                                                                        basic_ostream_47 :: operator_shl_3 ( &mut ( * ( unsafe { let ___os: *mut basic_ostream_47  = & mut ( * ( unsafe { let ___out: *mut basic_ostream_47  = & mut ( * ( unsafe { let ___out: *mut basic_ostream_47  = & mut ( * ( unsafe { let ___out: *mut basic_ostream_47  = & mut ( * ( unsafe { let ___out: *mut basic_ostream_47  = & mut ( * ( unsafe { basic_ostream_47 :: operator_shl_11 ( &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_661))  , i , ) } )  ) ; operator_shl_666 ( ___out , c" a" .as_ptr() , ) } )  ) ; operator_shl_667 ( ___out , ( * ( unsafe { vector_663 :: operator_index_46 ( &mut vec_  , 0_u64 , ) } )  ) , ) } )  ) ; let ___c: u8  = ( * ( unsafe { vector_663 :: operator_index_46 ( &mut vec_  , 1_u64 , ) } )  ) ; operator_shl_667 ( ___out , ___c , ) } )  ) ; operator_shl_668 ( ___out , ( 'o' as libc::c_char  ) , ) } )  ) ; operator_shl_669 ( ___os , & str , ) } )  )  , Some(endl_665) , )
                                                                    })),
                                                                    Some(hex_670),
                                                                )
                                                            }));
                                                        operator_shl_666(___out, c"0x".as_ptr())
                                                    })),
                                                    27,
                                                )
                                            }));
                                            operator_shl_666(___out, c" a\xc3\xa7ordas?".as_ptr())
                                        }));
                                        operator_shl_668(___out, ('\n' as libc::c_char))
                                    }));
                                    operator_shl_666(___out, c"Sim, 0x".as_ptr())
                                })),
                                ___n,
                            )
                        }));
                        operator_shl_668(___out, ('.' as libc::c_char))
                    })),
                    ___pf,
                )
            })),
            Some(dec_671),
        )
    });
    (unsafe {
        let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
            let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                    let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                        let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                            let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                                let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                                    let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                                        let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                                            let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                                                let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                                                    let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                                                        let ___out: *mut basic_ostream_47 =
                                                            &mut (*(unsafe {
                                                                let ___out: *mut basic_ostream_47  = & mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_661)) ;
                                                                operator_shl_668(
                                                                    ___out,
                                                                    ('H' as libc::c_char),
                                                                )
                                                            }));
                                                        operator_shl_668(
                                                            ___out,
                                                            ('e' as libc::c_char),
                                                        )
                                                    }));
                                                    operator_shl_668(___out, ('l' as libc::c_char))
                                                }));
                                                operator_shl_668(___out, ('l' as libc::c_char))
                                            }));
                                            operator_shl_668(___out, ('o' as libc::c_char))
                                        }));
                                        operator_shl_668(___out, (',' as libc::c_char))
                                    }));
                                    operator_shl_668(___out, (' ' as libc::c_char))
                                }));
                                operator_shl_668(___out, ('W' as libc::c_char))
                            }));
                            operator_shl_668(___out, ('o' as libc::c_char))
                        }));
                        operator_shl_668(___out, ('r' as libc::c_char))
                    }));
                    operator_shl_668(___out, ('l' as libc::c_char))
                }));
                operator_shl_668(___out, ('d' as libc::c_char))
            }));
            operator_shl_668(___out, ('!' as libc::c_char))
        }));
        operator_shl_668(___out, ('\n' as libc::c_char))
    });
    (unsafe {
        let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
            let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                let ___out: *mut basic_ostream_47 = &mut (*(unsafe {
                    let ___out: *mut basic_ostream_47 =
                        &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_661));
                    operator_shl_667(
                        ___out,
                        (*(unsafe { vector_663::operator_index_46(&mut vec_, 0_u64) })),
                    )
                }));
                operator_shl_668(___out, ('\n' as libc::c_char))
            }));
            let ___c: u8 = (*(unsafe { vector_663::operator_index_46(&mut vec_, 1_u64) }));
            operator_shl_667(___out, ___c)
        }));
        operator_shl_668(___out, ('\n' as libc::c_char))
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
pub struct std_error_category;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
#[track_caller]
pub fn __cpp2rust_unimplemented<T>() -> T {
    unimplemented!()
}
pub unsafe fn endl_665(__os: *mut basic_ostream_47) -> *mut basic_ostream_47 {
    unimplemented!()
}
pub unsafe fn operator_shl_666(
    __out: *mut basic_ostream_47,
    mut __s: *const libc::c_char,
) -> *mut basic_ostream_47 {
    unimplemented!()
}
pub unsafe fn operator_shl_667(__out: *mut basic_ostream_47, mut __c: u8) -> *mut basic_ostream_47 {
    unimplemented!()
}
pub unsafe fn operator_shl_668(
    __out: *mut basic_ostream_47,
    mut __c: libc::c_char,
) -> *mut basic_ostream_47 {
    unimplemented!()
}
pub unsafe fn operator_shl_669(
    __os: *mut basic_ostream_47,
    __str: *const basic_string_15,
) -> *mut basic_ostream_47 {
    unimplemented!()
}
pub unsafe fn hex_670(__base: *mut std_ios_base) -> *mut std_ios_base {
    unimplemented!()
}
pub unsafe fn dec_671(__base: *mut std_ios_base) -> *mut std_ios_base {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
