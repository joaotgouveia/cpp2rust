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
pub struct test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {}
impl CppMove for test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn DoNotOptimize_1(value: Ptr<Ptr<std::cmp::Ordering>>) -> Ptr<Ptr<std::cmp::Ordering>> {
    b"";
    return (value).clone();
}
thread_local!(
    pub static sink_2: Value<AnyPtr> = Rc::new(RefCell::new(AnyPtr::default()));
);
pub fn test_static_members_3() {
    ({
        let _value: Value<Ptr<std::cmp::Ordering>> =
            Rc::new(RefCell::new((libcc2rs::strong_ordering_less())));
        DoNotOptimize_1(_value.as_pointer())
    });
    ({
        let _value: Value<Ptr<std::cmp::Ordering>> =
            Rc::new(RefCell::new((libcc2rs::strong_ordering_equal())));
        DoNotOptimize_1(_value.as_pointer())
    });
    ({
        let _value: Value<Ptr<std::cmp::Ordering>> =
            Rc::new(RefCell::new((libcc2rs::strong_ordering_equivalent())));
        DoNotOptimize_1(_value.as_pointer())
    });
    ({
        let _value: Value<Ptr<std::cmp::Ordering>> =
            Rc::new(RefCell::new((libcc2rs::strong_ordering_greater())));
        DoNotOptimize_1(_value.as_pointer())
    });
}
pub fn test_signatures_4() {
    let Eq: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_equivalent();
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
    let StrongEq: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_equal();
    let PartialEq: Ptr<()> = unimplemented!();
    assert!(unimplemented!());
    let WeakEq: Ptr<()> = unimplemented!();
    assert!(unimplemented!());
}
pub fn test_conversion_6() -> bool {
    const _: () = assert!(
        true,
        "std::is_convertible<const std::strong_ordering&,\n      std::partial_ordering>::value"
    );;
    {
        let V: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
            ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
        ));
        let WV: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let V: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        ));
        let WV: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let V: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        ));
        let WV: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    const _: () = assert!(
        true,
        "std::is_convertible<const std::strong_ordering&,\n      std::weak_ordering>::value"
    );;
    {
        let V: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
            ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
        ));
        let WV: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let V: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        ));
        let WV: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let V: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        ));
        let WV: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    return true;
}
pub fn test_constexpr_7() -> bool {
    let Eq: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_equal();
    let Equiv: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_equivalent();
    let Less: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_less();
    let Greater: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_greater();
    let TestCases: Value<Box<[anon_8]>> = Rc::new(RefCell::new(Box::new([
        anon_8 {
            Value: ({ (*Eq.upgrade().deref()) }),
            ExpectEq: true,
            ExpectNeq: false,
            ExpectLess: false,
            ExpectGreater: false,
        },
        anon_8 {
            Value: ({ (*Equiv.upgrade().deref()) }),
            ExpectEq: true,
            ExpectNeq: false,
            ExpectLess: false,
            ExpectGreater: false,
        },
        anon_8 {
            Value: ({ (*Less.upgrade().deref()) }),
            ExpectEq: false,
            ExpectNeq: true,
            ExpectLess: true,
            ExpectGreater: false,
        },
        anon_8 {
            Value: ({ (*Greater.upgrade().deref()) }),
            ExpectEq: false,
            ExpectNeq: true,
            ExpectLess: false,
            ExpectGreater: true,
        },
    ])));
    'loop_: for TC in IterRange::new(
        (TestCases.as_pointer() as Ptr<anon_8>),
        (TestCases.as_pointer() as Ptr<anon_8>).offset((4_i64) as isize),
    ) {
        let mut TC: anon_8 = (*TC.upgrade().deref()).clone();
        let V: Value<std::cmp::Ordering> = Rc::new(RefCell::new(TC.Value));
        assert!(
            (((({ (*V.borrow()) }) == std::cmp::Ordering::Equal) as i32) == (TC.ExpectEq as i32))
        );
        assert!(
            (((({ (*V.borrow()) }) == std::cmp::Ordering::Equal) as i32) == (TC.ExpectEq as i32))
        );
        assert!(
            (((!(({ (*V.borrow()) }) == std::cmp::Ordering::Equal)) as i32)
                == (TC.ExpectNeq as i32))
        );
        assert!(
            (((!(({ (*V.borrow()) }) == std::cmp::Ordering::Equal)) as i32)
                == (TC.ExpectNeq as i32))
        );
        assert!(
            (((({ (*V.borrow()) }) == std::cmp::Ordering::Less) as i32) == (TC.ExpectLess as i32))
        );
        assert!(
            (((({ (*V.borrow()) }) == std::cmp::Ordering::Greater) as i32)
                == (TC.ExpectGreater as i32))
        );
        assert!(
            (((({ (*V.borrow()) }) != std::cmp::Ordering::Greater) as i32)
                == (((TC.ExpectLess) || (TC.ExpectEq)) as i32))
        );
        assert!(
            (((({ (*V.borrow()) }) != std::cmp::Ordering::Less) as i32)
                == (((TC.ExpectGreater) || (TC.ExpectEq)) as i32))
        );
        assert!((((unimplemented!()) as i32) == (TC.ExpectGreater as i32)));
        assert!((((unimplemented!()) as i32) == (TC.ExpectLess as i32)));
        assert!((((unimplemented!()) as i32) == (((TC.ExpectGreater) || (TC.ExpectEq)) as i32)));
        assert!((((unimplemented!()) as i32) == (((TC.ExpectLess) || (TC.ExpectEq)) as i32)));
    }
    {
        let res: Value<std::cmp::Ordering> = Rc::new(RefCell::new((unimplemented!())));
        (&(*res.borrow_mut()));
        unimplemented!();
        (&(*res.borrow_mut()));
    };
    let SpaceshipTestCases: Value<Box<[anon_10]>> = Rc::new(RefCell::new(Box::new([
        anon_10 {
            Value: ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
            Expect: ExpectRes_9_ER_Equiv,
        },
        anon_10 {
            Value: ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
            Expect: ExpectRes_9_ER_Less,
        },
        anon_10 {
            Value: ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
            Expect: ExpectRes_9_ER_Greater,
        },
    ])));
    'loop_: for TC in IterRange::new(
        (SpaceshipTestCases.as_pointer() as Ptr<anon_10>),
        (SpaceshipTestCases.as_pointer() as Ptr<anon_10>).offset((3_i64) as isize),
    ) {
        let mut TC: anon_10 = (*TC.upgrade().deref()).clone();
        let Res: Value<std::cmp::Ordering> = Rc::new(RefCell::new((unimplemented!())));
        'switch: {
            match { (TC.Expect as i32) } {
                __v if __v == (ExpectRes_9_ER_Equiv as i32) => {
                    assert!(({ (*Res.borrow()) }) == std::cmp::Ordering::Equal);
                    assert!(({ (*Res.borrow()) }) == std::cmp::Ordering::Equal);
                    break 'switch;
                }
                __v if __v == (ExpectRes_9_ER_Less as i32) => {
                    assert!(({ (*Res.borrow()) }) == std::cmp::Ordering::Less);
                    break 'switch;
                }
                __v if __v == (ExpectRes_9_ER_Greater as i32) => {
                    assert!(({ (*Res.borrow()) }) == std::cmp::Ordering::Greater);
                    break 'switch;
                }
                _ => {}
            }
        };
    }
    {
        const _: () = assert!(
            true,
            "std::strong_ordering::less == std::strong_ordering::less"
        );;
        const _: () = assert!(
            true,
            "std::strong_ordering::less != std::strong_ordering::equal"
        );;
        const _: () = assert!(
            true,
            "std::strong_ordering::less != std::strong_ordering::greater"
        );;
        const _: () = assert!(
            true,
            "std::strong_ordering::equal != std::strong_ordering::less"
        );;
        const _: () = assert!(
            true,
            "std::strong_ordering::equal == std::strong_ordering::equal"
        );;
        const _: () = assert!(
            true,
            "std::strong_ordering::equal != std::strong_ordering::greater"
        );;
        const _: () = assert!(
            true,
            "std::strong_ordering::greater != std::strong_ordering::less"
        );;
        const _: () = assert!(
            true,
            "std::strong_ordering::greater != std::strong_ordering::equal"
        );;
        const _: () = assert!(
            true,
            "std::strong_ordering::greater ==\n                  std::strong_ordering::greater"
        );;
    }
    ({ test_equality_5() });
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(5)]
pub struct anon_8 {
    #[offset(0)]
    #[byte_size(1)]
    pub Value: std::cmp::Ordering,
    #[offset(1)]
    pub ExpectEq: bool,
    #[offset(2)]
    pub ExpectNeq: bool,
    #[offset(3)]
    pub ExpectLess: bool,
    #[offset(4)]
    pub ExpectGreater: bool,
}
impl CppMove for anon_8 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type ExpectRes_9 = u32;
pub const ExpectRes_9_ER_Greater: ExpectRes_9 = 0;
pub const ExpectRes_9_ER_Less: ExpectRes_9 = 1;
pub const ExpectRes_9_ER_Equiv: ExpectRes_9 = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_10 {
    #[offset(0)]
    #[byte_size(1)]
    pub Value: std::cmp::Ordering,
    #[offset(4)]
    pub Expect: ExpectRes_9,
}
impl CppMove for anon_10 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
    ({ test_static_members_3() });
    ({ test_signatures_4() });
    ({ test_equality_5() });
    const _: () = assert!(true, "test_conversion()");;
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = sink_2.with(|_| ());
}
