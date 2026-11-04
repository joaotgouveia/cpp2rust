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
pub fn count_bytes_7(mut filename: Ptr<i8>) -> usize {
    let mut f: Ptr<CFile> = match CFile::open(
        &filename.to_rust_string(),
        &Ptr::<i8>::from_string_literal(b"rb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    let count: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while (({ fgetc_8((f).clone()) }) != (-1_i32)) {
        (*count.borrow_mut()).prefix_inc();
    }
    {
        let __r = f.with(|__f| __f.close());
        f.delete();
        __r
    };
    return (*count.borrow());
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
    {
        let buf: Value<std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                <std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__>::default(),
            ));
        let mut sz: i64 = ({
            ((*buf.borrow()) as ::std::fs::File)
                .sputn(Ptr::<i8>::from_string_literal(b"\xff"), 1_i64)
        });
        assert!((sz == 1_i64));
        assert!(((({ (*buf.borrow()).str_17() }).len() - 1) == 1_usize));
    }
    {
        let temp: Value<Vec<i8>> = Rc::new(RefCell::new(({ get_temp_file_name_0() })));
        {
            let buf: Value<std_basic_filebuf_char__std_char_traits_char__> = Rc::new(RefCell::new(
                <std_basic_filebuf_char__std_char_traits_char__>::default(),
            ));
            ({ (*buf.borrow()).open_11(temp.decay(), 16_i32) });
            let mut sz: i64 = ({
                ((*buf.borrow()) as ::std::fs::File)
                    .sputn(Ptr::<i8>::from_string_literal(b"\xff"), 1_i64)
            });
            assert!((sz == 1_i64));
        }
        assert!((({ count_bytes_7(temp.decay(),) }) == 1_usize));
        ({ remove_9(temp.decay()) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
