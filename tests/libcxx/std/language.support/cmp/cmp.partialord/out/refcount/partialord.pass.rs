extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
pub fn DoNotOptimize_1(value: Ptr<Ptr<()>>) -> Ptr<Ptr<()>> {
    b"";
    return (value).clone();
}
thread_local!(
    pub static sink_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn test_static_members_3() {
    ({
        let _value: Value<Ptr<()>> = Rc::new(RefCell::new((unimplemented!())));
        DoNotOptimize_1(_value.as_pointer())
    });
    ({
        let _value: Value<Ptr<()>> = Rc::new(RefCell::new((unimplemented!())));
        DoNotOptimize_1(_value.as_pointer())
    });
    ({
        let _value: Value<Ptr<()>> = Rc::new(RefCell::new((unimplemented!())));
        DoNotOptimize_1(_value.as_pointer())
    });
    ({
        let _value: Value<Ptr<()>> = Rc::new(RefCell::new((unimplemented!())));
        DoNotOptimize_1(_value.as_pointer())
    });
}
pub fn test_signatures_4() {
    let Eq: Ptr<()> = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_equality_5() {
    let PartialEq: Ptr<()> = unimplemented!();
    let WeakEq: Ptr<()> = unimplemented!();
    unimplemented!();
    let StrongEq: Ptr<()> = unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_6() -> () {
    let Eq: Ptr<()> = unimplemented!();
    let Less: Ptr<()> = unimplemented!();
    let Greater: Ptr<()> = unimplemented!();
    let Unord: Ptr<()> = unimplemented!();
    let TestCases: Value<Box<[anon_7]>> = Rc::new(RefCell::new(Box::new([
        anon_7 {
            Value: unimplemented!(),
            ExpectEq: true,
            ExpectNeq: false,
            ExpectLess: false,
            ExpectGreater: false,
        },
        anon_7 {
            Value: unimplemented!(),
            ExpectEq: false,
            ExpectNeq: true,
            ExpectLess: true,
            ExpectGreater: false,
        },
        anon_7 {
            Value: unimplemented!(),
            ExpectEq: false,
            ExpectNeq: true,
            ExpectLess: false,
            ExpectGreater: true,
        },
        anon_7 {
            Value: unimplemented!(),
            ExpectEq: false,
            ExpectNeq: true,
            ExpectLess: false,
            ExpectGreater: false,
        },
    ])));
    'loop_: for TC in IterRange::new(
        (TestCases.as_pointer() as Ptr<anon_7>),
        (TestCases.as_pointer() as Ptr<anon_7>).offset((4_()) as isize),
    ) {
        let mut TC: anon_7 = (*TC.upgrade().deref()).clone();
        let V: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let res: Value<()> = Rc::new(RefCell::new((unimplemented!())));
        (&(*res.borrow_mut()));
        unimplemented!();
        (&(*res.borrow_mut()));
    };
    let SpaceshipTestCases: Value<Box<[anon_9]>> = Rc::new(RefCell::new(Box::new([
        anon_9 {
            Value: unimplemented!(),
            Expect: ExpectRes_8_ER_Equiv,
        },
        anon_9 {
            Value: unimplemented!(),
            Expect: ExpectRes_8_ER_Less,
        },
        anon_9 {
            Value: unimplemented!(),
            Expect: ExpectRes_8_ER_Greater,
        },
        anon_9 {
            Value: unimplemented!(),
            Expect: ExpectRes_8_ER_Unord,
        },
    ])));
    'loop_: for TC in IterRange::new(
        (SpaceshipTestCases.as_pointer() as Ptr<anon_9>),
        (SpaceshipTestCases.as_pointer() as Ptr<anon_9>).offset((4_()) as isize),
    ) {
        let mut TC: anon_9 = (*TC.upgrade().deref()).clone();
        let Res: Value<()> = Rc::new(RefCell::new((unimplemented!())));
        'switch: {
            match { (TC.Expect as ()) } {
                __v if __v == (ExpectRes_8_ER_Equiv as ()) => {
                    unimplemented!();
                    unimplemented!();
                    break 'switch;
                }
                __v if __v == (ExpectRes_8_ER_Less as ()) => {
                    unimplemented!();
                    break 'switch;
                }
                __v if __v == (ExpectRes_8_ER_Greater as ()) => {
                    unimplemented!();
                    break 'switch;
                }
                __v if __v == (ExpectRes_8_ER_Unord as ()) => {
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    break 'switch;
                }
                _ => {}
            }
        };
    }
    {
        const _: () = assert!(
            true,
            "std::partial_ordering::less == std::partial_ordering::less"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::less !=\n                  std::partial_ordering::equivalent"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::less !=\n                  std::partial_ordering::greater"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::less !=\n                  std::partial_ordering::unordered"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::equivalent !=\n                  std::partial_ordering::less"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::equivalent ==\n                  std::partial_ordering::equivalent"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::equivalent !=\n                  std::partial_ordering::greater"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::equivalent !=\n                  std::partial_ordering::unordered"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::greater !=\n                  std::partial_ordering::less"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::greater !=\n                  std::partial_ordering::equivalent"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::greater ==\n                  std::partial_ordering::greater"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::greater !=\n                  std::partial_ordering::unordered"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::unordered !=\n                  std::partial_ordering::less"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::unordered !=\n                  std::partial_ordering::equivalent"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::unordered !=\n                  std::partial_ordering::greater"
        );;
        const _: () = assert!(
            true,
            "std::partial_ordering::unordered ==\n                  std::partial_ordering::unordered"
        );;
    }
    ({ test_equality_5() });
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(5)]
pub struct anon_7 {
    #[offset(0)]
    #[byte_size(1)]
    pub Value: (),
    #[offset(1)]
    pub ExpectEq: (),
    #[offset(2)]
    pub ExpectNeq: (),
    #[offset(3)]
    pub ExpectLess: (),
    #[offset(4)]
    pub ExpectGreater: (),
}
pub type ExpectRes_8 = ();
pub const ExpectRes_8_ER_Greater: ExpectRes_8 = 0;
pub const ExpectRes_8_ER_Less: ExpectRes_8 = 1;
pub const ExpectRes_8_ER_Equiv: ExpectRes_8 = 2;
pub const ExpectRes_8_ER_Unord: ExpectRes_8 = 3;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_9 {
    #[offset(0)]
    #[byte_size(1)]
    pub Value: (),
    #[offset(4)]
    pub Expect: ExpectRes_8,
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_static_members_3() });
    ({ test_signatures_4() });
    ({ test_equality_5() });
    const _: () = assert!(true, "test_constexpr()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = sink_2.with(|_| ());
}
