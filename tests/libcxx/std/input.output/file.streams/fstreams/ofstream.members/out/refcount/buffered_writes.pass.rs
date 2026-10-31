extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, Destructor, Default)]
#[cpp_from(usize, new, value)]
#[byte_size(56)]
pub struct UserManagedBuffer {
    #[offset(0)]
    size_: usize,
    #[offset(8)]
    #[byte_size(24)]
    buffers_: Vec<Ptr<i8>>,
    #[offset(32)]
    #[byte_size(24)]
    wbuffers_: Vec<Ptr<i32>>,
}
impl UserManagedBuffer {
    pub fn new(mut size: usize) -> Self {
        Self {
            size_: size,
            buffers_: Vec::new(),
            wbuffers_: Vec::new(),
        }
    }
    pub fn move_from(_a0: Ptr<UserManagedBuffer>) -> Self {
        Self {
            size_: { (*_a0.upgrade().deref()).size_ },
            buffers_: field!(_a0, buffers_).with_mut(std::mem::take),
            wbuffers_: field!(_a0, wbuffers_).with_mut(std::mem::take),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(usize, new, value)]
#[byte_size(8)]
pub struct LibraryManagedBuffer {
    #[offset(0)]
    size_: usize,
}
impl LibraryManagedBuffer {
    pub fn new(mut size: usize) -> Self {
        Self { size_: size }
    }
}
impl Callable1<Ptr<()>, ()> for LibraryManagedBuffer {
    fn call(&self, a1: Ptr<()>) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            LibraryManagedBufferImpl::operator_call_2_char(__it, a1)
        })
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
impl Callable1<Ptr<()>, ()> for LibraryDefaultBuffer {
    fn call(&self, a1: Ptr<()>) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            LibraryDefaultBufferImpl::operator_call_2_char(__it, a1)
        })
    }
}
impl Default for LibraryDefaultBuffer {
    fn default() -> Self {
        { LibraryDefaultBuffer::new() }
    }
}
pub fn test_eprintf_0(
    mut fmt: Ptr<i8>,
    args_1: Ptr<Ptr<i8>>,
    args_2: Ptr<Ptr<i8>>,
    args_3: Ptr<Ptr<i8>>,
    args_4: Ptr<i32>,
    args_5: Ptr<Ptr<i8>>,
) {
    {
        let __s = libcc2rs::format_c(
            &fmt.to_rust_string(),
            &Box::<[VaArg]>::from([
                (args_1.read()).into(),
                (args_2.read()).into(),
                (args_3.read()).into(),
                (args_4.read()).into(),
                (args_5.read()).into(),
            ]),
        );
        libcc2rs::c_stderr().with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn test_eprintf_1(
    mut fmt: Ptr<i8>,
    args_1: Ptr<Ptr<i8>>,
    args_2: Ptr<Ptr<i8>>,
    args_3: Ptr<i32>,
) {
    {
        let __s = libcc2rs::format_c(
            &fmt.to_rust_string(),
            &Box::<[VaArg]>::from([
                (args_1.read()).into(),
                (args_2.read()).into(),
                (args_3.read()).into(),
            ]),
        );
        libcc2rs::c_stderr().with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn test_eprintf_2(
    mut fmt: Ptr<i8>,
    args_1: Ptr<u64>,
    args_2: Ptr<i64>,
    args_3: Ptr<i64>,
    args_4: Ptr<i8>,
    args_5: Ptr<i8>,
) {
    {
        let __s = libcc2rs::format_c(
            &fmt.to_rust_string(),
            &Box::<[VaArg]>::from([
                (args_1.read()).into(),
                (args_2.read()).into(),
                (args_3.read()).into(),
                ((args_4.read()) as i32).into(),
                ((args_5.read()) as i32).into(),
            ]),
        );
        libcc2rs::c_stderr().with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn test_eprintf_3(mut fmt: Ptr<i8>, args_1: Ptr<i64>, args_2: Ptr<i64>) {
    {
        let __s = libcc2rs::format_c(
            &fmt.to_rust_string(),
            &Box::<[VaArg]>::from([(args_1.read()).into(), (args_2.read()).into()]),
        );
        libcc2rs::c_stderr().with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn test_eprintf_4(mut fmt: Ptr<i8>, args_1: Ptr<u64>, args_2: Ptr<i8>, args_3: Ptr<i8>) {
    {
        let __s = libcc2rs::format_c(
            &fmt.to_rust_string(),
            &Box::<[VaArg]>::from([
                (args_1.read()).into(),
                ((args_2.read()) as i32).into(),
                ((args_3.read()) as i32).into(),
            ]),
        );
        libcc2rs::c_stderr().with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn test_eprintf_5(
    mut fmt: Ptr<i8>,
    args_1: Ptr<u64>,
    args_2: Ptr<i64>,
    args_3: Ptr<i64>,
    args_4: Ptr<i32>,
    args_5: Ptr<i32>,
) {
    {
        let __s = libcc2rs::format_c(
            &fmt.to_rust_string(),
            &Box::<[VaArg]>::from([
                (args_1.read()).into(),
                (args_2.read()).into(),
                (args_3.read()).into(),
                (args_4.read()).into(),
                (args_5.read()).into(),
            ]),
        );
        libcc2rs::c_stderr().with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn test_eprintf_6(mut fmt: Ptr<i8>, args_1: Ptr<u64>, args_2: Ptr<i32>, args_3: Ptr<i32>) {
    {
        let __s = libcc2rs::format_c(
            &fmt.to_rust_string(),
            &Box::<[VaArg]>::from([
                (args_1.read()).into(),
                (args_2.read()).into(),
                (args_3.read()).into(),
            ]),
        );
        libcc2rs::c_stderr().with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn test_log_7(condition: Ptr<i8>, file: Ptr<i8>, line: i32, message: Ptr<i8>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    let message: Value<Ptr<i8>> = Rc::new(RefCell::new(message));
    let msg: Value<Ptr<i8>> = Rc::new(RefCell::new(if !(*condition.borrow()).is_null() {
        Ptr::<i8>::from_string_literal(b"Assertion failure: ")
    } else {
        Ptr::<i8>::from_string_literal(b"Unconditional failure:")
    }));
    ({
        test_eprintf_0(
            Ptr::<i8>::from_string_literal(b"%s%s %s %d\n%s"),
            msg.as_pointer(),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
            message.as_pointer(),
        )
    });
}
pub fn test_log_8(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_9(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_10(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_11(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_12(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_13(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_14(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_15(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_16(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_17(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_18(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_19(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_20(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_21(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_log_22(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_require_23(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_8(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_24(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_9(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_25(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_10(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_26(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_11(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_27(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_12(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_28(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_13(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_29(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_14(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_30(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_15(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_31(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_16(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_32(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_17(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_33(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_18(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_34(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_19(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_35(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_20(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_36(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_21(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_37(
    mut condition: bool,
    mut condition_str: Ptr<i8>,
    mut file: Ptr<i8>,
    mut line: i32,
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<i8> = (condition_str).clone();
        let _file: Ptr<i8> = (file).clone();
        let _line: i32 = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_22(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn get_temp_file_name_38() -> Vec<i8> {
    let Name: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"libcxx.XXXXXX").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let mut FD: i32 = unimplemented!();
    if (FD == -1_i32) {
        unimplemented!();
        std::process::abort();
    }
    FdRegistry::close(FD);
    return std::mem::take(&mut (*Name.borrow_mut()));
}
pub fn glibc_version_less_than_39(mut version: Ptr<i8>) -> bool {
    let test_version: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __tmp2 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"glibc ").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        __tmp2.pop();
        version.with_c_str(|__s| __tmp2.extend_from_slice(__s));
        __tmp2.push(0_i8);
        __tmp2
    }));
    let mut n: usize = unimplemented!();
    let mut current_version: Ptr<i8> =
        Ptr::alloc_array((0..n).map(|_| 0_i8).collect::<Box<[i8]>>());
    unimplemented!();
    let result: Value<bool> = Rc::new(RefCell::new((unimplemented!() < 0)));
    current_version.delete();
    return (*result.borrow());
}
pub fn test_write_40(policy: LibraryDefaultBuffer, payload_sizes: Ptr<Vec<i64>>) {
    let policy: Value<LibraryDefaultBuffer> = Rc::new(RefCell::new(policy));
    let previously_written: Value<usize> = Rc::new(RefCell::new(0_usize));
    let total_size: Value<()> = Rc::new(RefCell::new((unimplemented!() as ())));
    let data: Value<Vec<i8>> = Rc::new(RefCell::new(
        (0..((*total_size.borrow()) as usize) as usize)
            .map(|_| 0_i8)
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < (*data.borrow()).len()) {
        let __rhs = (((i).wrapping_rem(
            ((1 << (8_usize).wrapping_mul((::std::mem::size_of::<i8>() as usize))) as usize),
        )) as i8);
        data.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    let p: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_38() })));
    {
        let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(unimplemented!()));
        ({
            LibraryDefaultBufferImpl::operator_call_2_char(
                &policy.as_pointer(),
                (ofs.as_pointer()).reinterpret_cast::<()>(),
            )
        });
        unimplemented!();
        assert!(unimplemented!());
        'loop_: for payload_sz in
            IterRange::new(payload_sizes.decay().begin(), payload_sizes.decay().end())
        {
            let payload_sz: Ptr<i64> = payload_sz.iter_deref();
            unimplemented!();
            assert!(!(unimplemented!()));
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<i8> = Rc::new(RefCell::new(
                    ((((*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize)))
                        .wrapping_rem(
                            ((1 << (8_usize).wrapping_mul((::std::mem::size_of::<i8>() as usize)))
                                as usize),
                        )) as i8),
                ));
                ({
                    let _condition: bool = (((data
                        .decay()
                        .offset(
                            (*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize))
                                as isize,
                        )
                        .read()) as i32)
                        == ((*exp.borrow()) as i32));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let previously_written: Ptr<usize> = previously_written.as_pointer();
                            let j: Ptr<()> = j.as_pointer();
                            let payload_sz: Ptr<i64> = (payload_sz).clone();
                            let data: Ptr<Vec<i8>> = data.as_pointer();
                            let exp: Ptr<i8> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Value<u64> = Rc::new(RefCell::new(
                                    ((previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as u64),
                                ));
                                let _args_2: Ptr<i64> = (j).reinterpret_cast::<i64>();
                                let _args_3: Ptr<i64> = (payload_sz).clone();
                                let _args_4: Ptr<i8> = data.decay().offset(
                                    (previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as isize,
                                );
                                let _args_5: Ptr<i8> = (exp).clone();
                                test_eprintf_2 ( Ptr::<i8 >::from_string_literal(b"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" )  , _args_1.as_pointer() , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    )));
                    test_require_23(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"data[previously_written + j] == exp"),
                        Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                        76,
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_written.borrow_mut()) =
                { (*previously_written.borrow()).wrapping_add(((payload_sz.read()) as usize)) };
        }
        unimplemented!();
    }
    {
        let ifs: Value<::std::fs::File> = Rc::new(RefCell::new(unimplemented!()));
        let in_sz: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _condition: bool = ((*in_sz.borrow()) == (*total_size.borrow()));
            let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                {
                    let total_size: Ptr<()> = total_size.as_pointer();
                    let in_sz: Ptr<()> = in_sz.as_pointer();
                },
                || {
                    ({
                        test_eprintf_3(
                            Ptr::<i8>::from_string_literal(b"out_sz = %zu, in_sz = %ld\n"),
                            (total_size).reinterpret_cast::<i64>(),
                            (in_sz).reinterpret_cast::<i64>(),
                        )
                    });
                }
            )));
            test_require_24(
                _condition,
                Ptr::<i8>::from_string_literal(b"in_sz == total_size"),
                Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                85,
                _arg.as_pointer(),
            )
        });
        let in_buffer: Value<Vec<i8>> = Rc::new(RefCell::new(
            (0..((*total_size.borrow()) as usize) as usize)
                .map(|_| 0_i8)
                .collect::<Vec<_>>(),
        ));
        unimplemented!();
        assert!(unimplemented!());
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*in_buffer.borrow()).len()) {
            let exp: Value<i8> = Rc::new(RefCell::new(
                (((*i.borrow()).wrapping_rem(
                    ((1 << (8_usize).wrapping_mul((::std::mem::size_of::<i8>() as usize)))
                        as usize),
                )) as i8),
            ));
            ({
                let _condition: bool = (((in_buffer.decay().offset((*i.borrow()) as isize).read())
                    as i32)
                    == ((*exp.borrow()) as i32));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let i: Ptr<usize> = i.as_pointer();
                        let in_buffer: Ptr<Vec<i8>> = in_buffer.as_pointer();
                        let exp: Ptr<i8> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<u64> = (i).reinterpret_cast::<u64>();
                            let _args_2: Ptr<i8> = in_buffer.decay().offset((i.read()) as isize);
                            test_eprintf_4(
                                Ptr::<i8>::from_string_literal(
                                    b"failed after read() at offset %zu: got=%x, expected=%x\n",
                                ),
                                _args_1,
                                _args_2,
                                (exp).clone(),
                            )
                        });
                    }
                )));
                test_require_25(
                    _condition,
                    Ptr::<i8>::from_string_literal(b"in_buffer[i] == exp"),
                    Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                    93,
                    _arg.as_pointer(),
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_write_41(policy: LibraryManagedBuffer, payload_sizes: Ptr<Vec<i64>>) {
    let policy: Value<LibraryManagedBuffer> = Rc::new(RefCell::new(policy));
    let previously_written: Value<usize> = Rc::new(RefCell::new(0_usize));
    let total_size: Value<()> = Rc::new(RefCell::new((unimplemented!() as ())));
    let data: Value<Vec<i8>> = Rc::new(RefCell::new(
        (0..((*total_size.borrow()) as usize) as usize)
            .map(|_| 0_i8)
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < (*data.borrow()).len()) {
        let __rhs = (((i).wrapping_rem(
            ((1 << (8_usize).wrapping_mul((::std::mem::size_of::<i8>() as usize))) as usize),
        )) as i8);
        data.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    let p: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_38() })));
    {
        let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(unimplemented!()));
        ({
            LibraryManagedBufferImpl::operator_call_2_char(
                &policy.as_pointer(),
                (ofs.as_pointer()).reinterpret_cast::<()>(),
            )
        });
        unimplemented!();
        assert!(unimplemented!());
        'loop_: for payload_sz in
            IterRange::new(payload_sizes.decay().begin(), payload_sizes.decay().end())
        {
            let payload_sz: Ptr<i64> = payload_sz.iter_deref();
            unimplemented!();
            assert!(!(unimplemented!()));
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<i8> = Rc::new(RefCell::new(
                    ((((*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize)))
                        .wrapping_rem(
                            ((1 << (8_usize).wrapping_mul((::std::mem::size_of::<i8>() as usize)))
                                as usize),
                        )) as i8),
                ));
                ({
                    let _condition: bool = (((data
                        .decay()
                        .offset(
                            (*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize))
                                as isize,
                        )
                        .read()) as i32)
                        == ((*exp.borrow()) as i32));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let previously_written: Ptr<usize> = previously_written.as_pointer();
                            let j: Ptr<()> = j.as_pointer();
                            let payload_sz: Ptr<i64> = (payload_sz).clone();
                            let data: Ptr<Vec<i8>> = data.as_pointer();
                            let exp: Ptr<i8> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Value<u64> = Rc::new(RefCell::new(
                                    ((previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as u64),
                                ));
                                let _args_2: Ptr<i64> = (j).reinterpret_cast::<i64>();
                                let _args_3: Ptr<i64> = (payload_sz).clone();
                                let _args_4: Ptr<i8> = data.decay().offset(
                                    (previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as isize,
                                );
                                let _args_5: Ptr<i8> = (exp).clone();
                                test_eprintf_2 ( Ptr::<i8 >::from_string_literal(b"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" )  , _args_1.as_pointer() , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    )));
                    test_require_28(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"data[previously_written + j] == exp"),
                        Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                        76,
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_written.borrow_mut()) =
                { (*previously_written.borrow()).wrapping_add(((payload_sz.read()) as usize)) };
        }
        unimplemented!();
    }
    {
        let ifs: Value<::std::fs::File> = Rc::new(RefCell::new(unimplemented!()));
        let in_sz: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _condition: bool = ((*in_sz.borrow()) == (*total_size.borrow()));
            let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                {
                    let total_size: Ptr<()> = total_size.as_pointer();
                    let in_sz: Ptr<()> = in_sz.as_pointer();
                },
                || {
                    ({
                        test_eprintf_3(
                            Ptr::<i8>::from_string_literal(b"out_sz = %zu, in_sz = %ld\n"),
                            (total_size).reinterpret_cast::<i64>(),
                            (in_sz).reinterpret_cast::<i64>(),
                        )
                    });
                }
            )));
            test_require_29(
                _condition,
                Ptr::<i8>::from_string_literal(b"in_sz == total_size"),
                Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                85,
                _arg.as_pointer(),
            )
        });
        let in_buffer: Value<Vec<i8>> = Rc::new(RefCell::new(
            (0..((*total_size.borrow()) as usize) as usize)
                .map(|_| 0_i8)
                .collect::<Vec<_>>(),
        ));
        unimplemented!();
        assert!(unimplemented!());
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*in_buffer.borrow()).len()) {
            let exp: Value<i8> = Rc::new(RefCell::new(
                (((*i.borrow()).wrapping_rem(
                    ((1 << (8_usize).wrapping_mul((::std::mem::size_of::<i8>() as usize)))
                        as usize),
                )) as i8),
            ));
            ({
                let _condition: bool = (((in_buffer.decay().offset((*i.borrow()) as isize).read())
                    as i32)
                    == ((*exp.borrow()) as i32));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let i: Ptr<usize> = i.as_pointer();
                        let in_buffer: Ptr<Vec<i8>> = in_buffer.as_pointer();
                        let exp: Ptr<i8> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<u64> = (i).reinterpret_cast::<u64>();
                            let _args_2: Ptr<i8> = in_buffer.decay().offset((i.read()) as isize);
                            test_eprintf_4(
                                Ptr::<i8>::from_string_literal(
                                    b"failed after read() at offset %zu: got=%x, expected=%x\n",
                                ),
                                _args_1,
                                _args_2,
                                (exp).clone(),
                            )
                        });
                    }
                )));
                test_require_30(
                    _condition,
                    Ptr::<i8>::from_string_literal(b"in_buffer[i] == exp"),
                    Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                    93,
                    _arg.as_pointer(),
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_write_42(policy: UserManagedBuffer, payload_sizes: Ptr<Vec<i64>>) {
    let policy: Value<UserManagedBuffer> = Rc::new(RefCell::new(policy));
    let previously_written: Value<usize> = Rc::new(RefCell::new(0_usize));
    let total_size: Value<()> = Rc::new(RefCell::new((unimplemented!() as ())));
    let data: Value<Vec<i8>> = Rc::new(RefCell::new(
        (0..((*total_size.borrow()) as usize) as usize)
            .map(|_| 0_i8)
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < (*data.borrow()).len()) {
        let __rhs = (((i).wrapping_rem(
            ((1 << (8_usize).wrapping_mul((::std::mem::size_of::<i8>() as usize))) as usize),
        )) as i8);
        data.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    let p: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_38() })));
    {
        let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(unimplemented!()));
        ({
            UserManagedBufferImpl::operator_call_4(
                &policy.as_pointer(),
                (ofs.as_pointer()).reinterpret_cast::<()>(),
            )
        });
        unimplemented!();
        assert!(unimplemented!());
        'loop_: for payload_sz in
            IterRange::new(payload_sizes.decay().begin(), payload_sizes.decay().end())
        {
            let payload_sz: Ptr<i64> = payload_sz.iter_deref();
            unimplemented!();
            assert!(!(unimplemented!()));
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<i8> = Rc::new(RefCell::new(
                    ((((*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize)))
                        .wrapping_rem(
                            ((1 << (8_usize).wrapping_mul((::std::mem::size_of::<i8>() as usize)))
                                as usize),
                        )) as i8),
                ));
                ({
                    let _condition: bool = (((data
                        .decay()
                        .offset(
                            (*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize))
                                as isize,
                        )
                        .read()) as i32)
                        == ((*exp.borrow()) as i32));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let previously_written: Ptr<usize> = previously_written.as_pointer();
                            let j: Ptr<()> = j.as_pointer();
                            let payload_sz: Ptr<i64> = (payload_sz).clone();
                            let data: Ptr<Vec<i8>> = data.as_pointer();
                            let exp: Ptr<i8> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Value<u64> = Rc::new(RefCell::new(
                                    ((previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as u64),
                                ));
                                let _args_2: Ptr<i64> = (j).reinterpret_cast::<i64>();
                                let _args_3: Ptr<i64> = (payload_sz).clone();
                                let _args_4: Ptr<i8> = data.decay().offset(
                                    (previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as isize,
                                );
                                let _args_5: Ptr<i8> = (exp).clone();
                                test_eprintf_2 ( Ptr::<i8 >::from_string_literal(b"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" )  , _args_1.as_pointer() , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    )));
                    test_require_33(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"data[previously_written + j] == exp"),
                        Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                        76,
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_written.borrow_mut()) =
                { (*previously_written.borrow()).wrapping_add(((payload_sz.read()) as usize)) };
        }
        unimplemented!();
    }
    {
        let ifs: Value<::std::fs::File> = Rc::new(RefCell::new(unimplemented!()));
        let in_sz: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _condition: bool = ((*in_sz.borrow()) == (*total_size.borrow()));
            let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                {
                    let total_size: Ptr<()> = total_size.as_pointer();
                    let in_sz: Ptr<()> = in_sz.as_pointer();
                },
                || {
                    ({
                        test_eprintf_3(
                            Ptr::<i8>::from_string_literal(b"out_sz = %zu, in_sz = %ld\n"),
                            (total_size).reinterpret_cast::<i64>(),
                            (in_sz).reinterpret_cast::<i64>(),
                        )
                    });
                }
            )));
            test_require_34(
                _condition,
                Ptr::<i8>::from_string_literal(b"in_sz == total_size"),
                Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                85,
                _arg.as_pointer(),
            )
        });
        let in_buffer: Value<Vec<i8>> = Rc::new(RefCell::new(
            (0..((*total_size.borrow()) as usize) as usize)
                .map(|_| 0_i8)
                .collect::<Vec<_>>(),
        ));
        unimplemented!();
        assert!(unimplemented!());
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*in_buffer.borrow()).len()) {
            let exp: Value<i8> = Rc::new(RefCell::new(
                (((*i.borrow()).wrapping_rem(
                    ((1 << (8_usize).wrapping_mul((::std::mem::size_of::<i8>() as usize)))
                        as usize),
                )) as i8),
            ));
            ({
                let _condition: bool = (((in_buffer.decay().offset((*i.borrow()) as isize).read())
                    as i32)
                    == ((*exp.borrow()) as i32));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let i: Ptr<usize> = i.as_pointer();
                        let in_buffer: Ptr<Vec<i8>> = in_buffer.as_pointer();
                        let exp: Ptr<i8> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<u64> = (i).reinterpret_cast::<u64>();
                            let _args_2: Ptr<i8> = in_buffer.decay().offset((i.read()) as isize);
                            test_eprintf_4(
                                Ptr::<i8>::from_string_literal(
                                    b"failed after read() at offset %zu: got=%x, expected=%x\n",
                                ),
                                _args_1,
                                _args_2,
                                (exp).clone(),
                            )
                        });
                    }
                )));
                test_require_35(
                    _condition,
                    Ptr::<i8>::from_string_literal(b"in_buffer[i] == exp"),
                    Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                    93,
                    _arg.as_pointer(),
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_write_codecvt_43(policy: LibraryDefaultBuffer, payload_sizes: Ptr<Vec<i64>>) {
    let policy: Value<LibraryDefaultBuffer> = Rc::new(RefCell::new(policy));
    let previously_written: Value<usize> = Rc::new(RefCell::new(0_usize));
    let mut total_size: () = (unimplemented!() as ());
    let data: Value<Vec<i32>> = Rc::new(RefCell::new(
        (0..(total_size as usize) as usize)
            .map(|_| 0_i32)
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < (*data.borrow()).len()) {
        let __rhs = (i as i32);
        data.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    let p: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_38() })));
    {
        let ofs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        ({
            LibraryDefaultBufferImpl::operator_call_2_wchar_t(
                &policy.as_pointer(),
                ofs.as_pointer(),
            )
        });
        unimplemented!();
        assert!(unimplemented!());
        'loop_: for payload_sz in
            IterRange::new(payload_sizes.decay().begin(), payload_sizes.decay().end())
        {
            let payload_sz: Ptr<i64> = payload_sz.iter_deref();
            unimplemented!();
            assert!(!(unimplemented!()));
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<i32> = Rc::new(RefCell::new(
                    (((*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize)))
                        as i32),
                ));
                ({
                    let _condition: bool = (((data
                        .decay()
                        .offset(
                            (*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize))
                                as isize,
                        )
                        .read()) as i32)
                        == (*exp.borrow()));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let previously_written: Ptr<usize> = previously_written.as_pointer();
                            let j: Ptr<()> = j.as_pointer();
                            let payload_sz: Ptr<i64> = (payload_sz).clone();
                            let data: Ptr<Vec<i32>> = data.as_pointer();
                            let exp: Ptr<i32> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Value<u64> = Rc::new(RefCell::new(
                                    ((previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as u64),
                                ));
                                let _args_2: Ptr<i64> = (j).reinterpret_cast::<i64>();
                                let _args_3: Ptr<i64> = (payload_sz).clone();
                                let _args_4: Ptr<i32> = data.decay().offset(
                                    (previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as isize,
                                );
                                let _args_5: Ptr<i32> = (exp).clone();
                                test_eprintf_5 ( Ptr::<i8 >::from_string_literal(b"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" )  , _args_1.as_pointer() , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    )));
                    test_require_26(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"data[previously_written + j] == exp"),
                        Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                        129,
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_written.borrow_mut()) =
                { (*previously_written.borrow()).wrapping_add(((payload_sz.read()) as usize)) };
        }
        unimplemented!();
    }
    {
        let ifs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let in_buffer: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(total_size as usize) as usize)
                .map(|_| 0_i32)
                .collect::<Vec<_>>(),
        ));
        assert!(unimplemented!());
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*in_buffer.borrow()).len()) {
            let exp: Value<i32> = Rc::new(RefCell::new(((*i.borrow()) as i32)));
            ({
                let _condition: bool = (((in_buffer.decay().offset((*i.borrow()) as isize).read())
                    as i32)
                    == (*exp.borrow()));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let i: Ptr<usize> = i.as_pointer();
                        let in_buffer: Ptr<Vec<i32>> = in_buffer.as_pointer();
                        let exp: Ptr<i32> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<u64> = (i).reinterpret_cast::<u64>();
                            let _args_2: Ptr<i32> = in_buffer.decay().offset((i.read()) as isize);
                            test_eprintf_6(
                                Ptr::<i8>::from_string_literal(
                                    b"failed after read() at offset %zu: got=%x, expected=%x\n",
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
                    Ptr::<i8>::from_string_literal(b"in_buffer[i] == exp"),
                    Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                    144,
                    _arg.as_pointer(),
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_write_codecvt_44(policy: LibraryManagedBuffer, payload_sizes: Ptr<Vec<i64>>) {
    let policy: Value<LibraryManagedBuffer> = Rc::new(RefCell::new(policy));
    let previously_written: Value<usize> = Rc::new(RefCell::new(0_usize));
    let mut total_size: () = (unimplemented!() as ());
    let data: Value<Vec<i32>> = Rc::new(RefCell::new(
        (0..(total_size as usize) as usize)
            .map(|_| 0_i32)
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < (*data.borrow()).len()) {
        let __rhs = (i as i32);
        data.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    let p: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_38() })));
    {
        let ofs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        ({
            LibraryManagedBufferImpl::operator_call_2_wchar_t(
                &policy.as_pointer(),
                ofs.as_pointer(),
            )
        });
        unimplemented!();
        assert!(unimplemented!());
        'loop_: for payload_sz in
            IterRange::new(payload_sizes.decay().begin(), payload_sizes.decay().end())
        {
            let payload_sz: Ptr<i64> = payload_sz.iter_deref();
            unimplemented!();
            assert!(!(unimplemented!()));
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<i32> = Rc::new(RefCell::new(
                    (((*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize)))
                        as i32),
                ));
                ({
                    let _condition: bool = (((data
                        .decay()
                        .offset(
                            (*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize))
                                as isize,
                        )
                        .read()) as i32)
                        == (*exp.borrow()));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let previously_written: Ptr<usize> = previously_written.as_pointer();
                            let j: Ptr<()> = j.as_pointer();
                            let payload_sz: Ptr<i64> = (payload_sz).clone();
                            let data: Ptr<Vec<i32>> = data.as_pointer();
                            let exp: Ptr<i32> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Value<u64> = Rc::new(RefCell::new(
                                    ((previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as u64),
                                ));
                                let _args_2: Ptr<i64> = (j).reinterpret_cast::<i64>();
                                let _args_3: Ptr<i64> = (payload_sz).clone();
                                let _args_4: Ptr<i32> = data.decay().offset(
                                    (previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as isize,
                                );
                                let _args_5: Ptr<i32> = (exp).clone();
                                test_eprintf_5 ( Ptr::<i8 >::from_string_literal(b"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" )  , _args_1.as_pointer() , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    )));
                    test_require_31(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"data[previously_written + j] == exp"),
                        Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                        129,
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_written.borrow_mut()) =
                { (*previously_written.borrow()).wrapping_add(((payload_sz.read()) as usize)) };
        }
        unimplemented!();
    }
    {
        let ifs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let in_buffer: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(total_size as usize) as usize)
                .map(|_| 0_i32)
                .collect::<Vec<_>>(),
        ));
        assert!(unimplemented!());
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*in_buffer.borrow()).len()) {
            let exp: Value<i32> = Rc::new(RefCell::new(((*i.borrow()) as i32)));
            ({
                let _condition: bool = (((in_buffer.decay().offset((*i.borrow()) as isize).read())
                    as i32)
                    == (*exp.borrow()));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let i: Ptr<usize> = i.as_pointer();
                        let in_buffer: Ptr<Vec<i32>> = in_buffer.as_pointer();
                        let exp: Ptr<i32> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<u64> = (i).reinterpret_cast::<u64>();
                            let _args_2: Ptr<i32> = in_buffer.decay().offset((i.read()) as isize);
                            test_eprintf_6(
                                Ptr::<i8>::from_string_literal(
                                    b"failed after read() at offset %zu: got=%x, expected=%x\n",
                                ),
                                _args_1,
                                _args_2,
                                (exp).clone(),
                            )
                        });
                    }
                )));
                test_require_32(
                    _condition,
                    Ptr::<i8>::from_string_literal(b"in_buffer[i] == exp"),
                    Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                    144,
                    _arg.as_pointer(),
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_write_codecvt_45(policy: UserManagedBuffer, payload_sizes: Ptr<Vec<i64>>) {
    let policy: Value<UserManagedBuffer> = Rc::new(RefCell::new(policy));
    let previously_written: Value<usize> = Rc::new(RefCell::new(0_usize));
    let mut total_size: () = (unimplemented!() as ());
    let data: Value<Vec<i32>> = Rc::new(RefCell::new(
        (0..(total_size as usize) as usize)
            .map(|_| 0_i32)
            .collect::<Vec<_>>(),
    ));
    let mut i: usize = 0_usize;
    'loop_: while (i < (*data.borrow()).len()) {
        let __rhs = (i as i32);
        data.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    let p: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_38() })));
    {
        let ofs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        ({ UserManagedBufferImpl::operator_call_6(&policy.as_pointer(), ofs.as_pointer()) });
        unimplemented!();
        assert!(unimplemented!());
        'loop_: for payload_sz in
            IterRange::new(payload_sizes.decay().begin(), payload_sizes.decay().end())
        {
            let payload_sz: Ptr<i64> = payload_sz.iter_deref();
            unimplemented!();
            assert!(!(unimplemented!()));
            let j: Value<()> = Rc::new(RefCell::new(0_()));
            'loop_: while ({ (*j.borrow()) } < { (payload_sz.read()) }) {
                let exp: Value<i32> = Rc::new(RefCell::new(
                    (((*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize)))
                        as i32),
                ));
                ({
                    let _condition: bool = (((data
                        .decay()
                        .offset(
                            (*previously_written.borrow()).wrapping_add(((*j.borrow()) as usize))
                                as isize,
                        )
                        .read()) as i32)
                        == (*exp.borrow()));
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let previously_written: Ptr<usize> = previously_written.as_pointer();
                            let j: Ptr<()> = j.as_pointer();
                            let payload_sz: Ptr<i64> = (payload_sz).clone();
                            let data: Ptr<Vec<i32>> = data.as_pointer();
                            let exp: Ptr<i32> = exp.as_pointer();
                        },
                        || {
                            ({
                                let _args_1: Value<u64> = Rc::new(RefCell::new(
                                    ((previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as u64),
                                ));
                                let _args_2: Ptr<i64> = (j).reinterpret_cast::<i64>();
                                let _args_3: Ptr<i64> = (payload_sz).clone();
                                let _args_4: Ptr<i32> = data.decay().offset(
                                    (previously_written.read()).wrapping_add(((j.read()) as usize))
                                        as isize,
                                );
                                let _args_5: Ptr<i32> = (exp).clone();
                                test_eprintf_5 ( Ptr::<i8 >::from_string_literal(b"failed after write() at offset %zu (offset %zu in chunk size %zu): got=%x, expected=%x\n" )  , _args_1.as_pointer() , _args_2 , _args_3 , _args_4 , _args_5 , )
                            });
                        }
                    )));
                    test_require_36(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"data[previously_written + j] == exp"),
                        Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                        129,
                        _arg.as_pointer(),
                    )
                });
                (*j.borrow_mut()).prefix_inc();
            }
            (*previously_written.borrow_mut()) =
                { (*previously_written.borrow()).wrapping_add(((payload_sz.read()) as usize)) };
        }
        unimplemented!();
    }
    {
        let ifs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let in_buffer: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(total_size as usize) as usize)
                .map(|_| 0_i32)
                .collect::<Vec<_>>(),
        ));
        assert!(unimplemented!());
        let i: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while ((*i.borrow()) < (*in_buffer.borrow()).len()) {
            let exp: Value<i32> = Rc::new(RefCell::new(((*i.borrow()) as i32)));
            ({
                let _condition: bool = (((in_buffer.decay().offset((*i.borrow()) as isize).read())
                    as i32)
                    == (*exp.borrow()));
                let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let i: Ptr<usize> = i.as_pointer();
                        let in_buffer: Ptr<Vec<i32>> = in_buffer.as_pointer();
                        let exp: Ptr<i32> = exp.as_pointer();
                    },
                    || {
                        ({
                            let _args_1: Ptr<u64> = (i).reinterpret_cast::<u64>();
                            let _args_2: Ptr<i32> = in_buffer.decay().offset((i.read()) as isize);
                            test_eprintf_6(
                                Ptr::<i8>::from_string_literal(
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
                    Ptr::<i8>::from_string_literal(b"in_buffer[i] == exp"),
                    Ptr::<i8>::from_string_literal(b"buffered_writes.pass.cpp"),
                    144,
                    _arg.as_pointer(),
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    unimplemented!();
}
thread_local!(
    pub static buffer_sizes_46: Value<Vec<i64>> = Rc::new(RefCell::new(Vec::from([
        0_i64, 3_i64, 8_i64, 9_i64, 11_i64,
    ])));
);
thread_local!(
    pub static io_sizes_47: Value<Vec<i64>> = Rc::new(RefCell::new(Vec::from([
        0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 9_i64, 10_i64, 11_i64, 12_i64, 13_i64, 21_i64, 22_i64,
        23_i64,
    ])));
);
thread_local!(
    pub static io_sizes_default_48: Value<Vec<i64>> = Rc::new(RefCell::new(Vec::from([
        0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 4094_i64, 4095_i64, 4096_i64, 4097_i64, 4098_i64,
        8190_i64, 8191_i64, 8192_i64, 8193_i64, 8194_i64,
    ])));
);
pub fn test_1_write_49() {
    'loop_: for x in IterRange::new(
        io_sizes_default_48.with(|v| v.decay()).begin(),
        io_sizes_default_48.with(|v| v.decay()).end(),
    ) {
        let mut x: () = ((x.iter_deref().read()) as ());
        ({
            let _payload_sizes: Value<Vec<i64>> = Rc::new(RefCell::new(Vec::from([(x as i64)])));
            test_write_40(LibraryDefaultBuffer::new(), _payload_sizes.as_pointer())
        });
        ({
            let _payload_sizes: Value<Vec<i64>> = Rc::new(RefCell::new(Vec::from([(x as i64)])));
            test_write_codecvt_43(LibraryDefaultBuffer::new(), _payload_sizes.as_pointer())
        });
    }
    'loop_: for b in IterRange::new(
        buffer_sizes_46.with(|v| v.decay()).begin(),
        buffer_sizes_46.with(|v| v.decay()).end(),
    ) {
        let mut b: () = ((b.iter_deref().read()) as ());
        'loop_: for x in IterRange::new(
            io_sizes_47.with(|v| v.decay()).begin(),
            io_sizes_47.with(|v| v.decay()).end(),
        ) {
            let mut x: () = ((x.iter_deref().read()) as ());
            ({
                let _payload_sizes: Value<Vec<i64>> =
                    Rc::new(RefCell::new(Vec::from([(x as i64)])));
                test_write_41(
                    LibraryManagedBuffer::new({ (b as usize) }),
                    _payload_sizes.as_pointer(),
                )
            });
            ({
                let _payload_sizes: Value<Vec<i64>> =
                    Rc::new(RefCell::new(Vec::from([(x as i64)])));
                test_write_codecvt_44(
                    LibraryManagedBuffer::new({ (b as usize) }),
                    _payload_sizes.as_pointer(),
                )
            });
        }
    }
    'loop_: for b in IterRange::new(
        buffer_sizes_46.with(|v| v.decay()).begin(),
        buffer_sizes_46.with(|v| v.decay()).end(),
    ) {
        let mut b: () = ((b.iter_deref().read()) as ());
        'loop_: for x in IterRange::new(
            io_sizes_47.with(|v| v.decay()).begin(),
            io_sizes_47.with(|v| v.decay()).end(),
        ) {
            let mut x: () = ((x.iter_deref().read()) as ());
            ({
                let _payload_sizes: Value<Vec<i64>> =
                    Rc::new(RefCell::new(Vec::from([(x as i64)])));
                test_write_42(
                    UserManagedBuffer::new({ (b as usize) }),
                    _payload_sizes.as_pointer(),
                )
            });
            ({
                let _payload_sizes: Value<Vec<i64>> =
                    Rc::new(RefCell::new(Vec::from([(x as i64)])));
                test_write_codecvt_45(
                    UserManagedBuffer::new({ (b as usize) }),
                    _payload_sizes.as_pointer(),
                )
            });
        }
    }
}
pub fn test_2_writes_50() {
    'loop_: for a in IterRange::new(
        io_sizes_default_48.with(|v| v.decay()).begin(),
        io_sizes_default_48.with(|v| v.decay()).end(),
    ) {
        let mut a: () = ((a.iter_deref().read()) as ());
        'loop_: for b in IterRange::new(
            io_sizes_default_48.with(|v| v.decay()).begin(),
            io_sizes_default_48.with(|v| v.decay()).end(),
        ) {
            let mut b: () = ((b.iter_deref().read()) as ());
            ({
                let _payload_sizes: Value<Vec<i64>> =
                    Rc::new(RefCell::new(Vec::from([(a as i64), (b as i64)])));
                test_write_40(LibraryDefaultBuffer::new(), _payload_sizes.as_pointer())
            });
            ({
                let _payload_sizes: Value<Vec<i64>> =
                    Rc::new(RefCell::new(Vec::from([(a as i64), (b as i64)])));
                test_write_codecvt_43(LibraryDefaultBuffer::new(), _payload_sizes.as_pointer())
            });
        }
    }
    'loop_: for buf in IterRange::new(
        buffer_sizes_46.with(|v| v.decay()).begin(),
        buffer_sizes_46.with(|v| v.decay()).end(),
    ) {
        let mut buf: () = ((buf.iter_deref().read()) as ());
        'loop_: for a in IterRange::new(
            io_sizes_47.with(|v| v.decay()).begin(),
            io_sizes_47.with(|v| v.decay()).end(),
        ) {
            let mut a: () = ((a.iter_deref().read()) as ());
            'loop_: for b in IterRange::new(
                io_sizes_47.with(|v| v.decay()).begin(),
                io_sizes_47.with(|v| v.decay()).end(),
            ) {
                let mut b: () = ((b.iter_deref().read()) as ());
                ({
                    let _payload_sizes: Value<Vec<i64>> =
                        Rc::new(RefCell::new(Vec::from([(a as i64), (b as i64)])));
                    test_write_41(
                        LibraryManagedBuffer::new({ (buf as usize) }),
                        _payload_sizes.as_pointer(),
                    )
                });
                ({
                    let _payload_sizes: Value<Vec<i64>> =
                        Rc::new(RefCell::new(Vec::from([(a as i64), (b as i64)])));
                    test_write_codecvt_44(
                        LibraryManagedBuffer::new({ (buf as usize) }),
                        _payload_sizes.as_pointer(),
                    )
                });
            }
        }
    }
    'loop_: for buf in IterRange::new(
        buffer_sizes_46.with(|v| v.decay()).begin(),
        buffer_sizes_46.with(|v| v.decay()).end(),
    ) {
        let mut buf: () = ((buf.iter_deref().read()) as ());
        'loop_: for a in IterRange::new(
            io_sizes_47.with(|v| v.decay()).begin(),
            io_sizes_47.with(|v| v.decay()).end(),
        ) {
            let mut a: () = ((a.iter_deref().read()) as ());
            'loop_: for b in IterRange::new(
                io_sizes_47.with(|v| v.decay()).begin(),
                io_sizes_47.with(|v| v.decay()).end(),
            ) {
                let mut b: () = ((b.iter_deref().read()) as ());
                ({
                    let _payload_sizes: Value<Vec<i64>> =
                        Rc::new(RefCell::new(Vec::from([(a as i64), (b as i64)])));
                    test_write_42(
                        UserManagedBuffer::new({ (buf as usize) }),
                        _payload_sizes.as_pointer(),
                    )
                });
                ({
                    let _payload_sizes: Value<Vec<i64>> =
                        Rc::new(RefCell::new(Vec::from([(a as i64), (b as i64)])));
                    test_write_codecvt_45(
                        UserManagedBuffer::new({ (buf as usize) }),
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_1_write_49() });
    ({ test_2_writes_50() });
    return 0;
}
pub trait LibraryDefaultBufferImpl {
    fn operator_call_2_char(&self, _a0: Ptr<()>);
    fn operator_call_2_wchar_t(&self, _a0: Ptr<()>);
}
impl LibraryDefaultBufferImpl for Ptr<LibraryDefaultBuffer> {
    fn operator_call_2_char(&self, _a0: Ptr<()>) {}
    fn operator_call_2_wchar_t(&self, _a0: Ptr<()>) {}
}
pub trait LibraryManagedBufferImpl {
    fn operator_call_2_char(&self, stream: Ptr<()>);
    fn operator_call_2_wchar_t(&self, stream: Ptr<()>);
}
impl LibraryManagedBufferImpl for Ptr<LibraryManagedBuffer> {
    fn operator_call_2_char(&self, stream: Ptr<()>) {
        unimplemented!();
    }
    fn operator_call_2_wchar_t(&self, stream: Ptr<()>) {
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
        {
            let __init = Ptr::alloc_array(
                (0..(*self).with(|__s| __s.size_))
                    .map(|_| 0_i8)
                    .collect::<Box<[i8]>>(),
            );
            field!((*self), buffers_).with_mut(|__v: &mut Vec<Ptr<i8>>| __v.push(__init))
        };
        unimplemented!();
    }
    fn operator_call_5(&self, stream: Ptr<()>) {
        {
            let __init = Ptr::alloc_array(
                (0..(*self).with(|__s| __s.size_))
                    .map(|_| 0_i8)
                    .collect::<Box<[i8]>>(),
            );
            field!((*self), buffers_).with_mut(|__v: &mut Vec<Ptr<i8>>| __v.push(__init))
        };
        unimplemented!();
    }
    fn operator_call_6(&self, stream: Ptr<()>) {
        {
            let __init = Ptr::alloc_array(
                (0..(*self).with(|__s| __s.size_))
                    .map(|_| 0_i32)
                    .collect::<Box<[i32]>>(),
            );
            field!((*self), wbuffers_).with_mut(|__v: &mut Vec<Ptr<i32>>| __v.push(__init))
        };
        unimplemented!();
    }
    fn operator_call_7(&self, stream: Ptr<()>) {
        {
            let __init = Ptr::alloc_array(
                (0..(*self).with(|__s| __s.size_))
                    .map(|_| 0_i32)
                    .collect::<Box<[i32]>>(),
            );
            field!((*self), wbuffers_).with_mut(|__v: &mut Vec<Ptr<i32>>| __v.push(__init))
        };
        unimplemented!();
    }
    fn destructor(&self) {
        'loop_: for p in IterRange::new(
            field_ref!((*self), buffers_).decay().begin(),
            field_ref!((*self), buffers_).decay().end(),
        ) {
            let mut p: Ptr<i8> = (p.iter_deref().read());
            p.delete();
        }
        'loop_: for p in IterRange::new(
            field_ref!((*self), wbuffers_).decay().begin(),
            field_ref!((*self), wbuffers_).decay().end(),
        ) {
            let mut p: Ptr<i32> = (p.iter_deref().read());
            p.delete();
        }
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = buffer_sizes_46.with(|_| ());
    let _ = io_sizes_47.with(|_| ());
    let _ = io_sizes_default_48.with(|_| ());
}
