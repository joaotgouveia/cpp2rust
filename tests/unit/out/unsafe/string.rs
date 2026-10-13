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
impl allocator_traits_5 {
    pub unsafe fn select_on_container_copy_construction(__rhs: *const allocator_1) -> allocator_1 {
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
    pub unsafe fn find(
        mut __s: *const libc::c_char,
        mut __n: usize,
        __a: *const libc::c_char,
    ) -> *const libc::c_char {
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
    pub unsafe fn assign_9(
        mut __s: *mut libc::c_char,
        mut __n: usize,
        mut __a: libc::c_char,
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
pub static mut std_basic_string_char__std_char_traits_char___std_allocator_char___npos_15:
    std::cell::LazyCell<u64> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
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
pub type anon_16 = u32;
pub const anon_16__S_local_capacity: anon_16 = 15;
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
    pub unsafe fn new_2(__a: *const allocator_1) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn copy_from(__str: *const basic_string_13) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn new_3(__str: *const basic_string_13, mut __pos: u64, mut __n: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn new_4(
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
    pub unsafe fn to_basic_string_view_17(&self) -> basic_string_view_17 {
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
    pub unsafe fn resize_74(&mut self, mut __n: u64, mut __c: libc::c_char) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn resize_75(&mut self, mut __n: u64) {
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
    pub unsafe fn operator_index_84(&mut self, mut __pos: u64) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn append_98(
        &mut self,
        mut __s: *const libc::c_char,
        mut __n: u64,
    ) -> *mut basic_string_13 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn append_100(
        &mut self,
        mut __n: u64,
        mut __c: libc::c_char,
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
    pub unsafe fn data_159(&mut self) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn get_allocator(&self) -> allocator_1 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn find_last_of_178(
        &self,
        mut __s: *const libc::c_char,
        mut __pos: u64,
        mut __n: u64,
    ) -> u64 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn find_last_of_179(
        &self,
        mut __s: *const libc::c_char,
        mut __pos: Option<u64>,
    ) -> u64 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn substr(&self, mut __pos: u64, mut __n: u64) -> basic_string_13 {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn new_5(mut __s: *const libc::c_char, __a: Option<*const allocator_1>) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn new_6(
        mut __n: u64,
        mut __c: libc::c_char,
        __a: Option<*const allocator_1>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn new_7(
        mut __beg: *mut libc::c_char,
        mut __end: *mut libc::c_char,
        __a: Option<*const allocator_1>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_18(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_20(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_22(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_24(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_26(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_13 {
    pub unsafe fn __resize_and_overwrite_28(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub type anon_32 = u32;
pub const anon_32__S_local_capacity: anon_32 = 3;
#[derive(VaArg, FnPtrArg)]
pub struct anon_31 {
    pub _M_local_buf: [i32; 4],
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
    pub unsafe fn copy_from(__str: *const basic_string_30) -> Self {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub unsafe fn new_1(
        mut __s: *const i32,
        mut __n: u64,
        __a: Option<*const allocator_2>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub unsafe fn move_from(__str: *mut basic_string_30) -> Self {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_30) -> *mut basic_string_30 {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_30) -> *mut basic_string_30 {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub unsafe fn c_str(&self) -> *const i32 {
        unimplemented!()
    }
}
impl basic_string_30 {
    pub unsafe fn new_2(
        mut __beg: *const libc::c_char,
        mut __end: *const libc::c_char,
        __a: Option<*const allocator_2>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub type anon_35 = u32;
pub const anon_35__S_local_capacity: anon_35 = 7;
#[derive(VaArg, FnPtrArg)]
pub struct anon_34 {
    pub _M_local_buf: [u16; 8],
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
    pub unsafe fn copy_from(__str: *const basic_string_33) -> Self {
        unimplemented!()
    }
}
impl basic_string_33 {
    pub unsafe fn new_1(
        mut __s: *const u16,
        mut __n: u64,
        __a: Option<*const allocator_3>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_33 {
    pub unsafe fn move_from(__str: *mut basic_string_33) -> Self {
        unimplemented!()
    }
}
impl basic_string_33 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_33) -> *mut basic_string_33 {
        unimplemented!()
    }
}
impl basic_string_33 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_33) -> *mut basic_string_33 {
        unimplemented!()
    }
}
impl basic_string_33 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub type anon_38 = u32;
pub const anon_38__S_local_capacity: anon_38 = 3;
#[derive(VaArg, FnPtrArg)]
pub struct anon_37 {
    pub _M_local_buf: [u32; 4],
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
    pub unsafe fn copy_from(__str: *const basic_string_36) -> Self {
        unimplemented!()
    }
}
impl basic_string_36 {
    pub unsafe fn new_1(
        mut __s: *const u32,
        mut __n: u64,
        __a: Option<*const allocator_4>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_36 {
    pub unsafe fn move_from(__str: *mut basic_string_36) -> Self {
        unimplemented!()
    }
}
impl basic_string_36 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_36) -> *mut basic_string_36 {
        unimplemented!()
    }
}
impl basic_string_36 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_36) -> *mut basic_string_36 {
        unimplemented!()
    }
}
impl basic_string_36 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __mbstate_t {
    pub __count: i32,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_39 {
    pub __wch: u32,
    pub __wchb: [libc::c_char; 4],
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
pub struct integral_constant_40 {}
impl Clone for integral_constant_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_integral_constant_bool__true__value_41: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_40 {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_40 {}
impl integral_constant_40 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_42 {}
impl Clone for integral_constant_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_integral_constant_bool__false__value_43: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_42 {}
impl integral_constant_42 {}
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct __and__73 {}
impl Clone for __and__73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__79 {}
impl Clone for __not__79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_85 {}
impl Clone for is_reference_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_92 {}
impl Clone for is_function_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_115 {}
impl Clone for remove_cv_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_120 {}
impl Clone for is_const_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_125 {}
impl Clone for __is_array_unknown_bounds_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_132 {}
impl Clone for __is_integral_helper_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_137 {}
impl Clone for is_integral_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_161 {}
impl Clone for is_volatile_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct is_nothrow_default_constructible_166 {}
impl Clone for is_nothrow_default_constructible_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_168 {}
impl Clone for is_nothrow_copy_constructible_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct is_convertible_173 {}
impl Clone for is_convertible_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_186 {}
impl Clone for remove_reference_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_193 {}
impl Clone for __cv_selector_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_197 {}
impl Clone for __match_cv_qualifiers_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_202 {}
impl Clone for __make_unsigned_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_205 {}
impl Clone for __make_unsigned_selector_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct _List_214 {}
impl Clone for _List_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_215 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_216 {}
impl Clone for _List_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_217 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_218 {}
impl Clone for _List_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_219 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_220 {}
impl Clone for _List_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_223 {}
impl Clone for __select_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_228 {}
impl Clone for __make_signed_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_230 {}
impl Clone for __make_signed_selector_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct common_type_235 {}
impl Clone for common_type_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_235 {
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
pub static mut is_array_v_236: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_237: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_238: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_239: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_240: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_241: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_242: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_243: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_244: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_245: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_246: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_247: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_248: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_249: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_250: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_251: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_252: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_253: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_254: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_255: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Char_types_256 {}
impl Clone for _Char_types_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_258 {}
impl Clone for char_traits_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_258 {
    pub unsafe fn eq(__c1: *const libc::c_char, __c2: *const libc::c_char) -> bool {
        unimplemented!()
    }
}
impl char_traits_258 {
    pub unsafe fn length(mut __p: *const libc::c_char) -> usize {
        unimplemented!()
    }
}
impl char_traits_258 {
    pub unsafe fn find(
        mut __s: *const libc::c_char,
        mut __n: usize,
        __a: *const libc::c_char,
    ) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn eq(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_259 {
    pub unsafe fn lt(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_259 {
    pub unsafe fn compare(mut __s1: *const i32, mut __s2: *const i32, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_259 {
    pub unsafe fn length(mut __p: *const i32) -> usize {
        unimplemented!()
    }
}
impl char_traits_259 {
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
pub struct __new_allocator_260 {}
impl Clone for __new_allocator_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_260 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_260 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_260) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_260 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_260,
    ) -> *mut __new_allocator_260 {
        unimplemented!()
    }
}
impl __new_allocator_260 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl __new_allocator_260 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut libc::c_char, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_260 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_261 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_261) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_261 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_261,
    ) -> *mut __new_allocator_261 {
        unimplemented!()
    }
}
impl __new_allocator_261 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        unimplemented!()
    }
}
impl __new_allocator_261 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_261 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_262 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_262) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_262 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_262,
    ) -> *mut __new_allocator_262 {
        unimplemented!()
    }
}
impl __new_allocator_262 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u16 {
        unimplemented!()
    }
}
impl __new_allocator_262 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u16, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_262 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_263 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_263) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_263 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_263,
    ) -> *mut __new_allocator_263 {
        unimplemented!()
    }
}
impl __new_allocator_263 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u32 {
        unimplemented!()
    }
}
impl __new_allocator_263 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_263 {
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
pub struct __are_same_264 {}
impl Clone for __are_same_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_265 = u32;
pub const anon_265___value: anon_265 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_266 {}
impl Clone for __are_same_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_267 = u32;
pub const anon_267___value: anon_267 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_268 {}
impl Clone for __are_same_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_269 = u32;
pub const anon_269___value: anon_269 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_270 {}
impl Clone for __are_same_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_271 = u32;
pub const anon_271___value: anon_271 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_272 {}
impl Clone for __are_same_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_273 = u32;
pub const anon_273___value: anon_273 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_274 {}
impl Clone for __are_same_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_275 = u32;
pub const anon_275___value: anon_275 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_276 {}
impl Clone for __is_integer_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_277 = u32;
pub const anon_277___value: anon_277 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_278 {}
impl Clone for __is_integer_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_279 = u32;
pub const anon_279___value: anon_279 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_280 {}
impl Clone for __is_integer_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_281 = u32;
pub const anon_281___value: anon_281 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_282 {}
impl Clone for __is_integer_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_283 = u32;
pub const anon_283___value: anon_283 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_284 {}
impl Clone for __is_integer_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_285 = u32;
pub const anon_285___value: anon_285 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_286 {}
impl Clone for __is_integer_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_287 = u32;
pub const anon_287___value: anon_287 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_288 {}
impl Clone for __is_integer_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_289 = u32;
pub const anon_289___value: anon_289 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_290 {}
impl Clone for __is_integer_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_291 = u32;
pub const anon_291___value: anon_291 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_292 {}
impl Clone for __memcpyable_integer_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_292 {
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
pub struct iterator_293 {}
impl Clone for iterator_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_294 {}
impl Clone for iterator_traits_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_299 {}
impl Clone for __ptr_traits_ptr_to_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_299 {
    pub unsafe fn pointer_to(__r: *mut libc::c_char) -> *mut libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *const libc::c_char) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *mut i32) -> *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *const i32) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *mut u16) -> *mut u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *const u16) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *mut u32) -> *mut u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn pointer_to(__r: *const u32) -> *const u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_307 {}
impl Clone for pointer_traits_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_315 {}
impl Clone for binary_function_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_316 {}
impl Clone for greater_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_317 {}
impl Clone for less_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_318 {}
impl Clone for greater_equal_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_319 {}
impl Clone for less_equal_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_320 {}
impl Clone for __is_integer_nonstrict_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_321 = u32;
pub const anon_321___width: anon_321 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_322 {}
impl Clone for __is_integer_nonstrict_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_323 = u32;
pub const anon_323___width: anon_323 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_324 {}
impl Clone for __is_integer_nonstrict_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_325 = u32;
pub const anon_325___width: anon_325 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_326 {}
impl Clone for __is_integer_nonstrict_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_327 = u32;
pub const anon_327___width: anon_327 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_328 {}
impl Clone for __is_integer_nonstrict_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_329 = u32;
pub const anon_329___width: anon_329 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_330 {}
impl Clone for __numeric_traits_integer_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_int____is_signed_331: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____digits_332: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____max_333: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____min_334: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_335 {}
impl Clone for __numeric_traits_integer_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_336: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_337: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_338: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_339 {}
impl Clone for __numeric_traits_integer_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_340:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____digits_341: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____max_342: std::cell::LazyCell<
    u32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_343 {}
impl Clone for __numeric_traits_integer_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_344:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_345: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_346: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_347 {}
impl Clone for __numeric_traits_integer_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_348:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_349:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_350:
    std::cell::LazyCell<u64> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_351 {}
impl Clone for __numeric_traits_floating_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_float____max_exponent10_352:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_353 {}
impl Clone for __numeric_traits_floating_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_double____max_exponent10_354:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_355 {}
impl Clone for __numeric_traits_floating_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_356:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_357 {}
impl Clone for __numeric_traits_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut piecewise_construct_362: std::cell::LazyCell<std_piecewise_construct_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_363 {}
impl Clone for tuple_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_363 {}
impl tuple_363 {}
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
pub struct initializer_list_364 {}
impl Clone for initializer_list_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_364 {}
impl initializer_list_364 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct __is_allocator_369 {}
impl Clone for __is_allocator_369 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_369 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_373 {}
impl Clone for __alloc_traits_373 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_373 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_374 {}
impl Clone for rebind_374 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_374 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_373 {
    pub unsafe fn _S_select_on_copy(__a: *const allocator_1) -> allocator_1 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_375 {}
impl Clone for __alloc_traits_375 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_375 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_376 {}
impl Clone for rebind_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_377 {}
impl Clone for __alloc_traits_377 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_377 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_378 {}
impl Clone for rebind_378 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_378 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_379 {}
impl Clone for __alloc_traits_379 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_379 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_380 {}
impl Clone for rebind_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_380 {
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_406 {}
impl Clone for hash_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn hash_411(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_412(__val: *const f64) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_17 {}
impl Clone for basic_string_view_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_17 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_17) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_17 {
    pub unsafe fn new_1(mut __str: *const libc::c_char, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_17 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_17,
    ) -> *mut basic_string_view_17 {
        unimplemented!()
    }
}
impl basic_string_view_17 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_17 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_17 {
    pub unsafe fn data(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_413 {}
impl Clone for basic_string_view_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_413 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_413) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_413 {
    pub unsafe fn new_1(mut __str: *const i32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_413 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_413,
    ) -> *mut basic_string_view_413 {
        unimplemented!()
    }
}
impl basic_string_view_413 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_413 {
    pub unsafe fn data(&self) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(_a0: *const basic_string_view_414) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_414 {
    pub unsafe fn new_1(mut __str: *const u16, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_414 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_414,
    ) -> *mut basic_string_view_414 {
        unimplemented!()
    }
}
impl basic_string_view_414 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_414 {
    pub unsafe fn data(&self) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(_a0: *const basic_string_view_415) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_415 {
    pub unsafe fn new_1(mut __str: *const u32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_415 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_415,
    ) -> *mut basic_string_view_415 {
        unimplemented!()
    }
}
impl basic_string_view_415 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_415 {
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
    pub __value32: anon_416,
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
pub struct anon_416 {
    pub __low: u32,
    pub __high: u32,
}
impl Clone for anon_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_416 {
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
pub static mut __integer_to_chars_is_unsigned_417: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_418: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_419: std::cell::LazyCell<bool> =
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
pub static mut allocator_arg_420: std::cell::LazyCell<std_allocator_arg_t> =
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
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut empty: basic_string_13 = basic_string_13::new_1();
    (if ((unsafe { basic_string_13::length(&empty) }) == (0_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"empty.length() == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut s1: basic_string_13 = basic_string_13::new_5({ c"hello".as_ptr() }, None);
    (if ((unsafe { basic_string_13::length(&s1) }) == (5_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s1.length() == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { basic_string_13::size(&s1) }) == (unsafe { basic_string_13::length(&s1) })) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s1.size() == s1.length()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut s1, 0_u64) })) as i32)
        == (('h' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s1[0] == \'h\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut s1, 1_u64) })) as i32)
        == (('e' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s1[1] == \'e\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut s1, 2_u64) })) as i32)
        == (('l' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s1[2] == \'l\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut s1, 3_u64) })) as i32)
        == (('l' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s1[3] == \'l\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut s1, 4_u64) })) as i32)
        == (('o' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s1[4] == \'o\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let ___lhs: *const basic_string_13 = &s1;
        operator_eq_422(___lhs, c"hello".as_ptr())
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s1 == \"hello\"".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p1: *const libc::c_char = (unsafe { basic_string_13::data_159(&mut s1) }).cast_const();
    (if (((*p1.offset((0) as isize)) as i32) == (('h' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"p1[0] == \'h\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*p1.offset((1) as isize)) as i32) == (('e' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"p1[1] == \'e\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*p1.offset((2) as isize)) as i32) == (('l' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"p1[2] == \'l\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*p1.offset((3) as isize)) as i32) == (('l' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"p1[3] == \'l\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*p1.offset((4) as isize)) as i32) == (('o' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"p1[4] == \'o\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut s2: basic_string_13 =
        basic_string_13::new_6({ 10_u64 }, { ('a' as libc::c_char) }, None);
    let mut p2: *const libc::c_char = (unsafe { basic_string_13::data_159(&mut s2) }).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (unsafe { basic_string_13::size(&s2) })) {
        (if (((*p2.offset((i) as isize)) as i32) == (('a' as libc::c_char) as i32))
            && (((*(unsafe { basic_string_13::operator_index_84(&mut s2, (i as u64)) })) as i32)
                == (('a' as libc::c_char) as i32))
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_421(
                    c"p2[i] == \'a\' && s2[i] == \'a\'".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    (if ((unsafe { basic_string_13::length(&s2) }) == (10_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s2.length() == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { basic_string_13::size(&s2) }) == (unsafe { basic_string_13::length(&s2) })) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s2.size() == s2.length()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(unsafe { basic_string_13::operator_index_84(&mut s2, 0_u64) })) = ('b' as libc::c_char);
    (*(unsafe { basic_string_13::operator_index_84(&mut s2, 1_u64) })) = ('c' as libc::c_char);
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut s2, 0_u64) })) as i32)
        == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s2[0] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut s2, 1_u64) })) as i32)
        == (('c' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s2[1] == \'c\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut i: u32 = 2_u32;
    'loop_: while ((i as u64) < (unsafe { basic_string_13::size(&s2) })) {
        (if (((*p2.offset((i) as isize)) as i32) == (('a' as libc::c_char) as i32))
            && (((*(unsafe { basic_string_13::operator_index_84(&mut s2, (i as u64)) })) as i32)
                == (('a' as libc::c_char) as i32))
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_421(
                    c"p2[i] == \'a\' && s2[i] == \'a\'".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    let mut s3: basic_string_13 = (unsafe { basic_string_13::substr(&s2, 2_u64, 5_u64) });
    (if ((unsafe { basic_string_13::length(&s3) }) == (5_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s3.length() == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { basic_string_13::size(&s3) }) == (unsafe { basic_string_13::length(&s3) })) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s3.size() == s3.length()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p3: *const libc::c_char = (unsafe { basic_string_13::data_159(&mut s3) }).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (unsafe { basic_string_13::size(&s3) })) {
        (if (((*p3.offset((i) as isize)) as i32)
            == ((*(unsafe { basic_string_13::operator_index_84(&mut s3, (i as u64)) })) as i32))
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_421(
                    c"p3[i] == s3[i]".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    let mut s4: basic_string_13 = (unsafe {
        let ___n: u64 = (unsafe { basic_string_13::find_last_of_179(&s1, c"l".as_ptr(), None) });
        basic_string_13::substr(&s1, 1_u64, ___n)
    });
    (if ((unsafe { basic_string_13::length(&s4) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s4.length() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { basic_string_13::size(&s4) }) == (unsafe { basic_string_13::length(&s4) })) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s4.size() == s4.length()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p4: *const libc::c_char = (unsafe { basic_string_13::data_159(&mut s4) }).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (unsafe { basic_string_13::size(&s4) })) {
        (if (((*p4.offset((i) as isize)) as i32)
            == ((*(unsafe { basic_string_13::operator_index_84(&mut s4, (i as u64)) })) as i32))
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_421(
                    c"p4[i] == s4[i]".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    let mut s5: basic_string_13 = (unsafe {
        let ___lhs: *const basic_string_13 = &s1;
        operator_add_423(___lhs, c", world".as_ptr())
    });
    (if ((unsafe { basic_string_13::length(&s5) }) == (12_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s5.length() == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { basic_string_13::size(&s5) }) == (unsafe { basic_string_13::length(&s5) })) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"s5.size() == s5.length()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p5: *const libc::c_char = (unsafe { basic_string_13::data_159(&mut s5) }).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (unsafe { basic_string_13::size(&s5) })) {
        (if (((*p5.offset((i) as isize)) as i32)
            == ((*(unsafe { basic_string_13::operator_index_84(&mut s5, (i as u64)) })) as i32))
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_421(
                    c"p5[i] == s5[i]".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    let mut arr: [libc::c_char; 7] = [
        ('b' as libc::c_char),
        ('a' as libc::c_char),
        ('r' as libc::c_char),
        (' ' as libc::c_char),
        ('f' as libc::c_char),
        ('o' as libc::c_char),
        ('o' as libc::c_char),
    ];
    let mut string: basic_string_13 =
        basic_string_13::new_4({ (arr.as_mut_ptr()).cast_const() }, { 3_u64 }, None);
    (if ((unsafe { basic_string_13::size(&string) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 0_u64) })) as i32)
        == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[0] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 1_u64) })) as i32)
        == (('a' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[1] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 2_u64) })) as i32)
        == (('r' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[2] == \'r\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let ___lhs: *const basic_string_13 = &string;
        operator_eq_422(___lhs, c"bar".as_ptr())
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string == \"bar\"".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { basic_string_13::resize_75(&mut string, 3_u64) });
    (if ((unsafe { basic_string_13::size(&string) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 0_u64) })) as i32)
        == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[0] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 1_u64) })) as i32)
        == (('a' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[1] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 2_u64) })) as i32)
        == (('r' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[2] == \'r\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let ___lhs: *const basic_string_13 = &string;
        operator_eq_422(___lhs, c"bar".as_ptr())
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string == \"bar\"".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { basic_string_13::resize_75(&mut string, 5_u64) });
    (if ((unsafe { basic_string_13::size(&string) }) == (5_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string.size() == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 0_u64) })) as i32)
        == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[0] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 1_u64) })) as i32)
        == (('a' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[1] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 2_u64) })) as i32)
        == (('r' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[2] == \'r\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 3_u64) })) as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[3] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 4_u64) })) as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[4] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(unsafe { basic_string_13::operator_index_84(&mut string, 3_u64) })) = ('a' as libc::c_char);
    (*(unsafe { basic_string_13::operator_index_84(&mut string, 4_u64) })) = ('b' as libc::c_char);
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 3_u64) })) as i32)
        == (('a' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[3] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 4_u64) })) as i32)
        == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[4] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(unsafe { basic_string_13::operator_index_84(&mut string, 3_u64) })) = (0 as libc::c_char);
    (*(unsafe { basic_string_13::operator_index_84(&mut string, 4_u64) })) = (0 as libc::c_char);
    (unsafe { basic_string_13::resize_75(&mut string, 4_u64) });
    (if ((unsafe { basic_string_13::size(&string) }) == (4_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string.size() == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 0_u64) })) as i32)
        == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[0] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 1_u64) })) as i32)
        == (('a' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[1] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 2_u64) })) as i32)
        == (('r' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[2] == \'r\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut string, 3_u64) })) as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"string[3] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut result: basic_string_13 = (unsafe {
        let ___lhs: *const basic_string_13 = &string;
        operator_add_423(___lhs, c" foo".as_ptr())
    });
    (if ((unsafe { basic_string_13::size(&result) }) == (8_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"result.size() == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut result, 0_u64) })) as i32)
        == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"result[0] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut result, 1_u64) })) as i32)
        == (('a' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"result[1] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut result, 2_u64) })) as i32)
        == (('r' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"result[2] == \'r\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut result, 3_u64) })) as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"result[3] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut result, 4_u64) })) as i32)
        == ((' ' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"result[4] == \' \'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut result, 5_u64) })) as i32)
        == (('f' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"result[5] == \'f\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut result, 6_u64) })) as i32)
        == (('o' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"result[6] == \'o\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut result, 7_u64) })) as i32)
        == (('o' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"result[7] == \'o\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut substr_0: basic_string_13 = (unsafe { basic_string_13::substr(&result, 5_u64, 3_u64) });
    (if ((unsafe { basic_string_13::size(&substr_0) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_0.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_0, 0_u64) })) as i32)
        == (('f' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_0[0] == \'f\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_0, 1_u64) })) as i32)
        == (('o' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_0[1] == \'o\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_0, 2_u64) })) as i32)
        == (('o' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_0[2] == \'o\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut substr_1: basic_string_13 = (unsafe { basic_string_13::substr(&result, 0_u64, 5_u64) });
    (if ((unsafe { basic_string_13::size(&substr_1) }) == (5_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_1.size() == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_1, 0_u64) })) as i32)
        == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_1[0] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_1, 1_u64) })) as i32)
        == (('a' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_1[1] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_1, 2_u64) })) as i32)
        == (('r' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_1[2] == \'r\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_1, 3_u64) })) as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_1[3] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_1, 4_u64) })) as i32)
        == ((' ' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_1[4] == \' \'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut substr_2: basic_string_13 =
        (unsafe { basic_string_13::substr(&result, 0_u64, 15_u64) });
    (if ((unsafe { basic_string_13::size(&substr_2) }) == (8_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_2.size() == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_2, 0_u64) })) as i32)
        == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_2[0] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_2, 1_u64) })) as i32)
        == (('a' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_2[1] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_2, 2_u64) })) as i32)
        == (('r' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_2[2] == \'r\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_2, 3_u64) })) as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_2[3] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_2, 4_u64) })) as i32)
        == ((' ' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_2[4] == \' \'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_2, 5_u64) })) as i32)
        == (('f' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_2[5] == \'f\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_2, 6_u64) })) as i32)
        == (('o' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_2[6] == \'o\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { basic_string_13::operator_index_84(&mut substr_2, 7_u64) })) as i32)
        == (('o' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"substr_2[7] == \'o\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut pos: u64 = (unsafe { basic_string_13::find_last_of_179(&result, c"b".as_ptr(), None) });
    (if ((pos) == (0_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"pos == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    pos = (unsafe { basic_string_13::find_last_of_179(&result, c"f".as_ptr(), None) });
    (if ((pos) == (5_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"pos == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    pos = (unsafe { basic_string_13::find_last_of_179(&result, c"o".as_ptr(), None) });
    (if ((pos) == (7_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"pos == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    pos = (unsafe { basic_string_13::find_last_of_179(&result, c"x".as_ptr(), None) });
    (if ((pos) == (-1_i64 as u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"pos == -1ul".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut string_to_cast: basic_string_13 = basic_string_13::new_5({ c"cast".as_ptr() }, None);
    let mut output_data: *mut u8 =
        ((&mut (*(unsafe { basic_string_13::operator_index_84(&mut string_to_cast, 0_u64) }))
            as *mut libc::c_char) as *mut libc::c_char as *mut u8);
    (if (((*output_data) as i32) == (('c' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"*output_data == \'c\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*output_data.offset((1) as isize)) as i32) == (('a' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"output_data[1] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*output_data.offset((2) as isize)) as i32) == (('s' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"output_data[2] == \'s\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*output_data.offset((3) as isize)) as i32) == (('t' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"output_data[3] == \'t\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut t0: u64 = (unsafe { basic_string_13::length(&s1) });
    let mut t1: u64 = (t0).wrapping_add(((*p1) as u64));
    let mut t2: u64 = (t1).wrapping_add((unsafe { basic_string_13::length(&s2) }));
    let mut t3: u64 = (t2).wrapping_add((unsafe { basic_string_13::size(&s3) }));
    let mut t4: u64 = (t3).wrapping_add((unsafe { basic_string_13::size(&s4) }));
    (if (((t4).wrapping_add((unsafe { basic_string_13::size(&s5) }))) == (139_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_421(
                c"t4 + s5.size() == 139".as_ptr(),
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
pub struct std___uses_alloc0__Sink;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn __assert_fail_421(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn operator_eq_422(
    __lhs: *const basic_string_13,
    mut __rhs: *const libc::c_char,
) -> bool {
    unimplemented!()
}
pub unsafe fn operator_add_423(
    __lhs: *const basic_string_13,
    mut __rhs: *const libc::c_char,
) -> basic_string_13 {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
