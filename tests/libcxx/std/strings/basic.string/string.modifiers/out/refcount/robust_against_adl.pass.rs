extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i8, new, value)]
#[cpp_into(i8, to_i8)]
#[byte_size(1)]
pub struct Charlike_Holder_Incomplete__ {
    #[offset(0)]
    pub ch_: i8,
}
impl Charlike_Holder_Incomplete__ {
    pub fn new(mut ch: i8) -> Self {
        Self { ch_: ch }
    }
}
impl CppMove for Charlike_Holder_Incomplete__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_string_0() {
    let s: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    let a: Value<Box<[Charlike_Holder_Incomplete__]>> = Rc::new(RefCell::new(Box::new([
        Charlike_Holder_Incomplete__::new({ ('m' as i8) }),
        Charlike_Holder_Incomplete__::new({ ('a' as i8) }),
        Charlike_Holder_Incomplete__::new({ ('h' as i8) }),
        Charlike_Holder_Incomplete__::new({ ('i' as i8) }),
    ])));
    ({
        let ___first: Ptr<Charlike_Holder_Incomplete__> =
            (a.as_pointer() as Ptr<Charlike_Holder_Incomplete__>);
        let ___last: Ptr<Charlike_Holder_Incomplete__> =
            (a.as_pointer() as Ptr<Charlike_Holder_Incomplete__>).offset((4) as isize);
        (*s.borrow_mut()).append_108_struct_Charlike_struct_Holder_struct_Incomplete____ptr_void(
            ___first, ___last,
        )
    });
    ({
        let ___first: Ptr<Charlike_Holder_Incomplete__> =
            (a.as_pointer() as Ptr<Charlike_Holder_Incomplete__>);
        let ___last: Ptr<Charlike_Holder_Incomplete__> =
            (a.as_pointer() as Ptr<Charlike_Holder_Incomplete__>).offset((4) as isize);
        (*s.borrow_mut()).assign_118_struct_Charlike_struct_Holder_struct_Incomplete____ptr_void(
            ___first, ___last,
        )
    });
    ({
        let ___p: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(s.decay()));
        let ___beg: Ptr<Charlike_Holder_Incomplete__> =
            (a.as_pointer() as Ptr<Charlike_Holder_Incomplete__>);
        let ___end: Ptr<Charlike_Holder_Incomplete__> =
            (a.as_pointer() as Ptr<Charlike_Holder_Incomplete__>).offset((4) as isize);
        (*s.borrow_mut()).insert_124_struct_Charlike_struct_Holder_struct_Incomplete____ptr_void(
            ___p, ___beg, ___end,
        )
    });
    ({
        let ___i1: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(s.decay()));
        let ___i2: RandomAccessIterator<i8> = <RandomAccessIterator<i8>>::from(
            RandomAccessIterator::Vec(s.decay()).iter_add(4_isize),
        );
        let ___k1: Ptr<Charlike_Holder_Incomplete__> =
            (a.as_pointer() as Ptr<Charlike_Holder_Incomplete__>);
        let ___k2: Ptr<Charlike_Holder_Incomplete__> =
            (a.as_pointer() as Ptr<Charlike_Holder_Incomplete__>).offset((4) as isize);
        (*s.borrow_mut()).replace_148_struct_Charlike_struct_Holder_struct_Incomplete____ptr_void(
            ___i1, ___i2, ___k1, ___k2,
        )
    });
    assert!(
        ({
            Ptr::<i8>::from_string_literal(b"mahimahi")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        })
    );
}
pub fn test_1() -> bool {
    ({ test_string_0() });
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
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait Charlike_Holder_Incomplete__Impl {
    fn to_i8(&self) -> i8;
}
impl Charlike_Holder_Incomplete__Impl for Ptr<Charlike_Holder_Incomplete__> {
    fn to_i8(&self) -> i8 {
        return (*self).with(|__s| __s.ch_);
    }
}
pub fn __cpp2rust_init_globals() {}
