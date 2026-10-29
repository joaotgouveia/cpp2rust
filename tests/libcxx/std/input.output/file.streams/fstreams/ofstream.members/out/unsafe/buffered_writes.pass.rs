extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct UserManagedBuffer {
    size_: (),
    buffers_: (),
    wbuffers_: (),
}
impl UserManagedBuffer {
    pub unsafe fn new(mut size: ()) -> Self {
        let mut this = Self {
            size_: size,
            buffers_: unimplemented!(),
            wbuffers_: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut UserManagedBuffer) -> Self {
        let mut this = Self {
            size_: (*_a0).size_,
            buffers_: unimplemented!(),
            wbuffers_: unimplemented!(),
        };
        this
    }
    pub unsafe fn operator_call_4(&mut self, stream: *mut ()) {
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_5(&mut self, stream: *mut ()) {
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_6(&mut self, stream: *mut ()) {
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_7(&mut self, stream: *mut ()) {
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn destructor(&mut self) {
        'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut p: *mut () = (*unimplemented!());
            {
                let __p = p;
                if !__p.is_null() {
                    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                        __p,
                        libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                            / ::std::mem::size_of::<()>(),
                    )))
                }
            };
        }
        'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut p: *mut () = (*unimplemented!());
            {
                let __p = p;
                if !__p.is_null() {
                    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                        __p,
                        libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                            / ::std::mem::size_of::<()>(),
                    )))
                }
            };
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LibraryManagedBuffer {
    size_: (),
}
impl LibraryManagedBuffer {
    pub unsafe fn new(mut size: ()) -> Self {
        let mut this = Self { size_: size };
        this
    }
    pub unsafe fn operator_call_2_char(&self, stream: *mut ()) {
        unimplemented!();
    }
    pub unsafe fn operator_call_2_wchar_t(&self, stream: *mut ()) {
        unimplemented!();
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
    pub unsafe fn operator_call_2_char(&self, _a0: *mut ()) {}
    pub unsafe fn operator_call_2_wchar_t(&self, _a0: *mut ()) {}
}
impl Default for LibraryDefaultBuffer {
    fn default() -> Self {
        unsafe { LibraryDefaultBuffer::new() }
    }
}
pub unsafe fn test_eprintf_0(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const *const (),
    args_4: *const (),
    args_5: *const *const (),
) {
    unimplemented!();
}
pub unsafe fn test_eprintf_1(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const (),
) {
    unimplemented!();
}
pub unsafe fn test_eprintf_2(
    mut fmt: *const (),
    args_1: *const (),
    args_2: *const (),
    args_3: *const (),
    args_4: *const (),
    args_5: *const (),
) {
    unimplemented!();
}
pub unsafe fn test_eprintf_3(mut fmt: *const (), args_1: *const (), args_2: *const ()) {
    unimplemented!();
}
pub unsafe fn test_eprintf_4(
    mut fmt: *const (),
    args_1: *const (),
    args_2: *const (),
    args_3: *const (),
) {
    unimplemented!();
}
pub unsafe fn test_eprintf_5(
    mut fmt: *const (),
    args_1: *const (),
    args_2: *const (),
    args_3: *const (),
    args_4: *const (),
    args_5: *const (),
) {
    unimplemented!();
}
pub unsafe fn test_eprintf_6(
    mut fmt: *const (),
    args_1: *const (),
    args_2: *const (),
    args_3: *const (),
) {
    unimplemented!();
}
pub unsafe fn test_log_7(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    mut message: *const (),
) {
    let mut msg: *const () = if !(condition).is_null() {
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
pub unsafe fn test_log_8(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
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
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
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
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
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
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
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
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
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
pub unsafe fn test_require_13(
    mut condition: (),
    mut condition_str: *const (),
    mut file: *const (),
    mut line: (),
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const () = condition_str;
        let _file: *const () = file;
        let _line: () = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_8(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_14(
    mut condition: (),
    mut condition_str: *const (),
    mut file: *const (),
    mut line: (),
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const () = condition_str;
        let _file: *const () = file;
        let _line: () = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_9(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_15(
    mut condition: (),
    mut condition_str: *const (),
    mut file: *const (),
    mut line: (),
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const () = condition_str;
        let _file: *const () = file;
        let _line: () = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_10(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_16(
    mut condition: (),
    mut condition_str: *const (),
    mut file: *const (),
    mut line: (),
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const () = condition_str;
        let _file: *const () = file;
        let _line: () = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_11(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_17(
    mut condition: (),
    mut condition_str: *const (),
    mut file: *const (),
    mut line: (),
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const () = condition_str;
        let _file: *const () = file;
        let _line: () = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_12(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn get_temp_file_name_18() -> () {
    let mut Name: () = unimplemented!();
    let mut FD: () = unimplemented!();
    if ((FD) == (-1_())) {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn glibc_version_less_than_19(mut version: *const ()) -> () {
    let mut test_version: () = unimplemented!();
    let mut n: () = unimplemented!();
    let mut current_version: *mut () =
        Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    unimplemented!();
    let mut result: () = ((unimplemented!()) < (0_()));
    {
        let __p = current_version;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    return result;
}
pub unsafe fn test_write_20(mut policy: LibraryDefaultBuffer, payload_sizes: *const ()) {
    let mut previously_written: () = 0_();
    let mut total_size: () = unimplemented!();
    let mut data: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        (*unimplemented!()) = (i)
            .wrapping_rem(((1_()) << ((8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))));
        i.prefix_inc();
    }
    let mut p: () = (unsafe { get_temp_file_name_18() });
    {
        let mut ofs: () = unimplemented!();
        (unsafe { LibraryDefaultBuffer::operator_call_2_char(&policy, &mut ofs) });
        unimplemented!();
        unimplemented!();
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: *const () = &(*unimplemented!());
            unimplemented!();
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: () = ((previously_written).wrapping_add(j)).wrapping_rem(
                    ((1_()) << ((8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))),
                );
                (unsafe {
                    let _condition: () = ((*unimplemented!()) == (exp));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let previously_written: *mut () = &mut previously_written;
                            let j: *mut () = &mut j;
                            let payload_sz: *const () = payload_sz;
                            let data: *mut () = &mut data;
                            let exp: *mut () = &mut exp;
                        },
                        || {
                            (unsafe {
                                let mut _args_1: () = (*previously_written).wrapping_add((*j));
                                let _args_2: *const () = j;
                                let _args_3: *const () = payload_sz;
                                let _args_4: *const () = &(*unimplemented!());
                                let _args_5: *const () = exp;
                                test_eprintf_2 ( c"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" .as_ptr() , & mut _args_1 , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    );
                    test_require_13(
                        _condition,
                        c"data[previously_written + j] == exp".as_ptr(),
                        c"buffered_writes.pass.cpp".as_ptr(),
                        76_(),
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_written = (previously_written).wrapping_add((*payload_sz));
        }
        unimplemented!();
    }
    {
        let mut ifs: () = unimplemented!();
        let in_sz: () = unimplemented!();
        (unsafe {
            let _condition: () = ((in_sz) == (total_size));
            let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                {
                    let total_size: *mut () = &mut total_size;
                    let in_sz: *const () = &in_sz;
                },
                || {
                    (unsafe {
                        test_eprintf_3(c"out_sz = %zu, in_sz = %ld\n".as_ptr(), total_size, in_sz)
                    });
                }
            );
            test_require_14(
                _condition,
                c"in_sz == total_size".as_ptr(),
                c"buffered_writes.pass.cpp".as_ptr(),
                85_(),
                &mut _arg,
            )
        });
        let mut in_buffer: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            let mut exp: () = (i).wrapping_rem(
                ((1_()) << ((8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))),
            );
            (unsafe {
                let _condition: () = ((*unimplemented!()) == (exp));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let i: *mut () = &mut i;
                        let in_buffer: *mut () = &mut in_buffer;
                        let exp: *mut () = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const () = i;
                            let _args_2: *const () = &(*unimplemented!());
                            test_eprintf_4(
                                c"failed after read() at offset %zu: got=%x, expected=%x\n"
                                    .as_ptr(),
                                _args_1,
                                _args_2,
                                exp,
                            )
                        });
                    }
                );
                test_require_15(
                    _condition,
                    c"in_buffer[i] == exp".as_ptr(),
                    c"buffered_writes.pass.cpp".as_ptr(),
                    93_(),
                    &mut _arg,
                )
            });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_write_21(mut policy: LibraryManagedBuffer, payload_sizes: *const ()) {
    let mut previously_written: () = 0_();
    let mut total_size: () = unimplemented!();
    let mut data: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        (*unimplemented!()) = (i)
            .wrapping_rem(((1_()) << ((8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))));
        i.prefix_inc();
    }
    let mut p: () = (unsafe { get_temp_file_name_18() });
    {
        let mut ofs: () = unimplemented!();
        (unsafe { LibraryManagedBuffer::operator_call_2_char(&policy, &mut ofs) });
        unimplemented!();
        unimplemented!();
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: *const () = &(*unimplemented!());
            unimplemented!();
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: () = ((previously_written).wrapping_add(j)).wrapping_rem(
                    ((1_()) << ((8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))),
                );
                (unsafe {
                    let _condition: () = ((*unimplemented!()) == (exp));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let previously_written: *mut () = &mut previously_written;
                            let j: *mut () = &mut j;
                            let payload_sz: *const () = payload_sz;
                            let data: *mut () = &mut data;
                            let exp: *mut () = &mut exp;
                        },
                        || {
                            (unsafe {
                                let mut _args_1: () = (*previously_written).wrapping_add((*j));
                                let _args_2: *const () = j;
                                let _args_3: *const () = payload_sz;
                                let _args_4: *const () = &(*unimplemented!());
                                let _args_5: *const () = exp;
                                test_eprintf_2 ( c"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" .as_ptr() , & mut _args_1 , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    );
                    test_require_22(
                        _condition,
                        c"data[previously_written + j] == exp".as_ptr(),
                        c"buffered_writes.pass.cpp".as_ptr(),
                        76_(),
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_written = (previously_written).wrapping_add((*payload_sz));
        }
        unimplemented!();
    }
    {
        let mut ifs: () = unimplemented!();
        let in_sz: () = unimplemented!();
        (unsafe {
            let _condition: () = ((in_sz) == (total_size));
            let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                {
                    let total_size: *mut () = &mut total_size;
                    let in_sz: *const () = &in_sz;
                },
                || {
                    (unsafe {
                        test_eprintf_3(c"out_sz = %zu, in_sz = %ld\n".as_ptr(), total_size, in_sz)
                    });
                }
            );
            test_require_23(
                _condition,
                c"in_sz == total_size".as_ptr(),
                c"buffered_writes.pass.cpp".as_ptr(),
                85_(),
                &mut _arg,
            )
        });
        let mut in_buffer: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            let mut exp: () = (i).wrapping_rem(
                ((1_()) << ((8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))),
            );
            (unsafe {
                let _condition: () = ((*unimplemented!()) == (exp));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let i: *mut () = &mut i;
                        let in_buffer: *mut () = &mut in_buffer;
                        let exp: *mut () = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const () = i;
                            let _args_2: *const () = &(*unimplemented!());
                            test_eprintf_4(
                                c"failed after read() at offset %zu: got=%x, expected=%x\n"
                                    .as_ptr(),
                                _args_1,
                                _args_2,
                                exp,
                            )
                        });
                    }
                );
                test_require_24(
                    _condition,
                    c"in_buffer[i] == exp".as_ptr(),
                    c"buffered_writes.pass.cpp".as_ptr(),
                    93_(),
                    &mut _arg,
                )
            });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_write_25(mut policy: UserManagedBuffer, payload_sizes: *const ()) {
    let mut previously_written: () = 0_();
    let mut total_size: () = unimplemented!();
    let mut data: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        (*unimplemented!()) = (i)
            .wrapping_rem(((1_()) << ((8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))));
        i.prefix_inc();
    }
    let mut p: () = (unsafe { get_temp_file_name_18() });
    {
        let mut ofs: () = unimplemented!();
        (unsafe { UserManagedBuffer::operator_call_4(&mut policy, &mut ofs) });
        unimplemented!();
        unimplemented!();
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: *const () = &(*unimplemented!());
            unimplemented!();
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: () = ((previously_written).wrapping_add(j)).wrapping_rem(
                    ((1_()) << ((8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))),
                );
                (unsafe {
                    let _condition: () = ((*unimplemented!()) == (exp));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let previously_written: *mut () = &mut previously_written;
                            let j: *mut () = &mut j;
                            let payload_sz: *const () = payload_sz;
                            let data: *mut () = &mut data;
                            let exp: *mut () = &mut exp;
                        },
                        || {
                            (unsafe {
                                let mut _args_1: () = (*previously_written).wrapping_add((*j));
                                let _args_2: *const () = j;
                                let _args_3: *const () = payload_sz;
                                let _args_4: *const () = &(*unimplemented!());
                                let _args_5: *const () = exp;
                                test_eprintf_2 ( c"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" .as_ptr() , & mut _args_1 , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    );
                    test_require_26(
                        _condition,
                        c"data[previously_written + j] == exp".as_ptr(),
                        c"buffered_writes.pass.cpp".as_ptr(),
                        76_(),
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_written = (previously_written).wrapping_add((*payload_sz));
        }
        unimplemented!();
    }
    {
        let mut ifs: () = unimplemented!();
        let in_sz: () = unimplemented!();
        (unsafe {
            let _condition: () = ((in_sz) == (total_size));
            let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                {
                    let total_size: *mut () = &mut total_size;
                    let in_sz: *const () = &in_sz;
                },
                || {
                    (unsafe {
                        test_eprintf_3(c"out_sz = %zu, in_sz = %ld\n".as_ptr(), total_size, in_sz)
                    });
                }
            );
            test_require_27(
                _condition,
                c"in_sz == total_size".as_ptr(),
                c"buffered_writes.pass.cpp".as_ptr(),
                85_(),
                &mut _arg,
            )
        });
        let mut in_buffer: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            let mut exp: () = (i).wrapping_rem(
                ((1_()) << ((8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))),
            );
            (unsafe {
                let _condition: () = ((*unimplemented!()) == (exp));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let i: *mut () = &mut i;
                        let in_buffer: *mut () = &mut in_buffer;
                        let exp: *mut () = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const () = i;
                            let _args_2: *const () = &(*unimplemented!());
                            test_eprintf_4(
                                c"failed after read() at offset %zu: got=%x, expected=%x\n"
                                    .as_ptr(),
                                _args_1,
                                _args_2,
                                exp,
                            )
                        });
                    }
                );
                test_require_28(
                    _condition,
                    c"in_buffer[i] == exp".as_ptr(),
                    c"buffered_writes.pass.cpp".as_ptr(),
                    93_(),
                    &mut _arg,
                )
            });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_write_codecvt_29(mut policy: LibraryDefaultBuffer, payload_sizes: *const ()) {
    let mut previously_written: () = 0_();
    let mut total_size: () = unimplemented!();
    let mut data: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        (*unimplemented!()) = i;
        i.prefix_inc();
    }
    let mut p: () = (unsafe { get_temp_file_name_18() });
    {
        let mut ofs: () = unimplemented!();
        unimplemented!();
        (unsafe { LibraryDefaultBuffer::operator_call_2_wchar_t(&policy, &mut ofs) });
        unimplemented!();
        unimplemented!();
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: *const () = &(*unimplemented!());
            unimplemented!();
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: () = (previously_written).wrapping_add(j);
                (unsafe {
                    let _condition: () = ((*unimplemented!()) == (exp));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let previously_written: *mut () = &mut previously_written;
                            let j: *mut () = &mut j;
                            let payload_sz: *const () = payload_sz;
                            let data: *mut () = &mut data;
                            let exp: *mut () = &mut exp;
                        },
                        || {
                            (unsafe {
                                let mut _args_1: () = (*previously_written).wrapping_add((*j));
                                let _args_2: *const () = j;
                                let _args_3: *const () = payload_sz;
                                let _args_4: *const () = &(*unimplemented!());
                                let _args_5: *const () = exp;
                                test_eprintf_5 ( c"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" .as_ptr() , & mut _args_1 , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    );
                    test_require_16(
                        _condition,
                        c"data[previously_written + j] == exp".as_ptr(),
                        c"buffered_writes.pass.cpp".as_ptr(),
                        129_(),
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_written = (previously_written).wrapping_add((*payload_sz));
        }
        unimplemented!();
    }
    {
        let mut ifs: () = unimplemented!();
        unimplemented!();
        let mut in_buffer: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            let mut exp: () = i;
            (unsafe {
                let _condition: () = ((*unimplemented!()) == (exp));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let i: *mut () = &mut i;
                        let in_buffer: *mut () = &mut in_buffer;
                        let exp: *mut () = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const () = i;
                            let _args_2: *const () = &(*unimplemented!());
                            test_eprintf_6(
                                c"failed after read() at offset %zu: got=%x, expected=%x\n"
                                    .as_ptr(),
                                _args_1,
                                _args_2,
                                exp,
                            )
                        });
                    }
                );
                test_require_17(
                    _condition,
                    c"in_buffer[i] == exp".as_ptr(),
                    c"buffered_writes.pass.cpp".as_ptr(),
                    144_(),
                    &mut _arg,
                )
            });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_write_codecvt_30(mut policy: LibraryManagedBuffer, payload_sizes: *const ()) {
    let mut previously_written: () = 0_();
    let mut total_size: () = unimplemented!();
    let mut data: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        (*unimplemented!()) = i;
        i.prefix_inc();
    }
    let mut p: () = (unsafe { get_temp_file_name_18() });
    {
        let mut ofs: () = unimplemented!();
        unimplemented!();
        (unsafe { LibraryManagedBuffer::operator_call_2_wchar_t(&policy, &mut ofs) });
        unimplemented!();
        unimplemented!();
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: *const () = &(*unimplemented!());
            unimplemented!();
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: () = (previously_written).wrapping_add(j);
                (unsafe {
                    let _condition: () = ((*unimplemented!()) == (exp));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let previously_written: *mut () = &mut previously_written;
                            let j: *mut () = &mut j;
                            let payload_sz: *const () = payload_sz;
                            let data: *mut () = &mut data;
                            let exp: *mut () = &mut exp;
                        },
                        || {
                            (unsafe {
                                let mut _args_1: () = (*previously_written).wrapping_add((*j));
                                let _args_2: *const () = j;
                                let _args_3: *const () = payload_sz;
                                let _args_4: *const () = &(*unimplemented!());
                                let _args_5: *const () = exp;
                                test_eprintf_5 ( c"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" .as_ptr() , & mut _args_1 , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    );
                    test_require_31(
                        _condition,
                        c"data[previously_written + j] == exp".as_ptr(),
                        c"buffered_writes.pass.cpp".as_ptr(),
                        129_(),
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_written = (previously_written).wrapping_add((*payload_sz));
        }
        unimplemented!();
    }
    {
        let mut ifs: () = unimplemented!();
        unimplemented!();
        let mut in_buffer: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            let mut exp: () = i;
            (unsafe {
                let _condition: () = ((*unimplemented!()) == (exp));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let i: *mut () = &mut i;
                        let in_buffer: *mut () = &mut in_buffer;
                        let exp: *mut () = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const () = i;
                            let _args_2: *const () = &(*unimplemented!());
                            test_eprintf_6(
                                c"failed after read() at offset %zu: got=%x, expected=%x\n"
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
                    c"in_buffer[i] == exp".as_ptr(),
                    c"buffered_writes.pass.cpp".as_ptr(),
                    144_(),
                    &mut _arg,
                )
            });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_write_codecvt_33(mut policy: UserManagedBuffer, payload_sizes: *const ()) {
    let mut previously_written: () = 0_();
    let mut total_size: () = unimplemented!();
    let mut data: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        (*unimplemented!()) = i;
        i.prefix_inc();
    }
    let mut p: () = (unsafe { get_temp_file_name_18() });
    {
        let mut ofs: () = unimplemented!();
        unimplemented!();
        (unsafe { UserManagedBuffer::operator_call_6(&mut policy, &mut ofs) });
        unimplemented!();
        unimplemented!();
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: *const () = &(*unimplemented!());
            unimplemented!();
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while ((j) < (*payload_sz)) {
                let mut exp: () = (previously_written).wrapping_add(j);
                (unsafe {
                    let _condition: () = ((*unimplemented!()) == (exp));
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let previously_written: *mut () = &mut previously_written;
                            let j: *mut () = &mut j;
                            let payload_sz: *const () = payload_sz;
                            let data: *mut () = &mut data;
                            let exp: *mut () = &mut exp;
                        },
                        || {
                            (unsafe {
                                let mut _args_1: () = (*previously_written).wrapping_add((*j));
                                let _args_2: *const () = j;
                                let _args_3: *const () = payload_sz;
                                let _args_4: *const () = &(*unimplemented!());
                                let _args_5: *const () = exp;
                                test_eprintf_5 ( c"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" .as_ptr() , & mut _args_1 , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    );
                    test_require_34(
                        _condition,
                        c"data[previously_written + j] == exp".as_ptr(),
                        c"buffered_writes.pass.cpp".as_ptr(),
                        129_(),
                        &mut _arg,
                    )
                });
                j.prefix_inc();
            }
            previously_written = (previously_written).wrapping_add((*payload_sz));
        }
        unimplemented!();
    }
    {
        let mut ifs: () = unimplemented!();
        unimplemented!();
        let mut in_buffer: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            let mut exp: () = i;
            (unsafe {
                let _condition: () = ((*unimplemented!()) == (exp));
                let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                    {
                        let i: *mut () = &mut i;
                        let in_buffer: *mut () = &mut in_buffer;
                        let exp: *mut () = &mut exp;
                    },
                    || {
                        (unsafe {
                            let _args_1: *const () = i;
                            let _args_2: *const () = &(*unimplemented!());
                            test_eprintf_6(
                                c"failed after read() at offset %zu: got=%x, expected=%x\n"
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
                    c"in_buffer[i] == exp".as_ptr(),
                    c"buffered_writes.pass.cpp".as_ptr(),
                    144_(),
                    &mut _arg,
                )
            });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub static mut buffer_sizes_36: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut io_sizes_37: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut io_sizes_default_38: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn test_1_write_39() {
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: () = (*unimplemented!());
        (unsafe {
            let mut _payload_sizes: () = unimplemented!();
            test_write_20(LibraryDefaultBuffer::new(), &mut _payload_sizes)
        });
        (unsafe {
            let mut _payload_sizes: () = unimplemented!();
            test_write_codecvt_29(LibraryDefaultBuffer::new(), &mut _payload_sizes)
        });
    }
    'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut b: () = (*unimplemented!());
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*unimplemented!());
            (unsafe {
                let mut _payload_sizes: () = unimplemented!();
                test_write_21(LibraryManagedBuffer::new({ b }), &mut _payload_sizes)
            });
            (unsafe {
                let mut _payload_sizes: () = unimplemented!();
                test_write_codecvt_30(LibraryManagedBuffer::new({ b }), &mut _payload_sizes)
            });
        }
    }
    'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut b: () = (*unimplemented!());
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*unimplemented!());
            (unsafe {
                let mut _payload_sizes: () = unimplemented!();
                test_write_25(UserManagedBuffer::new({ b }), &mut _payload_sizes)
            });
            (unsafe {
                let mut _payload_sizes: () = unimplemented!();
                test_write_codecvt_33(UserManagedBuffer::new({ b }), &mut _payload_sizes)
            });
        }
    }
}
pub unsafe fn test_2_writes_40() {
    'loop_: for a in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut a: () = (*unimplemented!());
        'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut b: () = (*unimplemented!());
            (unsafe {
                let mut _payload_sizes: () = unimplemented!();
                test_write_20(LibraryDefaultBuffer::new(), &mut _payload_sizes)
            });
            (unsafe {
                let mut _payload_sizes: () = unimplemented!();
                test_write_codecvt_29(LibraryDefaultBuffer::new(), &mut _payload_sizes)
            });
        }
    }
    'loop_: for buf in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut buf: () = (*unimplemented!());
        'loop_: for a in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut a: () = (*unimplemented!());
            'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut b: () = (*unimplemented!());
                (unsafe {
                    let mut _payload_sizes: () = unimplemented!();
                    test_write_21(LibraryManagedBuffer::new({ buf }), &mut _payload_sizes)
                });
                (unsafe {
                    let mut _payload_sizes: () = unimplemented!();
                    test_write_codecvt_30(LibraryManagedBuffer::new({ buf }), &mut _payload_sizes)
                });
            }
        }
    }
    'loop_: for buf in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut buf: () = (*unimplemented!());
        'loop_: for a in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut a: () = (*unimplemented!());
            'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut b: () = (*unimplemented!());
                (unsafe {
                    let mut _payload_sizes: () = unimplemented!();
                    test_write_25(UserManagedBuffer::new({ buf }), &mut _payload_sizes)
                });
                (unsafe {
                    let mut _payload_sizes: () = unimplemented!();
                    test_write_codecvt_33(UserManagedBuffer::new({ buf }), &mut _payload_sizes)
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_1_write_39() });
    (unsafe { test_2_writes_40() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const buffer_sizes_36);
    std::cell::LazyCell::force(&*&raw const io_sizes_37);
    std::cell::LazyCell::force(&*&raw const io_sizes_default_38);
}
