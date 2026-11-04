extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    {
        let p1: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(1)))));
        assert!(({ (((*p1.borrow()).as_pointer().read()) == 1) }));
        p1.as_pointer()
            .write(Some(Rc::new(RefCell::new(0_i32))).take());
        assert!(({ (((*p1.borrow()).as_pointer().read()) == 0) }));
    }
    {
        let p2: Value<Option<Value<Vec<i8>>>> =
            Rc::new(RefCell::new(Some(Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"Meow!").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            })))));
        assert!(
            ({
                match ((*p2.borrow()).as_pointer(),) {
                    (__tmp_0,) => Ptr::<i8>::from_string_literal(b"Meow!").with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        p2.as_pointer()
            .write(Some(Rc::new(RefCell::new(vec![i8::default()]))).take());
        assert!(
            ({
                match ((*p2.borrow()).as_pointer(),) {
                    (__tmp_1,) => Ptr::<i8>::from_string_literal(b"").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        p2.as_pointer().write(
            Some(Rc::new(RefCell::new(
                vec![('z' as i8); (6 as usize) as usize]
                    .iter()
                    .cloned()
                    .chain(std::iter::once(0_i8))
                    .collect(),
            )))
            .take(),
        );
        assert!(
            ({
                match ((*p2.borrow()).as_pointer(),) {
                    (__tmp_2,) => Ptr::<i8>::from_string_literal(b"zzzzzz").with_c_str(|__s| {
                        (*__tmp_2.upgrade().deref())
                            [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
    }
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
