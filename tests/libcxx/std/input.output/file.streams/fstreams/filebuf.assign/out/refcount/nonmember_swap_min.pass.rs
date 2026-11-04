extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_temp_file_name_0() -> Vec<i8> {
    let Name: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"libcxx.XXXXXX").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let mut FD: i32 = ({ mkstemp_1((Name.decay().offset(0_usize as isize))) });
    if (FD == -1_i32) {
        ({ perror_2(Ptr::<i8>::from_string_literal(b"mkstemp")) });
        std::process::abort();
    }
    FdRegistry::close(FD);
    return ({ std::mem::take(&mut (*Name.borrow_mut())) });
}
pub fn glibc_version_less_than_3(mut version: Ptr<i8>) -> bool {
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
    let mut n: usize = ({
        confstr_4(
            (anon_5__CS_GNU_LIBC_VERSION as i32),
            Ptr::<i8>::null(),
            0_usize,
        )
    });
    let mut current_version: Ptr<i8> =
        Ptr::alloc_array((0..n).map(|_| 0_i8).collect::<Box<[i8]>>());
    ({
        confstr_4(
            (anon_5__CS_GNU_LIBC_VERSION as i32),
            (current_version).clone(),
            n,
        )
    });
    let result: Value<bool> = Rc::new(RefCell::new(
        (({ strverscmp_6((current_version).clone(), test_version.decay()) }) < 0),
    ));
    current_version.delete();
    return (*result.borrow());
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
    let tmpA: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_0() })));
    let tmpB: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_0() })));
    {
        let sa: Value<::std::fs::File> = Rc::new(RefCell::new(
            std_basic_ofstream_char__std_char_traits_char__::new_1({ tmpA.as_pointer() }, None),
        ));
        let sb: Value<::std::fs::File> = Rc::new(RefCell::new(
            std_basic_ofstream_char__std_char_traits_char__::new_1({ tmpB.as_pointer() }, None),
        ));
        {
            let __s = sa.as_pointer();
            let _ = Ptr::<i8>::from_string_literal(b"AAAA").with_c_str(|__cs| {
                __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
            });
            __s
        };
        {
            let __s = sb.as_pointer();
            let _ = Ptr::<i8>::from_string_literal(b"BBBB").with_c_str(|__cs| {
                __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
            });
            __s
        };
    }
    let f1: Value<std_basic_filebuf_char__std_char_traits_char__> = Rc::new(RefCell::new(
        <std_basic_filebuf_char__std_char_traits_char__>::default(),
    ));
    assert!(({ !(({ (*f1.borrow()).open_12(tmpA.as_pointer(), 8_i32,) }).is_null()) }));
    assert!(({ ({ (*f1.borrow()).is_open() }) }));
    ({ ((*f1.borrow()) as ::std::fs::File).pubsetbuf(Ptr::<i8>::null(), 0_i64) });
    let f2: Value<std_basic_filebuf_char__std_char_traits_char__> = Rc::new(RefCell::new(
        <std_basic_filebuf_char__std_char_traits_char__>::default(),
    ));
    assert!(({ !(({ (*f2.borrow()).open_12(tmpB.as_pointer(), 8_i32,) }).is_null()) }));
    assert!(({ ({ (*f2.borrow()).is_open() }) }));
    ({ ((*f2.borrow()) as ::std::fs::File).pubsetbuf(Ptr::<i8>::null(), 0_i64) });
    assert!(({ (({ ((*f1.borrow()) as ::std::fs::File).sgetc() }) == (('A' as i8) as i32)) }));
    assert!(({ (({ ((*f2.borrow()) as ::std::fs::File).sgetc() }) == (('B' as i8) as i32)) }));
    cpp_swap!(f1.as_pointer(), f2.as_pointer());
    assert!(({ ({ (*f1.borrow()).is_open() }) }));
    assert!(({ ({ (*f2.borrow()).is_open() }) }));
    assert!(({ (({ ((*f1.borrow()) as ::std::fs::File).sgetc() }) == (('B' as i8) as i32)) }));
    assert!(({ (({ ((*f2.borrow()) as ::std::fs::File).sgetc() }) == (('A' as i8) as i32)) }));
    ({ remove_7(tmpA.decay()) });
    ({ remove_7(tmpB.decay()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
