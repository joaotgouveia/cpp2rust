extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let s1: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([1_u16, 2_u16, 3_u16])));
    let s2: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([0_u16, 0_u16, 0_u16])));
    assert!(
        (({
            std_char_traits_char16_t_::copy(
                (s2.as_pointer() as Ptr<u16>),
                (s1.as_pointer() as Ptr<u16>),
                3_usize,
            )
        }) == (s2.as_pointer() as Ptr::<u16>))
    );
    assert!(({ (((*s2.borrow())[(0) as usize] as i32) == (1_u16 as i32)) }));
    assert!(({ (((*s2.borrow())[(1) as usize] as i32) == (2_u16 as i32)) }));
    assert!(({ (((*s2.borrow())[(2) as usize] as i32) == (3_u16 as i32)) }));
    assert!(
        ({
            std_char_traits_char16_t_::copy(
                Ptr::<u16>::null(),
                (s1.as_pointer() as Ptr<u16>),
                0_usize,
            )
        })
        .is_null()
    );
    assert!(
        (({
            std_char_traits_char16_t_::copy(
                (s1.as_pointer() as Ptr<u16>),
                Ptr::<u16>::null(),
                0_usize,
            )
        }) == (s1.as_pointer() as Ptr::<u16>))
    );
    return true;
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
