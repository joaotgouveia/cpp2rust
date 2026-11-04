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
    let vecss : Value<std_vector_std_basic_stringstream_char___std_allocator_std_basic_stringstream_char___ > = Rc::new(RefCell::new(<std_vector_std_basic_stringstream_char___std_allocator_std_basic_stringstream_char___  >::default() )) ;
    ({
        let ___x: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(
                ),
            ));
        (*vecss.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___s: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"hub started at [00 6b 8b 45 69]") },
                    None,
                ),
            ));
        (*({ (*vecss.borrow()).back_55() }).upgrade().deref()).str_22(___s.as_pointer())
    });
    ({
        let ___x: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(
                ),
            ));
        (*vecss.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___s: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"hub started at [00 6b 8b 45 69]") },
                    None,
                ),
            ));
        (*({ (*vecss.borrow()).back_55() }).upgrade().deref()).str_22(___s.as_pointer())
    });
    let mut n: u64 = 0_u64;
    'loop_: while (n < ({ (*vecss.borrow()).size() })) {
        ({ cpp2rust_assert_fail_0((({ ({ (*vecss.borrow())[n].str_16() }).size() }) == 31_u64)) });
        ({
            ((*vecss.borrow())[n] as std_basic_istream_char__std_char_traits_char__)
                .seekg_40(0_i64, beg_1.with(|rc| *rc.borrow()))
        });
        ({ cpp2rust_assert_fail_0((({ ({ (*vecss.borrow())[n].str_16() }).size() }) == 31_u64)) });
        n.postfix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
