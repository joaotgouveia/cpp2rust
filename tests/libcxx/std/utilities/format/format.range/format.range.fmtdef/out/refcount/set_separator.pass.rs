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
pub fn test_format_context_create_0(out_it: (), args: ()) -> () {
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    let args: Value<()> = Rc::new(RefCell::new(args));
    return ({
        __format_context_create_1(
            (*out_it.borrow_mut()).clone(),
            (*args.borrow()).clone(),
            None,
        )
    });
}
pub fn test_format_context_create_2(out_it: (), args: ()) -> () {
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    let args: Value<()> = Rc::new(RefCell::new(args));
    return ({
        __format_context_create_3(
            (*out_it.borrow_mut()).clone(),
            (*args.borrow()).clone(),
            None,
        )
    });
}
pub fn test_setter_4() {
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        ( (*formatter.borrow()) as std___range_default_formatter_std_range_format_sequence__std_vector_int__std_allocator_int____char_ ) . set_separator ( unimplemented!()  , )
    });
    const _: () = assert!(
        true,
        "noexcept(formatter.set_separator(std::basic_string_view<CharT>{{}}))"
    );;
    {
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let format_ctx: Value<()> = Rc::new(RefCell::new(
            ({
                test_format_context_create_0((*out.borrow()).clone(), {
                    let __tmp_0 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char__ > = Rc::new(RefCell::new(( { make_format_args_5 ( ) } )  ));
                    std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new ( { __tmp_0.as_pointer() } , )
                })
            }),
        ));
        ({
            let ___range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ( (*formatter.borrow()) as std___range_default_formatter_std_range_format_sequence__std_vector_int__std_allocator_int____char_ ) . format ( ___range.as_pointer() , format_ctx .as_pointer()  , )
        });
        unimplemented!();
    }
}
pub fn test_setter_6() {
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        ( (*formatter.borrow()) as std___range_default_formatter_std_range_format_sequence__std_vector_int__std_allocator_int____wchar_t_ ) . set_separator ( unimplemented!()  , )
    });
    const _: () = assert!(
        true,
        "noexcept(formatter.set_separator(std::basic_string_view<CharT>{{}}))"
    );;
    {
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let format_ctx: Value<()> = Rc::new(RefCell::new(
            ({
                test_format_context_create_2((*out.borrow()).clone(), {
                    let __tmp_1 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ > = Rc::new(RefCell::new(( { make_format_args_7 ( ) } )  ));
                    std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new ( { __tmp_1.as_pointer() } , )
                })
            }),
        ));
        ({
            let ___range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ( (*formatter.borrow()) as std___range_default_formatter_std_range_format_sequence__std_vector_int__std_allocator_int____wchar_t_ ) . format ( ___range.as_pointer() , format_ctx .as_pointer()  , )
        });
        unimplemented!();
    }
}
pub fn test_8() -> () {
    ({ test_setter_4() });
    ({ test_setter_6() });
    return true;
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
    ({ test_8() });
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
pub fn __cpp2rust_init_globals() {}
