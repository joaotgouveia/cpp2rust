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
    pub unsafe fn operator_shl_11(&mut self, mut __n: i32) -> *mut basic_ostream_45 {
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
pub static mut nothrow_49: std::cell::LazyCell<std_nothrow_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_50 {}
impl Clone for integral_constant_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_integral_constant_bool__true__value_51: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_50 {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_50 {}
impl integral_constant_50 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_52 {}
impl Clone for integral_constant_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_integral_constant_bool__false__value_53: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_52 {}
impl integral_constant_52 {}
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_65 {}
impl Clone for __conditional_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_179 {}
impl Clone for remove_cv_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_187 {}
impl Clone for is_const_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_204 {}
impl Clone for __is_integral_helper_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_209 {}
impl Clone for is_integral_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_214 {}
impl Clone for is_pointer_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_rvalue_reference_215 {}
impl Clone for is_rvalue_reference_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_rvalue_reference_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_object_pointer_216 {}
impl Clone for is_member_object_pointer_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_object_pointer_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_function_pointer_217 {}
impl Clone for is_member_function_pointer_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_class_218 {}
impl Clone for is_class_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_class_218 {
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_238 {}
impl Clone for is_volatile_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_empty_243 {}
impl Clone for is_empty_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct is_default_constructible_245 {}
impl Clone for is_default_constructible_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_247 {}
impl Clone for is_move_constructible_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_251 {}
impl Clone for is_nothrow_default_constructible_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_255 {}
impl Clone for is_nothrow_copy_constructible_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_260 {}
impl Clone for is_nothrow_move_constructible_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_263 {}
impl Clone for is_assignable_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_267 {}
impl Clone for is_move_assignable_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_271 {}
impl Clone for is_nothrow_move_assignable_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct __is_implicitly_default_constructible_impl_274 {}
impl Clone for __is_implicitly_default_constructible_impl_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_safe_276 {}
impl Clone for __is_implicitly_default_constructible_safe_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_278 {}
impl Clone for __is_implicitly_default_constructible_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_280 {}
impl Clone for is_convertible_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_291 {}
impl Clone for remove_reference_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct add_lvalue_reference_306 {}
impl Clone for add_lvalue_reference_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_lvalue_reference_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_307 {}
impl Clone for __cv_selector_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_311 {}
impl Clone for __match_cv_qualifiers_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_316 {}
impl Clone for __make_unsigned_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_319 {}
impl Clone for __make_unsigned_selector_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct _List_328 {}
impl Clone for _List_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_329 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_330 {}
impl Clone for _List_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_331 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_332 {}
impl Clone for _List_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_333 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_334 {}
impl Clone for _List_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_337 {}
impl Clone for __select_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_342 {}
impl Clone for __make_signed_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_344 {}
impl Clone for __make_signed_selector_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct add_pointer_349 {}
impl Clone for add_pointer_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_pointer_349 {
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
pub struct common_type_350 {}
impl Clone for common_type_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __success_type_351 {}
impl Clone for __success_type_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __success_type_351 {
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
pub struct __result_of_success_352 {}
impl Clone for __result_of_success_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_success_352 {
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
pub struct __result_of_impl_353 {}
impl Clone for __result_of_impl_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_impl_353 {
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
pub struct __invoke_result_354 {}
impl Clone for __invoke_result_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __invoke_result_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_impl_355 {}
impl Clone for __is_tuple_like_impl_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_358 {}
impl Clone for __is_tuple_like_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct __is_invocable_impl_361 {}
impl Clone for __is_invocable_impl_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_impl_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_invocable_362 {}
impl Clone for __is_invocable_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_362 {
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
pub static mut is_array_v_363: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_364: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_365: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_366: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_367: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_368: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_369: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_370: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_371: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_372: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_373: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_374: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_375: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_376: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_377: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_378: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_379: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_380: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_381: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_382: std::cell::LazyCell<bool> =
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
pub struct _Char_types_383 {}
impl Clone for _Char_types_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_385 {}
impl Clone for char_traits_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_385 {
    pub unsafe fn eq(__c1: *const libc::c_char, __c2: *const libc::c_char) -> bool {
        unimplemented!()
    }
}
impl char_traits_385 {
    pub unsafe fn length(mut __p: *const libc::c_char) -> usize {
        unimplemented!()
    }
}
impl char_traits_385 {
    pub unsafe fn find(
        mut __s: *const libc::c_char,
        mut __n: usize,
        __a: *const libc::c_char,
    ) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn eq(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_386 {
    pub unsafe fn lt(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_386 {
    pub unsafe fn compare(mut __s1: *const i32, mut __s2: *const i32, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_386 {
    pub unsafe fn length(mut __p: *const i32) -> usize {
        unimplemented!()
    }
}
impl char_traits_386 {
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
pub type anon_387 = u32;
pub const anon_387__ISupper: anon_387 = 256;
pub const anon_387__ISlower: anon_387 = 512;
pub const anon_387__ISalpha: anon_387 = 1024;
pub const anon_387__ISdigit: anon_387 = 2048;
pub const anon_387__ISxdigit: anon_387 = 4096;
pub const anon_387__ISspace: anon_387 = 8192;
pub const anon_387__ISprint: anon_387 = 16384;
pub const anon_387__ISgraph: anon_387 = 32768;
pub const anon_387__ISblank: anon_387 = 1;
pub const anon_387__IScntrl: anon_387 = 2;
pub const anon_387__ISpunct: anon_387 = 4;
pub const anon_387__ISalnum: anon_387 = 8;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct ctype_388 {}
impl Clone for ctype_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_get_390 {}
impl Clone for num_get_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_put_392 {}
impl Clone for num_put_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numpunct_394 {}
impl Clone for numpunct_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct collate_396 {}
impl Clone for collate_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub anon_398: i32,
    pub anon_399: i32,
    pub anon_400: i32,
    pub anon_401: i32,
    pub anon_402: i32,
    pub anon_403: i32,
    pub anon_404: i32,
    pub anon_405: i32,
    pub anon_406: i32,
    pub anon_407: i32,
    pub anon_408: i32,
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
    pub __value32: anon_409,
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
pub struct anon_409 {
    pub __low: u32,
    pub __high: u32,
}
impl Clone for anon_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_409 {
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
pub type anon_410 = u32;
pub const anon_410_PTHREAD_MUTEX_TIMED_NP: anon_410 = 0;
pub const anon_410_PTHREAD_MUTEX_RECURSIVE_NP: anon_410 = 1;
pub const anon_410_PTHREAD_MUTEX_ERRORCHECK_NP: anon_410 = 2;
pub const anon_410_PTHREAD_MUTEX_ADAPTIVE_NP: anon_410 = 3;
pub const anon_410_PTHREAD_MUTEX_NORMAL: anon_410 = 0;
pub const anon_410_PTHREAD_MUTEX_RECURSIVE: anon_410 = 1;
pub const anon_410_PTHREAD_MUTEX_ERRORCHECK: anon_410 = 2;
pub const anon_410_PTHREAD_MUTEX_DEFAULT: anon_410 = 0;
pub const anon_410_PTHREAD_MUTEX_FAST_NP: anon_410 = 0;
pub type anon_411 = u32;
pub const anon_411_PTHREAD_MUTEX_STALLED: anon_411 = 0;
pub const anon_411_PTHREAD_MUTEX_STALLED_NP: anon_411 = 0;
pub const anon_411_PTHREAD_MUTEX_ROBUST: anon_411 = 1;
pub const anon_411_PTHREAD_MUTEX_ROBUST_NP: anon_411 = 1;
pub type anon_412 = u32;
pub const anon_412_PTHREAD_RWLOCK_PREFER_READER_NP: anon_412 = 0;
pub const anon_412_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_412 = 1;
pub const anon_412_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_412 = 2;
pub const anon_412_PTHREAD_RWLOCK_DEFAULT_NP: anon_412 = 0;
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
pub type anon_413 = u32;
pub const anon_413_PTHREAD_CANCEL_DEFERRED: anon_413 = 0;
pub const anon_413_PTHREAD_CANCEL_ASYNCHRONOUS: anon_413 = 1;
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
pub static mut __libc_single_threaded_414: std::cell::LazyCell<libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_415 {}
impl Clone for __new_allocator_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_415 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_415 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_415) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_415 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_415,
    ) -> *mut __new_allocator_415 {
        unimplemented!()
    }
}
impl __new_allocator_415 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl __new_allocator_415 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut libc::c_char, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_415 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_416 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_416) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_416 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_416,
    ) -> *mut __new_allocator_416 {
        unimplemented!()
    }
}
impl __new_allocator_416 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        unimplemented!()
    }
}
impl __new_allocator_416 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_416 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_417) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_417,
    ) -> *mut __new_allocator_417 {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u16 {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u16, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_418 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_418) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_418 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_418,
    ) -> *mut __new_allocator_418 {
        unimplemented!()
    }
}
impl __new_allocator_418 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u32 {
        unimplemented!()
    }
}
impl __new_allocator_418 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_418 {
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
pub struct __are_same_419 {}
impl Clone for __are_same_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_420 = u32;
pub const anon_420___value: anon_420 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_421 {}
impl Clone for __are_same_421 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_421 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_422 = u32;
pub const anon_422___value: anon_422 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_423 {}
impl Clone for __are_same_423 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_423 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_424 = u32;
pub const anon_424___value: anon_424 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_425 {}
impl Clone for __are_same_425 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_425 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_426 = u32;
pub const anon_426___value: anon_426 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_427 {}
impl Clone for __are_same_427 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_427 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_428 = u32;
pub const anon_428___value: anon_428 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_429 {}
impl Clone for __are_same_429 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_429 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_430 = u32;
pub const anon_430___value: anon_430 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_431 {}
impl Clone for __is_integer_431 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_431 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_432 = u32;
pub const anon_432___value: anon_432 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_433 {}
impl Clone for __is_integer_433 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_433 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_434 = u32;
pub const anon_434___value: anon_434 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_435 {}
impl Clone for __is_integer_435 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_435 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_436 = u32;
pub const anon_436___value: anon_436 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_437 {}
impl Clone for __is_integer_437 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_437 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_438 = u32;
pub const anon_438___value: anon_438 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_439 {}
impl Clone for __is_integer_439 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_439 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_440 = u32;
pub const anon_440___value: anon_440 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_441 {}
impl Clone for __is_integer_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_442 = u32;
pub const anon_442___value: anon_442 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_443 {}
impl Clone for __is_integer_443 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_443 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_444 = u32;
pub const anon_444___value: anon_444 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_445 {}
impl Clone for __is_integer_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_446 = u32;
pub const anon_446___value: anon_446 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_447 {}
impl Clone for __is_integer_447 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_447 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_448 = u32;
pub const anon_448___value: anon_448 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_449 {}
impl Clone for __is_integer_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_450 = u32;
pub const anon_450___value: anon_450 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_451 {}
impl Clone for __memcpyable_integer_451 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_451 {
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
pub struct iterator_452 {}
impl Clone for iterator_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_452 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_453 {}
impl Clone for iterator_traits_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_type_458 {}
impl Clone for __conditional_type_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_459 {}
impl Clone for __ptr_traits_ptr_to_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_459 {
    pub unsafe fn pointer_to(__r: *mut libc::c_char) -> *mut libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *const libc::c_char) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *mut i32) -> *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *const i32) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *mut u16) -> *mut u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *const u16) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *mut u32) -> *mut u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *const u32) -> *const u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_467 {}
