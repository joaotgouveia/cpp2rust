extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn validate_prefixes_0(empty_matching_pattern: Ptr<()>) {
    let source: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(!(unimplemented!().with(|__s| __s.matched)));
    assert!((unimplemented!() == 0_()));
    assert!(
        ({
            (*((*unimplemented!().upgrade().deref()) as (Value<*const i8>, Value<*const i8>))
                .0
                .borrow())
            .clone()
        } == { (source.as_pointer() as Ptr::<i8>) })
    );
    assert!(
        ({
            (*((*unimplemented!().upgrade().deref()) as (Value<*const i8>, Value<*const i8>))
                .1
                .borrow())
            .clone()
        } == { (source.as_pointer() as Ptr::<i8>) })
    );
    unimplemented!();
    assert!(unimplemented!().with(|__s| __s.matched));
    assert!((unimplemented!() == 1_()));
    assert!(
        ({
            (*((*unimplemented!().upgrade().deref()) as (Value<*const i8>, Value<*const i8>))
                .0
                .borrow())
            .clone()
        } == { (source.as_pointer() as Ptr::<i8>) })
    );
    assert!(
        ({
            (*((*unimplemented!().upgrade().deref()) as (Value<*const i8>, Value<*const i8>))
                .1
                .borrow())
            .clone()
        } == { (source.as_pointer() as Ptr::<i8>).offset((1) as isize) })
    );
    assert!(
        Ptr::<i8>::from_string_literal(b"a")
            .with_c_str(|__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s)
    );
    unimplemented!();
    assert!(unimplemented!().with(|__s| __s.matched));
    assert!((unimplemented!() == 1_()));
    assert!(
        ({
            (*((*unimplemented!().upgrade().deref()) as (Value<*const i8>, Value<*const i8>))
                .0
                .borrow())
            .clone()
        } == { (source.as_pointer() as Ptr::<i8>).offset((1) as isize) })
    );
    assert!(
        ({
            (*((*unimplemented!().upgrade().deref()) as (Value<*const i8>, Value<*const i8>))
                .1
                .borrow())
            .clone()
        } == { (source.as_pointer() as Ptr::<i8>).offset((2) as isize) })
    );
    assert!(
        Ptr::<i8>::from_string_literal(b"b")
            .with_c_str(|__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s)
    );
    unimplemented!();
    assert!(unimplemented!().with(|__s| __s.matched));
    assert!((unimplemented!() == 1_()));
    assert!(
        ({
            (*((*unimplemented!().upgrade().deref()) as (Value<*const i8>, Value<*const i8>))
                .0
                .borrow())
            .clone()
        } == { (source.as_pointer() as Ptr::<i8>).offset((2) as isize) })
    );
    assert!(
        ({
            (*((*unimplemented!().upgrade().deref()) as (Value<*const i8>, Value<*const i8>))
                .1
                .borrow())
            .clone()
        } == { (source.as_pointer() as Ptr::<i8>).offset((3) as isize) })
    );
    assert!(
        Ptr::<i8>::from_string_literal(b"c")
            .with_c_str(|__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s)
    );
    unimplemented!();
    assert!(unimplemented!());
}
pub fn test_prefix_adjustment_1() {
    ({
        let _empty_matching_pattern: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        validate_prefixes_0(_empty_matching_pattern.as_pointer())
    });
    ({
        let _empty_matching_pattern: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        validate_prefixes_0(_empty_matching_pattern.as_pointer())
    });
    ({
        let _empty_matching_pattern: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        validate_prefixes_0(_empty_matching_pattern.as_pointer())
    });
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
    {
        let phone_numbers: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let phone_book: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
            b"555-1234, 555-2345, 555-3456\0",
        )));
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((*i.borrow()) != unimplemented!());
        assert!((*i2.borrow()) != unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!((*i.borrow()) != unimplemented!());
        assert!((*i2.borrow()) != unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 10_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-2345").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!((*i.borrow()) != unimplemented!());
        assert!((*i2.borrow()) != unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 20_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-3456").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(unimplemented!());
        assert!((*i2.borrow()) != unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    }
    {
        let phone_numbers: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let phone_book: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
            b"555-1234, 555-2345, 555-3456\0",
        )));
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((*i.borrow()) != unimplemented!());
        assert!((*i2.borrow()) != unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!((*i.borrow()) != unimplemented!());
        assert!((*i2.borrow()) != unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 10_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-2345").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!((*i.borrow()) != unimplemented!());
        assert!((*i2.borrow()) != unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 20_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-3456").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(unimplemented!());
        assert!((*i2.borrow()) != unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"555-1234").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    }
    {
        let rex: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let foo: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"foo\0")));
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((*i.borrow()) != (*e.borrow()));
        assert!((unimplemented!() == 1_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"foo").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!((*i.borrow()) != (*e.borrow()));
        assert!((unimplemented!() == 1_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(unimplemented!());
    }
    ({ test_prefix_adjustment_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
