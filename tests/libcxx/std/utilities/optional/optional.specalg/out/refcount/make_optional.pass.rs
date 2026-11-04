extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static can_make_optional_implicit_0: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_implicit_1: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_implicit_2: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_implicit_3: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_implicit_4: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_implicit_5: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_implicit_6: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_implicit_7: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_implicit_8: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_implicit_9: Value<bool> = Rc::new(RefCell::new(true));
);
const _: () = assert!(true, "can_make_optional_implicit<int>");
const _: () = assert!(true, "can_make_optional_implicit<const int>");
const _: () = assert!(true, "can_make_optional_implicit<int&>");
const _: () = assert!(true, "can_make_optional_implicit<const int&>");
const _: () = assert!(true, "can_make_optional_implicit<std::unique_ptr<int>>");
const _: () = assert!(
    true,
    "!can_make_optional_implicit<const std::unique_ptr<int>>"
);
const _: () = assert!(true, "!can_make_optional_implicit<std::unique_ptr<int>&>");
const _: () = assert!(
    true,
    "!can_make_optional_implicit<const std::unique_ptr<int>&>"
);
const _: () = assert!(true, "can_make_optional_implicit<int[42]>");
const _: () = assert!(true, "can_make_optional_implicit<const int (&)[42]>");
pub fn test_10() -> bool {
    {
        let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        ])));
        let opt: Value<std_optional_int_ptr_> =
            Rc::new(RefCell::new(({ make_optional_11(arr.as_pointer()) })));
        const _: () = assert!(true, "");;
        assert!(({ (((*opt.borrow()).read()) == (arr.as_pointer() as Ptr::<i32>)) }));
    }
    {
        let opt: Value<std_optional_int_> = Rc::new(RefCell::new(
            ({
                let ___t: Value<i32> = Rc::new(RefCell::new(2));
                make_optional_12(___t.as_pointer())
            }),
        ));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "opt.value() == 2");;
    }
    {
        let opt: Value<std_optional_int_> = Rc::new(RefCell::new(
            ({
                let ___t: Value<i32> = Rc::new(RefCell::new(2));
                make_optional_12(___t.as_pointer())
            }),
        ));
        const _: () = assert!(true, "");;
        assert!(({ (((*opt.borrow()).read()) == 2) }));
    }
    let test_string: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"123").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let opt: Value<
            std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char___,
        > = Rc::new(RefCell::new(
            ({ make_optional_13((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) }),
        ));
        const _: () = assert!(true, "");;
        assert!(
            ({
                match (opt.as_pointer(),) {
                    (__tmp_0,) => Ptr::<i8>::from_string_literal(b"123").with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
    })));
    {
        ({ (*test_string.borrow()).call() });
    }
    let test_unique_ptr: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        let s: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(3)))));
        let opt: Value<std_optional_std_unique_ptr_int__std_default_delete_int___> =
            Rc::new(RefCell::new(({ make_optional_14(s.as_pointer()) })));
        const _: () = assert!(true, "");;
        assert!(({ (((*(*opt.borrow()).upgrade().deref()).as_pointer().read()) == 3) }));
        assert!(({ (*s.borrow()).is_none() }));
    })));
    {
        ({ (*test_unique_ptr.borrow()).call() });
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
    ({ test_10() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = can_make_optional_implicit_0.with(|_| ());
    let _ = can_make_optional_implicit_1.with(|_| ());
    let _ = can_make_optional_implicit_2.with(|_| ());
    let _ = can_make_optional_implicit_3.with(|_| ());
    let _ = can_make_optional_implicit_4.with(|_| ());
    let _ = can_make_optional_implicit_5.with(|_| ());
    let _ = can_make_optional_implicit_6.with(|_| ());
    let _ = can_make_optional_implicit_7.with(|_| ());
    let _ = can_make_optional_implicit_8.with(|_| ());
    let _ = can_make_optional_implicit_9.with(|_| ());
}