impl Clone for pointer_traits_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_475 {}
impl Clone for __normal_iterator_475 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_475 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_475 {}
impl __normal_iterator_475 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_476 {}
impl Clone for binary_function_476 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_476 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl binary_function_477 {}
impl binary_function_477 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_479 {}
impl Clone for greater_479 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_479 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_480 {}
impl Clone for less_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl less_481 {
    pub unsafe fn operator_call(
        &self,
        mut __x: *const std_error_category,
        mut __y: *const std_error_category,
    ) -> bool {
        unimplemented!()
    }
}
impl less_481 {}
impl less_481 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl less_482 {
    pub unsafe fn operator_call(
        &self,
        mut __x: *const libc::c_char,
        mut __y: *const libc::c_char,
    ) -> bool {
        unimplemented!()
    }
}
impl less_482 {}
impl less_482 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_483 {}
impl Clone for greater_equal_483 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_483 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_484 {}
impl Clone for less_equal_484 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_484 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_485 {}
impl Clone for __is_integer_nonstrict_485 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_485 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_486 = u32;
pub const anon_486___width: anon_486 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_487 {}
impl Clone for __is_integer_nonstrict_487 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_487 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_488 = u32;
pub const anon_488___width: anon_488 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_489 {}
impl Clone for __is_integer_nonstrict_489 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_489 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_490 = u32;
pub const anon_490___width: anon_490 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_491 {}
impl Clone for __is_integer_nonstrict_491 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_491 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_492 = u32;
pub const anon_492___width: anon_492 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_493 {}
impl Clone for __is_integer_nonstrict_493 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_493 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_494 = u32;
pub const anon_494___width: anon_494 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_495 {}
impl Clone for __is_integer_nonstrict_495 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_495 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_496 = u32;
pub const anon_496___width: anon_496 = 8;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_497 {}
impl Clone for __is_integer_nonstrict_497 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_497 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_498 = u32;
pub const anon_498___width: anon_498 = 16;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_499 {}
impl Clone for __numeric_traits_integer_499 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_499 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_int____is_signed_500: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____digits_501: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____max_502: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____min_503: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_504 {}
impl Clone for __numeric_traits_integer_504 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_504 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_505: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_506: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_507: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____min_508: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_509 {}
impl Clone for __numeric_traits_integer_509 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_509 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_510:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____digits_511: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____max_512: std::cell::LazyCell<
    u32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_513 {}
