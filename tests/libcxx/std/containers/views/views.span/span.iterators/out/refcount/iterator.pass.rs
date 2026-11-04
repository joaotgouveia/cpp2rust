extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_type_0() {
    let ii1: Value<RandomAccessIterator<i8>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    let ii2: Value<RandomAccessIterator<i8>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    let ii4: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
    let cii: Value<RandomAccessIterator<i8>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    assert!(({ (*ii1.borrow()) == (*ii2.borrow()) }));
    assert!(({ (*ii1.borrow()) == (*ii4.borrow()) }));
    assert!((*ii1.borrow()) == (*cii.borrow()));
    assert!(({ !((*ii1.borrow()) != (*ii2.borrow())) }));
    assert!(({ !((*ii1.borrow()) != (*cii.borrow())) }));
    let v: Value<i8> = Rc::new(RefCell::new(0_i8));
    let c: Value<std_span_char__18446744073709551615_> = Rc::new(RefCell::new(
        std_span_char__18446744073709551615_::new_1({ (v.as_pointer()) }, { 1_u64 }),
    ));
    assert!(({ (*c.borrow()).begin() }) == ({ begin_1(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).rbegin() }) == ({ rbegin_2(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).cbegin() }) == ({ cbegin_3(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).crbegin() }) == ({ crbegin_4(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).end() }) == ({ libcc2rs::end_refcount(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).rend() }) == ({ rend_5(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).cend() }) == ({ cend_6(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).crend() }) == ({ crend_7(c.as_pointer(),) }));
    assert!(({ begin_1(c.as_pointer(),) }) != ({ libcc2rs::end_refcount(c.as_pointer(),) }));
    assert!(({ rbegin_2(c.as_pointer(),) }) != ({ rend_5(c.as_pointer(),) }));
    assert!(({ cbegin_3(c.as_pointer(),) }) != ({ cend_6(c.as_pointer(),) }));
    assert!(({ crbegin_4(c.as_pointer(),) }) != ({ crend_7(c.as_pointer(),) }));
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        (*ii1.borrow()).partial_cmp(&(*ii2.borrow())).unwrap(),
    ));
    assert!(({ (*r1.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        (*cii.borrow())
            .partial_cmp(&({ (*ii2.borrow()).clone() }).into())
            .unwrap(),
    ));
    assert!(({ (*r2.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
}
pub fn test_type_8() {
    let ii1: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    let ii2: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    let ii4: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
    let cii: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    assert!(({ (*ii1.borrow()) == (*ii2.borrow()) }));
    assert!(({ (*ii1.borrow()) == (*ii4.borrow()) }));
    assert!((*ii1.borrow()) == (*cii.borrow()));
    assert!(({ !((*ii1.borrow()) != (*ii2.borrow())) }));
    assert!(({ !((*ii1.borrow()) != (*cii.borrow())) }));
    let v: Value<i32> = Rc::new(RefCell::new(0_i32));
    let c: Value<std_span_int__18446744073709551615_> = Rc::new(RefCell::new(
        std_span_int__18446744073709551615_::new_1({ (v.as_pointer()) }, { 1_u64 }),
    ));
    assert!(({ (*c.borrow()).begin() }) == ({ begin_9(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).rbegin() }) == ({ rbegin_10(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).cbegin() }) == ({ cbegin_11(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).crbegin() }) == ({ crbegin_12(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).end() }) == ({ libcc2rs::end_refcount(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).rend() }) == ({ rend_13(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).cend() }) == ({ cend_14(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).crend() }) == ({ crend_15(c.as_pointer(),) }));
    assert!(({ begin_9(c.as_pointer(),) }) != ({ libcc2rs::end_refcount(c.as_pointer(),) }));
    assert!(({ rbegin_10(c.as_pointer(),) }) != ({ rend_13(c.as_pointer(),) }));
    assert!(({ cbegin_11(c.as_pointer(),) }) != ({ cend_14(c.as_pointer(),) }));
    assert!(({ crbegin_12(c.as_pointer(),) }) != ({ crend_15(c.as_pointer(),) }));
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        (*ii1.borrow()).partial_cmp(&(*ii2.borrow())).unwrap(),
    ));
    assert!(({ (*r1.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        (*cii.borrow())
            .partial_cmp(&({ (*ii2.borrow()).clone() }).into())
            .unwrap(),
    ));
    assert!(({ (*r2.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
}
pub fn test_type_16() {
    let ii1: Value<RandomAccessIterator<Vec<i8>>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    let ii2: Value<RandomAccessIterator<Vec<i8>>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    let ii4: Value<RandomAccessIterator<Vec<i8>>> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
    let cii: Value<RandomAccessIterator<Vec<i8>>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    assert!(({ (*ii1.borrow()) == (*ii2.borrow()) }));
    assert!(({ (*ii1.borrow()) == (*ii4.borrow()) }));
    assert!((*ii1.borrow()) == (*cii.borrow()));
    assert!(({ !((*ii1.borrow()) != (*ii2.borrow())) }));
    assert!(({ !((*ii1.borrow()) != (*cii.borrow())) }));
    let v: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    let c : Value<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { ( v .as_pointer()  )  } , { 1_u64  } , )  )) ;
    assert!(({ (*c.borrow()).begin() }) == ({ begin_17(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).rbegin() }) == ({ rbegin_18(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).cbegin() }) == ({ cbegin_19(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).crbegin() }) == ({ crbegin_20(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).end() }) == ({ libcc2rs::end_refcount(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).rend() }) == ({ rend_21(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).cend() }) == ({ cend_22(c.as_pointer(),) }));
    assert!(({ (*c.borrow()).crend() }) == ({ crend_23(c.as_pointer(),) }));
    assert!(({ begin_17(c.as_pointer(),) }) != ({ libcc2rs::end_refcount(c.as_pointer(),) }));
    assert!(({ rbegin_18(c.as_pointer(),) }) != ({ rend_21(c.as_pointer(),) }));
    assert!(({ cbegin_19(c.as_pointer(),) }) != ({ cend_22(c.as_pointer(),) }));
    assert!(({ crbegin_20(c.as_pointer(),) }) != ({ crend_23(c.as_pointer(),) }));
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        (*ii1.borrow()).partial_cmp(&(*ii2.borrow())).unwrap(),
    ));
    assert!(({ (*r1.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        (*cii.borrow())
            .partial_cmp(&({ (*ii2.borrow()).clone() }).into())
            .unwrap(),
    ));
    assert!(({ (*r2.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
}
pub fn test_24() -> bool {
    ({ test_type_0() });
    ({ test_type_8() });
    ({ test_type_16() });
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
    ({ test_24() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
