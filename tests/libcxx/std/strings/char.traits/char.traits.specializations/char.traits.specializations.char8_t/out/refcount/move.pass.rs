extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let s1: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    assert!(
        (({
            let ___s1: Ptr<u8> = (s1.as_pointer() as Ptr<u8>);
            let ___s2: Ptr<u8> = (s1.as_pointer() as Ptr<u8>).offset((1) as isize);
            std_char_traits_char8_t_::move_(___s1, ___s2, 2_usize)
        }) == (s1.as_pointer() as Ptr::<u8>))
    );
    assert!(({ (((*s1.borrow())[(0) as usize] as i32) == (2_u8 as i32)) }));
    assert!(({ (((*s1.borrow())[(1) as usize] as i32) == (3_u8 as i32)) }));
    assert!(({ (((*s1.borrow())[(2) as usize] as i32) == (3_u8 as i32)) }));
    (*s1.borrow_mut())[(2) as usize] = 0_u8;
    assert!(
        (({
            let ___s1: Ptr<u8> = (s1.as_pointer() as Ptr<u8>).offset((1) as isize);
            let ___s2: Ptr<u8> = (s1.as_pointer() as Ptr<u8>);
            std_char_traits_char8_t_::move_(___s1, ___s2, 2_usize)
        }) == (s1.as_pointer() as Ptr::<u8>).offset((1) as isize))
    );
    assert!(({ (((*s1.borrow())[(0) as usize] as i32) == (2_u8 as i32)) }));
    assert!(({ (((*s1.borrow())[(1) as usize] as i32) == (2_u8 as i32)) }));
    assert!(({ (((*s1.borrow())[(2) as usize] as i32) == (3_u8 as i32)) }));
    assert!(
        ({
            std_char_traits_char8_t_::move_(
                Ptr::<u8>::null(),
                (s1.as_pointer() as Ptr<u8>),
                0_usize,
            )
        })
        .is_null()
    );
    assert!(
        (({
            std_char_traits_char8_t_::move_(
                (s1.as_pointer() as Ptr<u8>),
                Ptr::<u8>::null(),
                0_usize,
            )
        }) == (s1.as_pointer() as Ptr::<u8>))
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
