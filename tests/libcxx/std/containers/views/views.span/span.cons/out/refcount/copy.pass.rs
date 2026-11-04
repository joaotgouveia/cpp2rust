extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_int__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_int__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_int__18446744073709551615_::new_1({ (array.as_pointer() as Ptr<i32>) }, {
                3_u64
            }),
        ));
        let copy: Value<std_span_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_int__18446744073709551615_::new_1({ (array.as_pointer() as Ptr<i32>) }, {
                2_u64
            }),
        ));
        let copy: Value<std_span_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_int__0_> = Rc::new(RefCell::new(<std_span_int__0_>::default()));
        let copy: Value<std_span_int__0_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_int__3_> = Rc::new(RefCell::new(std_span_int__3_::new_1({
            array.as_pointer()
        })));
        let copy: Value<std_span_int__3_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32])));
        let x: Value<std_span_int__2_> = Rc::new(RefCell::new(std_span_int__2_::new_1({
            array.as_pointer()
        })));
        let copy: Value<std_span_int__2_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_1() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_const_int__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_const_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_int__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i32>) },
                { 3_u64 },
            ),
        ));
        let copy: Value<std_span_const_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_int__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i32>) },
                { 2_u64 },
            ),
        ));
        let copy: Value<std_span_const_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_const_int__0_> =
            Rc::new(RefCell::new(<std_span_const_int__0_>::default()));
        let copy: Value<std_span_const_int__0_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_const_int__3_> =
            Rc::new(RefCell::new(std_span_const_int__3_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_int__3_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32])));
        let x: Value<std_span_const_int__2_> =
            Rc::new(RefCell::new(std_span_const_int__2_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_int__2_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_2() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_volatile_int__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_volatile_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_volatile_int__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i32>) },
                { 3_u64 },
            ),
        ));
        let copy: Value<std_span_volatile_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_volatile_int__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i32>) },
                { 2_u64 },
            ),
        ));
        let copy: Value<std_span_volatile_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_volatile_int__0_> =
            Rc::new(RefCell::new(<std_span_volatile_int__0_>::default()));
        let copy: Value<std_span_volatile_int__0_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_volatile_int__3_> =
            Rc::new(RefCell::new(std_span_volatile_int__3_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_volatile_int__3_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32])));
        let x: Value<std_span_volatile_int__2_> =
            Rc::new(RefCell::new(std_span_volatile_int__2_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_volatile_int__2_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_3() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_const_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_const_volatile_int__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_const_volatile_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_const_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_volatile_int__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i32>) },
                { 3_u64 },
            ),
        ));
        let copy: Value<std_span_const_volatile_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_const_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_volatile_int__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i32>) },
                { 2_u64 },
            ),
        ));
        let copy: Value<std_span_const_volatile_int__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_const_volatile_int__0_> =
            Rc::new(RefCell::new(<std_span_const_volatile_int__0_>::default()));
        let copy: Value<std_span_const_volatile_int__0_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let x: Value<std_span_const_volatile_int__3_> =
            Rc::new(RefCell::new(std_span_const_volatile_int__3_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_volatile_int__3_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32])));
        let x: Value<std_span_const_volatile_int__2_> =
            Rc::new(RefCell::new(std_span_const_volatile_int__2_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_volatile_int__2_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i32>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_4() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_long__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_long__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_long__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_long__18446744073709551615_::new_1({ (array.as_pointer() as Ptr<i64>) }, {
                3_u64
            }),
        ));
        let copy: Value<std_span_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_long__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_long__18446744073709551615_::new_1({ (array.as_pointer() as Ptr<i64>) }, {
                2_u64
            }),
        ));
        let copy: Value<std_span_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_long__0_> = Rc::new(RefCell::new(<std_span_long__0_>::default()));
        let copy: Value<std_span_long__0_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_long__3_> = Rc::new(RefCell::new(std_span_long__3_::new_1({
            array.as_pointer()
        })));
        let copy: Value<std_span_long__3_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64])));
        let x: Value<std_span_long__2_> = Rc::new(RefCell::new(std_span_long__2_::new_1({
            array.as_pointer()
        })));
        let copy: Value<std_span_long__2_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_5() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_const_long__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_const_long__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_const_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_const_long__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_long__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i64>) },
                { 3_u64 },
            ),
        ));
        let copy: Value<std_span_const_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_const_long__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_long__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i64>) },
                { 2_u64 },
            ),
        ));
        let copy: Value<std_span_const_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_const_long__0_> =
            Rc::new(RefCell::new(<std_span_const_long__0_>::default()));
        let copy: Value<std_span_const_long__0_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_const_long__3_> =
            Rc::new(RefCell::new(std_span_const_long__3_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_long__3_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64])));
        let x: Value<std_span_const_long__2_> =
            Rc::new(RefCell::new(std_span_const_long__2_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_long__2_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_6() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_volatile_long__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_volatile_long__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_volatile_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_volatile_long__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_volatile_long__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i64>) },
                { 3_u64 },
            ),
        ));
        let copy: Value<std_span_volatile_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_volatile_long__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_volatile_long__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i64>) },
                { 2_u64 },
            ),
        ));
        let copy: Value<std_span_volatile_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_volatile_long__0_> =
            Rc::new(RefCell::new(<std_span_volatile_long__0_>::default()));
        let copy: Value<std_span_volatile_long__0_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_volatile_long__3_> =
            Rc::new(RefCell::new(std_span_volatile_long__3_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_volatile_long__3_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64])));
        let x: Value<std_span_volatile_long__2_> =
            Rc::new(RefCell::new(std_span_volatile_long__2_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_volatile_long__2_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_7() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_const_volatile_long__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_const_volatile_long__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_const_volatile_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_const_volatile_long__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_volatile_long__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i64>) },
                { 3_u64 },
            ),
        ));
        let copy: Value<std_span_const_volatile_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_const_volatile_long__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_volatile_long__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<i64>) },
                { 2_u64 },
            ),
        ));
        let copy: Value<std_span_const_volatile_long__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_const_volatile_long__0_> =
            Rc::new(RefCell::new(<std_span_const_volatile_long__0_>::default()));
        let copy: Value<std_span_const_volatile_long__0_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64])));
        let x: Value<std_span_const_volatile_long__3_> =
            Rc::new(RefCell::new(std_span_const_volatile_long__3_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_volatile_long__3_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64])));
        let x: Value<std_span_const_volatile_long__2_> =
            Rc::new(RefCell::new(std_span_const_volatile_long__2_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_volatile_long__2_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<i64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_8() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_double__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_double__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_double__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_double__18446744073709551615_::new_1({ (array.as_pointer() as Ptr<f64>) }, {
                3_u64
            }),
        ));
        let copy: Value<std_span_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_double__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_double__18446744073709551615_::new_1({ (array.as_pointer() as Ptr<f64>) }, {
                2_u64
            }),
        ));
        let copy: Value<std_span_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_double__0_> = Rc::new(RefCell::new(<std_span_double__0_>::default()));
        let copy: Value<std_span_double__0_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_double__3_> = Rc::new(RefCell::new(std_span_double__3_::new_1({
            array.as_pointer()
        })));
        let copy: Value<std_span_double__3_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64])));
        let x: Value<std_span_double__2_> = Rc::new(RefCell::new(std_span_double__2_::new_1({
            array.as_pointer()
        })));
        let copy: Value<std_span_double__2_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_const_double__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_const_double__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_const_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_const_double__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_double__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<f64>) },
                { 3_u64 },
            ),
        ));
        let copy: Value<std_span_const_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_const_double__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_double__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<f64>) },
                { 2_u64 },
            ),
        ));
        let copy: Value<std_span_const_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_const_double__0_> =
            Rc::new(RefCell::new(<std_span_const_double__0_>::default()));
        let copy: Value<std_span_const_double__0_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_const_double__3_> =
            Rc::new(RefCell::new(std_span_const_double__3_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_double__3_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64])));
        let x: Value<std_span_const_double__2_> =
            Rc::new(RefCell::new(std_span_const_double__2_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_double__2_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_10() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_volatile_double__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_volatile_double__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_volatile_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_volatile_double__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_volatile_double__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<f64>) },
                { 3_u64 },
            ),
        ));
        let copy: Value<std_span_volatile_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_volatile_double__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_volatile_double__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<f64>) },
                { 2_u64 },
            ),
        ));
        let copy: Value<std_span_volatile_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_volatile_double__0_> =
            Rc::new(RefCell::new(<std_span_volatile_double__0_>::default()));
        let copy: Value<std_span_volatile_double__0_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_volatile_double__3_> =
            Rc::new(RefCell::new(std_span_volatile_double__3_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_volatile_double__3_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64])));
        let x: Value<std_span_volatile_double__2_> =
            Rc::new(RefCell::new(std_span_volatile_double__2_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_volatile_double__2_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_11() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_const_volatile_double__18446744073709551615_> = Rc::new(
            RefCell::new(<std_span_const_volatile_double__18446744073709551615_>::default()),
        );
        let copy: Value<std_span_const_volatile_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_const_volatile_double__18446744073709551615_> =
            Rc::new(RefCell::new(
                std_span_const_volatile_double__18446744073709551615_::new_1(
                    { (array.as_pointer() as Ptr<f64>) },
                    { 3_u64 },
                ),
            ));
        let copy: Value<std_span_const_volatile_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_const_volatile_double__18446744073709551615_> =
            Rc::new(RefCell::new(
                std_span_const_volatile_double__18446744073709551615_::new_1(
                    { (array.as_pointer() as Ptr<f64>) },
                    { 2_u64 },
                ),
            ));
        let copy: Value<std_span_const_volatile_double__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_const_volatile_double__0_> =
            Rc::new(RefCell::new(<std_span_const_volatile_double__0_>::default()));
        let copy: Value<std_span_const_volatile_double__0_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64, 0_f64])));
        let x: Value<std_span_const_volatile_double__3_> =
            Rc::new(RefCell::new(std_span_const_volatile_double__3_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_volatile_double__3_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64])));
        let x: Value<std_span_const_volatile_double__2_> =
            Rc::new(RefCell::new(std_span_const_volatile_double__2_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_volatile_double__2_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr::<f64>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_12() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_Foo__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_Foo__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_Foo__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
            <Foo>::default(),
            <Foo>::default(),
            <Foo>::default(),
        ])));
        let x: Value<std_span_Foo__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_Foo__18446744073709551615_::new_1({ (array.as_pointer() as Ptr<Foo>) }, {
                3_u64
            }),
        ));
        let copy: Value<std_span_Foo__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Foo>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
            <Foo>::default(),
            <Foo>::default(),
            <Foo>::default(),
        ])));
        let x: Value<std_span_Foo__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_Foo__18446744073709551615_::new_1({ (array.as_pointer() as Ptr<Foo>) }, {
                2_u64
            }),
        ));
        let copy: Value<std_span_Foo__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Foo>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_Foo__0_> = Rc::new(RefCell::new(<std_span_Foo__0_>::default()));
        let copy: Value<std_span_Foo__0_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
            <Foo>::default(),
            <Foo>::default(),
            <Foo>::default(),
        ])));
        let x: Value<std_span_Foo__3_> = Rc::new(RefCell::new(std_span_Foo__3_::new_1({
            array.as_pointer()
        })));
        let copy: Value<std_span_Foo__3_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Foo>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[Foo]>> =
            Rc::new(RefCell::new(Box::new([<Foo>::default(), <Foo>::default()])));
        let x: Value<std_span_Foo__2_> = Rc::new(RefCell::new(std_span_Foo__2_::new_1({
            array.as_pointer()
        })));
        let copy: Value<std_span_Foo__2_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Foo>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_13() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x: Value<std_span_const_Foo__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_const_Foo__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_const_Foo__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
            <Foo>::default(),
            <Foo>::default(),
            <Foo>::default(),
        ])));
        let x: Value<std_span_const_Foo__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_Foo__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<Foo>) },
                { 3_u64 },
            ),
        ));
        let copy: Value<std_span_const_Foo__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Foo>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
            <Foo>::default(),
            <Foo>::default(),
            <Foo>::default(),
        ])));
        let x: Value<std_span_const_Foo__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_Foo__18446744073709551615_::new_1(
                { (array.as_pointer() as Ptr<Foo>) },
                { 2_u64 },
            ),
        ));
        let copy: Value<std_span_const_Foo__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Foo>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x: Value<std_span_const_Foo__0_> =
            Rc::new(RefCell::new(<std_span_const_Foo__0_>::default()));
        let copy: Value<std_span_const_Foo__0_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
            <Foo>::default(),
            <Foo>::default(),
            <Foo>::default(),
        ])));
        let x: Value<std_span_const_Foo__3_> =
            Rc::new(RefCell::new(std_span_const_Foo__3_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_Foo__3_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Foo>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[Foo]>> =
            Rc::new(RefCell::new(Box::new([<Foo>::default(), <Foo>::default()])));
        let x: Value<std_span_const_Foo__2_> =
            Rc::new(RefCell::new(std_span_const_Foo__2_::new_1({
                array.as_pointer()
            })));
        let copy: Value<std_span_const_Foo__2_> = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Foo>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_14() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x : Value<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_  >::default() )) ;
        let copy : Value<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(((*x.borrow()) ).clone() )) ;
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
            vec![i8::default()],
            vec![i8::default()],
            vec![i8::default()],
        ])));
        let x : Value<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { ( array .as_pointer()  as Ptr< Vec<i8  >  >   )  } , { 3_u64  } , )  )) ;
        let copy : Value<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(((*x.borrow()) ).clone() )) ;
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Vec<i8>>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
            vec![i8::default()],
            vec![i8::default()],
            vec![i8::default()],
        ])));
        let x : Value<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { ( array .as_pointer()  as Ptr< Vec<i8  >  >   )  } , { 2_u64  } , )  )) ;
        let copy : Value<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(((*x.borrow()) ).clone() )) ;
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Vec<i8>>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x : Value<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____0_ > = Rc::new(RefCell::new(<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____0_  >::default() )) ;
        let copy: Value<
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____0_,
        > = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
            vec![i8::default()],
            vec![i8::default()],
            vec![i8::default()],
        ])));
        let x: Value<
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____3_,
        > = Rc::new(RefCell::new(
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____3_::new_1(
                { array.as_pointer() },
            ),
        ));
        let copy: Value<
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____3_,
        > = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Vec<i8>>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
            vec![i8::default()],
            vec![i8::default()],
        ])));
        let x: Value<
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____2_,
        > = Rc::new(RefCell::new(
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____2_::new_1(
                { array.as_pointer() },
            ),
        ));
        let copy: Value<
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____2_,
        > = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Vec<i8>>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
