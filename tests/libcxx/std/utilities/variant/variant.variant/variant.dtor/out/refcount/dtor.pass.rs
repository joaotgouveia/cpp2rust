extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct NonTDtor {
    #[offset(0)]
    #[byte_size(8)]
    pub count: Ptr<i32>,
}
impl NonTDtor {
    pub fn new(mut a: Ptr<i32>, mut _a1: Ptr<i32>) -> Self {
        Self { count: (a).clone() }
    }
}
impl CppMove for NonTDtor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "!std::is_trivially_destructible<NonTDtor>::value");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct NonTDtor1 {
    #[offset(0)]
    #[byte_size(8)]
    pub count: Ptr<i32>,
}
impl NonTDtor1 {
    pub fn new(mut _a0: Ptr<i32>, mut b: Ptr<i32>) -> Self {
        Self { count: (b).clone() }
    }
}
impl CppMove for NonTDtor1 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "!std::is_trivially_destructible<NonTDtor1>::value");
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct TDtor {}
impl TDtor {
    pub fn copy_from(_a0: Ptr<TDtor>) -> Self {
        Self {}
    }
}
impl Clone for TDtor {
    fn clone(&self) -> Self {
        let __src: Value<TDtor> = Rc::new(RefCell::new(TDtor {}));
        TDtor::copy_from(__src.as_pointer())
    }
}
impl CppMove for TDtor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "!std::is_trivially_copy_constructible<TDtor>::value");
const _: () = assert!(true, "std::is_trivially_destructible<TDtor>::value");
pub fn test_0() -> bool {
    {
        const _: () = assert!(true, "std::is_trivially_destructible<V>::value");;
        let v: Value<std_variant_int__long__TDtor_> =
            Rc::new(RefCell::new(std_variant_int__long__TDtor_::new_1({
                (*in_place_index_1.with(Value::clone).borrow()).clone()
            })));
    }
    {
        const _: () = assert!(true, "!std::is_trivially_destructible<V>::value");;
        {
            let count0: Value<i32> = Rc::new(RefCell::new(0));
            let count1: Value<i32> = Rc::new(RefCell::new(0));
            {
                let v: Value<std_variant_NonTDtor__int__NonTDtor1_> = Rc::new(RefCell::new({
                    let __tmp_0: Value<Ptr<i32>> = Rc::new(RefCell::new((count0.as_pointer())));
                    let __tmp_1: Value<Ptr<i32>> = Rc::new(RefCell::new((count1.as_pointer())));
                    std_variant_NonTDtor__int__NonTDtor1_::new_1(
                        { (*in_place_index_2.with(Value::clone).borrow()).clone() },
                        { __tmp_0.as_pointer() },
                        { __tmp_1.as_pointer() },
                    )
                }));
                assert!(({ ((*count0.borrow()) == 0) }));
                assert!(({ ((*count1.borrow()) == 0) }));
            }
            assert!(({ ((*count0.borrow()) == 1) }));
            assert!(({ ((*count1.borrow()) == 0) }));
        }
        {
            let mut count0: i32 = 0;
            let mut count1: i32 = 0;
            {
                let v: Value<std_variant_NonTDtor__int__NonTDtor1_> =
                    Rc::new(RefCell::new(std_variant_NonTDtor__int__NonTDtor1_::new_2(
                        { (*in_place_index_3.with(Value::clone).borrow()).clone() },
                    )));
            }
            assert!((count0 == 0));
            assert!((count1 == 0));
        }
        {
            let count0: Value<i32> = Rc::new(RefCell::new(0));
            let count1: Value<i32> = Rc::new(RefCell::new(0));
            {
                let v: Value<std_variant_NonTDtor__int__NonTDtor1_> = Rc::new(RefCell::new({
                    let __tmp_2: Value<Ptr<i32>> = Rc::new(RefCell::new((count0.as_pointer())));
                    let __tmp_3: Value<Ptr<i32>> = Rc::new(RefCell::new((count1.as_pointer())));
                    std_variant_NonTDtor__int__NonTDtor1_::new_3(
                        { (*in_place_index_1.with(Value::clone).borrow()).clone() },
                        { __tmp_2.as_pointer() },
                        { __tmp_3.as_pointer() },
                    )
                }));
                assert!(({ ((*count0.borrow()) == 0) }));
                assert!(({ ((*count1.borrow()) == 0) }));
            }
            assert!(({ ((*count0.borrow()) == 0) }));
            assert!(({ ((*count1.borrow()) == 1) }));
        }
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
pub trait NonTDtorImpl {
    fn destructor(&self);
}
impl NonTDtorImpl for Ptr<NonTDtor> {
    fn destructor(&self) {
        (*self).with(|__s| __s.count.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub trait NonTDtor1Impl {
    fn destructor(&self);
}
impl NonTDtor1Impl for Ptr<NonTDtor1> {
    fn destructor(&self) {
        (*self).with(|__s| __s.count.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub trait TDtorImpl {
    fn copy_assign(&self, _a0: Ptr<TDtor>) -> Ptr<TDtor>;
}
impl TDtorImpl for Ptr<TDtor> {
    fn copy_assign(&self, _a0: Ptr<TDtor>) -> Ptr<TDtor> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
