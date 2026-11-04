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
    let mut text: Ptr<i8> = Ptr::<i8>::from_string_literal(b"aaa\naa");
    let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
        std_basic_regex_char__std_regex_traits_char__::new_1(
            { Ptr::<i8>::from_string_literal(b"^a") },
            None,
        ),
    ));
    let it: Value<std_regex_iterator_const_char_ptr__char__std_regex_traits_char__> =
        Rc::new(RefCell::new(
            std_regex_iterator_const_char_ptr__char__std_regex_traits_char__::new_1(
                { (text).clone() },
                { text.offset((6) as isize) },
                { re.as_pointer() },
                None,
            ),
        ));
    let end: Value<std_regex_iterator_const_char_ptr__char__std_regex_traits_char__> = Rc::new(
        RefCell::new(<std_regex_iterator_const_char_ptr__char__std_regex_traits_char__>::default()),
    );
    assert!(match (Rc::new(RefCell::new(
        ({ (*(*it.borrow()).iter_deref().upgrade().deref()).str(None,) })
    ))
    .as_pointer(),)
    {
        (__tmp_0,) => {
            Ptr::<i8>::from_string_literal(b"a").with_c_str(|__s| {
                (*__tmp_0.upgrade().deref())[..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    assert!(
        ({ (({ (*(*it.borrow()).iter_deref().upgrade().deref()).position(0_u64,) }) == 0_i64) })
    );
    assert!(({ (({ (*(*it.borrow()).iter_deref().upgrade().deref()).length(0_u64,) }) == 1_i64) }));
    {
        it.as_pointer().with_mut(|__it| {
            __it.iter_inc();
        });
        it.as_pointer()
    };
    assert!(({ (*it.borrow()) == (*end.borrow()) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
