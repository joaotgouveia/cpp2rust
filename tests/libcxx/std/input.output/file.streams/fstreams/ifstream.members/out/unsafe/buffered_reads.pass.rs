extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(usize, new, value)]
pub struct UserManagedBuffer {
    size_: usize,
    buffers_: Vec<*mut libc::c_char>,
    wbuffers_: Vec<*mut i32>,
}
impl UserManagedBuffer {
    pub unsafe fn new(mut size: usize) -> Self {
        let mut this = Self {
            size_: size,
            buffers_: Vec::new(),
            wbuffers_: Vec::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut UserManagedBuffer) -> Self {
        let mut this = Self {
            size_: (*_a0).size_,
            buffers_: std::mem::take(&mut (*_a0).buffers_),
            wbuffers_: std::mem::take(&mut (*_a0).wbuffers_),
        };
        this
    }
    pub unsafe fn operator_call_4(&mut self, stream: *mut ()) {
        {
            let __init = Box::leak(
                (0..self.size_)
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            );
            self.buffers_.push(__init)
        };
        unimplemented!();
    }
    pub unsafe fn operator_call_5(&mut self, stream: *mut ()) {
        {
            let __init = Box::leak(
                (0..self.size_)
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            );
            self.buffers_.push(__init)
        };
        unimplemented!();
    }
    pub unsafe fn operator_call_6(&mut self, stream: *mut ()) {
        {
            let __init = Box::leak((0..self.size_).map(|_| 0_i32).collect::<Box<[i32]>>());
            self.wbuffers_.push(__init)
        };
        unimplemented!();
    }
    pub unsafe fn operator_call_7(&mut self, stream: *mut ()) {
        {
            let __init = Box::leak((0..self.size_).map(|_| 0_i32).collect::<Box<[i32]>>());
            self.wbuffers_.push(__init)
        };
        unimplemented!();
    }
    pub unsafe fn destructor(&mut self) {
        'loop_: for p in IterRange::new(self.buffers_.begin(), self.buffers_.end()) {
            let mut p: *mut libc::c_char = (*p.iter_deref());
            {
                let __p = p;
                if !__p.is_null() {
                    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                        __p,
                        libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                            / ::std::mem::size_of::<libc::c_char>(),
                    )))
                }
            };
        }
        'loop_: for p in IterRange::new(self.wbuffers_.begin(), self.wbuffers_.end()) {
            let mut p: *mut i32 = (*p.iter_deref());
            {
                let __p = p;
                if !__p.is_null() {
                    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                        __p,
                        libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                            / ::std::mem::size_of::<i32>(),
                    )))
                }
            };
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(usize, new, value)]
pub struct LibraryManagedBuffer {
    size_: usize,
}
impl LibraryManagedBuffer {
    pub unsafe fn new(mut size: usize) -> Self {
        let mut this = Self { size_: size };
        this
    }
    pub unsafe fn operator_call_3_char(&self, stream: *mut ()) {
        unimplemented!();
    }
    pub unsafe fn operator_call_3_wchar_t(&self, stream: *mut ()) {
        unimplemented!();
    }
}
impl Callable1<*mut (), ()> for LibraryManagedBuffer {
    fn call(&self, a1: *mut ()) -> () {
        unsafe { LibraryManagedBuffer::operator_call_3_char(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct LibraryDefaultBuffer {}
impl LibraryDefaultBuffer {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn operator_call_3_char(&self, _a0: *mut ()) {}
    pub unsafe fn operator_call_3_wchar_t(&self, _a0: *mut ()) {}
}
impl Callable1<*mut (), ()> for LibraryDefaultBuffer {
    fn call(&self, a1: *mut ()) -> () {
        unsafe { LibraryDefaultBuffer::operator_call_3_char(self, a1) }
    }
}
impl Default for LibraryDefaultBuffer {
    fn default() -> Self {
        unsafe { LibraryDefaultBuffer::new() }
    }
}
pub unsafe fn test_eprintf_0(
    mut fmt: *const libc::c_char,
    args_1: *const *const libc::c_char,
    args_2: *const *const libc::c_char,
    args_3: *const *const libc::c_char,
    args_4: *const i32,
    args_5: *const *const libc::c_char,
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
            (*args_4),
            (*args_5),
        )
    });
}
pub unsafe fn test_eprintf_1(
    mut fmt: *const libc::c_char,
    args_1: *const *const libc::c_char,
    args_2: *const *const libc::c_char,
    args_3: *const i32,
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
        )
    });
}
pub unsafe fn test_eprintf_2(
    mut fmt: *const libc::c_char,
    args_1: *const i64,
    args_2: *const libc::c_char,
    args_3: *const libc::c_char,
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            ((*args_2) as i32),
            ((*args_3) as i32),
        )
    });
}
pub unsafe fn test_eprintf_3(mut fmt: *const libc::c_char, args_1: *const i64, args_2: *const i64) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
        )
    });
}
pub unsafe fn test_eprintf_4(
    mut fmt: *const libc::c_char,
    args_1: *const u64,
    args_2: *const i64,
    args_3: *const i64,
    args_4: *const libc::c_char,
    args_5: *const libc::c_char,
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
            ((*args_4) as i32),
            ((*args_5) as i32),
        )
    });
}
pub unsafe fn test_eprintf_5(
    mut fmt: *const libc::c_char,
    args_1: *const i64,
    args_2: *const i32,
    args_3: *const i32,
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
        )
    });
}
pub unsafe fn test_log_6(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    mut message: *const libc::c_char,
) {
    let mut msg: *const libc::c_char = if !(condition).is_null() {
        c"Assertion failure: ".as_ptr()
    } else {
        c"Unconditional failure:".as_ptr()
    };
    (unsafe {
        test_eprintf_0(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_log_7(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_8(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_9(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_10(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_11(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_12(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_13(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_14(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_15(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_16(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_17(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_18(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_19(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_20(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_21(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_require_22(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_7(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_23(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_8(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_24(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_9(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_25(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_10(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_26(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_11(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_27(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_12(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_28(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_13(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_29(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_14(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_30(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_15(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_31(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_16(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_32(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_17(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_33(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_18(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_34(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_19(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_35(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_20(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_36(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_21(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn get_temp_file_name_37() -> Vec<libc::c_char> {
    let mut Name: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"libcxx.XXXXXX".as_ptr(),
        libcc2rs::c_str_len(c"libcxx.XXXXXX".as_ptr()) + 1,
    )
    .to_vec();
    let mut FD: i32 = unimplemented!();
    if ((FD) == (-1_i32)) {
        unimplemented!();
        std::process::abort();
    }
    libc::close(FD);
    return std::mem::take(&mut Name);
}
pub unsafe fn glibc_version_less_than_38(mut version: *const libc::c_char) -> bool {
    let mut test_version: Vec<libc::c_char> = {
        let mut __tmp2 = std::slice::from_raw_parts(
            c"glibc ".as_ptr(),
            libcc2rs::c_str_len(c"glibc ".as_ptr()) + 1,
        )
        .to_vec()
        .clone();
        __tmp2.pop();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            version,
            libcc2rs::c_str_len(version),
        ));
        __tmp2.push((0 as libc::c_char));
        __tmp2
    };
    let mut n: usize = unimplemented!();
    let mut current_version: *mut libc::c_char = Box::leak(
        (0..n)
            .map(|_| (0 as libc::c_char))
            .collect::<Box<[libc::c_char]>>(),
    )
    .as_mut_ptr();
    unimplemented!();
    let mut result: bool = ((unimplemented!()) < (0));
    {
        let __p = current_version;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<libc::c_char>(),
            )))
        }
    };
    return result;
}
pub unsafe fn test_read_39(mut policy: LibraryDefaultBuffer, payload_sizes: *const Vec<i64>) {
    let mut total_size: () = (unimplemented!() as ());
    let mut data: Vec<libc::c_char> = (0..(total_size as usize) as usize)
        .map(|_| (0 as libc::c_char))
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (data.len())) {
        (*(&mut data).as_mut_ptr().add(i as usize)) = (((i).wrapping_rem(
            (((1) << ((8_usize).wrapping_mul((::std::mem::size_of::<libc::c_char>() as usize))))
                as usize),
        )) as libc::c_char);
        i.prefix_inc();
    }
    let mut p: Vec<libc::c_char> = (unsafe { get_temp_file_name_37() });
    {
        let mut ofs: ::std::fs::File = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        assert!(!(unimplemented!()));
        let mut j: () = 0_();
        'loop_: while ((j) < (total_size)) {
            let mut exp: libc::c_char = (((j)
                % (((1)
                    << ((8_usize).wrapping_mul((::std::mem::size_of::<libc::c_char>() as usize))))
                    as ())) as libc::c_char);
            (unsafe {
                let _condition: bool = (((*(&mut data).as_mut_ptr().add((j as usize) as usize))
                    as i32)
                    == (exp as i32));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let j: *mut () = &mut j;
                        let data: *mut Vec<libc::c_char> = &mut data;
                        let exp: *mut libc::c_char = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const i64 = (j as *const ()).cast::<i64>();
                            let _args_2: *const libc::c_char =
                                &(*(&mut (*data)).as_mut_ptr().add(((*j) as usize) as usize));
                            test_eprintf_2(
                                c"failed after write() at offset %zu: got=%x, expected=%x\n"
                                    .as_ptr(),
                                _args_1,
                                _args_2,
                                exp,
                            )
                        });
                    }
                );
                test_require_22(
                    _condition,
                    c"data[j] == exp".as_ptr(),
                    c"buffered_reads.pass.cpp".as_ptr(),
                    67,
                    &mut _arg,
                )
            });
            j.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut ifs: ::std::fs::File = unimplemented!();
        (unsafe {
            LibraryDefaultBuffer::operator_call_3_char(&policy, &mut *(&raw mut ifs).cast::<()>())
        });
        unimplemented!();
        assert!(unimplemented!());
        let in_sz: () = unimplemented!();
        (unsafe {
            let _condition: bool = ((in_sz) == (total_size));
            let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                {
                    let total_size: *mut () = &mut total_size;
                    let in_sz: *const () = &in_sz;
                },
                || {
                    (unsafe {
                        test_eprintf_3(
                            c"out_sz = %zu, in_sz = %ld\n".as_ptr(),
                            (total_size as *const ()).cast::<i64>(),
                            (in_sz as *const ()).cast::<i64>(),
                        )
                    });
                }
            );
            test_require_23(
                _condition,
                c"in_sz == total_size".as_ptr(),
                c"buffered_reads.pass.cpp".as_ptr(),
                77,
                &mut _arg,
            )
        });
        unimplemented!();
        let mut previously_read: usize = 0_usize;
        let mut in_buffer: Vec<libc::c_char> = (0..(total_size as usize) as usize)
            .map(|_| (0 as libc::c_char))
            .collect::<Vec<_>>();
        'loop_: for payload_sz in IterRange::new((*payload_sizes).begin(), (*payload_sizes).end()) {
            let payload_sz: *const i64 = &(*payload_sz.iter_deref());
            unimplemented!();
            assert!(unimplemented!());
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: libc::c_char = ((((previously_read).wrapping_add((j as usize)))
                    .wrapping_rem(
                        (((1)
                            << ((8_usize)
                                .wrapping_mul((::std::mem::size_of::<libc::c_char>() as usize))))
                            as usize),
                    )) as libc::c_char);
                (unsafe {
                    let _condition: bool = (((*(&mut in_buffer)
                        .as_mut_ptr()
                        .add((previously_read).wrapping_add((j as usize)) as usize))
                        as i32)
                        == (exp as i32));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let previously_read: *mut usize = &mut previously_read;
                            let j: *mut () = &mut j;
                            let payload_sz: *const i64 = payload_sz;
                            let in_buffer: *mut Vec<libc::c_char> = &mut in_buffer;
                            let exp: *mut libc::c_char = &mut exp;
                        },
                        || {
                            (unsafe {
                                let mut _args_1: u64 =
                                    ((*previously_read).wrapping_add(((*j) as usize)) as u64);
                                let _args_2: *const i64 = (j as *const ()).cast::<i64>();
                                let _args_3: *const i64 = payload_sz;
                                let _args_4: *const libc::c_char =
                                    &(*(&mut (*in_buffer))
                                        .as_mut_ptr()
                                        .add((*previously_read).wrapping_add(((*j) as usize))
                                            as usize));
                                let _args_5: *const libc::c_char = exp;
                                test_eprintf_4 ( c"failed after read() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" .as_ptr() , & mut _args_1 , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    );
                    test_require_24(
                        _condition,
                        c"in_buffer[previously_read + j] == exp".as_ptr(),
                        c"buffered_reads.pass.cpp".as_ptr(),
                        94,
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_read = (previously_read).wrapping_add(((*payload_sz) as usize));
        }
    }
    unimplemented!();
}
pub unsafe fn test_read_40(mut policy: LibraryManagedBuffer, payload_sizes: *const Vec<i64>) {
    let mut total_size: () = (unimplemented!() as ());
    let mut data: Vec<libc::c_char> = (0..(total_size as usize) as usize)
        .map(|_| (0 as libc::c_char))
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (data.len())) {
        (*(&mut data).as_mut_ptr().add(i as usize)) = (((i).wrapping_rem(
            (((1) << ((8_usize).wrapping_mul((::std::mem::size_of::<libc::c_char>() as usize))))
                as usize),
        )) as libc::c_char);
        i.prefix_inc();
    }
    let mut p: Vec<libc::c_char> = (unsafe { get_temp_file_name_37() });
    {
        let mut ofs: ::std::fs::File = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        assert!(!(unimplemented!()));
        let mut j: () = 0_();
        'loop_: while ((j) < (total_size)) {
            let mut exp: libc::c_char = (((j)
                % (((1)
                    << ((8_usize).wrapping_mul((::std::mem::size_of::<libc::c_char>() as usize))))
                    as ())) as libc::c_char);
            (unsafe {
                let _condition: bool = (((*(&mut data).as_mut_ptr().add((j as usize) as usize))
                    as i32)
                    == (exp as i32));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let j: *mut () = &mut j;
                        let data: *mut Vec<libc::c_char> = &mut data;
                        let exp: *mut libc::c_char = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const i64 = (j as *const ()).cast::<i64>();
                            let _args_2: *const libc::c_char =
                                &(*(&mut (*data)).as_mut_ptr().add(((*j) as usize) as usize));
                            test_eprintf_2(
                                c"failed after write() at offset %zu: got=%x, expected=%x\n"
                                    .as_ptr(),
                                _args_1,
                                _args_2,
                                exp,
                            )
                        });
                    }
                );
                test_require_27(
                    _condition,
                    c"data[j] == exp".as_ptr(),
                    c"buffered_reads.pass.cpp".as_ptr(),
                    67,
                    &mut _arg,
                )
            });
            j.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut ifs: ::std::fs::File = unimplemented!();
        (unsafe {
            LibraryManagedBuffer::operator_call_3_char(&policy, &mut *(&raw mut ifs).cast::<()>())
        });
        unimplemented!();
        assert!(unimplemented!());
        let in_sz: () = unimplemented!();
        (unsafe {
            let _condition: bool = ((in_sz) == (total_size));
            let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                {
                    let total_size: *mut () = &mut total_size;
                    let in_sz: *const () = &in_sz;
                },
                || {
                    (unsafe {
                        test_eprintf_3(
                            c"out_sz = %zu, in_sz = %ld\n".as_ptr(),
                            (total_size as *const ()).cast::<i64>(),
                            (in_sz as *const ()).cast::<i64>(),
                        )
                    });
                }
            );
            test_require_28(
                _condition,
                c"in_sz == total_size".as_ptr(),
                c"buffered_reads.pass.cpp".as_ptr(),
                77,
                &mut _arg,
            )
        });
        unimplemented!();
        let mut previously_read: usize = 0_usize;
        let mut in_buffer: Vec<libc::c_char> = (0..(total_size as usize) as usize)
            .map(|_| (0 as libc::c_char))
            .collect::<Vec<_>>();
        'loop_: for payload_sz in IterRange::new((*payload_sizes).begin(), (*payload_sizes).end()) {
            let payload_sz: *const i64 = &(*payload_sz.iter_deref());
            unimplemented!();
            assert!(unimplemented!());
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: libc::c_char = ((((previously_read).wrapping_add((j as usize)))
                    .wrapping_rem(
                        (((1)
                            << ((8_usize)
                                .wrapping_mul((::std::mem::size_of::<libc::c_char>() as usize))))
                            as usize),
                    )) as libc::c_char);
                (unsafe {
                    let _condition: bool = (((*(&mut in_buffer)
                        .as_mut_ptr()
                        .add((previously_read).wrapping_add((j as usize)) as usize))
                        as i32)
                        == (exp as i32));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let previously_read: *mut usize = &mut previously_read;
                            let j: *mut () = &mut j;
                            let payload_sz: *const i64 = payload_sz;
                            let in_buffer: *mut Vec<libc::c_char> = &mut in_buffer;
                            let exp: *mut libc::c_char = &mut exp;
                        },
                        || {
                            (unsafe {
                                let mut _args_1: u64 =
                                    ((*previously_read).wrapping_add(((*j) as usize)) as u64);
                                let _args_2: *const i64 = (j as *const ()).cast::<i64>();
                                let _args_3: *const i64 = payload_sz;
                                let _args_4: *const libc::c_char =
                                    &(*(&mut (*in_buffer))
                                        .as_mut_ptr()
                                        .add((*previously_read).wrapping_add(((*j) as usize))
                                            as usize));
                                let _args_5: *const libc::c_char = exp;
                                test_eprintf_4 ( c"failed after read() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" .as_ptr() , & mut _args_1 , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    );
                    test_require_29(
                        _condition,
                        c"in_buffer[previously_read + j] == exp".as_ptr(),
                        c"buffered_reads.pass.cpp".as_ptr(),
                        94,
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_read = (previously_read).wrapping_add(((*payload_sz) as usize));
        }
    }
    unimplemented!();
}
pub unsafe fn test_read_41(mut policy: UserManagedBuffer, payload_sizes: *const Vec<i64>) {
    let mut total_size: () = (unimplemented!() as ());
    let mut data: Vec<libc::c_char> = (0..(total_size as usize) as usize)
        .map(|_| (0 as libc::c_char))
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (data.len())) {
        (*(&mut data).as_mut_ptr().add(i as usize)) = (((i).wrapping_rem(
            (((1) << ((8_usize).wrapping_mul((::std::mem::size_of::<libc::c_char>() as usize))))
                as usize),
        )) as libc::c_char);
        i.prefix_inc();
    }
    let mut p: Vec<libc::c_char> = (unsafe { get_temp_file_name_37() });
    {
        let mut ofs: ::std::fs::File = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        assert!(!(unimplemented!()));
        let mut j: () = 0_();
        'loop_: while ((j) < (total_size)) {
            let mut exp: libc::c_char = (((j)
                % (((1)
                    << ((8_usize).wrapping_mul((::std::mem::size_of::<libc::c_char>() as usize))))
                    as ())) as libc::c_char);
            (unsafe {
                let _condition: bool = (((*(&mut data).as_mut_ptr().add((j as usize) as usize))
                    as i32)
                    == (exp as i32));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let j: *mut () = &mut j;
                        let data: *mut Vec<libc::c_char> = &mut data;
                        let exp: *mut libc::c_char = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const i64 = (j as *const ()).cast::<i64>();
                            let _args_2: *const libc::c_char =
                                &(*(&mut (*data)).as_mut_ptr().add(((*j) as usize) as usize));
                            test_eprintf_2(
                                c"failed after write() at offset %zu: got=%x, expected=%x\n"
                                    .as_ptr(),
                                _args_1,
                                _args_2,
                                exp,
                            )
                        });
                    }
                );
                test_require_32(
                    _condition,
                    c"data[j] == exp".as_ptr(),
                    c"buffered_reads.pass.cpp".as_ptr(),
                    67,
                    &mut _arg,
                )
            });
            j.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut ifs: ::std::fs::File = unimplemented!();
        (unsafe {
            UserManagedBuffer::operator_call_5(&mut policy, &mut *(&raw mut ifs).cast::<()>())
        });
        unimplemented!();
        assert!(unimplemented!());
        let in_sz: () = unimplemented!();
        (unsafe {
            let _condition: bool = ((in_sz) == (total_size));
            let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                {
                    let total_size: *mut () = &mut total_size;
                    let in_sz: *const () = &in_sz;
                },
                || {
                    (unsafe {
                        test_eprintf_3(
                            c"out_sz = %zu, in_sz = %ld\n".as_ptr(),
                            (total_size as *const ()).cast::<i64>(),
                            (in_sz as *const ()).cast::<i64>(),
                        )
                    });
                }
            );
            test_require_33(
                _condition,
                c"in_sz == total_size".as_ptr(),
                c"buffered_reads.pass.cpp".as_ptr(),
                77,
                &mut _arg,
            )
        });
        unimplemented!();
        let mut previously_read: usize = 0_usize;
        let mut in_buffer: Vec<libc::c_char> = (0..(total_size as usize) as usize)
            .map(|_| (0 as libc::c_char))
            .collect::<Vec<_>>();
        'loop_: for payload_sz in IterRange::new((*payload_sizes).begin(), (*payload_sizes).end()) {
            let payload_sz: *const i64 = &(*payload_sz.iter_deref());
            unimplemented!();
            assert!(unimplemented!());
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: libc::c_char = ((((previously_read).wrapping_add((j as usize)))
                    .wrapping_rem(
                        (((1)
                            << ((8_usize)
                                .wrapping_mul((::std::mem::size_of::<libc::c_char>() as usize))))
                            as usize),
                    )) as libc::c_char);
                (unsafe {
                    let _condition: bool = (((*(&mut in_buffer)
                        .as_mut_ptr()
                        .add((previously_read).wrapping_add((j as usize)) as usize))
                        as i32)
                        == (exp as i32));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let previously_read: *mut usize = &mut previously_read;
                            let j: *mut () = &mut j;
                            let payload_sz: *const i64 = payload_sz;
                            let in_buffer: *mut Vec<libc::c_char> = &mut in_buffer;
                            let exp: *mut libc::c_char = &mut exp;
                        },
                        || {
                            (unsafe {
                                let mut _args_1: u64 =
                                    ((*previously_read).wrapping_add(((*j) as usize)) as u64);
                                let _args_2: *const i64 = (j as *const ()).cast::<i64>();
                                let _args_3: *const i64 = payload_sz;
                                let _args_4: *const libc::c_char =
                                    &(*(&mut (*in_buffer))
                                        .as_mut_ptr()
                                        .add((*previously_read).wrapping_add(((*j) as usize))
                                            as usize));
                                let _args_5: *const libc::c_char = exp;
                                test_eprintf_4 ( c"failed after read() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" .as_ptr() , & mut _args_1 , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    );
                    test_require_34(
                        _condition,
                        c"in_buffer[previously_read + j] == exp".as_ptr(),
                        c"buffered_reads.pass.cpp".as_ptr(),
                        94,
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_read = (previously_read).wrapping_add(((*payload_sz) as usize));
        }
    }
    unimplemented!();
}
pub unsafe fn test_read_codecvt_42(
    mut policy: LibraryDefaultBuffer,
    payload_sizes: *const Vec<i64>,
) {
    let mut total_size: () = (unimplemented!() as ());
    let mut data: Vec<i32> = (0..(total_size as usize) as usize)
        .map(|_| 0_i32)
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (data.len())) {
        (*(&mut data).as_mut_ptr().add(i as usize)) = (i as i32);
        i.prefix_inc();
    }
    let mut p: Vec<libc::c_char> = (unsafe { get_temp_file_name_37() });
    {
        let mut ofs: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        assert!(!(unimplemented!()));
        let mut j: () = 0_();
        'loop_: while ((j) < (total_size)) {
            let mut exp: i32 = (j as i32);
            (unsafe {
                let _condition: bool =
                    (((*(&mut data).as_mut_ptr().add((j as usize) as usize)) as i32) == (exp));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let j: *mut () = &mut j;
                        let data: *mut Vec<i32> = &mut data;
                        let exp: *mut i32 = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const i64 = (j as *const ()).cast::<i64>();
                            let _args_2: *const i32 =
                                &(*(&mut (*data)).as_mut_ptr().add(((*j) as usize) as usize));
                            test_eprintf_5(
                                c"failed after write() at offset %zu: got=%x, expected=%x\n"
                                    .as_ptr(),
                                _args_1,
                                _args_2,
                                exp,
                            )
                        });
                    }
                );
                test_require_25(
                    _condition,
                    c"data[j] == exp".as_ptr(),
                    c"buffered_reads.pass.cpp".as_ptr(),
                    123,
                    &mut _arg,
                )
            });
            j.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut ifs: () = unimplemented!();
        unimplemented!();
        (unsafe { LibraryDefaultBuffer::operator_call_3_wchar_t(&policy, &mut ifs) });
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        let mut previously_read: usize = 0_usize;
        let mut in_buffer: Vec<i32> = (0..(total_size as usize) as usize)
            .map(|_| 0_i32)
            .collect::<Vec<_>>();
        'loop_: for payload_sz in IterRange::new((*payload_sizes).begin(), (*payload_sizes).end()) {
            let payload_sz: *const i64 = &(*payload_sz.iter_deref());
            assert!(unimplemented!());
            assert!(unimplemented!());
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: i32 = (((j as usize).wrapping_add(previously_read)) as i32);
                (unsafe {
                    let _condition: bool = (((*(&mut in_buffer)
                        .as_mut_ptr()
                        .add((j as usize).wrapping_add(previously_read) as usize))
                        as i32)
                        == (exp));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let j: *mut () = &mut j;
                            let in_buffer: *mut Vec<i32> = &mut in_buffer;
                            let previously_read: *mut usize = &mut previously_read;
                            let exp: *mut i32 = &mut exp;
                        },
                        || {
                            (unsafe {
                                let _args_1: *const i64 = (j as *const ()).cast::<i64>();
                                let _args_2: *const i32 =
                                    &(*(&mut (*in_buffer))
                                        .as_mut_ptr()
                                        .add(((*j) as usize).wrapping_add((*previously_read))
                                            as usize));
                                test_eprintf_5(
                                    c"failed after read() at offset %zu: got=%x, expected=%x\n"
                                        .as_ptr(),
                                    _args_1,
                                    _args_2,
                                    exp,
                                )
                            });
                        }
                    );
                    test_require_26(
                        _condition,
                        c"in_buffer[j + previously_read] == exp".as_ptr(),
                        c"buffered_reads.pass.cpp".as_ptr(),
                        144,
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_read = (previously_read).wrapping_add(((*payload_sz) as usize));
        }
    }
    unimplemented!();
}
pub unsafe fn test_read_codecvt_43(
    mut policy: LibraryManagedBuffer,
    payload_sizes: *const Vec<i64>,
) {
    let mut total_size: () = (unimplemented!() as ());
    let mut data: Vec<i32> = (0..(total_size as usize) as usize)
        .map(|_| 0_i32)
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (data.len())) {
        (*(&mut data).as_mut_ptr().add(i as usize)) = (i as i32);
        i.prefix_inc();
    }
    let mut p: Vec<libc::c_char> = (unsafe { get_temp_file_name_37() });
    {
        let mut ofs: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        assert!(!(unimplemented!()));
        let mut j: () = 0_();
        'loop_: while ((j) < (total_size)) {
            let mut exp: i32 = (j as i32);
            (unsafe {
                let _condition: bool =
                    (((*(&mut data).as_mut_ptr().add((j as usize) as usize)) as i32) == (exp));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let j: *mut () = &mut j;
                        let data: *mut Vec<i32> = &mut data;
                        let exp: *mut i32 = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const i64 = (j as *const ()).cast::<i64>();
                            let _args_2: *const i32 =
                                &(*(&mut (*data)).as_mut_ptr().add(((*j) as usize) as usize));
                            test_eprintf_5(
                                c"failed after write() at offset %zu: got=%x, expected=%x\n"
                                    .as_ptr(),
                                _args_1,
                                _args_2,
                                exp,
                            )
                        });
                    }
                );
                test_require_30(
                    _condition,
                    c"data[j] == exp".as_ptr(),
                    c"buffered_reads.pass.cpp".as_ptr(),
                    123,
                    &mut _arg,
                )
            });
            j.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut ifs: () = unimplemented!();
        unimplemented!();
        (unsafe { LibraryManagedBuffer::operator_call_3_wchar_t(&policy, &mut ifs) });
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        let mut previously_read: usize = 0_usize;
        let mut in_buffer: Vec<i32> = (0..(total_size as usize) as usize)
            .map(|_| 0_i32)
            .collect::<Vec<_>>();
        'loop_: for payload_sz in IterRange::new((*payload_sizes).begin(), (*payload_sizes).end()) {
            let payload_sz: *const i64 = &(*payload_sz.iter_deref());
            assert!(unimplemented!());
            assert!(unimplemented!());
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: i32 = (((j as usize).wrapping_add(previously_read)) as i32);
                (unsafe {
                    let _condition: bool = (((*(&mut in_buffer)
                        .as_mut_ptr()
                        .add((j as usize).wrapping_add(previously_read) as usize))
                        as i32)
                        == (exp));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let j: *mut () = &mut j;
                            let in_buffer: *mut Vec<i32> = &mut in_buffer;
                            let previously_read: *mut usize = &mut previously_read;
                            let exp: *mut i32 = &mut exp;
                        },
                        || {
                            (unsafe {
                                let _args_1: *const i64 = (j as *const ()).cast::<i64>();
                                let _args_2: *const i32 =
                                    &(*(&mut (*in_buffer))
                                        .as_mut_ptr()
                                        .add(((*j) as usize).wrapping_add((*previously_read))
                                            as usize));
                                test_eprintf_5(
                                    c"failed after read() at offset %zu: got=%x, expected=%x\n"
                                        .as_ptr(),
                                    _args_1,
                                    _args_2,
                                    exp,
                                )
                            });
                        }
                    );
                    test_require_31(
                        _condition,
                        c"in_buffer[j + previously_read] == exp".as_ptr(),
                        c"buffered_reads.pass.cpp".as_ptr(),
                        144,
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_read = (previously_read).wrapping_add(((*payload_sz) as usize));
        }
    }
    unimplemented!();
}
pub unsafe fn test_read_codecvt_44(mut policy: UserManagedBuffer, payload_sizes: *const Vec<i64>) {
    let mut total_size: () = (unimplemented!() as ());
    let mut data: Vec<i32> = (0..(total_size as usize) as usize)
        .map(|_| 0_i32)
        .collect::<Vec<_>>();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (data.len())) {
        (*(&mut data).as_mut_ptr().add(i as usize)) = (i as i32);
        i.prefix_inc();
    }
    let mut p: Vec<libc::c_char> = (unsafe { get_temp_file_name_37() });
    {
        let mut ofs: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        assert!(!(unimplemented!()));
        let mut j: () = 0_();
        'loop_: while ((j) < (total_size)) {
            let mut exp: i32 = (j as i32);
            (unsafe {
                let _condition: bool =
                    (((*(&mut data).as_mut_ptr().add((j as usize) as usize)) as i32) == (exp));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let j: *mut () = &mut j;
                        let data: *mut Vec<i32> = &mut data;
                        let exp: *mut i32 = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const i64 = (j as *const ()).cast::<i64>();
                            let _args_2: *const i32 =
                                &(*(&mut (*data)).as_mut_ptr().add(((*j) as usize) as usize));
                            test_eprintf_5(
                                c"failed after write() at offset %zu: got=%x, expected=%x\n"
                                    .as_ptr(),
                                _args_1,
                                _args_2,
                                exp,
                            )
                        });
                    }
                );
                test_require_35(
                    _condition,
                    c"data[j] == exp".as_ptr(),
                    c"buffered_reads.pass.cpp".as_ptr(),
                    123,
                    &mut _arg,
                )
            });
            j.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut ifs: () = unimplemented!();
        unimplemented!();
        (unsafe { UserManagedBuffer::operator_call_7(&mut policy, &mut ifs) });
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        let mut previously_read: usize = 0_usize;
        let mut in_buffer: Vec<i32> = (0..(total_size as usize) as usize)
            .map(|_| 0_i32)
            .collect::<Vec<_>>();
        'loop_: for payload_sz in IterRange::new((*payload_sizes).begin(), (*payload_sizes).end()) {
            let payload_sz: *const i64 = &(*payload_sz.iter_deref());
            assert!(unimplemented!());
            assert!(unimplemented!());
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: i32 = (((j as usize).wrapping_add(previously_read)) as i32);
                (unsafe {
                    let _condition: bool = (((*(&mut in_buffer)
                        .as_mut_ptr()
                        .add((j as usize).wrapping_add(previously_read) as usize))
                        as i32)
                        == (exp));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let j: *mut () = &mut j;
                            let in_buffer: *mut Vec<i32> = &mut in_buffer;
                            let previously_read: *mut usize = &mut previously_read;
                            let exp: *mut i32 = &mut exp;
                        },
                        || {
                            (unsafe {
                                let _args_1: *const i64 = (j as *const ()).cast::<i64>();
                                let _args_2: *const i32 =
                                    &(*(&mut (*in_buffer))
                                        .as_mut_ptr()
                                        .add(((*j) as usize).wrapping_add((*previously_read))
                                            as usize));
                                test_eprintf_5(
                                    c"failed after read() at offset %zu: got=%x, expected=%x\n"
                                        .as_ptr(),
                                    _args_1,
                                    _args_2,
                                    exp,
                                )
                            });
                        }
                    );
                    test_require_36(
                        _condition,
                        c"in_buffer[j + previously_read] == exp".as_ptr(),
                        c"buffered_reads.pass.cpp".as_ptr(),
                        144,
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_read = (previously_read).wrapping_add(((*payload_sz) as usize));
        }
    }
    unimplemented!();
}
pub static mut buffer_sizes_45: std::cell::LazyCell<Vec<i64>> =
    std::cell::LazyCell::new(|| unsafe { Vec::from([0_i64, 3_i64, 8_i64, 9_i64, 11_i64]) });
pub static mut io_sizes_46: std::cell::LazyCell<Vec<i64>> = std::cell::LazyCell::new(|| unsafe {
    Vec::from([
        0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 9_i64, 10_i64, 11_i64, 12_i64, 13_i64, 21_i64, 22_i64,
        23_i64,
    ])
});
pub static mut io_sizes_default_47: std::cell::LazyCell<Vec<i64>> =
    std::cell::LazyCell::new(|| unsafe {
        Vec::from([
            0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 4094_i64, 4095_i64, 4096_i64, 4097_i64, 4098_i64,
            8190_i64, 8191_i64, 8192_i64, 8193_i64, 8194_i64,
        ])
    });
pub unsafe fn test_1_read_48() {
    'loop_: for x in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_default_47)).begin(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_default_47)).end(),
    ) {
        let mut x: () = ((*x.iter_deref()) as ());
        (unsafe {
            let mut _payload_sizes: Vec<i64> = Vec::from([(x as i64)]);
            test_read_39(LibraryDefaultBuffer::new(), &mut _payload_sizes)
        });
        (unsafe {
            let mut _payload_sizes: Vec<i64> = Vec::from([(x as i64)]);
            test_read_codecvt_42(LibraryDefaultBuffer::new(), &mut _payload_sizes)
        });
    }
    'loop_: for b in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut buffer_sizes_45)).begin(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut buffer_sizes_45)).end(),
    ) {
        let mut b: () = ((*b.iter_deref()) as ());
        'loop_: for x in IterRange::new(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).begin(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).end(),
        ) {
            let mut x: () = ((*x.iter_deref()) as ());
            (unsafe {
                let mut _payload_sizes: Vec<i64> = Vec::from([(x as i64)]);
                test_read_40(
                    LibraryManagedBuffer::new({ (b as usize) }),
                    &mut _payload_sizes,
                )
            });
            (unsafe {
                let mut _payload_sizes: Vec<i64> = Vec::from([(x as i64)]);
                test_read_codecvt_43(
                    LibraryManagedBuffer::new({ (b as usize) }),
                    &mut _payload_sizes,
                )
            });
        }
    }
    'loop_: for b in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut buffer_sizes_45)).begin(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut buffer_sizes_45)).end(),
    ) {
        let mut b: () = ((*b.iter_deref()) as ());
        'loop_: for x in IterRange::new(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).begin(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).end(),
        ) {
            let mut x: () = ((*x.iter_deref()) as ());
            (unsafe {
                let mut _payload_sizes: Vec<i64> = Vec::from([(x as i64)]);
                test_read_41(
                    UserManagedBuffer::new({ (b as usize) }),
                    &mut _payload_sizes,
                )
            });
            (unsafe {
                let mut _payload_sizes: Vec<i64> = Vec::from([(x as i64)]);
                test_read_codecvt_44(
                    UserManagedBuffer::new({ (b as usize) }),
                    &mut _payload_sizes,
                )
            });
        }
    }
}
pub unsafe fn test_2_reads_49() {
    'loop_: for a in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_default_47)).begin(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_default_47)).end(),
    ) {
        let mut a: () = ((*a.iter_deref()) as ());
        'loop_: for b in IterRange::new(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_default_47)).begin(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_default_47)).end(),
        ) {
            let mut b: () = ((*b.iter_deref()) as ());
            (unsafe {
                let mut _payload_sizes: Vec<i64> = Vec::from([(a as i64), (b as i64)]);
                test_read_39(LibraryDefaultBuffer::new(), &mut _payload_sizes)
            });
            (unsafe {
                let mut _payload_sizes: Vec<i64> = Vec::from([(a as i64), (b as i64)]);
                test_read_codecvt_42(LibraryDefaultBuffer::new(), &mut _payload_sizes)
            });
        }
    }
    'loop_: for buf in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut buffer_sizes_45)).begin(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut buffer_sizes_45)).end(),
    ) {
        let mut buf: () = ((*buf.iter_deref()) as ());
        'loop_: for a in IterRange::new(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).begin(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).end(),
        ) {
            let mut a: () = ((*a.iter_deref()) as ());
            'loop_: for b in IterRange::new(
                (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).begin(),
                (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).end(),
            ) {
                let mut b: () = ((*b.iter_deref()) as ());
                (unsafe {
                    let mut _payload_sizes: Vec<i64> = Vec::from([(a as i64), (b as i64)]);
                    test_read_40(
                        LibraryManagedBuffer::new({ (buf as usize) }),
                        &mut _payload_sizes,
                    )
                });
                (unsafe {
                    let mut _payload_sizes: Vec<i64> = Vec::from([(a as i64), (b as i64)]);
                    test_read_codecvt_43(
                        LibraryManagedBuffer::new({ (buf as usize) }),
                        &mut _payload_sizes,
                    )
                });
            }
        }
    }
    'loop_: for buf in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut buffer_sizes_45)).begin(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut buffer_sizes_45)).end(),
    ) {
        let mut buf: () = ((*buf.iter_deref()) as ());
        'loop_: for a in IterRange::new(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).begin(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).end(),
        ) {
            let mut a: () = ((*a.iter_deref()) as ());
            'loop_: for b in IterRange::new(
                (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).begin(),
                (*std::cell::LazyCell::force_mut(&mut *&raw mut io_sizes_46)).end(),
            ) {
                let mut b: () = ((*b.iter_deref()) as ());
                (unsafe {
                    let mut _payload_sizes: Vec<i64> = Vec::from([(a as i64), (b as i64)]);
                    test_read_41(
                        UserManagedBuffer::new({ (buf as usize) }),
                        &mut _payload_sizes,
                    )
                });
                (unsafe {
                    let mut _payload_sizes: Vec<i64> = Vec::from([(a as i64), (b as i64)]);
                    test_read_codecvt_44(
                        UserManagedBuffer::new({ (buf as usize) }),
                        &mut _payload_sizes,
                    )
                });
            }
        }
    }
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_1_read_48() });
    (unsafe { test_2_reads_49() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const buffer_sizes_45);
    std::cell::LazyCell::force(&*&raw const io_sizes_46);
    std::cell::LazyCell::force(&*&raw const io_sizes_default_47);
}