impl Clone for __numeric_traits_integer_513 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_513 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_514:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_515: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_516: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_517 {}
impl Clone for __numeric_traits_integer_517 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_517 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_518:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_519:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_520:
    std::cell::LazyCell<u64> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_521 {}
impl Clone for __numeric_traits_integer_521 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_521 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_char____is_signed_522: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_char____digits_523: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_char____max_524: std::cell::LazyCell<
    libc::c_char,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_525 {}
impl Clone for __numeric_traits_integer_525 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_525 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_short____is_signed_526: std::cell::LazyCell<
    bool,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____digits_527: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____max_528: std::cell::LazyCell<i16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____min_529: std::cell::LazyCell<i16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_530 {}
impl Clone for __numeric_traits_floating_530 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_530 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_float____max_exponent10_531:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_532 {}
impl Clone for __numeric_traits_floating_532 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_532 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_double____max_exponent10_533:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_534 {}
impl Clone for __numeric_traits_floating_534 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_534 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_535:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_536 {}
impl Clone for __numeric_traits_536 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_536 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_element_544 {}
impl Clone for tuple_element_544 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_544 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct _Nth_type_546 {}
impl Clone for _Nth_type_546 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_546 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut piecewise_construct_548: std::cell::LazyCell<std_piecewise_construct_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_549 {}
impl Clone for tuple_549 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_549 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_549 {}
impl tuple_549 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl tuple_550 {
    pub unsafe fn copy_from(_a0: *const tuple_550) -> Self {
        unimplemented!()
    }
}
impl tuple_550 {
    pub unsafe fn move_from(_a0: *mut tuple_550) -> Self {
        unimplemented!()
    }
}
impl tuple_550 {
    pub unsafe fn copy_assign(&mut self, __in: *const tuple_550) -> *mut tuple_550 {
        unimplemented!()
    }
}
impl tuple_550 {
    pub unsafe fn move_assign(&mut self, __in: *mut tuple_550) -> *mut tuple_550 {
        unimplemented!()
    }
}
impl tuple_550 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
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
pub struct initializer_list_551 {}
impl Clone for initializer_list_551 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_551 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_551 {}
impl initializer_list_551 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct __is_allocator_556 {}
impl Clone for __is_allocator_556 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_556 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_560 {}
impl Clone for __alloc_traits_560 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_560 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_561 {}
impl Clone for rebind_561 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_561 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_562 {}
impl Clone for __alloc_traits_562 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_562 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_563 {}
impl Clone for rebind_563 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_563 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_564 {}
impl Clone for __alloc_traits_564 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_564 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_565 {}
impl Clone for rebind_565 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_565 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_566 {}
impl Clone for __alloc_traits_566 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_566 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_567 {}
impl Clone for rebind_567 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_567 {
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_595 {}
impl Clone for hash_595 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_595 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn hash_600(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_601(__val: *const f64) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_602(__val: *const i32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn __hash_combine_603(
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
pub struct basic_string_view_604 {}
impl Clone for basic_string_view_604 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_604 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_604 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_604) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_604 {
    pub unsafe fn new_1(mut __str: *const i32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_604 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_604,
    ) -> *mut basic_string_view_604 {
        unimplemented!()
    }
}
impl basic_string_view_604 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_604 {
    pub unsafe fn data(&self) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(_a0: *const basic_string_view_605) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_605 {
    pub unsafe fn new_1(mut __str: *const u16, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_605 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_605,
    ) -> *mut basic_string_view_605 {
        unimplemented!()
    }
}
impl basic_string_view_605 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_605 {
    pub unsafe fn data(&self) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(_a0: *const basic_string_view_606) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_606 {
    pub unsafe fn new_1(mut __str: *const u32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_606 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_606,
    ) -> *mut basic_string_view_606 {
        unimplemented!()
    }
}
impl basic_string_view_606 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_606 {
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
pub static mut __integer_to_chars_is_unsigned_607: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_608: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_609: std::cell::LazyCell<bool> =
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
pub static mut allocator_arg_610: std::cell::LazyCell<std_allocator_arg_t> =
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
pub struct __is_empty_non_tuple_611 {}
impl Clone for __is_empty_non_tuple_611 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_611 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg)]
pub struct _Head_base_613 {
    pub _M_head_impl: default_delete_614,
}
impl Clone for _Head_base_613 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_613 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_613 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_613 {
    pub unsafe fn copy_from(_a0: *const _Head_base_613) -> Self {
        unimplemented!()
    }
}
impl _Head_base_613 {
    pub unsafe fn move_from(_a0: *mut _Head_base_613) -> Self {
        unimplemented!()
    }
}
impl _Head_base_613 {
    pub unsafe fn _M_head_12(__b: *mut _Head_base_613) -> *mut default_delete_614 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Head_base_615 {
    pub _M_head_impl: *mut i32,
}
impl Clone for _Head_base_615 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_615 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_615 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_615 {
    pub unsafe fn copy_from(_a0: *const _Head_base_615) -> Self {
        unimplemented!()
    }
}
impl _Head_base_615 {
    pub unsafe fn move_from(_a0: *mut _Head_base_615) -> Self {
        unimplemented!()
    }
}
impl _Head_base_615 {
    pub unsafe fn _M_head_12(__b: *mut _Head_base_615) -> *mut *mut i32 {
        unimplemented!()
    }
}
impl _Head_base_615 {
    pub unsafe fn _M_head_13(__b: *const _Head_base_615) -> *const *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Tuple_impl_616 {}
impl Clone for _Tuple_impl_616 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_616 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_616 {
    pub unsafe fn _M_head_1(__t: *mut _Tuple_impl_616) -> *mut *mut i32 {
        unimplemented!()
    }
}
impl _Tuple_impl_616 {
    pub unsafe fn _M_head_2(__t: *const _Tuple_impl_616) -> *const *mut i32 {
        unimplemented!()
    }
}
impl _Tuple_impl_616 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_616 {
    pub unsafe fn copy_from(_a0: *const _Tuple_impl_616) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_616 {
    pub unsafe fn move_from(_a0: *mut _Tuple_impl_616) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn _M_head_1(__t: *mut _Tuple_impl_617) -> *mut default_delete_614 {
        unimplemented!()
    }
}
impl _Tuple_impl_617 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_617 {
    pub unsafe fn copy_from(_a0: *const _Tuple_impl_617) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_617 {
    pub unsafe fn move_from(__in: *mut _Tuple_impl_617) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _TupleConstraints_618 {}
impl Clone for _TupleConstraints_618 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _TupleConstraints_618 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _TupleConstraints_618 {
    pub unsafe fn __is_implicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
impl _TupleConstraints_618 {
    pub unsafe fn __is_explicitly_default_constructible() -> bool {
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
pub static mut std_locale_ctype_619: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_numeric_620: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_collate_621: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_time_622: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_monetary_623: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_messages_624: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type anon_625 = u32;
pub const anon_625__S_categories_size: anon_625 = 12;
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
    pub anon_626: anon_626,
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
pub struct anon_626 {
    pub _M_p: *const libc::c_char,
    pub _M_bytes: [libc::c_char; 8],
}
impl Clone for anon_626 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_626 {
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
pub struct is_error_code_enum_627 {}
impl Clone for is_error_code_enum_627 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_627 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_condition_enum_634 {}
impl Clone for is_error_condition_enum_634 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_634 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_ios_base_boolalpha_637: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_dec_638: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_fixed_639: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_hex_640: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_internal_641: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_left_642: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_oct_643: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_right_644: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_scientific_645: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showbase_646: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showpoint_647: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showpos_648: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_skipws_649: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_unitbuf_650: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_uppercase_651: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_adjustfield_652: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_basefield_653: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_floatfield_654: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_badbit_655: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_eofbit_656: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_failbit_657: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_goodbit_658: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_in_659: std::cell::LazyCell<std__Ios_Openmode> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_out_660: std::cell::LazyCell<std__Ios_Openmode> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_cur_661: std::cell::LazyCell<std__Ios_Seekdir> =
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
pub type anon_662 = u32;
pub const anon_662__S_local_word_size: anon_662 = 8;
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
pub type anon_663 = i32;
pub const anon_663___ISwupper: anon_663 = 0;
pub const anon_663___ISwlower: anon_663 = 1;
pub const anon_663___ISwalpha: anon_663 = 2;
pub const anon_663___ISwdigit: anon_663 = 3;
pub const anon_663___ISwxdigit: anon_663 = 4;
pub const anon_663___ISwspace: anon_663 = 5;
pub const anon_663___ISwprint: anon_663 = 6;
pub const anon_663___ISwgraph: anon_663 = 7;
pub const anon_663___ISwblank: anon_663 = 8;
pub const anon_663___ISwcntrl: anon_663 = 9;
pub const anon_663___ISwpunct: anon_663 = 10;
pub const anon_663___ISwalnum: anon_663 = 11;
pub const anon_663__ISwupper: anon_663 = 16777216;
pub const anon_663__ISwlower: anon_663 = 33554432;
pub const anon_663__ISwalpha: anon_663 = 67108864;
pub const anon_663__ISwdigit: anon_663 = 134217728;
pub const anon_663__ISwxdigit: anon_663 = 268435456;
pub const anon_663__ISwspace: anon_663 = 536870912;
pub const anon_663__ISwprint: anon_663 = 1073741824;
pub const anon_663__ISwgraph: anon_663 = -2147483648;
pub const anon_663__ISwblank: anon_663 = 65536;
pub const anon_663__ISwcntrl: anon_663 = 131072;
pub const anon_663__ISwpunct: anon_663 = 262144;
pub const anon_663__ISwalnum: anon_663 = 524288;
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
pub static mut std_ctype_base_upper_664: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_lower_665: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_alpha_666: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_digit_667: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_xdigit_668: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_space_669: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_print_670: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_graph_671: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_cntrl_672: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_punct_673: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_alnum_674: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_blank_675: std::cell::LazyCell<u16> =
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
pub type anon_676 = u32;
pub const anon_676__S_ominus: anon_676 = 0;
pub const anon_676__S_oplus: anon_676 = 1;
pub const anon_676__S_ox: anon_676 = 2;
pub const anon_676__S_oX: anon_676 = 3;
pub const anon_676__S_odigits: anon_676 = 4;
pub const anon_676__S_odigits_end: anon_676 = 20;
pub const anon_676__S_oudigits: anon_676 = 20;
pub const anon_676__S_oudigits_end: anon_676 = 36;
pub const anon_676__S_oe: anon_676 = 18;
pub const anon_676__S_oE: anon_676 = 34;
pub const anon_676__S_oend: anon_676 = 36;
pub static mut std___num_base__S_atoms_out_677: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std___num_base__S_atoms_in_678: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type anon_679 = u32;
pub const anon_679__S_iminus: anon_679 = 0;
pub const anon_679__S_iplus: anon_679 = 1;
pub const anon_679__S_ix: anon_679 = 2;
pub const anon_679__S_iX: anon_679 = 3;
pub const anon_679__S_izero: anon_679 = 4;
pub const anon_679__S_ie: anon_679 = 18;
pub const anon_679__S_iE: anon_679 = 24;
pub const anon_679__S_iend: anon_679 = 26;
pub static mut cout_680: std::cell::LazyCell<basic_ostream_45> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct default_delete_614 {}
impl Clone for default_delete_614 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for default_delete_614 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl default_delete_614 {
    pub unsafe fn operator_call(&self, mut __ptr: *mut i32) {
        unimplemented!()
    }
}
impl default_delete_614 {}
impl default_delete_614 {}
impl default_delete_614 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const default_delete_614,
    ) -> *mut default_delete_614 {
        unimplemented!()
    }
}
impl default_delete_614 {
    pub unsafe fn move_assign(&mut self, _a0: *mut default_delete_614) -> *mut default_delete_614 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __uniq_ptr_impl_681 {}
impl Clone for __uniq_ptr_impl_681 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_impl_681 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Ptr_682 {}
impl Clone for _Ptr_682 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_682 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_681 {
    pub unsafe fn new_1(mut __p: *mut i32) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_681 {
    pub unsafe fn move_from(__u: *mut __uniq_ptr_impl_681) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_681 {
    pub unsafe fn move_assign(
        &mut self,
        __u: *mut __uniq_ptr_impl_681,
    ) -> *mut __uniq_ptr_impl_681 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_681 {
    pub unsafe fn _M_ptr_6(&mut self) -> *mut *mut i32 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_681 {
    pub unsafe fn _M_ptr_7(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_681 {
    pub unsafe fn _M_deleter_8(&mut self) -> *mut default_delete_614 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __uniq_ptr_data_683 {}
impl Clone for __uniq_ptr_data_683 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_data_683 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_683 {
    pub unsafe fn move_from(_a0: *mut __uniq_ptr_data_683) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_683 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __uniq_ptr_data_683,
    ) -> *mut __uniq_ptr_data_683 {
        unimplemented!()
    }
}
impl __uniq_ptr_data_683 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct unique_ptr_684 {}
impl Clone for unique_ptr_684 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unique_ptr_684 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unique_ptr_684 {
    pub unsafe fn move_from(_a0: *mut unique_ptr_684) -> Self {
        unimplemented!()
    }
}
impl unique_ptr_684 {
    pub unsafe fn move_assign(&mut self, _a0: *mut unique_ptr_684) -> *mut unique_ptr_684 {
        unimplemented!()
    }
}
impl unique_ptr_684 {
    pub unsafe fn get(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl unique_ptr_684 {
    pub unsafe fn get_deleter_17(&mut self) -> *mut default_delete_614 {
        unimplemented!()
    }
}
impl unique_ptr_684 {
    pub unsafe fn new(mut __p: *mut i32) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _MakeUniq_685 {}
impl Clone for _MakeUniq_685 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _MakeUniq_685 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type __gnu_cxx__Lock_policy = u32;
pub const __gnu_cxx__Lock_policy__S_single: __gnu_cxx__Lock_policy = 0;
pub const __gnu_cxx__Lock_policy__S_mutex: __gnu_cxx__Lock_policy = 1;
pub const __gnu_cxx__Lock_policy__S_atomic: __gnu_cxx__Lock_policy = 2;
pub static mut __default_lock_policy_686: std::cell::LazyCell<__gnu_cxx__Lock_policy> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {}
impl __gnu_cxx___concurrence_lock_error {}
impl __gnu_cxx___concurrence_lock_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_lock_error,
    ) -> *mut __gnu_cxx___concurrence_lock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_lock_error,
    ) -> *mut __gnu_cxx___concurrence_lock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {}
#[derive(VaArg, FnPtrArg, Copy)]
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
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {}
impl __gnu_cxx___concurrence_unlock_error {}
impl __gnu_cxx___concurrence_unlock_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_unlock_error,
    ) -> *mut __gnu_cxx___concurrence_unlock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_unlock_error,
    ) -> *mut __gnu_cxx___concurrence_unlock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {}
#[derive(VaArg, FnPtrArg, Copy)]
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
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {}
impl __gnu_cxx___concurrence_broadcast_error {}
impl __gnu_cxx___concurrence_broadcast_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_broadcast_error,
    ) -> *mut __gnu_cxx___concurrence_broadcast_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_broadcast_error,
    ) -> *mut __gnu_cxx___concurrence_broadcast_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {}
#[derive(VaArg, FnPtrArg, Copy)]
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
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {}
impl __gnu_cxx___concurrence_wait_error {}
impl __gnu_cxx___concurrence_wait_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_wait_error,
    ) -> *mut __gnu_cxx___concurrence_wait_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_wait_error,
    ) -> *mut __gnu_cxx___concurrence_wait_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl __gnu_cxx___mutex {
    pub unsafe fn lock(&mut self) {
        unimplemented!()
    }
}
impl __gnu_cxx___mutex {
    pub unsafe fn unlock(&mut self) {
        unimplemented!()
    }
}
impl __gnu_cxx___mutex {
    pub unsafe fn gthread_mutex(&mut self) -> *mut pthread_mutex_t {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl __gnu_cxx___recursive_mutex {
    pub unsafe fn gthread_recursive_mutex(&mut self) -> *mut pthread_mutex_t {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new(__name: *mut __gnu_cxx___mutex) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_assign(&mut self, _a0: *const std_bad_weak_ptr) -> *mut std_bad_weak_ptr {
        unimplemented!()
    }
}
impl std_bad_weak_ptr {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Mutex_base_687 {}
impl Clone for _Mutex_base_687 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_687 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_688 = u32;
pub const anon_688__S_need_barriers: anon_688 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Mutex_base_689 {}
impl Clone for _Mutex_base_689 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_689 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Mutex_base_689 {}
impl _Mutex_base_689 {}
impl _Mutex_base_689 {
    pub unsafe fn copy_assign(&mut self, _a0: *const _Mutex_base_689) -> *mut _Mutex_base_689 {
        unimplemented!()
    }
}
impl _Mutex_base_689 {
    pub unsafe fn move_assign(&mut self, _a0: *mut _Mutex_base_689) -> *mut _Mutex_base_689 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub type anon_691 = u32;
pub const anon_691__S_need_barriers: anon_691 = 0;
impl _Mutex_base_690 {}
impl _Mutex_base_690 {}
impl _Mutex_base_690 {
    pub unsafe fn copy_assign(&mut self, _a0: *const _Mutex_base_690) -> *mut _Mutex_base_690 {
        unimplemented!()
    }
}
impl _Mutex_base_690 {
    pub unsafe fn move_assign(&mut self, _a0: *mut _Mutex_base_690) -> *mut _Mutex_base_690 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Sp_counted_ptr_692 {}
impl Clone for _Sp_counted_ptr_692 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_692 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg)]
pub struct _Sp_alloc_shared_tag_695 {
    pub _M_a: *const allocator_0,
}
impl Clone for _Sp_alloc_shared_tag_695 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_alloc_shared_tag_695 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Sp_owner_less_696 {}
impl Clone for _Sp_owner_less_696 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_owner_less_696 {
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
#[derive(VaArg, FnPtrArg)]
pub struct std___atomic_flag_base {
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
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std___atomic_flag_base,
    ) -> *mut std___atomic_flag_base {
        unimplemented!()
    }
}
impl std___atomic_flag_base {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std___atomic_flag_base,
    ) -> *mut std___atomic_flag_base {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub unsafe fn identity_697(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn swap_by_ptr_698(mut a: *mut i32, mut b: *mut i32) {
    let mut tmp: i32 = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub unsafe fn swap_by_ref_699(a: *mut i32, b: *mut i32) {
    let mut tmp: i32 = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 0;
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            basic_ostream_45::operator_shl_11(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_680)),
                x,
            )
        }));
        operator_shl_700(___out, ('\n' as libc::c_char))
    });
    let mut a: i32 = 1;
    let mut b: i32 = (unsafe { identity_697(a) });
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            basic_ostream_45::operator_shl_11(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_680)),
                b,
            )
        }));
        operator_shl_700(___out, ('\n' as libc::c_char))
    });
    let mut c: i32 = 2;
    let mut p: *mut i32 = (&mut c as *mut i32);
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            let ___n: i32 = (*p);
            basic_ostream_45::operator_shl_11(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_680)),
                ___n,
            )
        }));
        operator_shl_700(___out, ('\n' as libc::c_char))
    });
    let mut d: i32 = 3;
    let mut e: i32 = 4;
    (unsafe { swap_by_ptr_698((&mut d as *mut i32), (&mut e as *mut i32)) });
    let mut f: i32 = 4;
    let mut g: i32 = 5;
    (unsafe { swap_by_ref_699(&mut f, &mut g) });
    let mut h: *mut i32 = (Box::leak(Box::new(6)) as *mut i32);
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            let ___n: i32 = (*h);
            basic_ostream_45::operator_shl_11(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_680)),
                ___n,
            )
        }));
        operator_shl_700(___out, ('\n' as libc::c_char))
    });
    {
        let __p = h;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut i: *mut i32 = Box::leak(Box::new([7, 8, 0_i32])).as_mut_ptr();
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            let ___n: i32 = (*i.offset((1) as isize));
            basic_ostream_45::operator_shl_11(
                &mut (*(unsafe {
                    let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
                        let ___n: i32 = (*i.offset((0) as isize));
                        basic_ostream_45::operator_shl_11(
                            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_680)),
                            ___n,
                        )
                    }));
                    operator_shl_700(___out, (' ' as libc::c_char))
                })),
                ___n,
            )
        }));
        operator_shl_700(___out, ('\n' as libc::c_char))
    });
    {
        let __p = i;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
    (unsafe {
        swap_by_ptr_698(
            (Box::leak(Box::new(7)) as *mut i32),
            (Box::leak(Box::new(8)) as *mut i32),
        )
    });
    (unsafe {
        swap_by_ptr_698(
            (Box::leak(Box::new(7)) as *mut i32).offset((0) as isize),
            (Box::leak(Box::new(8)) as *mut i32).offset((0) as isize),
        )
    });
    (unsafe {
        swap_by_ref_699(
            &mut (*(Box::leak(Box::new(9)) as *mut i32)),
            &mut (*(Box::leak(Box::new(10)) as *mut i32)),
        )
    });
    (unsafe {
        swap_by_ref_699(
            &mut (*(Box::leak(Box::new(9)) as *mut i32).offset((0) as isize)),
            &mut (*(Box::leak(Box::new(10)) as *mut i32).offset((0) as isize)),
        )
    });
    let mut j: unique_ptr_684 = unique_ptr_684::new({ (Box::leak(Box::new(11)) as *mut i32) });
    let mut k: *mut i32 = (unsafe { unique_ptr_684::get(&j) });
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            let ___n: i32 = (*k);
            basic_ostream_45::operator_shl_11(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_680)),
                ___n,
            )
        }));
        operator_shl_700(___out, ('\n' as libc::c_char))
    });
    let mut l: unique_ptr_684 = (unsafe {
        let mut ___args: i32 = 11;
        make_unique_701(&mut ___args)
    });
    let mut m: *mut i32 = (unsafe { unique_ptr_684::get(&l) });
    (unsafe {
        let ___out: *mut basic_ostream_45 = &mut (*(unsafe {
            let ___n: i32 = (*m);
            basic_ostream_45::operator_shl_11(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_680)),
                ___n,
            )
        }));
        operator_shl_700(___out, ('\n' as libc::c_char))
    });
    (if ((c) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_702(
                c"c == 2".as_ptr(),
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
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std_error_category;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std___uses_alloc0__Sink;
pub unsafe fn operator_shl_700(
    __out: *mut basic_ostream_45,
    mut __c: libc::c_char,
) -> *mut basic_ostream_45 {
    unimplemented!()
}
pub unsafe fn make_unique_701(__args: *mut i32) -> unique_ptr_684 {
    unimplemented!()
}
pub unsafe fn __assert_fail_702(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
