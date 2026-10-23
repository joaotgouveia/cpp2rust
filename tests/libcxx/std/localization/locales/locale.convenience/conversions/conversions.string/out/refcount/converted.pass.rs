extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestHelper_wchar_t__4_ {}
impl TestHelper_wchar_t__4_ {
    pub fn test() {
        const _: () = assert!(true, "(std::is_same<CharT, wchar_t>::value)");;
        {
            let myconv : Value<std_wstring_convert_std_codecvt_utf8_wchar_t__1114111__0___wchar_t__std_allocator_wchar_t___std_allocator_char__ > = Rc::new(RefCell::new(<std_wstring_convert_std_codecvt_utf8_wchar_t__1114111__0___wchar_t__std_allocator_wchar_t___std_allocator_char__  >::default() )) ;
            unimplemented!();
            let mut bs: () = ({
                (*myconv.borrow())
                    .to_bytes_14(Ptr::<()>::from_string_literal(&[262147 as (), 0 as ()]))
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut ws: () = ({
                (*myconv.borrow())
                    .from_bytes_10(Ptr::<()>::from_string_literal(b"\xf1\x80\x80\x83"))
            });
            unimplemented!();
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
    ({ TestHelper_wchar_t__4_::test() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
