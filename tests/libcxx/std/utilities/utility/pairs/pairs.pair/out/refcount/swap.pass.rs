extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub i: i32,
}
impl S {
    pub fn new_2() -> Self {
        Self { i: 0 }
    }
    pub fn new_1(mut j: i32) -> Self {
        Self { i: j }
    }
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new_2() }
    }
}
pub fn test_0() -> bool {
    {
        let p1: Value<std_pair_int__short_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(3));
            let __tmp_1: Value<i16> = Rc::new(RefCell::new(4_i16));
            std_pair_int__short_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        let p2: Value<std_pair_int__short_> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(5));
            let __tmp_3: Value<i16> = Rc::new(RefCell::new(6_i16));
            std_pair_int__short_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
        }));
        ({ (*p1.borrow()).swap_4(p2.as_pointer()) });
        ({ cpp2rust_assert_fail_1(({ (*p1.borrow()).first } == 5)) });
        ({ cpp2rust_assert_fail_1((({ (*p1.borrow()).second } as i32) == 6)) });
        ({ cpp2rust_assert_fail_1(({ (*p2.borrow()).first } == 3)) });
        ({ cpp2rust_assert_fail_1((({ (*p2.borrow()).second } as i32) == 4)) });
    }
    {
        let p1: Value<std_pair_int__S_> = Rc::new(RefCell::new({
            let __tmp_4: Value<i32> = Rc::new(RefCell::new(3));
            let __tmp_5: Value<S> = Rc::new(RefCell::new(S::new_1({ 4 })));
            std_pair_int__S_::new_1({ __tmp_4.as_pointer() }, { __tmp_5.as_pointer() })
        }));
        let p2: Value<std_pair_int__S_> = Rc::new(RefCell::new({
            let __tmp_6: Value<i32> = Rc::new(RefCell::new(5));
            let __tmp_7: Value<S> = Rc::new(RefCell::new(S::new_1({ 6 })));
            std_pair_int__S_::new_1({ __tmp_6.as_pointer() }, { __tmp_7.as_pointer() })
        }));
        ({ (*p1.borrow()).swap_4(p2.as_pointer()) });
        ({ cpp2rust_assert_fail_1(({ (*p1.borrow()).first } == 5)) });
        ({
            cpp2rust_assert_fail_1(
                ({ SImpl::operator_eq(&field_ptr!(p1.as_pointer(), second), 6) }),
            )
        });
        ({ cpp2rust_assert_fail_1(({ (*p2.borrow()).first } == 3)) });
        ({
            cpp2rust_assert_fail_1(
                ({ SImpl::operator_eq(&field_ptr!(p2.as_pointer(), second), 4) }),
            )
        });
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
pub trait SImpl {
    fn operator_eq(&self, x: i32) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_eq(&self, mut x: i32) -> bool {
        return ((*self).with(|__s| __s.i) == x);
    }
}
pub fn __cpp2rust_init_globals() {}
