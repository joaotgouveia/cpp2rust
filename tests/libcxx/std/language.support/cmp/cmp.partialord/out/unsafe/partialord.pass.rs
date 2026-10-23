extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
pub unsafe fn DoNotOptimize_1(value: *const *const ()) -> *const *const () {
    c"";
    return value;
}
pub static mut sink_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn test_static_members_3() {
    (unsafe {
        let mut _value: *const () = (&raw const (*unimplemented!()) as *const ());
        DoNotOptimize_1(&mut _value)
    });
    (unsafe {
        let mut _value: *const () = (&raw const (*unimplemented!()) as *const ());
        DoNotOptimize_1(&mut _value)
    });
    (unsafe {
        let mut _value: *const () = (&raw const (*unimplemented!()) as *const ());
        DoNotOptimize_1(&mut _value)
    });
    (unsafe {
        let mut _value: *const () = (&raw const (*unimplemented!()) as *const ());
        DoNotOptimize_1(&mut _value)
    });
}
pub unsafe fn test_signatures_4() {
    let Eq: *const () = &(*unimplemented!());
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
    let PartialEq: *const () = &(*unimplemented!());
    let WeakEq: *const () = &(*unimplemented!());
    unimplemented!();
    let StrongEq: *const () = &(*unimplemented!());
    unimplemented!();
}
pub unsafe fn test_constexpr_6() -> () {
    let Eq: *const () = &(*unimplemented!());
    let Less: *const () = &(*unimplemented!());
    let Greater: *const () = &(*unimplemented!());
    let Unord: *const () = &(*unimplemented!());
    let mut TestCases: [anon_7; 4] = [
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
    ];
    'loop_: for TC in IterRange::new(
        TestCases.as_mut_ptr(),
        TestCases.as_mut_ptr().offset((4_()) as isize),
    ) {
        let mut TC: anon_7 = (*TC).clone();
        let mut V: () = unimplemented!();
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
        let mut res: () = (unimplemented!());
        (&(res));
        res = (unimplemented!());
        (&(res));
    };
    let mut SpaceshipTestCases: [anon_9; 4] = [
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
    ];
    'loop_: for TC in IterRange::new(
        SpaceshipTestCases.as_mut_ptr(),
        SpaceshipTestCases.as_mut_ptr().offset((4_()) as isize),
    ) {
        let mut TC: anon_9 = (*TC).clone();
        let mut Res: () = (unimplemented!());
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
    (unsafe { test_equality_5() });
    return true;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct anon_7 {
    pub Value: (),
    pub ExpectEq: (),
    pub ExpectNeq: (),
    pub ExpectLess: (),
    pub ExpectGreater: (),
}
pub type ExpectRes_8 = ();
pub const ExpectRes_8_ER_Greater: ExpectRes_8 = 0;
pub const ExpectRes_8_ER_Less: ExpectRes_8 = 1;
pub const ExpectRes_8_ER_Equiv: ExpectRes_8 = 2;
pub const ExpectRes_8_ER_Unord: ExpectRes_8 = 3;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct anon_9 {
    pub Value: (),
    pub Expect: ExpectRes_8,
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_static_members_3() });
    (unsafe { test_signatures_4() });
    (unsafe { test_equality_5() });
    const _: () = assert!(true, "test_constexpr()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sink_2);
}
