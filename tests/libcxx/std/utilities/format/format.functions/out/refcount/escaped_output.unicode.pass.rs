extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(Ptr< i8  >  , to_pconsti8)]
#[cpp_into(Ptr< i32  >  , to_pconsti32)]
#[cpp_into(Ptr< u8  >  , to_pconstu8)]
#[cpp_into(Ptr< u16  >  , to_pconstu16)]
#[cpp_into(Ptr< u32  >  , to_pconstu32)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<i32>,
    #[offset(8)]
    pub wn_: usize,
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<u8>,
    #[offset(24)]
    pub u8n_: usize,
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<u16>,
    #[offset(40)]
    pub u16n_: usize,
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<u32>,
    #[offset(56)]
    pub u32n_: usize,
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<i8>,
    #[offset(72)]
    pub sn_: usize,
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<i32>,
        mut wn: usize,
        mut u8: Ptr<u8>,
        mut u8n: usize,
        mut u16: Ptr<u16>,
        mut u16n: usize,
        mut u32: Ptr<u32>,
        mut u32n: usize,
        mut s: Ptr<i8>,
        mut sn: usize,
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
impl CppMove for MultiStringType {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
pub fn test_eprintf_2(mut fmt: Ptr<i8>, args: Ptr<Ptr<i8>>) {
    {
        let __s = libcc2rs::format_c(
            &fmt.to_rust_string(),
            &Box::<[VaArg]>::from([(args.read()).into()]),
        );
        libcc2rs::c_stderr().with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn test_log_3(condition: Ptr<i8>, file: Ptr<i8>, line: i32, message: Ptr<i8>) {
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
pub fn test_log_4(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
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
pub fn test_log_5(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
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
pub fn test_log_6(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
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
pub fn test_log_7(condition: Ptr<i8>, file: Ptr<i8>, line: i32, functor: Ptr<FnPtr<fn()>>) {
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
pub fn test_require_19(
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
        test_log_4(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_20(
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
        test_log_5(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_21(
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
        test_log_6(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub fn test_require_22(
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
        test_log_7(_condition, _file, _line, _functor)
    });
    std::process::abort();
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
pub fn test_is_high_surrogate_34(mut value: u32) -> bool {
    return (value >= 55296_u32) && (value <= 56319_u32);
}
pub fn test_is_low_surrogate_35(mut value: u32) -> bool {
    return (value >= 56320_u32) && (value <= 57343_u32);
}
pub fn test_is_surrogate_36(mut value: u32) -> bool {
    return (value >= 55296_u32) && (value <= 57343_u32);
}
pub fn test_is_code_point_37(mut value: u32) -> bool {
    return (value <= 1114111_u32);
}
pub fn test_is_scalar_value_38(mut value: u32) -> bool {
    return ({ test_is_code_point_37(value) }) && (!({ test_is_surrogate_36(value) }));
}
thread_local!(
    pub static test_replacement_character_39: Value<u32> = Rc::new(RefCell::new(65533_u32));
);
pub fn test_encode_40(out_it: Ptr<OstreamIterator<i8>>, mut value: u16) {
    if ((value as i32) < 128) {
        unimplemented!();
    } else if ((value as i32) < 2048) {
        unimplemented!();
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_encode_41(out_it: Ptr<OstreamIterator<i8>>, mut value: u32) {
    if ((value & 4294901760_u32) == 0_u32) {
        ({
            let _out_it: Ptr<OstreamIterator<i8>> = (out_it).clone();
            let _value: u16 = (value as u16);
            test_encode_40(_out_it, _value)
        });
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_transcode_42(
    first: RandomAccessIterator<i32>,
    last: RandomAccessIterator<i32>,
    out_it: OstreamIterator<i8>,
) -> OstreamIterator<i8> {
    let first: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(first));
    let last: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(last));
    let out_it: Value<OstreamIterator<i8>> = Rc::new(RefCell::new(out_it));
    'loop_: while (*first.borrow()) != (*last.borrow()) {
        let mut value: u32 = (((*first.borrow_mut()).iter_post_inc().iter_deref().read()) as u32);
        if ({ test_is_code_point_37(value) }) {
            ({ test_encode_41(out_it.as_pointer(), value) });
        } else {
            ({
                test_encode_40(
                    out_it.as_pointer(),
                    (test_replacement_character_39.with(|rc| *rc.borrow()) as u16),
                )
            });
        }
    }
    return (*out_it.borrow()).clone();
}
pub fn test_transcode_43(
    mut first: Ptr<i32>,
    mut last: Ptr<i32>,
    out_it: OstreamIterator<i8>,
) -> OstreamIterator<i8> {
    let out_it: Value<OstreamIterator<i8>> = Rc::new(RefCell::new(out_it));
    'loop_: while ({ (first).clone() } != { (last).clone() }) {
        let mut value: u32 = ((first.postfix_inc().read()) as u32);
        if ({ test_is_code_point_37(value) }) {
            ({ test_encode_41(out_it.as_pointer(), value) });
        } else {
            ({
                test_encode_40(
                    out_it.as_pointer(),
                    (test_replacement_character_39.with(|rc| *rc.borrow()) as u16),
                )
            });
        }
    }
    return (*out_it.borrow()).clone();
}
pub fn operator_shl_44(os: Ptr<::std::fs::File>, loc: Ptr<()>) -> Ptr<::std::fs::File> {
    return {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (os).clone();
                        let _ = unimplemented!().with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                        __s
                    };
                    let _ = __s.write_all(&[(':' as i8) as u8]);
                    __s
                };
                write_int_refcount(&__s, unimplemented!());
                __s
            };
            let _ = __s.write_all(&[(':' as i8) as u8]);
            __s
        };
        write_int_refcount(&__s, unimplemented!());
        __s
    };
}
pub fn test_concat_45(stream: Ptr<::std::fs::File>, value: Ptr<i8>) -> Ptr<::std::fs::File> {
    return {
        let __s = (stream).clone();
        let _ = __s.write_all(&[(value.read()) as u8]);
        __s
    };
}
pub fn test_concat_46(stream: Ptr<::std::fs::File>, value: Ptr<Vec<i8>>) -> Ptr<::std::fs::File> {
    return {
        let __s = (stream).clone();
        let mut __bytes = Vec::new();
        for &__c in
            &(*value.upgrade().deref())[..(*value.upgrade().deref()).len().saturating_sub(1)]
        {
            __c.push_stream_bytes(&mut __bytes);
        }
        let _ = __s.write_all(&__bytes);
        __s
    };
}
pub fn test_concat_47(stream: Ptr<::std::fs::File>, value: Ptr<i8>) -> Ptr<::std::fs::File> {
    return {
        let __s = (stream).clone();
        let _ = (value).with_c_str(|__cs| {
            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
        });
        __s
    };
}
pub fn test_concat_48(stream: Ptr<::std::fs::File>, value: Ptr<()>) -> Ptr<::std::fs::File> {
    return unimplemented!();
}
pub fn test_concat_49(stream: Ptr<::std::fs::File>, value: Ptr<()>) -> Ptr<::std::fs::File> {
    return unimplemented!();
}
pub fn test_concat_50(stream: Ptr<::std::fs::File>, value: Ptr<Vec<i32>>) -> Ptr<::std::fs::File> {
    let b: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
    let e: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
    if (*b.borrow()) != (*e.borrow()) {
        {}
        ({
            test_transcode_42(
                (*b.borrow()).clone(),
                (*e.borrow()).clone(),
                OstreamIterator::new((stream).clone(), Ptr::null()),
            )
        });
    }
    return (stream).clone();
}
pub fn test_concat_51(stream: Ptr<::std::fs::File>, value: Ptr<()>) -> Ptr<::std::fs::File> {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ({ (b).clone() } != { (e).clone() }) {
        {}
        ({
            test_transcode_43(
                (b).reinterpret_cast::<i32>(),
                (e).reinterpret_cast::<i32>(),
                OstreamIterator::new((stream).clone(), Ptr::null()),
            )
        });
    }
    return (stream).clone();
}
pub fn test_concat_52(stream: Ptr<::std::fs::File>, value: Ptr<()>) -> Ptr<::std::fs::File> {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ({ (b).clone() } != { (e).clone() }) {
        {}
        ({
            test_transcode_43(
                (b).reinterpret_cast::<i32>(),
                (e).reinterpret_cast::<i32>(),
                OstreamIterator::new((stream).clone(), Ptr::null()),
            )
        });
    }
    return (stream).clone();
}
pub fn test_concat_message_53(
    args_0: Ptr<i8>,
    args_1: Ptr<()>,
    args_2: Ptr<i8>,
    args_3: Ptr<()>,
    args_4: Ptr<i8>,
    args_5: Ptr<Vec<i8>>,
    args_6: Ptr<i8>,
) -> Vec<i8> {
    {
        let sstr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                let _value: Ptr<i8> = (args_0).clone();
                test_concat_47(_stream, _value)
            });
            {
                ({
                    let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                    let _value: Ptr<()> = (args_1).clone();
                    test_concat_49(_stream, _value)
                });
                {
                    ({
                        let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                        let _value: Ptr<i8> = (args_2).clone();
                        test_concat_47(_stream, _value)
                    });
                    {
                        ({
                            let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                            let _value: Ptr<()> = (args_3).clone();
                            test_concat_48(_stream, _value)
                        });
                        {
                            ({
                                let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                                let _value: Ptr<i8> = (args_4).clone();
                                test_concat_47(_stream, _value)
                            });
                            {
                                ({
                                    let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                                    let _value: Ptr<Vec<i8>> = (args_5).clone();
                                    test_concat_46(_stream, _value)
                                });
                                ({
                                    let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                                    let _value: Ptr<i8> = (args_6).clone();
                                    test_concat_45(_stream, _value)
                                })
                            }
                        }
                    }
                }
            }
        };
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn test_concat_message_54(
    args_0: Ptr<i8>,
    args_1: Ptr<()>,
    args_2: Ptr<i8>,
    args_3: Ptr<()>,
    args_4: Ptr<i8>,
    args_5: Ptr<Vec<i32>>,
    args_6: Ptr<i8>,
) -> Vec<i8> {
    {
        let sstr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                let _value: Ptr<i8> = (args_0).clone();
                test_concat_47(_stream, _value)
            });
            {
                ({
                    let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                    let _value: Ptr<()> = (args_1).clone();
                    test_concat_52(_stream, _value)
                });
                {
                    ({
                        let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                        let _value: Ptr<i8> = (args_2).clone();
                        test_concat_47(_stream, _value)
                    });
                    {
                        ({
                            let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                            let _value: Ptr<()> = (args_3).clone();
                            test_concat_51(_stream, _value)
                        });
                        {
                            ({
                                let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                                let _value: Ptr<i8> = (args_4).clone();
                                test_concat_47(_stream, _value)
                            });
                            {
                                ({
                                    let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                                    let _value: Ptr<Vec<i32>> = (args_5).clone();
                                    test_concat_50(_stream, _value)
                                });
                                ({
                                    let _stream: Ptr<::std::fs::File> = sstr.as_pointer();
                                    let _value: Ptr<i8> = (args_6).clone();
                                    test_concat_45(_stream, _value)
                                })
                            }
                        }
                    }
                }
            }
        };
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
thread_local!(
    pub static test_format_55: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |expected: (), fmt: (), args: Ptr<i8>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i8>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_53(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_19(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i8> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<i8>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i8>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_53(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_20(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i8> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<i8>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i8>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_53(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_21(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i8> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<i8>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i8>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_53(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_22(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i8> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<i8>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i8>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_53(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_23(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i8> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<i32>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i32>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_54(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_24(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i32> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args_2: Ptr<i8>, args_3: Ptr<i8>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i8>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_53(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_25(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i8> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<i8>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i8>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_53(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_26(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i8> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i8>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_53(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_27(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i8> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args_2: Ptr<i32>, args_3: Ptr<i32>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i32>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_54(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_28(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i32> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<i32>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i32>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_54(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_29(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i32> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i32>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_54(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_30(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i32> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<i32>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i32>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_54(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_31(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i32> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<i32>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i32>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_54(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_32(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i32> = unimplemented!();
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<i32>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: bool = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<Vec<i32>> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<i8>> = Rc::new(RefCell::new(
                                    Rc::new(RefCell::new(
                                        ({
                                            let _args_1: Value<()> =
                                                Rc::new(RefCell::new(unimplemented!()));
                                            let _args_6: Value<i8> =
                                                Rc::new(RefCell::new(('\n' as i8)));
                                            test_concat_message_54(
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nFormat string   ",
                                                ),
                                                _args_1.as_pointer(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nExpected output ",
                                                ),
                                                (expected).clone(),
                                                Ptr::<i8>::from_string_literal(
                                                    b"\nActual output   ",
                                                ),
                                                (out).clone(),
                                                _args_6.as_pointer(),
                                            )
                                        }),
                                    ))
                                    .decay(),
                                ));
                                test_eprintf_2(
                                    Ptr::<i8>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_33(
                        _condition,
                        Ptr::<i8>::from_string_literal(b"out == expected"),
                        Ptr::<i8>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50,
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: Vec<i32> = unimplemented!();
                assert!(unimplemented!());
            }
        }
    )));
);
thread_local!(
    pub static test_format_to_56: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(
                    vec![(' ' as i8); (unimplemented!() as usize) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i8))
                        .collect(),
                ));
                let it: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new(unimplemented!()));
                assert!((*it.borrow()) == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(unimplemented!());
            }
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(
                    vec![(' ' as i8); (unimplemented!() as usize) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i8))
                        .collect(),
                ));
                let it: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new(unimplemented!()));
                assert!((*it.borrow()) == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(unimplemented!());
            }
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                assert!(unimplemented!());
            }
            {
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(Vec::new()));
                unimplemented!();
                assert!(unimplemented!());
            }
            {
                assert!(
                    (unimplemented!() < 4096_())
                        && (!(Ptr::<i8>::from_string_literal(b"Update the size of the buffer."))
                            .is_null())
                );
                let out: Value<Box<[i8]>> =
                    Rc::new(RefCell::new((0..4096).map(|_| 0_i8).collect::<Box<[i8]>>()));
                let mut it: Ptr<i8> = unimplemented!();
                assert!(({ unimplemented!() } == { ((unimplemented!() as i32) as isize) }));
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(
                    vec![((' ' as i8) as i32); (unimplemented!() as usize) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i32))
                        .collect(),
                ));
                let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!((*it.borrow()) == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(unimplemented!());
            }
            {
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(
                    vec![((' ' as i8) as i32); (unimplemented!() as usize) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i32))
                        .collect(),
                ));
                let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!((*it.borrow()) == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(unimplemented!());
            }
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                assert!(unimplemented!());
            }
            {
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
                unimplemented!();
                assert!(unimplemented!());
            }
            {
                assert!(
                    (unimplemented!() < 4096_())
                        && (!(Ptr::<i8>::from_string_literal(b"Update the size of the buffer."))
                            .is_null())
                );
                let out: Value<Box<[i32]>> = Rc::new(RefCell::new(
                    (0..4096).map(|_| 0_i32).collect::<Box<[i32]>>(),
                ));
                let mut it: Ptr<i32> = unimplemented!();
                assert!(({ unimplemented!() } == { ((unimplemented!() as i32) as isize) }));
                assert!(unimplemented!());
            }
        }
    )));
);
thread_local!(
    pub static test_formatted_size_57: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let mut size: usize = unimplemented!();
                assert!((size == (unimplemented!() as usize)));
            }
            {
                let mut size: usize = unimplemented!();
                assert!((size == (unimplemented!() as usize)));
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let mut size: usize = unimplemented!();
                assert!((size == (unimplemented!() as usize)));
            }
            {
                let mut size: usize = unimplemented!();
                assert!((size == (unimplemented!() as usize)));
            }
        }
    )));
);
thread_local!(
    pub static test_format_to_n_58: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let mut n: usize = (unimplemented!() as usize);
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(
                    vec![(' ' as i8); (n) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i8))
                        .collect(),
                ));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                assert!(
                    ({ (({ (*result.borrow()).size } as isize) == (unimplemented!() as isize)) })
                );
                assert!((*result.borrow()).out == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(unimplemented!());
            }
            {
                let mut n: usize = (unimplemented!() as usize);
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(
                    vec![(' ' as i8); (n) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i8))
                        .collect(),
                ));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                assert!(
                    ({ (({ (*result.borrow()).size } as isize) == (unimplemented!() as isize)) })
                );
                assert!((*result.borrow()).out == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(unimplemented!());
            }
            {
                let mut n: isize = 0_isize;
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                assert!(
                    ({ (({ (*result.borrow()).size } as isize) == (unimplemented!() as isize)) })
                );
                assert!((*result.borrow()).out == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(({ (*out.borrow()).len() <= 1 }));
            }
            {
                let mut n: isize = (((unimplemented!()).wrapping_div(2_())) as isize);
                let out: Value<Vec<i8>> = Rc::new(RefCell::new(
                    vec![(' ' as i8); (n as usize) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i8))
                        .collect(),
                ));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                assert!(
                    ({ (({ (*result.borrow()).size } as isize) == (unimplemented!() as isize)) })
                );
                assert!((*result.borrow()).out == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(unimplemented!());
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let mut n: usize = (unimplemented!() as usize);
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(
                    vec![((' ' as i8) as i32); (n) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i32))
                        .collect(),
                ));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                assert!(
                    ({ (({ (*result.borrow()).size } as isize) == (unimplemented!() as isize)) })
                );
                assert!((*result.borrow()).out == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(unimplemented!());
            }
            {
                let mut n: usize = (unimplemented!() as usize);
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(
                    vec![((' ' as i8) as i32); (n) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i32))
                        .collect(),
                ));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                assert!(
                    ({ (({ (*result.borrow()).size } as isize) == (unimplemented!() as isize)) })
                );
                assert!((*result.borrow()).out == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(unimplemented!());
            }
            {
                let mut n: isize = 0_isize;
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(vec![i32::default()]));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                assert!(
                    ({ (({ (*result.borrow()).size } as isize) == (unimplemented!() as isize)) })
                );
                assert!((*result.borrow()).out == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(({ (*out.borrow()).len() <= 1 }));
            }
            {
                let mut n: isize = (((unimplemented!()).wrapping_div(2_())) as isize);
                let out: Value<Vec<i32>> = Rc::new(RefCell::new(
                    vec![((' ' as i8) as i32); (n as usize) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i32))
                        .collect(),
                ));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                assert!(
                    ({ (({ (*result.borrow()).size } as isize) == (unimplemented!() as isize)) })
                );
                assert!((*result.borrow()).out == RandomAccessIterator::Vec(out.decay().to_last()));
                assert!(unimplemented!());
            }
        }
    )));
);
pub fn test_char_59() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: Value<i8> = Rc::new(RefCell::new(('\'' as i8)));
        let _args_3: Value<i8> = Rc::new(RefCell::new(('\"' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>, Ptr<i8>)>(6)
            .call(_expected, _fmt, _args_2.as_pointer(), _args_3.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('\t' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('\n' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('\r' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('\\' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('\'' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('\"' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new((' ' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('a' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('b' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('c' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('\0' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i8> = Rc::new(RefCell::new(('\x1f' as i8)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            let _args: Value<i8> = Rc::new(RefCell::new((b'\x80' as i8)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(7)
                .call(_expected, _fmt, _args.as_pointer())
        });
    }
    {}
    {}
}
pub fn test_char_60() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: Value<i32> = Rc::new(RefCell::new((('\'' as i8) as i32)));
        let _args_3: Value<i32> = Rc::new(RefCell::new((('\"' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>, Ptr<i32>)>(9)
            .call(_expected, _fmt, _args_2.as_pointer(), _args_3.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('\t' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('\n' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('\r' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('\\' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('\'' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('\"' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new(((' ' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('a' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('b' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('c' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('\0' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<i32> = Rc::new(RefCell::new((('\x1f' as i8) as i32)));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    {
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((160 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((12288 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((8232 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((8233 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((173 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((1536 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((65279 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((55296 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((57343 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((57344 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((63743 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((888 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((6004 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((65535 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((768 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((65056 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
    }
    {
        const _: () = assert!(
            (::std::mem::size_of::<i32>() == 4_usize),
            "sizeof(CharT) == 4"
        );;
        let mut x: i32 = 131071;
        let mut y: u32 = 131071_u32;
        const _: () = assert!(true, "x == y");;
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((69821 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((917631 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((983040 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((1048573 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((1048576 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((1114109 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((65548 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((1048575 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((1114110 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((66045 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((917760 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((1114112 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<i32> = Rc::new(RefCell::new((4294967295 as i32)));
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
    }
}
pub fn test_string_61() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(0)
                .call(_expected, _fmt, Ptr::<i8>::from_string_literal(b"\xc3"))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(1)
                .call(_expected, _fmt, Ptr::<i8>::from_string_literal(b"\xc3("))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(1)
                .call(_expected, _fmt, Ptr::<i8>::from_string_literal(b"\xc0\x80"))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(1)
                .call(_expected, _fmt, Ptr::<i8>::from_string_literal(b"\xc1\xbf"))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(1)
                .call(_expected, _fmt, Ptr::<i8>::from_string_literal(b"\xc2\x80"))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xe0\x80\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xe0\x81\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xe0\x82\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xe0\x9f\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xe0\xa0\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xed\x9f\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xed\xa0\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xed\xaf\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xed\xbf\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xed\xbf\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xee\x80\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xf0\x80\x80\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xf0\x80\x81\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xf0\x80\x82\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xf0\x80\x9f\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xf0\x80\xa0\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xf0\x8f\xbf\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xf0\x90\x80\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xf4\x8f\xbf\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xf4\x90\x80\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i8>::from_string_literal(b"\xf4\xbf\xbf\xbf"),
                )
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i8>)>(0)
                .call(_expected, _fmt, Ptr::<i8>::from_string_literal(b"\x80"))
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    {}
}
pub fn test_string_62() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(5)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i32>::from_string_literal(&[195 as i32, 0 as i32]),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(12)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<i32>::from_string_literal(&[195 as i32, 40 as i32, 0 as i32]),
                )
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[160 as i32, 12288 as i32, 0 as i32]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(5)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[8232 as i32, 0 as i32]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(5)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[8233 as i32, 0 as i32]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[
                        173 as i32,
                        1536 as i32,
                        65279 as i32,
                        0 as i32,
                    ]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(5)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[55296 as i32, 0 as i32]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[57344 as i32, 63743 as i32, 0 as i32]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[
                        888 as i32,
                        6004 as i32,
                        65535 as i32,
                        0 as i32,
                    ]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[768 as i32, 65056 as i32, 0 as i32]),
                )
        });
    }
    {
        const _: () = assert!(
            (::std::mem::size_of::<i32>() == 4_usize),
            "sizeof(CharT) == 4"
        );;
        let mut x: i32 = 131071;
        let mut y: u32 = 131071_u32;
        const _: () = assert!(true, "x == y");;
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[69821 as i32, 917631 as i32, 0 as i32]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(14)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[
                        983040 as i32,
                        1048573 as i32,
                        1048576 as i32,
                        1114109 as i32,
                        0 as i32,
                    ]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[
                        65548 as i32,
                        1048575 as i32,
                        1114110 as i32,
                        0 as i32,
                    ]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[66045 as i32, 917760 as i32, 0 as i32]),
                )
        });
        ({
            (*test_format_55.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<i32>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<i32>::from_string_literal(&[1114112 as i32, 4294967295 as i32, 0 as i32]),
                )
        });
    }
}
pub fn test_format_functions_63(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_64(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_65(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_66(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_67(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_68(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_69(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_70(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_71() {
    ({ test_char_59() });
    ({ test_string_61() });
    ({ test_format_functions_63((*test_format_55.with(Value::clone).borrow()).copy_from()) });
    ({ test_format_functions_64((*test_format_to_56.with(Value::clone).borrow()).copy_from()) });
    ({
        test_format_functions_65((*test_formatted_size_57.with(Value::clone).borrow()).copy_from())
    });
    ({ test_format_functions_66((*test_format_to_n_58.with(Value::clone).borrow()).copy_from()) });
}
pub fn test_72() {
    ({ test_char_60() });
    ({ test_string_62() });
    ({ test_format_functions_67((*test_format_55.with(Value::clone).borrow()).copy_from()) });
    ({ test_format_functions_68((*test_format_to_56.with(Value::clone).borrow()).copy_from()) });
    ({
        test_format_functions_69((*test_formatted_size_57.with(Value::clone).borrow()).copy_from())
    });
    ({ test_format_functions_70((*test_format_to_n_58.with(Value::clone).borrow()).copy_from()) });
}
pub fn test_ill_formed_utf8_73() {
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xdf"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xef"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xef\xbf"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xf7"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xf7\xbf"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xf7\xbf\xbf"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xdfa"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xefa"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xef\xbfa"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xf7a"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xf7\xbfa"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xf7\xbf\xbfa"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(4)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"a\xf1\x80\x80\xe1\x80\xc2b"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xf4\x8f\xbf\xbf"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xf4\x90\x80\x80"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xf5\xb1\xb2\xb3"),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i8>)>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i8>::from_string_literal(b"\xf7\xbf\xbf\xbf"),
            )
    });
}
pub fn test_ill_formed_utf32_74() {
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i32>::from_string_literal(&[1114111 as i32, 0 as i32]),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i32>::from_string_literal(&[1114112 as i32, 0 as i32]),
            )
    });
    ({
        (*test_format_55.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<i32>)>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<i32>::from_string_literal(&[4294967295 as i32, 0 as i32]),
            )
    });
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
    ({ test_71() });
    ({ test_72() });
    ({ test_ill_formed_utf8_73() });
    ({ test_ill_formed_utf32_74() });
    return 0;
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<i8>) -> Ptr<i8>;
    fn as_ptr_3(&self, _a0: Ptr<i32>) -> Ptr<i32>;
    fn as_ptr_4(&self, _a0: Ptr<u8>) -> Ptr<u8>;
    fn as_ptr_5(&self, _a0: Ptr<u16>) -> Ptr<u16>;
    fn as_ptr_6(&self, _a0: Ptr<u32>) -> Ptr<u32>;
    fn length_7(&self, _a0: Ptr<i8>) -> usize;
    fn length_8(&self, _a0: Ptr<i32>) -> usize;
    fn length_9(&self, _a0: Ptr<u8>) -> usize;
    fn length_10(&self, _a0: Ptr<u16>) -> usize;
    fn length_11(&self, _a0: Ptr<u32>) -> usize;
    fn to_pconsti8(&self) -> Ptr<i8>;
    fn to_pconsti32(&self) -> Ptr<i32>;
    fn to_pconstu8(&self) -> Ptr<u8>;
    fn to_pconstu16(&self) -> Ptr<u16>;
    fn to_pconstu32(&self) -> Ptr<u32>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<i8>) -> Ptr<i8> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<i32>) -> Ptr<i32> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<u8>) -> Ptr<u8> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<u16>) -> Ptr<u16> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<u32>) -> Ptr<u32> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<i8>) -> usize {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<i32>) -> usize {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<u8>) -> usize {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<u16>) -> usize {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<u32>) -> usize {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconsti8(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconsti32(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconstu8(&self) -> Ptr<u8> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconstu16(&self) -> Ptr<u16> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconstu32(&self) -> Ptr<u32> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = test_replacement_character_39.with(|_| ());
    let _ = test_format_55.with(|_| ());
    let _ = test_format_to_56.with(|_| ());
    let _ = test_formatted_size_57.with(|_| ());
    let _ = test_format_to_n_58.with(|_| ());
}
