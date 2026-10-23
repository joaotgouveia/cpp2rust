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
pub fn operator_cmp_2(lhs: Ptr<parse_call_validator>, rhs: Ptr<parse_call_validator>) -> () {
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
            operator_cmp_2(
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
            operator_cmp_2(
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for parse_call_validator {}
pub fn get_format_types_3() -> () {
    return unimplemented!();
}
pub fn test_format_context_create_4(out_it: (), args: ()) -> () {
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    let args: Value<()> = Rc::new(RefCell::new(args));
    return ({
        __format_context_create_5(
            (*out_it.borrow_mut()).clone(),
            (*args.borrow()).clone(),
            None,
        )
    });
}
pub fn test_format_context_create_6(out_it: (), args: ()) -> () {
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    let args: Value<()> = Rc::new(RefCell::new(args));
    return ({
        __format_context_create_7(
            (*out_it.borrow_mut()).clone(),
            (*args.borrow()).clone(),
            None,
        )
    });
}
pub fn test_8(expected: (), arg: ()) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_4((*out.borrow()).clone(), {
                let __tmp_0 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___std_tuple_int__ > = Rc::new(RefCell::new(( { make_format_args_9 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_1 ( { __tmp_0.as_pointer() } , )
            })
        }),
    ));
    unimplemented!();
    unimplemented!();
}
pub fn test_10(expected: (), arg: ()) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_4((*out.borrow()).clone(), {
                let __tmp_1 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___std_tuple_int__char__ > = Rc::new(RefCell::new(( { make_format_args_11 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_2 ( { __tmp_1.as_pointer() } , )
            })
        }),
    ));
    unimplemented!();
    unimplemented!();
}
pub fn test_12(expected: (), arg: ()) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_4((*out.borrow()).clone(), {
                let __tmp_2 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___std_pair_int__char__ > = Rc::new(RefCell::new(( { make_format_args_13 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_3 ( { __tmp_2.as_pointer() } , )
            })
        }),
    ));
    unimplemented!();
    unimplemented!();
}
pub fn test_14(expected: (), arg: ()) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_4((*out.borrow()).clone(), {
                let __tmp_3 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___std_tuple_int__char__bool__ > = Rc::new(RefCell::new(( { make_format_args_15 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_4 ( { __tmp_3.as_pointer() } , )
            })
        }),
    ));
    unimplemented!();
    unimplemented!();
}
pub fn test_16(expected: (), arg: ()) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_6((*out.borrow()).clone(), {
                let __tmp_4 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___std_tuple_int__ > = Rc::new(RefCell::new(( { make_format_args_17 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_1 ( { __tmp_4.as_pointer() } , )
            })
        }),
    ));
    unimplemented!();
    unimplemented!();
}
pub fn test_18(expected: (), arg: ()) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_6((*out.borrow()).clone(), {
                let __tmp_5 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___std_tuple_int__wchar_t__ > = Rc::new(RefCell::new(( { make_format_args_19 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_2 ( { __tmp_5.as_pointer() } , )
            })
        }),
    ));
    unimplemented!();
    unimplemented!();
}
pub fn test_20(expected: (), arg: ()) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_6((*out.borrow()).clone(), {
                let __tmp_6 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___std_pair_int__wchar_t__ > = Rc::new(RefCell::new(( { make_format_args_21 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_3 ( { __tmp_6.as_pointer() } , )
            })
        }),
    ));
    unimplemented!();
    unimplemented!();
}
pub fn test_22(expected: (), arg: ()) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_6((*out.borrow()).clone(), {
                let __tmp_7 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___std_tuple_int__wchar_t__bool__ > = Rc::new(RefCell::new(( { make_format_args_23 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_4 ( { __tmp_7.as_pointer() } , )
            })
        }),
    ));
    unimplemented!();
    unimplemented!();
}
pub fn test_assure_parse_is_called_24(fmt: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_4((*out.borrow()).clone(), {
                let __tmp_8 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___std_pair_parse_call_validator__parse_call_validator__ > = Rc::new(RefCell::new(( { make_format_args_25 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_5 ( { __tmp_8.as_pointer() } , )
            })
        }),
    ));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_assure_parse_is_called_26(fmt: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_6((*out.borrow()).clone(), {
                let __tmp_9 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___std_pair_parse_call_validator__parse_call_validator__ > = Rc::new(RefCell::new(( { make_format_args_27 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_5 ( { __tmp_9.as_pointer() } , )
            })
        }),
    ));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_assure_parse_is_called_28() {
    let arg: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_4((*out.borrow()).clone(), {
                let __tmp_10 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___std_pair_parse_call_validator__parse_call_validator__ > = Rc::new(RefCell::new(( { make_format_args_25 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_5 ( { __tmp_10.as_pointer() } , )
            })
        }),
    ));
    {
        let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    ({ test_assure_parse_is_called_24(unimplemented!()) });
    ({ test_assure_parse_is_called_24(unimplemented!()) });
    ({ test_assure_parse_is_called_24(unimplemented!()) });
    ({ test_assure_parse_is_called_24(unimplemented!()) });
    ({ test_assure_parse_is_called_24(unimplemented!()) });
}
pub fn test_assure_parse_is_called_29() {
    let arg: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_6((*out.borrow()).clone(), {
                let __tmp_11 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___std_pair_parse_call_validator__parse_call_validator__ > = Rc::new(RefCell::new(( { make_format_args_27 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_5 ( { __tmp_11.as_pointer() } , )
            })
        }),
    ));
    {
        let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    ({ test_assure_parse_is_called_26(unimplemented!()) });
    ({ test_assure_parse_is_called_26(unimplemented!()) });
    ({ test_assure_parse_is_called_26(unimplemented!()) });
    ({ test_assure_parse_is_called_26(unimplemented!()) });
    ({ test_assure_parse_is_called_26(unimplemented!()) });
}
pub fn test_30() {
    ({ test_8(unimplemented!(), 1_()) });
    ({
        test_10(unimplemented!(), {
            let __tmp_12: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_13: Value<()> = Rc::new(RefCell::new(('1' as ())));
            std_tuple_int__char_::new({ __tmp_12.as_pointer() }, { __tmp_13.as_pointer() })
        })
    });
    ({ test_12(unimplemented!(), unimplemented!()) });
    ({
        test_14(unimplemented!(), {
            let __tmp_14: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_15: Value<()> = Rc::new(RefCell::new(('1' as ())));
            let __tmp_16: Value<()> = Rc::new(RefCell::new(true));
            std_tuple_int__char__bool_::new({ __tmp_14.as_pointer() }, { __tmp_15.as_pointer() }, {
                __tmp_16.as_pointer()
            })
        })
    });
    ({ test_assure_parse_is_called_28() });
}
pub fn test_31() {
    ({ test_16(unimplemented!(), 1_()) });
    ({
        test_18(unimplemented!(), {
            let __tmp_17: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_18: Value<()> = Rc::new(RefCell::new(('1' as ())));
            std_tuple_int__wchar_t_::new({ __tmp_17.as_pointer() }, { __tmp_18.as_pointer() })
        })
    });
    ({ test_20(unimplemented!(), unimplemented!()) });
    ({
        test_22(unimplemented!(), {
            let __tmp_19: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_20: Value<()> = Rc::new(RefCell::new(('1' as ())));
            let __tmp_21: Value<()> = Rc::new(RefCell::new(true));
            std_tuple_int__wchar_t__bool_::new(
                { __tmp_19.as_pointer() },
                { __tmp_20.as_pointer() },
                { __tmp_21.as_pointer() },
            )
        })
    });
    ({ test_assure_parse_is_called_29() });
}
pub fn test_32() {
    ({ test_30() });
    ({ test_31() });
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
    ({ test_32() });
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
pub fn __cpp2rust_init_globals() {}
