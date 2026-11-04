extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(
    true,
    "std::constructible_from<std::unexpected<int>, std::in_place_t, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Foo {}
impl CppMove for Foo {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::unexpected<Foo>, std::in_place_t, int>"
);
const _: () = assert!(true, "ImplicitlyConstructible<int, int>");
const _: () = assert!(
    !(false),
    "!ImplicitlyConstructible<std::unexpected<int>, std::in_place_t, int>"
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Arg {
    #[offset(0)]
    pub i: i32,
}
impl Arg {
    pub fn new(mut ii: i32) -> Self {
        Self { i: ii }
    }
    pub fn copy_from(other: Ptr<Arg>) -> Self {
        Self {
            i: other.with(|__s| __s.i),
        }
    }
    pub fn move_from(other: Ptr<Arg>) -> Self {
        let __this: Arg = Self {
            i: other.with(|__s| __s.i),
        };
        field!(other, i).write(0);
        __this
    }
}
impl Clone for Arg {
    fn clone(&self) -> Self {
        let __src: Value<Arg> = Rc::new(RefCell::new(Arg { i: self.i.clone() }));
        Arg::copy_from(__src.as_pointer())
    }
}
impl CppMove for Arg {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(Vec<Error  >  , new_1, value)]
#[byte_size(4)]
pub struct Error {
    #[offset(0)]
    #[byte_size(4)]
    pub arg: Arg,
}
impl Error {
    pub fn new_2(a: Ptr<Arg>) -> Self {
        Self {
            arg: Arg::copy_from({ (a).clone() }),
        }
    }
    pub fn new_3(a: Ptr<Arg>) -> Self {
        Self {
            arg: Arg::move_from({ (a).clone() }),
        }
    }
    pub fn new_1(_a0: Vec<Error>) -> Self {
        let _a0: Value<Vec<Error>> = Rc::new(RefCell::new(_a0));
        let __this: Error = Self {
            arg: Arg::new({ 0 }),
        };
        assert!(false);
        __this
    }
    pub fn move_from(_a0: Ptr<Error>) -> Self {
        Self {
            arg: Arg::move_from({ field_ptr!(_a0, arg) }),
        }
    }
}
impl CppMove for Error {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn test_0() -> bool {
    {
        let a: Value<Arg> = Rc::new(RefCell::new(Arg::new({ 5 })));
        let unex: Value<std_unexpected_Error_> = Rc::new(RefCell::new(
            std_unexpected_Error_::new_1({ (*in_place_1.with(Value::clone).borrow()).clone() }, {
                a.as_pointer()
            }),
        ));
        assert!(({ (({ (*unex.borrow()).error_9() }).with(|__s| __s.arg.i) == 5) }));
        assert!(({ ({ (*a.borrow()).i } == 5) }));
    }
    {
        let a: Value<Arg> = Rc::new(RefCell::new(Arg::new({ 5 })));
        let unex: Value<std_unexpected_Error_> = Rc::new(RefCell::new(
            std_unexpected_Error_::new_2({ (*in_place_1.with(Value::clone).borrow()).clone() }, {
                a.as_pointer()
            }),
        ));
        assert!(({ (({ (*unex.borrow()).error_9() }).with(|__s| __s.arg.i) == 5) }));
        assert!(({ ({ (*a.borrow()).i } == 0) }));
    }
    {
        let e: Value<Error> = Rc::new(RefCell::new({
            let __tmp_0: Value<Arg> = Rc::new(RefCell::new(Arg::new({ 5 })));
            Error::new_3({ __tmp_0.as_pointer() })
        }));
        let unex: Value<std_unexpected_Error_> = Rc::new(RefCell::new(
            std_unexpected_Error_::new_3({ (*in_place_1.with(Value::clone).borrow()).clone() }, {
                e.as_pointer()
            }),
        ));
    }
    return true;
}
pub fn testException_2() {}
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
    ({ testException_2() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
