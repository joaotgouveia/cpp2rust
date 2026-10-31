extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_TestT___std_optional_TestT__ {}
thread_local!(
    pub static value_2: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
thread_local!(
    pub static can_make_optional_explicit_impl_3: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_impl_4: Value<bool> =
        Rc::new(RefCell::new(value_2.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_explicit_impl_5: Value<bool> =
        Rc::new(RefCell::new(value_2.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_explicit_impl_6: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_7: Value<bool> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_3.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_8: Value<bool> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_4.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_9: Value<bool> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_5.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_10: Value<bool> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_6.with(|rc| *rc.borrow()),
    ));
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
            x: (unimplemented!().read()),
            size: ((*il.borrow()).len() as i32),
            ptr: Ptr::<i32>::null(),
        }
    }
    pub fn new_2(il: Vec<i32>, mut p: Ptr<i32>) -> Self {
        let il: Value<Vec<i32>> = Rc::new(RefCell::new(il));
        Self {
            x: (unimplemented!().read()),
            size: ((*il.borrow()).len() as i32),
            ptr: (p).clone(),
        }
    }
}
pub fn test_11() -> bool {
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(({ unimplemented!().x } == 42));
        assert!(({ unimplemented!().size } == 3));
        assert!(({ unimplemented!().ptr.clone() }).is_null());
    }
    {
        let i: Value<i32> = Rc::new(RefCell::new(42));
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(({ unimplemented!().x } == 42));
        assert!(({ unimplemented!().size } == 3));
        assert!(({ { unimplemented!().ptr.clone() } } == { (i.as_pointer()) }));
    }
    let test_string: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        {
            let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(Ptr::<i8>::from_string_literal(b"123").with_c_str(|__s| {
                (*unimplemented!().upgrade().deref())[..(*unimplemented!().upgrade().deref())
                    .len()
                    .saturating_sub(1)]
                    == *__s
            }));
        }
        {
            let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(Ptr::<i8>::from_string_literal(b"abc").with_c_str(|__s| {
                (*unimplemented!().upgrade().deref())[..(*unimplemented!().upgrade().deref())
                    .len()
                    .saturating_sub(1)]
                    == *__s
            }));
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
    ({ test_11() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = value_2.with(|_| ());
    let _ = can_make_optional_explicit_impl_3.with(|_| ());
    let _ = can_make_optional_explicit_impl_4.with(|_| ());
    let _ = can_make_optional_explicit_impl_5.with(|_| ());
    let _ = can_make_optional_explicit_impl_6.with(|_| ());
    let _ = can_make_optional_explicit_7.with(|_| ());
    let _ = can_make_optional_explicit_8.with(|_| ());
    let _ = can_make_optional_explicit_9.with(|_| ());
    let _ = can_make_optional_explicit_10.with(|_| ());
}
