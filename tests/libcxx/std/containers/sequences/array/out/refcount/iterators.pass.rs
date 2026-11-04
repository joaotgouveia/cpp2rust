extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(1)]
pub struct NoDefault {}
impl NoDefault {
    pub fn new(mut _a0: i32) -> Self {
        Self {}
    }
}
impl CppMove for NoDefault {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn check_noexcept_0(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: Ptr<Vec<i32>> = (c).clone();
    &(*cc.upgrade().deref());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn check_noexcept_1(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: Ptr<Vec<i32>> = (c).clone();
    &(*cc.upgrade().deref());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn check_noexcept_2(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: Ptr<Vec<i32>> = (c).clone();
    &(*cc.upgrade().deref());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn check_noexcept_3(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: Ptr<Vec<i32>> = (c).clone();
    &(*cc.upgrade().deref());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn check_noexcept_4(c: Ptr<Vec<f64>>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: Ptr<Vec<f64>> = (c).clone();
    &(*cc.upgrade().deref());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn tests_5() -> bool {
    {
        let array: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 5, _>(|_| 0_i32).to_vec(),
        ));
        ({ check_noexcept_0(array.as_pointer()) });
        let mut i: Ptr<i32> = ({ (*array.borrow_mut()).begin_3() });
        let mut j: Ptr<i32> = ({ (*array.borrow()).cbegin() });
        assert!(({ (i).clone() } == { (j).clone() }));
    }
    {
        let array: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        ({ check_noexcept_1(array.as_pointer()) });
        let mut i: Ptr<i32> = ({ (*array.borrow_mut()).begin_3() });
        let mut j: Ptr<i32> = ({ (*array.borrow()).cbegin() });
        assert!(({ (i).clone() } == { (j).clone() }));
    }
    {
        let array: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        ({ check_noexcept_1(array.as_pointer()) });
        let mut i: Ptr<i32> = ({ (*array.borrow_mut()).begin_3() });
        let mut j: Ptr<i32> = ({ (*array.borrow()).cbegin() });
        assert!(({ ({ (i).clone() } == { ({ (*array.borrow_mut()).end_5() }) }) }));
        assert!(({ ({ (j).clone() } == { ({ (*array.borrow()).cend() }) }) }));
    }
    {
        let array: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1])));
        ({ check_noexcept_2(array.as_pointer()) });
        let mut i: Ptr<i32> = ({ (*array.borrow_mut()).begin_3() });
        assert!(((i.read()) == 1));
        assert!(({ (i).clone() } == { array.decay() }));
        i.write(99);
        assert!(((array.decay().offset(0_usize as isize).read()) == 99));
    }
    {
        let array: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2])));
        ({ check_noexcept_3(array.as_pointer()) });
        let mut i: Ptr<i32> = ({ (*array.borrow_mut()).begin_3() });
        assert!(((i.read()) == 1));
        assert!(({ (i).clone() } == { array.decay() }));
        i.write(99);
        assert!(((array.decay().offset(0_usize as isize).read()) == 99));
        assert!(((array.decay().offset(1_usize as isize).read()) == 2));
    }
    {
        let array: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([1_f64, 2_f64, 3.5_f64])));
        ({ check_noexcept_4(array.as_pointer()) });
        let mut i: Ptr<f64> = ({ (*array.borrow_mut()).begin_3() });
        assert!(((i.read()) == 1_f64));
        assert!(({ (i).clone() } == { array.decay() }));
        i.write(5.5_f64);
        assert!(((array.decay().offset(0_usize as isize).read()) == 5.5_f64));
        assert!(((array.decay().offset(1_usize as isize).read()) == 2_f64));
    }
    {
        let array: Value<Vec<NoDefault>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| <NoDefault>::default()).to_vec(),
        ));
        let mut ib: Ptr<NoDefault> = ({ (*array.borrow_mut()).begin_3() });
        let mut ie: Ptr<NoDefault> = ({ (*array.borrow_mut()).end_5() });
        assert!(({ (ib).clone() } == { (ie).clone() }));
    }
    {
        {
            let mut ii1: Ptr<i32> = Ptr::null();
            let mut ii2: Ptr<i32> = Ptr::null();
            let mut ii4: Ptr<i32> = (ii1).clone();
            let mut cii: Ptr<i32> = Ptr::null();
            assert!(({ (ii1).clone() } == { (ii2).clone() }));
            assert!(({ (ii1).clone() } == { (ii4).clone() }));
            assert!(({ (ii1).clone() } == { (cii).clone() }));
            assert!(!({ (ii1).clone() } != { (ii2).clone() }));
            assert!(!({ (ii1).clone() } != { (cii).clone() }));
            let c: Value<Vec<i32>> = Rc::new(RefCell::new(
                std::array::from_fn::<_, 5, _>(|_| 0_i32).to_vec(),
            ));
            ({ check_noexcept_0(c.as_pointer()) });
            assert!(({ (({ (*c.borrow_mut()).begin_3() }) == ({ begin_6(c.as_pointer(),) })) }));
            assert!(({ (({ (*c.borrow()).cbegin() }) == ({ cbegin_7(c.as_pointer(),) })) }));
            assert!(({ (*c.borrow_mut()).rbegin_7() }) == ({ rbegin_8(c.as_pointer(),) }));
            assert!(({ (*c.borrow()).crbegin() }) == ({ crbegin_9(c.as_pointer(),) }));
            assert!(
                ({
                    (({ (*c.borrow_mut()).end_5() })
                        == ({ libcc2rs::end_refcount(c.as_pointer()) }))
                })
            );
            assert!(({ (({ (*c.borrow()).cend() }) == ({ cend_10(c.as_pointer(),) })) }));
            assert!(({ (*c.borrow_mut()).rend_9() }) == ({ rend_11(c.as_pointer(),) }));
            assert!(({ (*c.borrow()).crend() }) == ({ crend_12(c.as_pointer(),) }));
            assert!(
                (({ begin_6(c.as_pointer(),) }) != ({ libcc2rs::end_refcount(c.as_pointer(),) }))
            );
            assert!(({ rbegin_8(c.as_pointer(),) }) != ({ rend_11(c.as_pointer(),) }));
            assert!((({ cbegin_7(c.as_pointer(),) }) != ({ cend_10(c.as_pointer(),) })));
            assert!(({ crbegin_9(c.as_pointer(),) }) != ({ crend_12(c.as_pointer(),) }));
            let r1: Value<std::cmp::Ordering> =
                Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
            assert!(
                ({ (*r1.borrow()) })
                    == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) })
            );
            let r2: Value<std::cmp::Ordering> =
                Rc::new(RefCell::new(std::cmp::Ord::cmp(&(cii), &(ii2))));
            assert!(
                ({ (*r2.borrow()) })
                    == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) })
            );
        }
        {
            let mut ii1: Ptr<i32> = Ptr::null();
            let mut ii2: Ptr<i32> = Ptr::null();
            let mut ii4: Ptr<i32> = (ii1).clone();
            let mut cii: Ptr<i32> = Ptr::null();
            assert!(({ (ii1).clone() } == { (ii2).clone() }));
            assert!(({ (ii1).clone() } == { (ii4).clone() }));
            assert!(!({ (ii1).clone() } != { (ii2).clone() }));
            assert!(({ (ii1).clone() } == { (cii).clone() }));
            assert!(({ (cii).clone() } == { (ii1).clone() }));
            assert!(!({ (ii1).clone() } != { (cii).clone() }));
            assert!(!({ (cii).clone() } != { (ii1).clone() }));
            assert!(!({ (ii1).clone() } < { (cii).clone() }));
            assert!(!({ (cii).clone() } < { (ii1).clone() }));
            assert!(({ (ii1).clone() } <= { (cii).clone() }));
            assert!(({ (cii).clone() } <= { (ii1).clone() }));
            assert!(!({ (ii1).clone() } > { (cii).clone() }));
            assert!(!({ (cii).clone() } > { (ii1).clone() }));
            assert!(({ (ii1).clone() } >= { (cii).clone() }));
            assert!(({ (cii).clone() } >= { (ii1).clone() }));
            assert!((((cii).clone() - (ii1).clone()) as i64 == 0_i64));
            assert!((((ii1).clone() - (cii).clone()) as i64 == 0_i64));
            let c: Value<Vec<i32>> = Rc::new(RefCell::new(
                std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ));
            ({ check_noexcept_1(c.as_pointer()) });
            assert!(({ (({ (*c.borrow_mut()).begin_3() }) == ({ begin_13(c.as_pointer(),) })) }));
            assert!(({ (({ (*c.borrow()).cbegin() }) == ({ cbegin_14(c.as_pointer(),) })) }));
            assert!(({ (*c.borrow_mut()).rbegin_7() }) == ({ rbegin_15(c.as_pointer(),) }));
            assert!(({ (*c.borrow()).crbegin() }) == ({ crbegin_16(c.as_pointer(),) }));
            assert!(
                ({
                    (({ (*c.borrow_mut()).end_5() })
                        == ({ libcc2rs::end_refcount(c.as_pointer()) }))
                })
            );
            assert!(({ (({ (*c.borrow()).cend() }) == ({ cend_17(c.as_pointer(),) })) }));
            assert!(({ (*c.borrow_mut()).rend_9() }) == ({ rend_18(c.as_pointer(),) }));
            assert!(({ (*c.borrow()).crend() }) == ({ crend_19(c.as_pointer(),) }));
            assert!(
                (({ begin_13(c.as_pointer(),) }) == ({ libcc2rs::end_refcount(c.as_pointer(),) }))
            );
            assert!(({ rbegin_15(c.as_pointer(),) }) == ({ rend_18(c.as_pointer(),) }));
            assert!((({ cbegin_14(c.as_pointer(),) }) == ({ cend_17(c.as_pointer(),) })));
            assert!(({ crbegin_16(c.as_pointer(),) }) == ({ crend_19(c.as_pointer(),) }));
            let r1: Value<std::cmp::Ordering> =
                Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
            assert!(
                ({ (*r1.borrow()) })
                    == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) })
            );
            let r2: Value<std::cmp::Ordering> =
                Rc::new(RefCell::new(std::cmp::Ord::cmp(&(cii), &(ii2))));
            assert!(
                ({ (*r2.borrow()) })
                    == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) })
            );
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
    ({ tests_5() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
