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
    let temp: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_0() })));
    {
        let fs: Value<std_basic_fstream_char__std_char_traits_char__> = Rc::new(RefCell::new(
            <std_basic_fstream_char__std_char_traits_char__>::default(),
        ));
        assert!(({ !({ (*fs.borrow()).is_open_12() }) }));
        ({ (*fs.borrow()).open_14(temp.decay(), 16_i32) });
        assert!(({ ({ (*fs.borrow()).is_open_12() }) }));
        ({ (*fs.borrow()).close() });
        assert!(({ !({ (*fs.borrow()).is_open_12() }) }));
    }
    ({ remove_7(temp.decay()) });
    {
        let fs: Value<std_basic_fstream_wchar_t__std_char_traits_wchar_t__> = Rc::new(
            RefCell::new(<std_basic_fstream_wchar_t__std_char_traits_wchar_t__>::default()),
        );
        assert!(({ !({ (*fs.borrow()).is_open_12() }) }));
        ({ (*fs.borrow()).open_14(temp.decay(), 16_i32) });
        assert!(({ ({ (*fs.borrow()).is_open_12() }) }));
        ({ (*fs.borrow()).close() });
        assert!(({ !({ (*fs.borrow()).is_open_12() }) }));
    }
    ({ remove_7(temp.decay()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
