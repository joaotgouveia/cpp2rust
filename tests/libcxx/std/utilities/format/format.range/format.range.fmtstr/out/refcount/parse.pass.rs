extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub fn operator_cmp_0(lhs: Ptr<parse_call_validator>, rhs: Ptr<parse_call_validator>) -> () {
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
            operator_cmp_0(
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
            operator_cmp_0(
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for parse_call_validator {}
pub fn get_format_types_1() -> () {
    return unimplemented!();
}
pub fn get_temp_file_name_2() -> () {
    let mut Name: () = unimplemented!();
    let mut FD: () = ({ mkstemp_3((unimplemented!())) });
    if (FD == -1_()) {
        ({ perror_4(Ptr::<()>::from_string_literal(b"mkstemp")) });
        unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub fn glibc_version_less_than_5(mut version: Ptr<()>) -> () {
    let mut test_version: () = unimplemented!();
    let mut n: () = ({ confstr_6((anon_7__CS_GNU_LIBC_VERSION as ()), Ptr::<()>::null(), 0_()) });
    let mut current_version: Ptr<()> =
        Ptr::alloc_array((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>());
    ({
        confstr_6(
            (anon_7__CS_GNU_LIBC_VERSION as ()),
            (current_version).clone(),
            n,
        )
    });
    let result: Value<()> = Rc::new(RefCell::new(
        (({ strverscmp_8((current_version).clone(), unimplemented!()) }) < 0_()),
    ));
    current_version.delete();
    return (*result.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct test_range_format_string_std_string_ {
    #[offset(0)]
    #[byte_size(24)]
    str_: (),
}
impl test_range_format_string_std_string_ {}
impl Default for test_range_format_string_std_string_ {
    fn default() -> Self {
        {
            test_range_format_string_std_string_ {
                str_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct test_range_format_string_std_wstring_ {
    #[offset(0)]
    #[byte_size(24)]
    str_: (),
}
impl test_range_format_string_std_wstring_ {}
impl Default for test_range_format_string_std_wstring_ {
    fn default() -> Self {
        {
            test_range_format_string_std_wstring_ {
                str_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct test_range_format_debug_string_std_string_ {
    #[offset(0)]
    #[byte_size(24)]
    str_: (),
}
impl test_range_format_debug_string_std_string_ {}
impl Default for test_range_format_debug_string_std_string_ {
    fn default() -> Self {
        {
            test_range_format_debug_string_std_string_ {
                str_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct test_range_format_debug_string_std_wstring_ {
    #[offset(0)]
    #[byte_size(24)]
    str_: (),
}
impl test_range_format_debug_string_std_wstring_ {}
impl Default for test_range_format_debug_string_std_wstring_ {
    fn default() -> Self {
        {
            test_range_format_debug_string_std_wstring_ {
                str_: unimplemented!(),
            }
        }
    }
}
pub fn test_parse_9(fmt: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
}
pub fn test_parse_10(fmt: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
}
pub fn test_parse_11(fmt: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
}
pub fn test_parse_12(fmt: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
}
pub fn test_formatters_13(fmt: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    ({ test_parse_9(unimplemented!(), offset) });
    ({ test_parse_10(unimplemented!(), offset) });
}
pub fn test_formatters_14(fmt: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    ({ test_parse_11(unimplemented!(), offset) });
    ({ test_parse_12(unimplemented!(), offset) });
}
pub fn test_char_type_15() {
    ({ test_formatters_13(unimplemented!(), 0_()) });
    ({ test_formatters_13(unimplemented!(), 1_()) });
}
pub fn test_char_type_16() {
    ({ test_formatters_14(unimplemented!(), 0_()) });
    ({ test_formatters_14(unimplemented!(), 1_()) });
}
pub fn test_17() -> () {
    ({ test_char_type_15() });
    ({ test_char_type_16() });
    return true;
}
thread_local!(
    pub static format_kind_18: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static format_kind_19: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static format_kind_20: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static format_kind_21: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);

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
    ({ test_17() });
    const _: () = assert!(true, "test()");;
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
pub trait test_range_format_debug_string_std_string_Impl {
    fn begin(&self) -> ();
    fn end(&self) -> ();
}
impl test_range_format_debug_string_std_string_Impl
    for Ptr<test_range_format_debug_string_std_string_>
{
    fn begin(&self) -> () {
        return unimplemented!();
    }
    fn end(&self) -> () {
        return unimplemented!();
    }
}
pub trait test_range_format_debug_string_std_wstring_Impl {
    fn begin(&self) -> ();
    fn end(&self) -> ();
}
impl test_range_format_debug_string_std_wstring_Impl
    for Ptr<test_range_format_debug_string_std_wstring_>
{
    fn begin(&self) -> () {
        return unimplemented!();
    }
    fn end(&self) -> () {
        return unimplemented!();
    }
}
pub trait test_range_format_string_std_string_Impl {
    fn begin(&self) -> ();
    fn end(&self) -> ();
}
impl test_range_format_string_std_string_Impl for Ptr<test_range_format_string_std_string_> {
    fn begin(&self) -> () {
        return unimplemented!();
    }
    fn end(&self) -> () {
        return unimplemented!();
    }
}
pub trait test_range_format_string_std_wstring_Impl {
    fn begin(&self) -> ();
    fn end(&self) -> ();
}
impl test_range_format_string_std_wstring_Impl for Ptr<test_range_format_string_std_wstring_> {
    fn begin(&self) -> () {
        return unimplemented!();
    }
    fn end(&self) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = format_kind_18.with(|_| ());
    let _ = format_kind_19.with(|_| ());
    let _ = format_kind_20.with(|_| ());
    let _ = format_kind_21.with(|_| ());
}
