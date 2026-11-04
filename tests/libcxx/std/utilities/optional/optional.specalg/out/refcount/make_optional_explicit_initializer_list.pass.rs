extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static can_make_optional_explicit_impl_0: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_impl_1: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_impl_2: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_impl_3: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_4: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_5: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_6: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_make_optional_explicit_7: Value<bool> = Rc::new(RefCell::new(false));
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<int, std::initializer_list<int>&>"
);
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::string, std::initializer_list<char>&>"
);
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::string, std::initializer_list<char>&, std::allocator<char>>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::string, std::initializer_list<std::string>&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Vec<i32  >  , new_1, value)]
#[byte_size(16)]
pub struct TestT {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub size: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub ptr: Ptr<i32>,
}
impl TestT {
    pub fn new_1(il: Vec<i32>) -> Self {
        let il: Value<Vec<i32>> = Rc::new(RefCell::new(il));
        Self {
            x: (({ (*il.borrow()).begin() }).read()),
            size: ((*il.borrow()).len() as i32),
            ptr: Ptr::<i32>::null(),
        }
    }
    pub fn new_2(il: Vec<i32>, mut p: Ptr<i32>) -> Self {
        let il: Value<Vec<i32>> = Rc::new(RefCell::new(il));
        Self {
            x: (({ (*il.borrow()).begin() }).read()),
            size: ((*il.borrow()).len() as i32),
            ptr: (p).clone(),
        }
    }
}
impl CppMove for TestT {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_8() -> bool {
    {
        let opt: Value<std_optional_TestT_> =
            Rc::new(RefCell::new(({ make_optional_9(Vec::from([42, 2, 3])) })));
        const _: () = assert!(true, "");;
        assert!(({ ({ ((*opt.borrow()).read()).x } == 42) }));
        assert!(({ ({ ((*opt.borrow()).read()).size } == 3) }));
        assert!(({ ({ ((*opt.borrow()).read()).ptr.clone() }).is_null() }));
    }
    {
        let i: Value<i32> = Rc::new(RefCell::new(42));
        let opt: Value<std_optional_TestT_> = Rc::new(RefCell::new(
            ({
                let ___args: Value<Ptr<i32>> = Rc::new(RefCell::new((i.as_pointer())));
                make_optional_10(Vec::from([42, 2, 3]), ___args.as_pointer())
            }),
        ));
        const _: () = assert!(true, "");;
        assert!(({ ({ ((*opt.borrow()).read()).x } == 42) }));
        assert!(({ ({ ((*opt.borrow()).read()).size } == 3) }));
        assert!(({ ({ { ((*opt.borrow()).read()).ptr.clone() } } == { (i.as_pointer()) }) }));
    }
    let test_string: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        {
            let opt: Value<
                std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char___,
            > = Rc::new(RefCell::new(
                ({ make_optional_11(Vec::from([('1' as i8), ('2' as i8), ('3' as i8)])) }),
            ));
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
        }
        {
            let opt: Value<
                std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char___,
            > = Rc::new(RefCell::new(
                ({
                    let ___args: Value<std_allocator_char_> =
                        Rc::new(RefCell::new(<std_allocator_char_>::default()));
                    make_optional_12(
                        Vec::from([('a' as i8), ('b' as i8), ('c' as i8)]),
                        ___args.as_pointer(),
                    )
                }),
            ));
            assert!(
                ({
                    match (opt.as_pointer(),) {
                        (__tmp_1,) => Ptr::<i8>::from_string_literal(b"abc").with_c_str(|__s| {
                            (*__tmp_1.upgrade().deref())
                                [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        }),
                    }
                })
            );
        }
    })));
    {
        ({ (*test_string.borrow()).call() });
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
    ({ test_8() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = can_make_optional_explicit_impl_0.with(|_| ());
    let _ = can_make_optional_explicit_impl_1.with(|_| ());
    let _ = can_make_optional_explicit_impl_2.with(|_| ());
    let _ = can_make_optional_explicit_impl_3.with(|_| ());
    let _ = can_make_optional_explicit_4.with(|_| ());
    let _ = can_make_optional_explicit_5.with(|_| ());
    let _ = can_make_optional_explicit_6.with(|_| ());
    let _ = can_make_optional_explicit_7.with(|_| ());
}
