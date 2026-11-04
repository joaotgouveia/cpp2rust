extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_container_iterators_0(c: Vec<i32>) {
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(c));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    assert!(
        ({
            (({ libcc2rs::to_address_refcount(({ (*c.borrow_mut()).begin_3() })) }) == c.decay())
        })
    );
    assert!(
        ({
            (({ libcc2rs::to_address_refcount(({ (*c.borrow_mut()).end_5() })) })
                == c.decay().offset(((*c.borrow()).len()) as isize))
        })
    );
    assert!(
        ({
            ({ ({ libcc2rs::to_address_refcount(({ (*cc.upgrade().deref()).begin_4() })) }) } == {
                cc.decay()
            })
        })
    );
    assert!(
        ({
            ({ ({ libcc2rs::to_address_refcount(({ (*cc.upgrade().deref()).end_6() })) }) } == {
                cc.decay().offset(((*cc.upgrade().deref()).len()) as isize)
            })
        })
    );
}
pub fn test_container_iterators_1(c: Vec<i32>) {
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(c));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    assert!((CppRange::begin(&c.decay()).iter_deref() == c.decay()));
    assert!(
        (CppRange::end(&c.decay()).iter_deref()
            == c.decay().offset(((*c.borrow()).len()) as isize))
    );
    assert!(({ CppRange::begin(&cc.decay()).iter_deref() } == { cc.decay() }));
    assert!(
        ({ CppRange::end(&cc.decay()).iter_deref() } == {
            cc.decay().offset(((*cc.upgrade().deref()).len()) as isize)
        })
    );
}
pub fn test_container_iterators_2(c: Vec<i8>) {
    let c: Value<Vec<i8>> = Rc::new(RefCell::new(c));
    let cc: Ptr<Vec<i8>> = c.as_pointer();
    assert!((RandomAccessIterator::Vec(c.decay()).iter_deref() == c.decay()));
    assert!(
        (RandomAccessIterator::Vec(c.decay().to_last()).iter_deref()
            == c.decay().offset(((*c.borrow()).len() - 1) as isize))
    );
    assert!(({ ({ (*cc.upgrade().deref()).begin_64() }).iter_deref() } == { cc.decay() }));
    assert!(
        ({ ({ (*cc.upgrade().deref()).end_66() }).iter_deref() } == {
            cc.decay()
                .offset(((*cc.upgrade().deref()).len() - 1) as isize)
        })
    );
}
pub fn test_valarray_iterators_3() {
    let v: Value<std_valarray_int_> =
        Rc::new(RefCell::new(std_valarray_int_::new_1({ 100_usize })));
    let mut p: Ptr<i32> = ({ libcc2rs::to_address_refcount(({ begin_4(v.as_pointer()) })) });
    let mut q: Ptr<i32> =
        ({ libcc2rs::to_address_refcount(({ libcc2rs::end_refcount(v.as_pointer()) })) });
    assert!((((q).clone() - (p).clone()) as i64 == 100_i64));
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
    ({ test_container_iterators_0(std::array::from_fn::<_, 3, _>(|_| 0_i32).to_vec()) });
    ({ test_container_iterators_1((0..(3_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>()) });
    ({
        test_container_iterators_2({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"abc").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        })
    });
    ({
        test_container_iterators_5(std_basic_string_view_char__std_char_traits_char__::new_1({
            Ptr::<i8>::from_string_literal(b"abc")
        }))
    });
    ({
        test_container_iterators_6(std_span_const_char__18446744073709551615_::new_1({
            Ptr::<i8>::from_string_literal(b"abc")
        }))
    });
    ({ test_valarray_iterators_3() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
