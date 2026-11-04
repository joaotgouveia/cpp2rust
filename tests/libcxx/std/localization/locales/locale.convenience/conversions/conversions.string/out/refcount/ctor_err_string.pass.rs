extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    const _: () = assert!(true, "!std::is_convertible<std::string, Myconv>::value");;
    const _: () = assert!(true, "std::is_constructible<Myconv, std::string>::value");;
    {
        let myconv : Value<std_wstring_convert_std_codecvt_utf8_wchar_t__1114111__0___wchar_t__std_allocator_wchar_t___std_allocator_char__ > = Rc::new(RefCell::new({ let __tmp_0 : Value<Vec<i8  > > = Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"byte error" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes} )); std_wstring_convert_std_codecvt_utf8_wchar_t__1114111__0___wchar_t__std_allocator_wchar_t___std_allocator_char__ :: new_1 ( { __tmp_0.as_pointer() } , None , )  } )) ;
        let mut bs: Vec<i8> = ({
            (*myconv.borrow())
                .to_bytes_13(Ptr::<i32>::from_string_literal(&[55939 as i32, 0 as i32]))
        });
        assert!(
            Ptr::<i8>::from_string_literal(b"byte error")
                .with_c_str(|__s| bs[..bs.len().saturating_sub(1)] == *__s)
        );
    }
    {
        let myconv : Value<std_wstring_convert_std_codecvt_utf8_wchar_t__1114111__0___wchar_t__std_allocator_wchar_t___std_allocator_char__ > = Rc::new(RefCell::new({ let __tmp_1 : Value<Vec<i8  > > = Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"byte error" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes} ));let __tmp_2 : Value<Vec<i32  > > = Rc::new(RefCell::new({let mut __bytes = Ptr::<i32 >::from_string_literal(&[119 as i32 , 105 as i32 , 100 as i32 , 101 as i32 , 32 as i32 , 101 as i32 , 114 as i32 , 114 as i32 , 111 as i32 , 114 as i32 , 0 as i32 , ] ) .to_c_bytes();
    __bytes.push(0_i32);
    __bytes} )); std_wstring_convert_std_codecvt_utf8_wchar_t__1114111__0___wchar_t__std_allocator_wchar_t___std_allocator_char__ :: new_1 ( { __tmp_1.as_pointer() } , { Some ( __tmp_2.as_pointer() ) } , )  } )) ;
        let mut bs: Vec<i8> = ({
            (*myconv.borrow())
                .to_bytes_13(Ptr::<i32>::from_string_literal(&[55939 as i32, 0 as i32]))
        });
        assert!(
            Ptr::<i8>::from_string_literal(b"byte error")
                .with_c_str(|__s| bs[..bs.len().saturating_sub(1)] == *__s)
        );
        let mut ws: Vec<i32> = ({ (*myconv.borrow()).from_bytes_8((b'\xa5' as i8)) });
        assert!(
            Ptr::<i32>::from_string_literal(&[
                119 as i32, 105 as i32, 100 as i32, 101 as i32, 32 as i32, 101 as i32, 114 as i32,
                114 as i32, 111 as i32, 114 as i32, 0 as i32,
            ])
            .with_c_str(|__s| ws[..ws.len().saturating_sub(1)] == *__s)
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
