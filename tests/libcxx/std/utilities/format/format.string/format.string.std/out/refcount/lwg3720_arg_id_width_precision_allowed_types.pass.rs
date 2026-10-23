extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub fn test_log_1(condition: Ptr<()>, file: Ptr<()>, line: (), message: Ptr<()>) {
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
pub fn test_is_high_surrogate_2(mut value: ()) -> () {
    return (value >= 55296_()) && (value <= 56319_());
}
pub fn test_is_low_surrogate_3(mut value: ()) -> () {
    return (value >= 56320_()) && (value <= 57343_());
}
pub fn test_is_surrogate_4(mut value: ()) -> () {
    return (value >= 55296_()) && (value <= 57343_());
}
pub fn test_is_code_point_5(mut value: ()) -> () {
    return (value <= 1114111_());
}
pub fn test_is_scalar_value_6(mut value: ()) -> () {
    return ({ test_is_code_point_5(value) }) && (!({ test_is_surrogate_4(value) }));
}
thread_local!(
    pub static test_replacement_character_7: Value<()> = Rc::new(RefCell::new((65533 as ())));
);
pub fn operator_shl_8(os: Ptr<()>, loc: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct context_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct context_wchar_t_ {}
pub type status = u16;
pub const status_foo: status = 43690;
pub const status_bar: status = 21845;
pub const status_foobar: status = 43605;
pub fn operator_cmp_9(lhs: Ptr<parse_call_validator>, rhs: Ptr<parse_call_validator>) -> () {
    return std::cmp::Ord::cmp(&(lhs), &(rhs));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct parse_call_validator_parse_function_not_called {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct parse_call_validator {}
impl std::cmp::Ord for parse_call_validator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_9(
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for parse_call_validator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for parse_call_validator {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_cmp_9(
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for parse_call_validator {}
pub fn get_format_types_10() -> () {
    return unimplemented!();
}
pub fn test_exception_11(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_12(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_13(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_14(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_15(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_16(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_17(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_18(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_19(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_20(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_21(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_22(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_23(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_24(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_25(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_26(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_27(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_28(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_29(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_30(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_31(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_32(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_33(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_34(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_35(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_36(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_37(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_38(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_39(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_exception_40(fmt: (), args_1: Ptr<()>, args_2: Ptr<()>) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let what: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_41() {
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(true));
        test_exception_11(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(('0' as ())));
        test_exception_12(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    {}
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(0_()));
        test_exception_13(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(0_()));
        test_exception_14(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f32));
        test_exception_15(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f64));
        test_exception_16(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f64));
        test_exception_17(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(true));
        test_exception_18(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(('0' as ())));
        test_exception_19(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    {}
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(0_()));
        test_exception_20(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(0_()));
        test_exception_21(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f32));
        test_exception_22(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f64));
        test_exception_23(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f64));
        test_exception_24(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
}
pub fn test_42() {
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(true));
        test_exception_25(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(('0' as ())));
        test_exception_26(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    {
        ({
            let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
            let _args_2: Value<()> = Rc::new(RefCell::new((48 as ())));
            test_exception_27(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
        });
    }
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(0_()));
        test_exception_28(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(0_()));
        test_exception_29(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f32));
        test_exception_30(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f64));
        test_exception_31(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_()));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f64));
        test_exception_32(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(true));
        test_exception_33(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(('0' as ())));
        test_exception_34(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    {
        ({
            let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
            let _args_2: Value<()> = Rc::new(RefCell::new((48 as ())));
            test_exception_35(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
        });
    }
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(0_()));
        test_exception_36(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(0_()));
        test_exception_37(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f32));
        test_exception_38(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f64));
        test_exception_39(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
    });
    ({
        let _args_1: Value<()> = Rc::new(RefCell::new(42_f64));
        let _args_2: Value<()> = Rc::new(RefCell::new(42_f64));
        test_exception_40(unimplemented!(), _args_1.as_pointer(), _args_2.as_pointer())
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_41() });
    ({ test_42() });
    return 0_();
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = test_replacement_character_7.with(|_| ());
}
