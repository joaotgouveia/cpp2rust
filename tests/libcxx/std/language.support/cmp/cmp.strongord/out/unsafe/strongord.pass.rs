extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {}
pub unsafe fn DoNotOptimize_1(
    value: *const *const std::cmp::Ordering,
) -> *const *const std::cmp::Ordering {
    c"";
    return value;
}
pub static mut sink_2: std::cell::LazyCell<*const ::libc::c_void> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
pub unsafe fn test_static_members_3() {
    (unsafe {
        let mut _value: *const std::cmp::Ordering =
            (&raw const std::cmp::Ordering::Less as *const std::cmp::Ordering);
        DoNotOptimize_1(&mut _value)
    });
    (unsafe {
        let mut _value: *const std::cmp::Ordering =
            (&raw const std::cmp::Ordering::Equal as *const std::cmp::Ordering);
        DoNotOptimize_1(&mut _value)
    });
    (unsafe {
        let mut _value: *const std::cmp::Ordering =
            (&raw const std::cmp::Ordering::Equal as *const std::cmp::Ordering);
        DoNotOptimize_1(&mut _value)
    });
    (unsafe {
        let mut _value: *const std::cmp::Ordering =
            (&raw const std::cmp::Ordering::Greater as *const std::cmp::Ordering);
        DoNotOptimize_1(&mut _value)
    });
}
pub unsafe fn test_signatures_4() {
    let Eq: *const std::cmp::Ordering = &std::cmp::Ordering::Equal;
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
pub unsafe fn test_equality_5() {
    let StrongEq: *const std::cmp::Ordering = &std::cmp::Ordering::Equal;
    let PartialEq: *const () = &(*unimplemented!());
    assert!(unimplemented!());
    let WeakEq: *const () = &(*unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn test_conversion_6() -> bool {
    const _: () = assert!(
        true,
        "std::is_convertible<const std::strong_ordering&,\n      std::partial_ordering>::value"
    );;
    {
        let mut V: std::cmp::Ordering = std::cmp::Ordering::Equal;
        let mut WV: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut V: std::cmp::Ordering = std::cmp::Ordering::Less;
        let mut WV: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut V: std::cmp::Ordering = std::cmp::Ordering::Greater;
        let mut WV: () = unimplemented!();
        assert!(unimplemented!());
    }
    const _: () = assert!(
        true,
        "std::is_convertible<const std::strong_ordering&,\n      std::weak_ordering>::value"
    );;
    {
        let mut V: std::cmp::Ordering = std::cmp::Ordering::Equal;
        let mut WV: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut V: std::cmp::Ordering = std::cmp::Ordering::Less;
        let mut WV: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut V: std::cmp::Ordering = std::cmp::Ordering::Greater;
        let mut WV: () = unimplemented!();
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_constexpr_7() -> bool {
    let Eq: *const std::cmp::Ordering = &std::cmp::Ordering::Equal;
    let Equiv: *const std::cmp::Ordering = &std::cmp::Ordering::Equal;
    let Less: *const std::cmp::Ordering = &std::cmp::Ordering::Less;
    let Greater: *const std::cmp::Ordering = &std::cmp::Ordering::Greater;
    let mut TestCases: [anon_8; 4] = [
        anon_8 {
            Value: (*Eq),
            ExpectEq: true,
            ExpectNeq: false,
            ExpectLess: false,
            ExpectGreater: false,
        },
        anon_8 {
            Value: (*Equiv),
            ExpectEq: true,
            ExpectNeq: false,
            ExpectLess: false,
            ExpectGreater: false,
        },
        anon_8 {
            Value: (*Less),
            ExpectEq: false,
            ExpectNeq: true,
            ExpectLess: true,
            ExpectGreater: false,
        },
        anon_8 {
            Value: (*Greater),
            ExpectEq: false,
            ExpectNeq: true,
            ExpectLess: false,
            ExpectGreater: true,
        },
    ];
    'loop_: for TC in IterRange::new(
        TestCases.as_mut_ptr(),
        TestCases.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let mut TC: anon_8 = (*TC);
        let mut V: std::cmp::Ordering = TC.Value;
        assert!((((V == std::cmp::Ordering::Equal) as i32) == (TC.ExpectEq as i32)));
        assert!((((V == std::cmp::Ordering::Equal) as i32) == (TC.ExpectEq as i32)));
        assert!((((!(V == std::cmp::Ordering::Equal)) as i32) == (TC.ExpectNeq as i32)));
        assert!((((!(V == std::cmp::Ordering::Equal)) as i32) == (TC.ExpectNeq as i32)));
        assert!((((V == std::cmp::Ordering::Less) as i32) == (TC.ExpectLess as i32)));
        assert!((((V == std::cmp::Ordering::Greater) as i32) == (TC.ExpectGreater as i32)));
        assert!(
            (((V != std::cmp::Ordering::Greater) as i32)
                == (((TC.ExpectLess) || (TC.ExpectEq)) as i32))
        );
        assert!(
            (((V != std::cmp::Ordering::Less) as i32)
                == (((TC.ExpectGreater) || (TC.ExpectEq)) as i32))
        );
        assert!((((unimplemented!()) as i32) == (TC.ExpectGreater as i32)));
        assert!((((unimplemented!()) as i32) == (TC.ExpectLess as i32)));
        assert!((((unimplemented!()) as i32) == (((TC.ExpectGreater) || (TC.ExpectEq)) as i32)));
        assert!((((unimplemented!()) as i32) == (((TC.ExpectLess) || (TC.ExpectEq)) as i32)));
    }
    {
        let mut res: std::cmp::Ordering = (unimplemented!());
        (&(res));
        unimplemented!();
        (&(res));
    };
    let mut SpaceshipTestCases: [anon_10; 3] = [
        anon_10 {
            Value: std::cmp::Ordering::Equal,
            Expect: ExpectRes_9_ER_Equiv,
        },
        anon_10 {
            Value: std::cmp::Ordering::Less,
            Expect: ExpectRes_9_ER_Less,
        },
        anon_10 {
            Value: std::cmp::Ordering::Greater,
            Expect: ExpectRes_9_ER_Greater,
        },
    ];
    'loop_: for TC in IterRange::new(
        SpaceshipTestCases.as_mut_ptr(),
        SpaceshipTestCases.as_mut_ptr().offset((3_i64) as isize),
    ) {
        let mut TC: anon_10 = (*TC);
        let mut Res: std::cmp::Ordering = (unimplemented!());
        'switch: {
            match { (TC.Expect as i32) } {
                __v if __v == (ExpectRes_9_ER_Equiv as i32) => {
                    assert!(Res == std::cmp::Ordering::Equal);
                    assert!(Res == std::cmp::Ordering::Equal);
                    break 'switch;
                }
                __v if __v == (ExpectRes_9_ER_Less as i32) => {
                    assert!(Res == std::cmp::Ordering::Less);
                    break 'switch;
                }
                __v if __v == (ExpectRes_9_ER_Greater as i32) => {
                    assert!(Res == std::cmp::Ordering::Greater);
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
    (unsafe { test_equality_5() });
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_8 {
    pub Value: std::cmp::Ordering,
    pub ExpectEq: bool,
    pub ExpectNeq: bool,
    pub ExpectLess: bool,
    pub ExpectGreater: bool,
}
pub type ExpectRes_9 = u32;
pub const ExpectRes_9_ER_Greater: ExpectRes_9 = 0;
pub const ExpectRes_9_ER_Less: ExpectRes_9 = 1;
pub const ExpectRes_9_ER_Equiv: ExpectRes_9 = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_10 {
    pub Value: std::cmp::Ordering,
    pub Expect: ExpectRes_9,
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_static_members_3() });
    (unsafe { test_signatures_4() });
    (unsafe { test_equality_5() });
    const _: () = assert!(true, "test_conversion()");;
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sink_2);
}