pub fn test_15() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let x : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_  >::default() )) ;
        let copy : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(((*x.borrow()) ).clone() )) ;
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
            vec![i8::default()],
            vec![i8::default()],
            vec![i8::default()],
        ])));
        let x : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { ( array .as_pointer()  as Ptr< Vec<i8  >  >   )  } , { 3_u64  } , )  )) ;
        let copy : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(((*x.borrow()) ).clone() )) ;
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Vec<i8>>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
            vec![i8::default()],
            vec![i8::default()],
            vec![i8::default()],
        ])));
        let x : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { ( array .as_pointer()  as Ptr< Vec<i8  >  >   )  } , { 2_u64  } , )  )) ;
        let copy : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(((*x.borrow()) ).clone() )) ;
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Vec<i8>>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
    {
        let x : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____0_ > = Rc::new(RefCell::new(<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____0_  >::default() )) ;
        let copy: Value<
            std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____0_,
        > = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
    }
    {
        let array: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
            vec![i8::default()],
            vec![i8::default()],
            vec![i8::default()],
        ])));
        let x : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____3_ > = Rc::new(RefCell::new(std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____3_ :: new_1 ( { array .as_pointer()  } , )  )) ;
        let copy: Value<
            std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____3_,
        > = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Vec<i8>>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
    }
    {
        let array: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
            vec![i8::default()],
            vec![i8::default()],
        ])));
        let x : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____2_ > = Rc::new(RefCell::new(std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____2_ :: new_1 ( { array .as_pointer()  } , )  )) ;
        let copy: Value<
            std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____2_,
        > = Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == (array.as_pointer() as Ptr<Vec<i8>>)) }));
        assert!(({ (({ (*copy.borrow()).size() }) == 2_u64) }));
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Foo {}
impl CppMove for Foo {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_all_16() -> bool {
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    ({ test_6() });
    ({ test_7() });
    ({ test_8() });
    ({ test_9() });
    ({ test_10() });
    ({ test_11() });
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    ({ test_15() });
    {
        let x: Value<std_span_Incomplete__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_Incomplete__18446744073709551615_>::default(),
        ));
        let copy: Value<std_span_Incomplete__18446744073709551615_> =
            Rc::new(RefCell::new((*x.borrow()).clone()));
        assert!(({ (({ (*copy.borrow()).data() }) == ({ (*x.borrow()).data() })) }));
        assert!(({ (({ (*copy.borrow()).size() }) == ({ (*x.borrow()).size() })) }));
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
    ({ test_all_16() });
    const _: () = assert!(true, "test_all()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
