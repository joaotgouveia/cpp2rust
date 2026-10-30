// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1001() {
    std::process::abort();
}

unsafe fn f1002(a0: *mut ::libc::c_void) {
    libcc2rs::free_unsafe(a0)
}

unsafe fn f1003(a0: usize) -> *mut ::libc::c_void {
    libcc2rs::malloc_unsafe(a0)
}

unsafe fn f1004(a0: *mut ::libc::c_void, a1: usize) -> *mut ::libc::c_void {
    libcc2rs::realloc_unsafe(a0, a1)
}

unsafe fn f1005(a0: usize, a1: usize) -> *mut ::libc::c_void {
    libcc2rs::calloc_unsafe(a0, a1)
}

unsafe fn f1006(a0: *const libc::c_char) -> *mut libc::c_char {
    libc::getenv(a0)
}

unsafe fn f1007(a0: *const libc::c_char, a1: *const libc::c_char, a2: i32) -> i32 {
    libc::setenv(a0, a1, a2)
}

unsafe fn f1008(
    a0: *const ::libc::c_void,
    a1: *const ::libc::c_void,
    a2: usize,
    a3: usize,
    a4: unsafe fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
) -> *mut ::libc::c_void {
    libc::bsearch(
        a0,
        a1,
        a2,
        a3,
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(a4 as *const ())),
    )
}

unsafe fn f1009(
    a0: *mut ::libc::c_void,
    a1: usize,
    a2: usize,
    a3: unsafe fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
) {
    libc::qsort(
        a0,
        a1,
        a2,
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(a3 as *const ())),
    )
}

unsafe fn f1010(a0: *const libc::c_char, a1: *mut libc::c_char) -> *mut libc::c_char {
    libc::realpath(a0, a1)
}

unsafe fn f1016(a0: i32) -> i32 {
    a0.abs()
}
