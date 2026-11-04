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
    {
        let t: Value<std_regex_traits_char_> =
            Rc::new(RefCell::new(<std_regex_traits_char_>::default()));
        assert!(
            ({ ((({ (*t.borrow()).translate(('a' as i8),) }) as i32) == (('a' as i8) as i32)) })
        );
        assert!(
            ({ ((({ (*t.borrow()).translate(('B' as i8),) }) as i32) == (('B' as i8) as i32)) })
        );
        assert!(
            ({ ((({ (*t.borrow()).translate(('c' as i8),) }) as i32) == (('c' as i8) as i32)) })
        );
    }
    {
        let t: Value<std_regex_traits_wchar_t_> =
            Rc::new(RefCell::new(<std_regex_traits_wchar_t_>::default()));
        assert!(({ (({ (*t.borrow()).translate((97 as i32),) }) == (97 as i32)) }));
        assert!(({ (({ (*t.borrow()).translate((66 as i32),) }) == (66 as i32)) }));
        assert!(({ (({ (*t.borrow()).translate((99 as i32),) }) == (99 as i32)) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
