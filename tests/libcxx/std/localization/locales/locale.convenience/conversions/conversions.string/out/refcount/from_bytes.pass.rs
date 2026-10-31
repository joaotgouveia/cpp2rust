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
            let myconv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let bs: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"\xf1\x80\x80\x83").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            let ws: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
            assert!(
                Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32,]).with_c_str(|__s| (*ws
                    .borrow())[..(*ws.borrow()).len().saturating_sub(1)]
                    == *__s)
            );
            ws.as_pointer().write(unimplemented!());
            assert!(
                Ptr::<i32>::from_string_literal(&[262147 as i32, 0 as i32,]).with_c_str(|__s| (*ws
                    .borrow())[..(*ws.borrow()).len().saturating_sub(1)]
                    == *__s)
            );
            ws.as_pointer().write(unimplemented!());
            assert!(
                Ptr::<i32>::from_string_literal(&[262147 as i32, 0 as i32,]).with_c_str(|__s| (*ws
                    .borrow())[..(*ws.borrow()).len().saturating_sub(1)]
                    == *__s)
            );
            ws.as_pointer().write(unimplemented!());
            assert!(
                Ptr::<i32>::from_string_literal(&[262147 as i32, 0 as i32,]).with_c_str(|__s| (*ws
                    .borrow())[..(*ws.borrow()).len().saturating_sub(1)]
                    == *__s)
            );
            ws.as_pointer().write(unimplemented!());
            assert!((((*ws.borrow()).len() - 1) == 0_usize));
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
    ({ TestHelper_wchar_t__4_::test() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
