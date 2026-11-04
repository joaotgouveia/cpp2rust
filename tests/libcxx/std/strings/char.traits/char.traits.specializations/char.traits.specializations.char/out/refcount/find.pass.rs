extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_constexpr_0() -> bool {
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"123");
    return ((({
        ({
            let ___s: Ptr<i8> = (p).clone();
            let ___a: Value<i8> = Rc::new(RefCell::new(('1' as i8)));
            std_char_traits_char_::find(___s, 3_usize, ___a.as_pointer())
        })
    } == { (p).clone() })
        && ({
            ({
                let ___s: Ptr<i8> = (p).clone();
                let ___a: Value<i8> = Rc::new(RefCell::new(('2' as i8)));
                std_char_traits_char_::find(___s, 3_usize, ___a.as_pointer())
            })
        } == { p.offset((1) as isize) }))
        && ({
            ({
                let ___s: Ptr<i8> = (p).clone();
                let ___a: Value<i8> = Rc::new(RefCell::new(('3' as i8)));
                std_char_traits_char_::find(___s, 3_usize, ___a.as_pointer())
            })
        } == { p.offset((2) as isize) }))
        && (({
            let ___s: Ptr<i8> = (p).clone();
            let ___a: Value<i8> = Rc::new(RefCell::new(('4' as i8)));
            std_char_traits_char_::find(___s, 3_usize, ___a.as_pointer())
        })
        .is_null());
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
    let s1: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8])));
    assert!(
        (({
            let ___a: Value<i8> = Rc::new(RefCell::new(1_i8));
            std_char_traits_char_::find((s1.as_pointer() as Ptr<i8>), 3_usize, ___a.as_pointer())
        }) == (s1.as_pointer() as Ptr::<i8>))
    );
    assert!(
        (({
            let ___a: Value<i8> = Rc::new(RefCell::new(2_i8));
            std_char_traits_char_::find((s1.as_pointer() as Ptr<i8>), 3_usize, ___a.as_pointer())
        }) == (s1.as_pointer() as Ptr::<i8>).offset((1) as isize))
    );
    assert!(
        (({
            let ___a: Value<i8> = Rc::new(RefCell::new(3_i8));
            std_char_traits_char_::find((s1.as_pointer() as Ptr<i8>), 3_usize, ___a.as_pointer())
        }) == (s1.as_pointer() as Ptr::<i8>).offset((2) as isize))
    );
    assert!(
        ({
            let ___a: Value<i8> = Rc::new(RefCell::new(4_i8));
            std_char_traits_char_::find((s1.as_pointer() as Ptr<i8>), 3_usize, ___a.as_pointer())
        })
        .is_null()
    );
    assert!(
        ({
            let ___a: Value<i8> = Rc::new(RefCell::new(0_i8));
            std_char_traits_char_::find((s1.as_pointer() as Ptr<i8>), 3_usize, ___a.as_pointer())
        })
        .is_null()
    );
    assert!(
        ({
            let ___a: Value<i8> = Rc::new(RefCell::new(0_i8));
            std_char_traits_char_::find(Ptr::<i8>::null(), 0_usize, ___a.as_pointer())
        })
        .is_null()
    );
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
