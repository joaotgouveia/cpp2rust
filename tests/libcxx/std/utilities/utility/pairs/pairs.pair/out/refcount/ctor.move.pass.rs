extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct Dummy {}
impl Dummy {
    pub fn move_from(_a0: Ptr<Dummy>) -> Self {
        Self {}
    }
}
impl CppMove for Dummy {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotCopyOrMoveConstructible {}
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
        const _: () = assert!(true, "std::is_move_constructible<P1>::value");;
        let p1: Value<std_pair_int__short_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(3));
            let __tmp_1: Value<i16> = Rc::new(RefCell::new(4_i16));
            std_pair_int__short_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        let p2: Value<std_pair_int__short_> = Rc::new(RefCell::new((*p1.borrow_mut()).clone()));
        ({ cpp2rust_assert_fail_0(({ (*p2.borrow()).first } == 3)) });
        ({ cpp2rust_assert_fail_0((({ (*p2.borrow()).second } as i32) == 4)) });
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible<P>::value");;
        const _: () = assert!(true, "std::is_move_constructible<P>::value");;
    }
    {
        {
            const _: () = assert!(true, "std::is_move_constructible<P>::value");;
            let obj: Value<NotCopyOrMoveConstructible> =
                Rc::new(RefCell::new(<NotCopyOrMoveConstructible>::default()));
            let p2: Value<std_pair_NotCopyOrMoveConstructible_ref__int_> = Rc::new(RefCell::new({
                let __tmp_2: Value<i32> = Rc::new(RefCell::new(3));
                std_pair_NotCopyOrMoveConstructible_ref__int_::new_1({ obj.as_pointer() }, {
                    __tmp_2.as_pointer()
                })
            }));
            let p1: Value<std_pair_NotCopyOrMoveConstructible_ref__int_> =
                Rc::new(RefCell::new((*p2.borrow_mut()).clone()));
            ({
                cpp2rust_assert_fail_0(
                    ({ ({ (*p1.borrow()).first.clone() }).clone() } == { (obj.as_pointer()) }),
                )
            });
            ({
                cpp2rust_assert_fail_0(
                    ({ ({ (*p2.borrow()).first.clone() }).clone() } == { (obj.as_pointer()) }),
                )
            });
        }
        {
            const _: () = assert!(true, "std::is_move_constructible<P>::value");;
            let obj: Value<NotCopyOrMoveConstructible> =
                Rc::new(RefCell::new(<NotCopyOrMoveConstructible>::default()));
            let p2: Value<std_pair_NotCopyOrMoveConstructible_refref__int_> =
                Rc::new(RefCell::new({
                    let __tmp_3: Value<i32> = Rc::new(RefCell::new(3));
                    std_pair_NotCopyOrMoveConstructible_refref__int_::new_1({ obj.as_pointer() }, {
                        __tmp_3.as_pointer()
                    })
                }));
            let p1: Value<std_pair_NotCopyOrMoveConstructible_refref__int_> =
                Rc::new(RefCell::new((*p2.borrow_mut()).clone()));
            ({
                cpp2rust_assert_fail_0(
                    ({ ({ (*p1.borrow()).first.clone() }).clone() } == { (obj.as_pointer()) }),
                )
            });
            ({
                cpp2rust_assert_fail_0(
                    ({ ({ (*p2.borrow()).first.clone() }).clone() } == { (obj.as_pointer()) }),
                )
            });
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
