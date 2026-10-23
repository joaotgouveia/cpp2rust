extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Destructor, Default)]
#[byte_size(56)]
pub struct UserManagedBuffer {
    #[offset(0)]
    size_: (),
    #[offset(8)]
    #[byte_size(24)]
    buffers_: (),
    #[offset(32)]
    #[byte_size(24)]
    wbuffers_: (),
}
impl UserManagedBuffer {
    pub fn new(mut size: ()) -> Self {
        Self {
            size_: size,
            buffers_: unimplemented!(),
            wbuffers_: unimplemented!(),
        }
    }
    pub fn move_from(_a0: Ptr<UserManagedBuffer>) -> Self {
        Self {
            size_: { (*_a0.upgrade().deref()).size_ },
            buffers_: unimplemented!(),
            wbuffers_: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct LibraryManagedBuffer {
    #[offset(0)]
    size_: (),
}
impl LibraryManagedBuffer {
    pub fn new(mut size: ()) -> Self {
        Self { size_: size }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct LibraryDefaultBuffer {}
impl LibraryDefaultBuffer {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for LibraryDefaultBuffer {
    fn default() -> Self {
        { LibraryDefaultBuffer::new() }
    }
}
pub fn test_eprintf_0(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<Ptr<()>>,
    args_3: Ptr<Ptr<()>>,
    args_4: Ptr<()>,
    args_5: Ptr<Ptr<()>>,
) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args_1.read()),
            (args_2.read()),
            (args_3.read()),
            (args_4.read()),
            (args_5.read()),
        )
    });
}
pub fn test_eprintf_1(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<Ptr<()>>,
    args_3: Ptr<()>,
) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args_1.read()),
            (args_2.read()),
            (args_3.read()),
        )
    });
}
pub fn test_eprintf_2(mut fmt: Ptr<()>, args_1: Ptr<()>, args_2: Ptr<()>, args_3: Ptr<()>) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args_1.read()),
            (args_2.read()),
            (args_3.read()),
        )
    });
}
pub fn test_eprintf_3(mut fmt: Ptr<()>, args_1: Ptr<()>, args_2: Ptr<()>) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args_1.read()),
            (args_2.read()),
        )
    });
}
pub fn test_eprintf_4(
    mut fmt: Ptr<()>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
    args_4: Ptr<()>,
    args_5: Ptr<()>,
) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args_1.read()),
            (args_2.read()),
            (args_3.read()),
            (args_4.read()),
            (args_5.read()),
        )
    });
}
pub fn test_eprintf_5(mut fmt: Ptr<()>, args_1: Ptr<()>, args_2: Ptr<()>, args_3: Ptr<()>) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args_1.read()),
            (args_2.read()),
            (args_3.read()),
        )
    });
}
pub fn test_log_6(condition: Ptr<()>, file: Ptr<()>, line: (), message: Ptr<()>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    let message: Value<Ptr<()>> = Rc::new(RefCell::new(message));
    let msg: Value<Ptr<()>> = Rc::new(RefCell::new(if !(*condition.borrow()).is_null() {
        Ptr::<()>::from_string_literal(b"Assertion failure: ")
    } else {
        Ptr::<()>::from_string_literal(b"Unconditional failure:")
    }));
    ({
        test_eprintf_0(
            Ptr::<()>::from_string_literal(b"%s%s %s %d\n%s"),
            msg.as_pointer(),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
            message.as_pointer(),
        )
    });
}
pub fn test_log_7(condition: Ptr<()>, file: Ptr<()>, line: (), functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<()>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_8(condition: Ptr<()>, file: Ptr<()>, line: (), functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<()>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_9(condition: Ptr<()>, file: Ptr<()>, line: (), functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<()>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_10(condition: Ptr<()>, file: Ptr<()>, line: (), functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<()>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_11(condition: Ptr<()>, file: Ptr<()>, line: (), functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<()>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_require_12(
    mut condition: (),
    mut condition_str: Ptr<()>,
    mut file: Ptr<()>,
    mut line: (),
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<()> = (condition_str).clone();
        let _file: Ptr<()> = (file).clone();
        let _line: () = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_7(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub fn test_require_13(
    mut condition: (),
    mut condition_str: Ptr<()>,
    mut file: Ptr<()>,
    mut line: (),
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<()> = (condition_str).clone();
        let _file: Ptr<()> = (file).clone();
        let _line: () = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_8(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub fn test_require_14(
    mut condition: (),
    mut condition_str: Ptr<()>,
    mut file: Ptr<()>,
    mut line: (),
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<()> = (condition_str).clone();
        let _file: Ptr<()> = (file).clone();
        let _line: () = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_9(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub fn test_require_15(
    mut condition: (),
    mut condition_str: Ptr<()>,
    mut file: Ptr<()>,
    mut line: (),
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<()> = (condition_str).clone();
        let _file: Ptr<()> = (file).clone();
        let _line: () = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_10(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub fn test_require_16(
    mut condition: (),
    mut condition_str: Ptr<()>,
    mut file: Ptr<()>,
    mut line: (),
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<()> = (condition_str).clone();
        let _file: Ptr<()> = (file).clone();
        let _line: () = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_11(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub fn get_temp_file_name_17() -> () {
    let mut Name: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { Ptr::<()>::from_string_literal(b"libcxx.XXXXXX") },
        None,
    );
    let mut FD: () = ({ mkstemp_18((unimplemented!())) });
    if (FD == -1_()) {
        ({ perror_19(Ptr::<()>::from_string_literal(b"mkstemp")) });
        unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub fn glibc_version_less_than_20(mut version: Ptr<()>) -> () {
    let mut test_version: () = unimplemented!();
    let mut n: () = ({
        confstr_21(
            (anon_22__CS_GNU_LIBC_VERSION as ()),
            Ptr::<()>::null(),
            0_(),
        )
    });
    let mut current_version: Ptr<()> =
        Ptr::alloc_array((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>());
    ({
        confstr_21(
            (anon_22__CS_GNU_LIBC_VERSION as ()),
            (current_version).clone(),
            n,
        )
    });
    let result: Value<()> = Rc::new(RefCell::new(
        (({ strverscmp_23((current_version).clone(), unimplemented!()) }) < 0_()),
    ));
    current_version.delete();
    return (*result.borrow());
}
pub fn test_read_24(policy: LibraryDefaultBuffer, payload_sizes: Ptr<()>) {
    let policy: Value<LibraryDefaultBuffer> = Rc::new(RefCell::new(policy));
    let total_size: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let __rhs =
            (i).wrapping_rem((1_() << (8_()).wrapping_mul((::std::mem::size_of::<()>() as ()))));
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    let mut p: () = ({ get_temp_file_name_17() });
    {
        let ofs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let j: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while ((*j.borrow()) < (*total_size.borrow())) {
            let exp: Value<()> = Rc::new(RefCell::new(
                ((*j.borrow())
                    % (1_() << (8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))),
            ));
            ({
                let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let j: Ptr<()> = j.as_pointer();
                        let data: Ptr<()> = data.as_pointer();
                        let exp: Ptr<()> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<()> = (j).clone();
                            let _args_2: Ptr<()> = unimplemented!();
                            test_eprintf_2(
                                Ptr::<()>::from_string_literal(
                                    b"failed after write() at offset %zu: got=%x, expected=%x\n",
                                ),
                                _args_1,
                                _args_2,
                                (exp).clone(),
                            )
                        });
                    }
                )));
                test_require_12(
                    _condition,
                    Ptr::<()>::from_string_literal(b"data[j] == exp"),
                    Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                    67_(),
                    _arg.as_pointer(),
                )
            });
            (*j.borrow_mut()).prefix_inc();
        }
        unimplemented!();
    }
    {
        let ifs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            LibraryDefaultBufferImpl::operator_call_3_char(&policy.as_pointer(), ifs.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        let in_sz: Value<()> = Rc::new(RefCell::new(({ unimplemented!()() })));
        ({
            let _condition: () = ((*in_sz.borrow()) == (*total_size.borrow()));
            let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                {
                    let total_size: Ptr<()> = total_size.as_pointer();
                    let in_sz: Ptr<()> = in_sz.as_pointer();
                },
                || {
                    ({
                        test_eprintf_3(
                            Ptr::<()>::from_string_literal(b"out_sz = %zu, in_sz = %ld\n"),
                            (total_size).clone(),
                            (in_sz).clone(),
                        )
                    });
                }
            )));
            test_require_13(
                _condition,
                Ptr::<()>::from_string_literal(b"in_sz == total_size"),
                Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                77_(),
                _arg.as_pointer(),
            )
        });
        unimplemented!();
        let previously_read: Value<()> = Rc::new(RefCell::new(0_()));
        let in_buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: Ptr<()> = unimplemented!();
            unimplemented!();
            unimplemented!();
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<()> = Rc::new(RefCell::new(
                    ((*previously_read.borrow()).wrapping_add((*j.borrow()))).wrapping_rem(
                        (1_() << (8_()).wrapping_mul((::std::mem::size_of::<()>() as ()))),
                    ),
                ));
                ({
                    let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let previously_read: Ptr<()> = previously_read.as_pointer();
                            let j: Ptr<()> = j.as_pointer();
                            let payload_sz: Ptr<()> = (payload_sz).clone();
                            let in_buffer: Ptr<()> = in_buffer.as_pointer();
                            let exp: Ptr<()> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Value<()> = Rc::new(RefCell::new(
                                    (previously_read.read()).wrapping_add((j.read())),
                                ));
                                let _args_2: Ptr<()> = (j).clone();
                                let _args_3: Ptr<()> = (payload_sz).clone();
                                let _args_4: Ptr<()> = unimplemented!();
                                let _args_5: Ptr<()> = (exp).clone();
                                test_eprintf_4 ( Ptr::<() >::from_string_literal(b"failed after read() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" )  , _args_1.as_pointer() , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    )));
                    test_require_14(
                        _condition,
                        Ptr::<()>::from_string_literal(b"in_buffer[previously_read + j] == exp"),
                        Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                        94_(),
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_read.borrow_mut()) =
                { (*previously_read.borrow()).wrapping_add((payload_sz.read())) };
        }
    }
    ({ remove_25(unimplemented!()) });
}
pub fn test_read_26(policy: LibraryManagedBuffer, payload_sizes: Ptr<()>) {
    let policy: Value<LibraryManagedBuffer> = Rc::new(RefCell::new(policy));
    let total_size: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let __rhs =
            (i).wrapping_rem((1_() << (8_()).wrapping_mul((::std::mem::size_of::<()>() as ()))));
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    let mut p: () = ({ get_temp_file_name_17() });
    {
        let ofs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let j: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while ((*j.borrow()) < (*total_size.borrow())) {
            let exp: Value<()> = Rc::new(RefCell::new(
                ((*j.borrow())
                    % (1_() << (8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))),
            ));
            ({
                let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let j: Ptr<()> = j.as_pointer();
                        let data: Ptr<()> = data.as_pointer();
                        let exp: Ptr<()> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<()> = (j).clone();
                            let _args_2: Ptr<()> = unimplemented!();
                            test_eprintf_2(
                                Ptr::<()>::from_string_literal(
                                    b"failed after write() at offset %zu: got=%x, expected=%x\n",
                                ),
                                _args_1,
                                _args_2,
                                (exp).clone(),
                            )
                        });
                    }
                )));
                test_require_27(
                    _condition,
                    Ptr::<()>::from_string_literal(b"data[j] == exp"),
                    Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                    67_(),
                    _arg.as_pointer(),
                )
            });
            (*j.borrow_mut()).prefix_inc();
        }
        unimplemented!();
    }
    {
        let ifs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            LibraryManagedBufferImpl::operator_call_3_char(&policy.as_pointer(), ifs.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        let in_sz: Value<()> = Rc::new(RefCell::new(({ unimplemented!()() })));
        ({
            let _condition: () = ((*in_sz.borrow()) == (*total_size.borrow()));
            let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                {
                    let total_size: Ptr<()> = total_size.as_pointer();
                    let in_sz: Ptr<()> = in_sz.as_pointer();
                },
                || {
                    ({
                        test_eprintf_3(
                            Ptr::<()>::from_string_literal(b"out_sz = %zu, in_sz = %ld\n"),
                            (total_size).clone(),
                            (in_sz).clone(),
                        )
                    });
                }
            )));
            test_require_28(
                _condition,
                Ptr::<()>::from_string_literal(b"in_sz == total_size"),
                Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                77_(),
                _arg.as_pointer(),
            )
        });
        unimplemented!();
        let previously_read: Value<()> = Rc::new(RefCell::new(0_()));
        let in_buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: Ptr<()> = unimplemented!();
            unimplemented!();
            unimplemented!();
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<()> = Rc::new(RefCell::new(
                    ((*previously_read.borrow()).wrapping_add((*j.borrow()))).wrapping_rem(
                        (1_() << (8_()).wrapping_mul((::std::mem::size_of::<()>() as ()))),
                    ),
                ));
                ({
                    let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let previously_read: Ptr<()> = previously_read.as_pointer();
                            let j: Ptr<()> = j.as_pointer();
                            let payload_sz: Ptr<()> = (payload_sz).clone();
                            let in_buffer: Ptr<()> = in_buffer.as_pointer();
                            let exp: Ptr<()> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Value<()> = Rc::new(RefCell::new(
                                    (previously_read.read()).wrapping_add((j.read())),
                                ));
                                let _args_2: Ptr<()> = (j).clone();
                                let _args_3: Ptr<()> = (payload_sz).clone();
                                let _args_4: Ptr<()> = unimplemented!();
                                let _args_5: Ptr<()> = (exp).clone();
                                test_eprintf_4 ( Ptr::<() >::from_string_literal(b"failed after read() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" )  , _args_1.as_pointer() , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    )));
                    test_require_29(
                        _condition,
                        Ptr::<()>::from_string_literal(b"in_buffer[previously_read + j] == exp"),
                        Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                        94_(),
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_read.borrow_mut()) =
                { (*previously_read.borrow()).wrapping_add((payload_sz.read())) };
        }
    }
    ({ remove_25(unimplemented!()) });
}
pub fn test_read_30(policy: UserManagedBuffer, payload_sizes: Ptr<()>) {
    let policy: Value<UserManagedBuffer> = Rc::new(RefCell::new(policy));
    let total_size: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let __rhs =
            (i).wrapping_rem((1_() << (8_()).wrapping_mul((::std::mem::size_of::<()>() as ()))));
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    let mut p: () = ({ get_temp_file_name_17() });
    {
        let ofs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let j: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while ((*j.borrow()) < (*total_size.borrow())) {
            let exp: Value<()> = Rc::new(RefCell::new(
                ((*j.borrow())
                    % (1_() << (8_()).wrapping_mul((::std::mem::size_of::<()>() as ())))),
            ));
            ({
                let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let j: Ptr<()> = j.as_pointer();
                        let data: Ptr<()> = data.as_pointer();
                        let exp: Ptr<()> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<()> = (j).clone();
                            let _args_2: Ptr<()> = unimplemented!();
                            test_eprintf_2(
                                Ptr::<()>::from_string_literal(
                                    b"failed after write() at offset %zu: got=%x, expected=%x\n",
                                ),
                                _args_1,
                                _args_2,
                                (exp).clone(),
                            )
                        });
                    }
                )));
                test_require_31(
                    _condition,
                    Ptr::<()>::from_string_literal(b"data[j] == exp"),
                    Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                    67_(),
                    _arg.as_pointer(),
                )
            });
            (*j.borrow_mut()).prefix_inc();
        }
        unimplemented!();
    }
    {
        let ifs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ UserManagedBufferImpl::operator_call_5(&policy.as_pointer(), ifs.as_pointer()) });
        unimplemented!();
        unimplemented!();
        let in_sz: Value<()> = Rc::new(RefCell::new(({ unimplemented!()() })));
        ({
            let _condition: () = ((*in_sz.borrow()) == (*total_size.borrow()));
            let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                {
                    let total_size: Ptr<()> = total_size.as_pointer();
                    let in_sz: Ptr<()> = in_sz.as_pointer();
                },
                || {
                    ({
                        test_eprintf_3(
                            Ptr::<()>::from_string_literal(b"out_sz = %zu, in_sz = %ld\n"),
                            (total_size).clone(),
                            (in_sz).clone(),
                        )
                    });
                }
            )));
            test_require_32(
                _condition,
                Ptr::<()>::from_string_literal(b"in_sz == total_size"),
                Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                77_(),
                _arg.as_pointer(),
            )
        });
        unimplemented!();
        let previously_read: Value<()> = Rc::new(RefCell::new(0_()));
        let in_buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: Ptr<()> = unimplemented!();
            unimplemented!();
            unimplemented!();
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<()> = Rc::new(RefCell::new(
                    ((*previously_read.borrow()).wrapping_add((*j.borrow()))).wrapping_rem(
                        (1_() << (8_()).wrapping_mul((::std::mem::size_of::<()>() as ()))),
                    ),
                ));
                ({
                    let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let previously_read: Ptr<()> = previously_read.as_pointer();
                            let j: Ptr<()> = j.as_pointer();
                            let payload_sz: Ptr<()> = (payload_sz).clone();
                            let in_buffer: Ptr<()> = in_buffer.as_pointer();
                            let exp: Ptr<()> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Value<()> = Rc::new(RefCell::new(
                                    (previously_read.read()).wrapping_add((j.read())),
                                ));
                                let _args_2: Ptr<()> = (j).clone();
                                let _args_3: Ptr<()> = (payload_sz).clone();
                                let _args_4: Ptr<()> = unimplemented!();
                                let _args_5: Ptr<()> = (exp).clone();
                                test_eprintf_4 ( Ptr::<() >::from_string_literal(b"failed after read() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" )  , _args_1.as_pointer() , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    )));
                    test_require_33(
                        _condition,
                        Ptr::<()>::from_string_literal(b"in_buffer[previously_read + j] == exp"),
                        Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                        94_(),
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_read.borrow_mut()) =
                { (*previously_read.borrow()).wrapping_add((payload_sz.read())) };
        }
    }
    ({ remove_25(unimplemented!()) });
}
pub fn test_read_codecvt_34(policy: LibraryDefaultBuffer, payload_sizes: Ptr<()>) {
    let policy: Value<LibraryDefaultBuffer> = Rc::new(RefCell::new(policy));
    let mut total_size: () = unimplemented!();
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let __rhs = i;
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    let mut p: () = ({ get_temp_file_name_17() });
    {
        let ofs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let j: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while ((*j.borrow()) < total_size) {
            let exp: Value<()> = Rc::new(RefCell::new((*j.borrow())));
            ({
                let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let j: Ptr<()> = j.as_pointer();
                        let data: Ptr<()> = data.as_pointer();
                        let exp: Ptr<()> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<()> = (j).clone();
                            let _args_2: Ptr<()> = unimplemented!();
                            test_eprintf_5(
                                Ptr::<()>::from_string_literal(
                                    b"failed after write() at offset %zu: got=%x, expected=%x\n",
                                ),
                                _args_1,
                                _args_2,
                                (exp).clone(),
                            )
                        });
                    }
                )));
                test_require_15(
                    _condition,
                    Ptr::<()>::from_string_literal(b"data[j] == exp"),
                    Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                    123_(),
                    _arg.as_pointer(),
                )
            });
            (*j.borrow_mut()).prefix_inc();
        }
        unimplemented!();
    }
    {
        let ifs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        ({
            LibraryDefaultBufferImpl::operator_call_3_wchar_t(
                &policy.as_pointer(),
                ifs.as_pointer(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let previously_read: Value<()> = Rc::new(RefCell::new(0_()));
        let in_buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: Ptr<()> = unimplemented!();
            unimplemented!();
            unimplemented!();
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<()> = Rc::new(RefCell::new(
                    (*j.borrow()).wrapping_add((*previously_read.borrow())),
                ));
                ({
                    let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let j: Ptr<()> = j.as_pointer();
                            let in_buffer: Ptr<()> = in_buffer.as_pointer();
                            let previously_read: Ptr<()> = previously_read.as_pointer();
                            let exp: Ptr<()> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Ptr<()> = (j).clone();
                                let _args_2: Ptr<()> = unimplemented!();
                                test_eprintf_5(
                                    Ptr::<()>::from_string_literal(
                                        b"failed after read() at offset %zu: got=%x, expected=%x\n",
                                    ),
                                    _args_1,
                                    _args_2,
                                    (exp).clone(),
                                )
                            });
                        }
                    )));
                    test_require_16(
                        _condition,
                        Ptr::<()>::from_string_literal(b"in_buffer[j + previously_read] == exp"),
                        Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                        144_(),
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_read.borrow_mut()) =
                { (*previously_read.borrow()).wrapping_add((payload_sz.read())) };
        }
    }
    ({ remove_25(unimplemented!()) });
}
pub fn test_read_codecvt_35(policy: LibraryManagedBuffer, payload_sizes: Ptr<()>) {
    let policy: Value<LibraryManagedBuffer> = Rc::new(RefCell::new(policy));
    let mut total_size: () = unimplemented!();
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let __rhs = i;
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    let mut p: () = ({ get_temp_file_name_17() });
    {
        let ofs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let j: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while ((*j.borrow()) < total_size) {
            let exp: Value<()> = Rc::new(RefCell::new((*j.borrow())));
            ({
                let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let j: Ptr<()> = j.as_pointer();
                        let data: Ptr<()> = data.as_pointer();
                        let exp: Ptr<()> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<()> = (j).clone();
                            let _args_2: Ptr<()> = unimplemented!();
                            test_eprintf_5(
                                Ptr::<()>::from_string_literal(
                                    b"failed after write() at offset %zu: got=%x, expected=%x\n",
                                ),
                                _args_1,
                                _args_2,
                                (exp).clone(),
                            )
                        });
                    }
                )));
                test_require_36(
                    _condition,
                    Ptr::<()>::from_string_literal(b"data[j] == exp"),
                    Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                    123_(),
                    _arg.as_pointer(),
                )
            });
            (*j.borrow_mut()).prefix_inc();
        }
        unimplemented!();
    }
    {
        let ifs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        ({
            LibraryManagedBufferImpl::operator_call_3_wchar_t(
                &policy.as_pointer(),
                ifs.as_pointer(),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let previously_read: Value<()> = Rc::new(RefCell::new(0_()));
        let in_buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: Ptr<()> = unimplemented!();
            unimplemented!();
            unimplemented!();
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<()> = Rc::new(RefCell::new(
                    (*j.borrow()).wrapping_add((*previously_read.borrow())),
                ));
                ({
                    let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let j: Ptr<()> = j.as_pointer();
                            let in_buffer: Ptr<()> = in_buffer.as_pointer();
                            let previously_read: Ptr<()> = previously_read.as_pointer();
                            let exp: Ptr<()> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Ptr<()> = (j).clone();
                                let _args_2: Ptr<()> = unimplemented!();
                                test_eprintf_5(
                                    Ptr::<()>::from_string_literal(
                                        b"failed after read() at offset %zu: got=%x, expected=%x\n",
                                    ),
                                    _args_1,
                                    _args_2,
                                    (exp).clone(),
                                )
                            });
                        }
                    )));
                    test_require_37(
                        _condition,
                        Ptr::<()>::from_string_literal(b"in_buffer[j + previously_read] == exp"),
                        Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                        144_(),
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_read.borrow_mut()) =
                { (*previously_read.borrow()).wrapping_add((payload_sz.read())) };
        }
    }
    ({ remove_25(unimplemented!()) });
}
pub fn test_read_codecvt_38(policy: UserManagedBuffer, payload_sizes: Ptr<()>) {
    let policy: Value<UserManagedBuffer> = Rc::new(RefCell::new(policy));
    let mut total_size: () = unimplemented!();
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let __rhs = i;
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    let mut p: () = ({ get_temp_file_name_17() });
    {
        let ofs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let j: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while ((*j.borrow()) < total_size) {
            let exp: Value<()> = Rc::new(RefCell::new((*j.borrow())));
            ({
                let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let j: Ptr<()> = j.as_pointer();
                        let data: Ptr<()> = data.as_pointer();
                        let exp: Ptr<()> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<()> = (j).clone();
                            let _args_2: Ptr<()> = unimplemented!();
                            test_eprintf_5(
                                Ptr::<()>::from_string_literal(
                                    b"failed after write() at offset %zu: got=%x, expected=%x\n",
                                ),
                                _args_1,
                                _args_2,
                                (exp).clone(),
                            )
                        });
                    }
                )));
                test_require_39(
                    _condition,
                    Ptr::<()>::from_string_literal(b"data[j] == exp"),
                    Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                    123_(),
                    _arg.as_pointer(),
                )
            });
            (*j.borrow_mut()).prefix_inc();
        }
        unimplemented!();
    }
    {
        let ifs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        ({ UserManagedBufferImpl::operator_call_7(&policy.as_pointer(), ifs.as_pointer()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let previously_read: Value<()> = Rc::new(RefCell::new(0_()));
        let in_buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: for payload_sz in IterRange::new(unimplemented!(), unimplemented!()) {
            let payload_sz: Ptr<()> = unimplemented!();
            unimplemented!();
            unimplemented!();
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<()> = Rc::new(RefCell::new(
                    (*j.borrow()).wrapping_add((*previously_read.borrow())),
                ));
                ({
                    let _condition: () = ((unimplemented!().read()) == (*exp.borrow()));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let j: Ptr<()> = j.as_pointer();
                            let in_buffer: Ptr<()> = in_buffer.as_pointer();
                            let previously_read: Ptr<()> = previously_read.as_pointer();
                            let exp: Ptr<()> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Ptr<()> = (j).clone();
                                let _args_2: Ptr<()> = unimplemented!();
                                test_eprintf_5(
                                    Ptr::<()>::from_string_literal(
                                        b"failed after read() at offset %zu: got=%x, expected=%x\n",
                                    ),
                                    _args_1,
                                    _args_2,
                                    (exp).clone(),
                                )
                            });
                        }
                    )));
                    test_require_40(
                        _condition,
                        Ptr::<()>::from_string_literal(b"in_buffer[j + previously_read] == exp"),
                        Ptr::<()>::from_string_literal(b"buffered_reads.pass.cpp"),
                        144_(),
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_read.borrow_mut()) =
                { (*previously_read.borrow()).wrapping_add((payload_sz.read())) };
        }
    }
    ({ remove_25(unimplemented!()) });
}
thread_local!(
    pub static buffer_sizes_41: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static io_sizes_42: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static io_sizes_default_43: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn test_1_read_44() {
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: () = (unimplemented!().read());
        ({
            let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_read_24(LibraryDefaultBuffer::new(), _payload_sizes.as_pointer())
        });
        ({
            let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_read_codecvt_34(LibraryDefaultBuffer::new(), _payload_sizes.as_pointer())
        });
    }
    'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut b: () = (unimplemented!().read());
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (unimplemented!().read());
            ({
                let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                test_read_26(
                    LibraryManagedBuffer::new({ b }),
                    _payload_sizes.as_pointer(),
                )
            });
            ({
                let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                test_read_codecvt_35(
                    LibraryManagedBuffer::new({ b }),
                    _payload_sizes.as_pointer(),
                )
            });
        }
    }
    'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut b: () = (unimplemented!().read());
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (unimplemented!().read());
            ({
                let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                test_read_30(UserManagedBuffer::new({ b }), _payload_sizes.as_pointer())
            });
            ({
                let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                test_read_codecvt_38(UserManagedBuffer::new({ b }), _payload_sizes.as_pointer())
            });
        }
    }
}
pub fn test_2_reads_45() {
    'loop_: for a in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut a: () = (unimplemented!().read());
        'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut b: () = (unimplemented!().read());
            ({
                let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                test_read_24(LibraryDefaultBuffer::new(), _payload_sizes.as_pointer())
            });
            ({
                let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                test_read_codecvt_34(LibraryDefaultBuffer::new(), _payload_sizes.as_pointer())
            });
        }
    }
    'loop_: for buf in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut buf: () = (unimplemented!().read());
        'loop_: for a in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut a: () = (unimplemented!().read());
            'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut b: () = (unimplemented!().read());
                ({
                    let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    test_read_26(
                        LibraryManagedBuffer::new({ buf }),
                        _payload_sizes.as_pointer(),
                    )
                });
                ({
                    let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    test_read_codecvt_35(
                        LibraryManagedBuffer::new({ buf }),
                        _payload_sizes.as_pointer(),
                    )
                });
            }
        }
    }
    'loop_: for buf in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut buf: () = (unimplemented!().read());
        'loop_: for a in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut a: () = (unimplemented!().read());
            'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut b: () = (unimplemented!().read());
                ({
                    let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    test_read_30(UserManagedBuffer::new({ buf }), _payload_sizes.as_pointer())
                });
                ({
                    let _payload_sizes: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    test_read_codecvt_38(
                        UserManagedBuffer::new({ buf }),
                        _payload_sizes.as_pointer(),
                    )
                });
            }
        }
    }
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_1_read_44() });
    ({ test_2_reads_45() });
    return 0_();
}
pub trait LibraryDefaultBufferImpl {
    fn operator_call_3_char(&self, _a0: Ptr<()>);
    fn operator_call_3_wchar_t(&self, _a0: Ptr<()>);
}
impl LibraryDefaultBufferImpl for Ptr<LibraryDefaultBuffer> {
    fn operator_call_3_char(&self, _a0: Ptr<()>) {}
    fn operator_call_3_wchar_t(&self, _a0: Ptr<()>) {}
}
pub trait LibraryManagedBufferImpl {
    fn operator_call_3_char(&self, stream: Ptr<()>);
    fn operator_call_3_wchar_t(&self, stream: Ptr<()>);
}
impl LibraryManagedBufferImpl for Ptr<LibraryManagedBuffer> {
    fn operator_call_3_char(&self, stream: Ptr<()>) {
        unimplemented!();
    }
    fn operator_call_3_wchar_t(&self, stream: Ptr<()>) {
        unimplemented!();
    }
}
pub trait UserManagedBufferImpl {
    fn operator_call_4(&self, stream: Ptr<()>);
    fn operator_call_5(&self, stream: Ptr<()>);
    fn operator_call_6(&self, stream: Ptr<()>);
    fn operator_call_7(&self, stream: Ptr<()>);
    fn destructor(&self);
}
impl UserManagedBufferImpl for Ptr<UserManagedBuffer> {
    fn operator_call_4(&self, stream: Ptr<()>) {
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_5(&self, stream: Ptr<()>) {
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_6(&self, stream: Ptr<()>) {
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_7(&self, stream: Ptr<()>) {
        unimplemented!();
        unimplemented!();
    }
    fn destructor(&self) {
        'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut p: Ptr<()> = (unimplemented!().read());
            p.delete();
        }
        'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut p: Ptr<()> = (unimplemented!().read());
            p.delete();
        }
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = buffer_sizes_41.with(|_| ());
    let _ = io_sizes_42.with(|_| ());
    let _ = io_sizes_default_43.with(|_| ());
}
